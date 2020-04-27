use nom::bytes::complete::{tag, take_until};

// TODO: Implement more expression types.
// HACK: Rust mistakenly doesn’t realise that the variants of this enum are actually used.
#[allow(dead_code)]
#[derive(Debug, PartialEq)]
pub(crate) enum Expr<'text> {
    String {
        start_quote: &'text str,
        contents: &'text str,
        end_quote: &'text str,
    },
}

impl<'text> Expr<'text> {
    pub(crate) fn new(s: &'text str) -> nom::IResult<&'text str, Self> {
        let (s, start_quote) = tag("\"")(s)?;
        let (s, contents) = take_until("\"")(s)?;
        let (s, end_quote) = tag("\"")(s)?;

        Ok((
            s,
            Self::String {
                start_quote,
                contents,
                end_quote,
            },
        ))
    }
}

impl<'e> From<Expr<'e>> for Vec<syntax::HighlightedSpan<'e>> {
    fn from(expr: Expr<'e>) -> Self {
        match expr {
            Expr::String {
                start_quote,
                contents,
                end_quote,
            } => vec![
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
            ],
        }
    }
}
