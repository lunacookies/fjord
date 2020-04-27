use nom::{
    branch::alt,
    bytes::complete::{tag, take, take_until},
    combinator::map,
};

// TODO: Implement more expression types.
// HACK: Rust mistakenly doesn’t realise that the variants of this enum are actually used.
#[allow(dead_code)]
#[derive(Debug, PartialEq)]
pub(crate) enum Expr<'text> {
    Variable {
        name: crate::Ident<'text>,
    },
    Character {
        start_quote: &'text str,
        contents: &'text str,
        end_quote: &'text str,
    },
    String {
        start_quote: &'text str,
        contents: &'text str,
        end_quote: &'text str,
    },
}

impl<'text> Expr<'text> {
    pub(crate) fn new(s: &'text str) -> nom::IResult<&'text str, Self> {
        alt((Self::new_variable, Self::new_character, Self::new_string))(s)
    }

    fn new_variable(s: &'text str) -> nom::IResult<&'text str, Self> {
        map(crate::Ident::new, |name| Self::Variable { name })(s)
    }

    fn new_character(s: &'text str) -> nom::IResult<&'text str, Self> {
        let (s, start_quote) = tag("'")(s)?;
        let (s, contents) = take(1usize)(s)?;
        let (s, end_quote) = tag("'")(s)?;

        Ok((
            s,
            Self::Character {
                start_quote,
                contents,
                end_quote,
            },
        ))
    }

    fn new_string(s: &'text str) -> nom::IResult<&'text str, Self> {
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
            Expr::Variable { name } => vec![syntax::HighlightedSpan {
                text: name.name,
                group: Some(syntax::HighlightGroup::VariableUse),
            }],
            Expr::Character {
                start_quote,
                contents,
                end_quote,
            } => vec![
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
            ],
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
