//! Provides highlighting for Rust code.

#![warn(missing_debug_implementations, missing_docs, rust_2018_idioms)]

mod parser;
mod utils;

/// Highlights Rust code.
#[derive(Debug, PartialEq)]
pub struct RustHighlighter;

impl syntax::Highlight for RustHighlighter {
    fn highlight<'input>(&self, input: &'input str) -> Vec<syntax::HighlightedSpan<'input>> {
        let (_remaining_input, spans) = nom::multi::many0(parser::parse)(input).unwrap();
        spans.into_iter().flatten().collect()
    }
}
