mod named_structure_def_fields;

use {
    crate::utils::{
        ident, {take_whitespace0, take_whitespace1},
    },
    named_structure_def_fields::fields as named_structure_def_fields,
    nom::{
        branch::alt,
        bytes::complete::{tag, take},
        combinator::{map, opt},
    },
};

type ParseResult<'text> = nom::IResult<&'text str, Vec<syntax::HighlightedSpan<'text>>>;

pub(crate) fn parse(s: &str) -> ParseResult<'_> {
    alt((item, whitespace, error))(s)
}

fn item(s: &str) -> ParseResult<'_> {
    alt((function, structure_def))(s)
}

fn whitespace(s: &str) -> ParseResult<'_> {
    map(take_whitespace1, |s| {
        vec![syntax::HighlightedSpan {
            text: s,
            group: None,
        }]
    })(s)
}

fn error(s: &str) -> ParseResult<'_> {
    map(take(1usize), |s| {
        vec![syntax::HighlightedSpan {
            text: s,
            group: Some(syntax::HighlightGroup::Error),
        }]
    })(s)
}

fn function(s: &str) -> ParseResult<'_> {
    let (s, keyword) = tag("fn")(s)?;
    let (s, keyword_space) = take_whitespace1(s)?;

    let (s, name) = ident(s)?;
    let (s, name_space) = take_whitespace0(s)?;

    let (s, open_paren) = tag("(")(s)?;
    let (s, open_paren_space) = take_whitespace0(s)?;

    let (s, close_paren) = tag(")")(s)?;
    let (s, close_paren_space) = take_whitespace0(s)?;

    let (s, return_type) = opt(function_return_type)(s)?;

    let mut output = vec![
        syntax::HighlightedSpan {
            text: keyword,
            group: Some(syntax::HighlightGroup::OtherKeyword),
        },
        syntax::HighlightedSpan {
            text: keyword_space,
            group: None,
        },
        syntax::HighlightedSpan {
            text: name,
            group: Some(syntax::HighlightGroup::FunctionDef),
        },
        syntax::HighlightedSpan {
            text: name_space,
            group: None,
        },
        syntax::HighlightedSpan {
            text: open_paren,
            group: Some(syntax::HighlightGroup::Delimiter),
        },
        syntax::HighlightedSpan {
            text: open_paren_space,
            group: None,
        },
        syntax::HighlightedSpan {
            text: close_paren,
            group: Some(syntax::HighlightGroup::Delimiter),
        },
        syntax::HighlightedSpan {
            text: close_paren_space,
            group: None,
        },
    ];

    if let Some(mut return_type) = return_type {
        output.append(&mut return_type);
    }

    Ok((s, output))
}

fn function_return_type(s: &str) -> ParseResult<'_> {
    let (s, arrow) = tag("->")(s)?;
    let (s, arrow_space) = take_whitespace0(s)?;

    let (s, mut ty) = ty(s)?;

    let mut output = vec![
        syntax::HighlightedSpan {
            text: arrow,
            group: Some(syntax::HighlightGroup::Separator),
        },
        syntax::HighlightedSpan {
            text: arrow_space,
            group: None,
        },
    ];

    output.append(&mut ty);

    Ok((s, output))
}

fn structure_def(s: &str) -> ParseResult<'_> {
    let (s, keyword) = tag("struct")(s)?;
    let (s, keyword_space) = take_whitespace1(s)?;

    let (s, name) = ident(s)?;
    let (s, name_space) = take_whitespace0(s)?;

    let (s, mut fields) = structure_def_fields(s)?;

    let mut output = vec![
        syntax::HighlightedSpan {
            text: keyword,
            group: Some(syntax::HighlightGroup::OtherKeyword),
        },
        syntax::HighlightedSpan {
            text: keyword_space,
            group: None,
        },
        syntax::HighlightedSpan {
            text: name,
            group: Some(syntax::HighlightGroup::TyDef),
        },
        syntax::HighlightedSpan {
            text: name_space,
            group: None,
        },
    ];

    output.append(&mut fields);

    Ok((s, output))
}

fn structure_def_fields(s: &str) -> ParseResult<'_> {
    alt((named_structure_def_fields, unnamed_structure))(s)
}

fn unnamed_structure(s: &str) -> ParseResult<'_> {
    map(tag(";"), |semicolon| {
        vec![syntax::HighlightedSpan {
            text: semicolon,
            group: Some(syntax::HighlightGroup::Terminator),
        }]
    })(s)
}

fn ty(s: &str) -> ParseResult<'_> {
    map(ident, |s| {
        vec![syntax::HighlightedSpan {
            text: s,
            group: Some(syntax::HighlightGroup::TyUse),
        }]
    })(s)
}
