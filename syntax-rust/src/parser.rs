use {
    crate::utils::{
        ident, {take_whitespace0, take_whitespace1},
    },
    nom::{
        branch::alt,
        bytes::complete::{tag, take},
        combinator::map,
    },
};

type ParseResult<'text> = nom::IResult<&'text str, Vec<syntax::HighlightedSpan<'text>>>;

pub(crate) fn parse(s: &str) -> ParseResult<'_> {
    alt((item, whitespace, error))(s)
}

fn item(s: &str) -> ParseResult<'_> {
    function(s)
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

    Ok((
        s,
        vec![
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
        ],
    ))
}
