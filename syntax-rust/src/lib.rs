//! Provides highlighting for Rust code.

#![warn(missing_debug_implementations, missing_docs, rust_2018_idioms)]

/// Highlights Rust code.
#[derive(Debug, PartialEq)]
pub struct RustHighlighter;

impl syntax::Highlight for RustHighlighter {
    fn highlight<'input>(&self, input: &'input str) -> Vec<syntax::HighlightedSpan<'input>> {
        vec![syntax::HighlightedSpan {
            text: input,
            group: None,
        }]
    }
}
