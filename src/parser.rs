use crate::lexer::Lexer;
use rowan::GreenNodeBuilder;

/// Parses Fjord code.
pub struct Parser<'a> {
    lexer: Lexer<'a>,
    builder: GreenNodeBuilder<'static>,
}

impl<'a> Parser<'a> {
    /// Creates a new Parser given the input.
    pub fn new(input: &'a str) -> Self {
        Self {
            lexer: Lexer::new(input),
            builder: GreenNodeBuilder::new(),
        }
    }
}
