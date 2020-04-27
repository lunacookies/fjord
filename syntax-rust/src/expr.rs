use nom::{
    branch::alt,
    bytes::complete::{tag, take, take_until},
    combinator::map,
    multi::many0,
    sequence::pair,
};

// TODO: Implement more expression types.
// HACK: Rust mistakenly doesn’t realise that the variants of this enum are actually used.
#[allow(dead_code)]
#[derive(Debug, PartialEq)]
pub(crate) enum Expr<'text> {
    Variable {
        name: crate::Ident<'text>,
    },
    FunctionCall {
        name: crate::Ident<'text>,
        name_space: &'text str,
        open_paren: &'text str,
        open_paren_space: &'text str,
        // The second item in this tuple is whitespace.
        params: Vec<(crate::FunctionCallParam<'text>, &'text str)>,
        params_space: &'text str,
        close_paren: &'text str,
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
        alt((
            Self::new_function_call,
            Self::new_variable,
            Self::new_character,
            Self::new_string,
        ))(s)
    }

    fn new_function_call(s: &'text str) -> nom::IResult<&'text str, Self> {
        let (s, name) = crate::Ident::new(s)?;
        let (s, name_space) = crate::take_whitespace0(s)?;

        let (s, open_paren) = tag("(")(s)?;
        let (s, open_paren_space) = crate::take_whitespace0(s)?;

        let (s, params) = many0(pair(crate::FunctionCallParam::new, crate::take_whitespace0))(s)?;
        let (s, params_space) = crate::take_whitespace0(s)?;

        let (s, close_paren) = tag(")")(s)?;

        Ok((
            s,
            Self::FunctionCall {
                name,
                name_space,
                open_paren,
                open_paren_space,
                params,
                params_space,
                close_paren,
            },
        ))
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
            Expr::FunctionCall {
                name,
                name_space,
                open_paren,
                open_paren_space,
                params,
                params_space,
                close_paren,
            } => std::iter::once(syntax::HighlightedSpan {
                text: name.name,
                group: Some(syntax::HighlightGroup::FunctionCall),
            })
            .chain(std::iter::once(syntax::HighlightedSpan {
                text: name_space,
                group: None,
            }))
            .chain(std::iter::once(syntax::HighlightedSpan {
                text: open_paren,
                group: Some(syntax::HighlightGroup::Delimiter),
            }))
            .chain(std::iter::once(syntax::HighlightedSpan {
                text: open_paren_space,
                group: None,
            }))
            .chain(
                params
                    .into_iter()
                    .map(|(param, space)| {
                        Vec::from(param).into_iter().chain(std::iter::once(
                            syntax::HighlightedSpan {
                                text: space,
                                group: None,
                            },
                        ))
                    })
                    .flatten(),
            )
            .chain(std::iter::once(syntax::HighlightedSpan {
                text: params_space,
                group: None,
            }))
            .chain(std::iter::once(syntax::HighlightedSpan {
                text: close_paren,
                group: Some(syntax::HighlightGroup::Delimiter),
            }))
            .collect(),
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
