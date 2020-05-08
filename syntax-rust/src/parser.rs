use {
    crate::{utils::take_whitespace1, ParseResult},
    nom::{
        branch::alt,
        bytes::complete::{take, take_till1},
        combinator::map,
    },
};

pub(crate) fn parse(s: &str) -> ParseResult<'_> {
    alt((crate::item, whitespace, error))(s)
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
    map(
        alt((
            // ‘Reset’ errors after any of these characters.
            take_till1(|c| c == '}' || c == ';'),
            // This will fail, however, if the input starts with any of these ‘reset’
            // characters. In that case, we simply take a single character.
            take(1usize),
        )),
        |s| {
            vec![syntax::HighlightedSpan {
                text: s,
                group: Some(syntax::HighlightGroup::Error),
            }]
        },
    )(s)
}
