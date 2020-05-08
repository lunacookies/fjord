//! Provides highlighting for Rust code.

#![warn(missing_debug_implementations, missing_docs, rust_2018_idioms)]

mod block;
mod bounds;
mod expr;
mod generics;
mod item;
mod lifetime;
mod module_name;
mod parser;
mod path;
mod pattern;
mod statement;
mod trait_;
mod ty;
mod ty_name;
mod utils;

use {
    block::parse as block, bounds::parse as bounds, expr::parse as expr, item::parse as item,
    module_name::parse as module_name, path::parse as path, pattern::parse as pattern,
    statement::parse as statement, trait_::parse as trait_, ty::parse as ty,
    ty_name::parse as ty_name,
};

/// Highlights Rust code.
#[derive(Debug, PartialEq)]
pub struct RustHighlighter;

impl syntax::Highlight for RustHighlighter {
    fn highlight<'input>(&self, input: &'input str) -> Vec<syntax::HighlightedSpan<'input>> {
        let (_remaining_input, spans) = nom::multi::many0(parser::parse)(input).unwrap();
        spans.into_iter().flatten().collect()
    }
}

type ParseResult<'text> = nom::IResult<&'text str, Vec<syntax::HighlightedSpan<'text>>>;
