mod expr;
mod item;
mod statement;

use expr::parse_expr;
use item::parse_item;
use statement::parse_statement;

use crate::lexer::{Lexer, SyntaxKind};
use crate::SyntaxNode;
use rowan::{GreenNode, GreenNodeBuilder};
use std::iter::Peekable;

/// The output of parsing Fjord code.
pub struct ParseOutput {
    green_node: GreenNode,
}

#[cfg(test)]
impl ParseOutput {
    fn syntax(&self) -> SyntaxNode {
        SyntaxNode::new_root(self.green_node.clone())
    }

    fn debug_tree(&self) -> String {
        format!("{:#?}", self.syntax()).trim().to_string()
    }
}

/// Parses Fjord code.
pub struct Parser<'a> {
    lexer: Peekable<Lexer<'a>>,
    builder: GreenNodeBuilder<'static>,
    errors: Vec<&'static str>,
}

impl<'a> Parser<'a> {
    /// Creates a new Parser given the input.
    pub fn new(input: &'a str) -> Self {
        Self {
            lexer: Lexer::new(input).peekable(),
            builder: GreenNodeBuilder::new(),
            errors: Vec::new(),
        }
    }

    fn peek(&mut self) -> Option<SyntaxKind> {
        self.lexer.peek().map(|(kind, _)| *kind)
    }

    fn at_end(&mut self) -> bool {
        self.lexer.peek().is_none()
    }

    fn bump(&mut self) {
        let (kind, text) = self.lexer.next().unwrap();
        self.builder.token(kind.into(), text);
    }

    fn skip_ws(&mut self) {
        while let Some(SyntaxKind::Whitespace) = self.peek() {
            self.bump();
        }
    }

    fn error(&mut self, message: &'static str) {
        self.errors.push(message);

        if let Some((_, text)) = self.lexer.next() {
            self.builder.token(SyntaxKind::Error.into(), text)
        }
    }

    /// Parses the input the `Parser` was constructed with.
    pub fn parse(mut self) -> ParseOutput {
        self.builder.start_node(SyntaxKind::Root.into());

        if !self.at_end() {
            parse_item(&mut self);
        }

        self.builder.finish_node();

        ParseOutput {
            green_node: self.builder.finish(),
        }
    }

    #[cfg(test)]
    fn test(f: impl Fn(&mut Self), input: &'static str, expected_output: &'static str) {
        use pretty_assertions::assert_eq;

        let mut p = Self::new(input);

        p.builder.start_node(SyntaxKind::Root.into());
        f(&mut p);
        p.builder.finish_node();

        let parse_output = ParseOutput {
            green_node: p.builder.finish(),
        };

        assert_eq!(parse_output.debug_tree(), expected_output.trim());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_nothing() {
        let parse_output = Parser::new("").parse();
        assert_eq!(parse_output.debug_tree(), "Root@0..0");
    }
}
