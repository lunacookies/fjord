//! Provides [`RustHighlighter`](struct.RustHighlighter.html).

#![warn(missing_debug_implementations, missing_docs, rust_2018_idioms)]

mod ident;
mod path;

pub(crate) use {ident::Ident, path::Path};

use nom::{
    branch::alt,
    bytes::complete::{tag, take_while, take_while1},
    combinator::opt,
};

/// Highlights Rust code.
#[derive(Debug)]
pub struct RustHighlighter;

impl syntax::Highlight for RustHighlighter {
    fn highlight<'input>(&self, input: &'input str) -> Vec<syntax::HighlightedSpan<'input>> {
        // FIXME: At the moment, we just don’t highlight if highlighting fails. Ideally,
        // highlighting should always succeed, i.e. it should be fault-tolerant.
        Item::new(input).map_or(
            vec![syntax::HighlightedSpan {
                text: input,
                group: None,
            }],
            |(s, item)| {
                let mut spans: Vec<_> = item.into();
                spans.push(syntax::HighlightedSpan {
                    text: s,
                    group: None,
                });

                spans
            },
        )
    }
}

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
    },
}

impl<'input> Item<'input> {
    fn new(s: &'input str) -> nom::IResult<&'input str, Self> {
        alt((Self::new_use, Self::new_function))(s)
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

        Ok((
            s,
            Self::Function {
                keyword,
                keyword_space,
                name,
            },
        ))
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
                        group: None,
                    });
                }

                output
            }
            Item::Function {
                keyword,
                keyword_space,
                name,
            } => vec![
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
            ],
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
