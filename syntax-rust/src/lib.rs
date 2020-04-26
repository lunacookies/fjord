//! Provides [`RustHighlighter`](struct.RustHighlighter.html).

#![warn(missing_debug_implementations, missing_docs, rust_2018_idioms)]

mod function_param;
mod function_return_type;
mod generics;
mod ident;
mod lifetime;
mod path;
mod ty;
mod ty_ident;

pub(crate) use {
    function_param::FunctionParam, function_return_type::FunctionReturnType, generics::Generics,
    ident::Ident, lifetime::Lifetime, path::Path, ty::Ty, ty_ident::TyIdent,
};

use nom::{
    branch::alt,
    bytes::complete::{tag, take, take_till1, take_while, take_while1},
    combinator::{map, opt},
    multi::{many0, many1},
};

/// Highlights Rust code.
#[derive(Debug)]
pub struct RustHighlighter;

impl syntax::Highlight for RustHighlighter {
    fn highlight<'input>(&self, input: &'input str) -> Vec<syntax::HighlightedSpan<'input>> {
        // FIXME: At the moment, we just don’t highlight if highlighting fails. Ideally,
        // highlighting should always succeed, i.e. it should be fault-tolerant.
        match many1(Item::new)(input) {
            Ok((s, items)) => items
                .into_iter()
                // Convert each item into a vector of HighlightedSpans.
                .map(Vec::from)
                .flatten()
                // Add on remaining text that couldn’t be parsed.
                .chain(std::iter::once(syntax::HighlightedSpan {
                    text: s,
                    group: None,
                }))
                .collect(),
            // If the parser failed, then return the unhighlighted input.
            Err(_) => vec![syntax::HighlightedSpan {
                text: input,
                group: None,
            }],
        }
    }
}

// HACK: Rust mistakenly doesn’t realise that the variants of this enum are actually used.
#[allow(dead_code)]
#[derive(Debug, PartialEq)]
enum Item<'input> {
    Use {
        keyword: &'input str,
        keyword_space: &'input str,
        path: Path<'input>,
        semicolon: Option<&'input str>,
    },
    Function {
        keyword: &'input str,
        keyword_space: &'input str,
        name: Ident<'input>,
        name_space: &'input str,
        open_paren: &'input str,
        open_paren_space: &'input str,
        params: Vec<FunctionParam<'input>>,
        params_space: &'input str,
        close_paren_space: &'input str,
        close_paren: &'input str,
        return_type: Option<FunctionReturnType<'input>>,
    },
    Whitespace {
        text: &'input str,
    },
    Error {
        text: &'input str,
    },
}

impl<'input> Item<'input> {
    fn new(s: &'input str) -> nom::IResult<&'input str, Self> {
        alt((
            Self::new_use,
            Self::new_function,
            Self::new_whitespace,
            Self::new_error,
        ))(s)
    }

    fn new_use(s: &'input str) -> nom::IResult<&'input str, Self> {
        let (s, keyword) = tag("use")(s)?;
        let (s, keyword_space) = take_whitespace1(s)?;

        let (s, path) = Path::new(s)?;
        let (s, semicolon) = opt(tag(";"))(s)?;

        Ok((
            s,
            Self::Use {
                keyword,
                keyword_space,
                path,
                semicolon,
            },
        ))
    }

    fn new_function(s: &'input str) -> nom::IResult<&'input str, Self> {
        let (s, keyword) = tag("fn")(s)?;
        let (s, keyword_space) = take_whitespace1(s)?;

        let (s, name) = Ident::new(s)?;
        let (s, name_space) = take_whitespace0(s)?;

        let (s, open_paren) = tag("(")(s)?;
        let (s, open_paren_space) = take_whitespace0(s)?;

        let (s, params) = many0(FunctionParam::new)(s)?;
        let (s, params_space) = take_whitespace0(s)?;

        let (s, close_paren) = tag(")")(s)?;
        let (s, close_paren_space) = take_whitespace0(s)?;

        let (s, return_type) = opt(FunctionReturnType::new)(s)?;

        Ok((
            s,
            Self::Function {
                keyword,
                keyword_space,
                name,
                name_space,
                open_paren,
                open_paren_space,
                params,
                params_space,
                close_paren_space,
                close_paren,
                return_type,
            },
        ))
    }

    fn new_whitespace(s: &'input str) -> nom::IResult<&'input str, Self> {
        map(take_whitespace1, |s| Self::Whitespace { text: s })(s)
    }

    fn new_error(s: &'input str) -> nom::IResult<&'input str, Self> {
        map(
            alt((
                // ‘Reset’ errors after any of these characters.
                take_till1(|c| c == '}' || c == ')' || c == ';' || c == '\n'),
                // This will fail, however, if the input starts with any of these ‘reset’
                // characters. In the case that this fails, we simply take a single chara
                take(1usize),
            )),
            |s| Self::Error { text: s },
        )(s)
    }
}

impl<'input> From<Item<'input>> for Vec<syntax::HighlightedSpan<'input>> {
    fn from(item: Item<'input>) -> Self {
        let spans = match item {
            Item::Use {
                keyword,
                keyword_space,
                path,
                semicolon,
            } => {
                let mut output = vec![
                    syntax::HighlightedSpan {
                        text: keyword,
                        group: Some(syntax::HighlightGroup::Keyword),
                    },
                    syntax::HighlightedSpan {
                        text: keyword_space,
                        group: None,
                    },
                ];

                output.append(&mut path.into());

                if let Some(semicolon) = semicolon {
                    output.push(syntax::HighlightedSpan {
                        text: semicolon,
                        group: Some(syntax::HighlightGroup::Terminator),
                    });
                }

                output
            }
            Item::Function {
                keyword,
                keyword_space,
                name,
                name_space,
                open_paren,
                open_paren_space,
                params,
                params_space,
                close_paren_space,
                close_paren,
                return_type,
            } => {
                let mut output = vec![
                    syntax::HighlightedSpan {
                        text: keyword,
                        group: Some(syntax::HighlightGroup::Keyword),
                    },
                    syntax::HighlightedSpan {
                        text: keyword_space,
                        group: None,
                    },
                    syntax::HighlightedSpan {
                        text: name.name,
                        group: Some(syntax::HighlightGroup::Function),
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
                ];

                output.extend(
                    params
                        .into_iter()
                        .map(Vec::from)
                        .flatten()
                        .chain(std::iter::once(syntax::HighlightedSpan {
                            text: params_space,
                            group: None,
                        }))
                        .chain(std::iter::once(syntax::HighlightedSpan {
                            text: close_paren,
                            group: Some(syntax::HighlightGroup::Delimiter),
                        }))
                        .chain(std::iter::once(syntax::HighlightedSpan {
                            text: close_paren_space,
                            group: None,
                        })),
                );

                if let Some(return_type) = return_type {
                    output.append(&mut Vec::from(return_type));
                }

                output
            }
            Item::Whitespace { text } => vec![syntax::HighlightedSpan { text, group: None }],
            Item::Error { text } => vec![syntax::HighlightedSpan {
                text,
                group: Some(syntax::HighlightGroup::Error),
            }],
        };

        spans
    }
}

// TODO: Rust uses ‘Pattern_White_Space’, not ASCII whitespace.
fn is_whitespace(c: char) -> bool {
    c.is_ascii_whitespace()
}

fn take_whitespace0(s: &str) -> nom::IResult<&str, &str> {
    take_while(is_whitespace)(s)
}

fn take_whitespace1(s: &str) -> nom::IResult<&str, &str> {
    take_while1(is_whitespace)(s)
}
