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
            match self.peek() {
                Some(SyntaxKind::Let) => self.parse_statement(),
                _ => self.parse_expr(),
            }
        }

        self.builder.finish_node();

        ParseOutput {
            green_node: self.builder.finish(),
        }
    }

    fn parse_statement(&mut self) {
        assert_eq!(self.peek(), Some(SyntaxKind::Let));

        self.builder.start_node(SyntaxKind::Statement.into());
        self.bump();
        self.skip_ws();

        if let Some(SyntaxKind::Atom) = self.peek() {
            self.bump();
        } else {
            self.error("expected binding name");
        }

        self.skip_ws();

        if let Some(SyntaxKind::Equals) = self.peek() {
            self.bump();
        } else {
            self.error("expected equals sign");
        }

        self.skip_ws();

        self.parse_expr();

        self.builder.finish_node();
    }

    fn parse_expr(&mut self) {
        match self.peek() {
            Some(SyntaxKind::Atom) => {
                self.builder.start_node(SyntaxKind::Expr.into());
                self.parse_function_call();
                self.builder.finish_node();
            }
            Some(SyntaxKind::Digits)
            | Some(SyntaxKind::StringLiteral)
            | Some(SyntaxKind::Dollar) => self.parse_contained_expr(),
            _ => {
                self.builder.start_node(SyntaxKind::Expr.into());
                self.error("expected expression");
                self.builder.finish_node();
            }
        }
    }

    fn parse_contained_expr(&mut self) {
        self.builder.start_node(SyntaxKind::Expr.into());

        match self.peek() {
            Some(SyntaxKind::Digits) | Some(SyntaxKind::StringLiteral) | Some(SyntaxKind::Atom) => {
                self.bump()
            }
            Some(SyntaxKind::Dollar) => self.parse_binding_usage(),
            _ => self.error("expected expression"),
        }

        self.builder.finish_node();
    }

    fn parse_function_call(&mut self) {
        assert_eq!(self.peek(), Some(SyntaxKind::Atom));

        self.builder.start_node(SyntaxKind::FunctionCall.into());
        self.bump();
        self.skip_ws();

        self.builder
            .start_node(SyntaxKind::FunctionCallParams.into());

        loop {
            if self.at_end() {
                break;
            }

            self.parse_contained_expr();
            self.skip_ws();
        }

        self.builder.finish_node();

        self.builder.finish_node();
    }

    fn parse_binding_usage(&mut self) {
        assert_eq!(self.peek(), Some(SyntaxKind::Dollar));

        self.builder.start_node(SyntaxKind::BindingUsage.into());
        self.bump();

        match self.peek() {
            Some(SyntaxKind::Atom) => self.bump(),
            _ => self.error("expected atom"),
        }

        self.builder.finish_node();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

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
  Expr@0..2
    Digits@0..2 "10""#,
        );
    }

    #[test]
    fn parse_string_literal() {
        test(
            "\"Hello, world!\"",
            r#"
Root@0..15
  Expr@0..15
    StringLiteral@0..15 "\"Hello, world!\"""#,
        );
    }

    #[test]
    fn parse_function_call() {
        test(
            "func a 1",
            r#"
Root@0..8
  Expr@0..8
    FunctionCall@0..8
      Atom@0..4 "func"
      Whitespace@4..5 " "
      FunctionCallParams@5..8
        Expr@5..6
          Atom@5..6 "a"
        Whitespace@6..7 " "
        Expr@7..8
          Digits@7..8 "1""#,
        );
    }

    #[test]
    fn parse_binding_usage() {
        test(
            "$var",
            r#"
Root@0..4
  Expr@0..4
    BindingUsage@0..4
      Dollar@0..1 "$"
      Atom@1..4 "var""#,
        );
    }

    #[test]
    fn parse_binding_definition() {
        test(
            r#"let foo = bar "baz" $quux 5"#,
            r#"
Root@0..27
  Statement@0..27
    Let@0..3 "let"
    Whitespace@3..4 " "
    Atom@4..7 "foo"
    Whitespace@7..8 " "
    Equals@8..9 "="
    Whitespace@9..10 " "
    Expr@10..27
      FunctionCall@10..27
        Atom@10..13 "bar"
        Whitespace@13..14 " "
        FunctionCallParams@14..27
          Expr@14..19
            StringLiteral@14..19 "\"baz\""
          Whitespace@19..20 " "
          Expr@20..25
            BindingUsage@20..25
              Dollar@20..21 "$"
              Atom@21..25 "quux"
          Whitespace@25..26 " "
          Expr@26..27
            Digits@26..27 "5"
          "#,
        );
    }

    #[test]
    fn recover_from_junk_binding_name_in_binding_definition() {
        test(
            "let 5 = 10",
            r#"
Root@0..10
  Statement@0..10
    Let@0..3 "let"
    Whitespace@3..4 " "
    Error@4..5 "5"
    Whitespace@5..6 " "
    Equals@6..7 "="
    Whitespace@7..8 " "
    Expr@8..10
      Digits@8..10 "10""#,
        );
    }

    #[test]
    fn recover_from_junk_equals_sign_in_binding_definition() {
        test(
            "let x _ 10",
            r#"
Root@0..10
  Statement@0..10
    Let@0..3 "let"
    Whitespace@3..4 " "
    Atom@4..5 "x"
    Whitespace@5..6 " "
    Error@6..7 "_"
    Whitespace@7..8 " "
    Expr@8..10
      Digits@8..10 "10""#,
        );
    }

    #[test]
    fn recover_from_junk_expression() {
        test(
            "let a = =",
            r#"
Root@0..9
  Statement@0..9
    Let@0..3 "let"
    Whitespace@3..4 " "
    Atom@4..5 "a"
    Whitespace@5..6 " "
    Equals@6..7 "="
    Whitespace@7..8 " "
    Expr@8..9
      Error@8..9 "=""#,
        );
    }

    #[test]
    fn recover_from_junk_binding_usage() {
        test(
            "$let",
            r#"
Root@0..4
  Expr@0..4
    BindingUsage@0..4
      Dollar@0..1 "$"
      Error@1..4 "let""#,
        )
    }
}
