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
}

impl<'a> Parser<'a> {
    /// Creates a new Parser given the input.
    pub fn new(input: &'a str) -> Self {
        Self {
            lexer: Lexer::new(input).peekable(),
            builder: GreenNodeBuilder::new(),
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

    /// Parses the input the `Parser` was constructed with.
    pub fn parse(mut self) -> ParseOutput {
        self.builder.start_node(SyntaxKind::Root.into());

        if !self.at_end() {
            self.parse_expr();
        }

        self.builder.finish_node();

        ParseOutput {
            green_node: self.builder.finish(),
        }
    }

    fn parse_expr(&mut self) {
        match self.peek() {
            Some(SyntaxKind::Digits) | Some(SyntaxKind::StringLiteral) => self.bump(),
            _ => panic!("expected expression"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(input: &str, expected_output: &str) {
        let parse_output = Parser::new(input).parse();
        assert_eq!(parse_output.debug_tree(), expected_output.trim());
    }

    #[test]
    fn parse_nothing() {
        test(
            "",
            r#"
Root@0..0"#,
        );
    }

    #[test]
    fn parse_number_literal() {
        test(
            "10",
            r#"
Root@0..2
  Digits@0..2 "10""#,
        );
    }

    #[test]
    fn parse_string_literal() {
        test(
            "\"Hello, world!\"",
            r#"
Root@0..15
  StringLiteral@0..15 "\"Hello, world!\"""#,
        );
    }
}
