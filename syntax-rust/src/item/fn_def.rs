use {
    crate::{
        utils::{comma_separated, snake_case, take_whitespace0, take_whitespace1},
        ParseResult,
    },
    nom::{
        branch::alt,
        bytes::complete::tag,
        combinator::{map, opt},
    },
};

const PARAMS_START: &str = "(";
const PARAMS_END: &str = ")";

pub(super) fn parse(s: &str) -> ParseResult<'_> {
    let (s, keyword) = tag("fn")(s)?;
    let (s, keyword_space) = take_whitespace1(s)?;

    let (s, name) = snake_case(s)?;
    let (s, name_space) = take_whitespace0(s)?;

    let (s, generics) = opt(crate::generics::def)(s)?;
    let (s, generics_space) = take_whitespace0(s)?;

    let (s, open_paren) = tag(PARAMS_START)(s)?;
    let (s, open_paren_space) = take_whitespace0(s)?;

    let (s, mut params) = comma_separated(&param, PARAMS_END)(s)?;

    let (s, close_paren) = tag(PARAMS_END)(s)?;
    let (s, close_paren_space) = take_whitespace0(s)?;

    let (s, return_type) = opt(return_type)(s)?;
    let (s, return_type_space) = take_whitespace0(s)?;

    let semicolon = map(tag(";"), |s| {
        vec![syntax::HighlightedSpan {
            text: s,
            group: Some(syntax::HighlightGroup::Terminator),
        }]
    });

    // Function bodies can be either a block expression, or simply a semicolon (as in traits).
    let (s, mut body) = alt((crate::block, semicolon))(s)?;

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
    ];

    if let Some(mut generics) = generics {
        output.append(&mut generics);
    }

    output.extend_from_slice(&[
        syntax::HighlightedSpan {
            text: generics_space,
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
    ]);

    output.append(&mut params);

    output.extend_from_slice(&[
        syntax::HighlightedSpan {
            text: close_paren,
            group: Some(syntax::HighlightGroup::Delimiter),
        },
        syntax::HighlightedSpan {
            text: close_paren_space,
            group: None,
        },
    ]);

    if let Some(mut return_type) = return_type {
        output.append(&mut return_type);
    }

    output.push(syntax::HighlightedSpan {
        text: return_type_space,
        group: None,
    });

    output.append(&mut body);

    Ok((s, output))
}

fn param(s: &str) -> ParseResult<'_> {
    let (s, name) = snake_case(s)?;
    let (s, name_space) = take_whitespace0(s)?;

    let (s, colon) = tag(":")(s)?;
    let (s, colon_space) = take_whitespace0(s)?;

    let (s, mut ty) = crate::ty(s)?;

    let mut output = vec![
        syntax::HighlightedSpan {
            text: name,
            group: Some(syntax::HighlightGroup::FunctionParam),
        },
        syntax::HighlightedSpan {
            text: name_space,
            group: None,
        },
        syntax::HighlightedSpan {
            text: colon,
            group: Some(syntax::HighlightGroup::Separator),
        },
        syntax::HighlightedSpan {
            text: colon_space,
            group: None,
        },
    ];

    output.append(&mut ty);

    Ok((s, output))
}

fn return_type(s: &str) -> ParseResult<'_> {
    let (s, arrow) = tag("->")(s)?;
    let (s, arrow_space) = take_whitespace0(s)?;

    let (s, mut ty) = crate::ty(s)?;

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
