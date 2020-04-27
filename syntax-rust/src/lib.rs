//! Provides [`RustHighlighter`](struct.RustHighlighter.html).

#![warn(missing_debug_implementations, missing_docs, rust_2018_idioms)]

mod block;
mod expr;
mod function_call_param;
mod function_def_param;
mod function_return_type;
mod generics;
mod ident;
mod item;
mod lifetime;
mod path;
mod pattern;
mod statement;
mod ty;
mod ty_ident;

pub(crate) use {
    block::Block, expr::Expr, function_call_param::FunctionCallParam,
    function_def_param::FunctionDefParam, function_return_type::FunctionReturnType,
    generics::Generics, ident::Ident, item::Item, lifetime::Lifetime, path::Path, pattern::Pattern,
    statement::Statement, ty::Ty, ty_ident::TyIdent,
};

use nom::{
    branch::alt,
    bytes::complete::{take, take_till1, take_while, take_while1},
    combinator::map,
    multi::many1,
};

/// Highlights Rust code.
#[derive(Debug)]
pub struct RustHighlighter;

impl syntax::Highlight for RustHighlighter {
    fn highlight<'input>(&self, input: &'input str) -> Vec<syntax::HighlightedSpan<'input>> {
        // FIXME: At the moment, we just don’t highlight if highlighting fails. Ideally,
        // highlighting should always succeed, i.e. it should be fault-tolerant.
        match many1(Thing::new)(input) {
            Ok((s, things)) => things
                .into_iter()
                // Convert each thing into a vector of HighlightedSpans.
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
enum Thing<'text> {
    Item(Item<'text>),
    Whitespace { text: &'text str },
    Error { text: &'text str },
}

impl<'text> Thing<'text> {
    fn new(s: &'text str) -> nom::IResult<&'text str, Self> {
        alt((Self::new_item, Self::new_whitespace, Self::new_error))(s)
    }

    fn new_item(s: &'text str) -> nom::IResult<&'text str, Self> {
        map(Item::new, Self::Item)(s)
    }

    fn new_whitespace(s: &'text str) -> nom::IResult<&'text str, Self> {
        map(crate::take_whitespace1, |s| Self::Whitespace { text: s })(s)
    }

    fn new_error(s: &'text str) -> nom::IResult<&'text str, Self> {
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

impl<'t> From<Thing<'t>> for Vec<syntax::HighlightedSpan<'t>> {
    fn from(thing: Thing<'t>) -> Self {
        match thing {
            Thing::Item(item) => Vec::from(item),
            Thing::Whitespace { text } => vec![syntax::HighlightedSpan { text, group: None }],
            Thing::Error { text } => vec![syntax::HighlightedSpan {
                text,
                group: Some(syntax::HighlightGroup::Error),
            }],
        }
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
