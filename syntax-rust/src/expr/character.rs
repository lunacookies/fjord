use {
    crate::ParseResult,
    nom::bytes::complete::{tag, take},
};

pub(super) fn parse(s: &str) -> ParseResult<'_> {
    let (s, start_quote) = tag("'")(s)?;
    let (s, contents) = take(1usize)(s)?;
    let (s, end_quote) = tag("'")(s)?;

    let output = vec![
        syntax::HighlightedSpan {
            text: start_quote,
            group: Some(syntax::HighlightGroup::CharacterDelimiter),
        },
        syntax::HighlightedSpan {
            text: contents,
            group: Some(syntax::HighlightGroup::Character),
        },
        syntax::HighlightedSpan {
            text: end_quote,
            group: Some(syntax::HighlightGroup::CharacterDelimiter),
        },
    ];

    Ok((s, output))
}
