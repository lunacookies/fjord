use {
    crate::{utils::snake_case, ParseResult},
    nom::{bytes::complete::tag, combinator::map, sequence::pair},
};

pub(crate) fn usage(s: &str) -> ParseResult<'_> {
    map(pair(tag("'"), snake_case), |(tick, name)| {
        vec![
            syntax::HighlightedSpan {
                text: tick,
                group: Some(syntax::HighlightGroup::SpecialIdentUse),
            },
            syntax::HighlightedSpan {
                text: name,
                group: Some(syntax::HighlightGroup::SpecialIdentUse),
            },
        ]
    })(s)
}

pub(crate) fn def(s: &str) -> ParseResult<'_> {
    map(pair(tag("'"), snake_case), |(tick, name)| {
        vec![
            syntax::HighlightedSpan {
                text: tick,
                group: Some(syntax::HighlightGroup::SpecialIdentDef),
            },
            syntax::HighlightedSpan {
                text: name,
                group: Some(syntax::HighlightGroup::SpecialIdentDef),
            },
        ]
    })(s)
}
