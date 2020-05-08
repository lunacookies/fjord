use {
    crate::ParseResult,
    nom::bytes::complete::{tag, take_until},
};

pub(super) fn parse(s: &str) -> ParseResult<'_> {
    let (s, start_quote) = tag("\"")(s)?;
    let (s, contents) = take_until("\"")(s)?;
    let (s, end_quote) = tag("\"")(s)?;

    let output = vec![
        syntax::HighlightedSpan {
            text: start_quote,
            group: Some(syntax::HighlightGroup::StringDelimiter),
        },
        syntax::HighlightedSpan {
            text: contents,
            group: Some(syntax::HighlightGroup::String),
        },
        syntax::HighlightedSpan {
            text: end_quote,
            group: Some(syntax::HighlightGroup::StringDelimiter),
        },
    ];

    Ok((s, output))
}
