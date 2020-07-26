mod expr;
mod item;
mod statement;

use expr::parse_expr;
use item::parse_item;
use statement::parse_statement;

use crate::ast::Root;
use crate::env::Env;
use crate::lexer::{Lexer, SyntaxKind};
use crate::val::Val;
use crate::SyntaxNode;
use rowan::{GreenNode, GreenNodeBuilder};
use std::iter::Peekable;

/// The output of parsing Fjord code.
pub struct ParseOutput {
    green_node: GreenNode,
}

impl ParseOutput {
    pub(crate) fn eval(&self) -> Option<Val> {
        let mut env = Env::new();
        let root = Root::cast(self.syntax())?;

        Some(root.eval(&mut env))
    }

    fn syntax(&self) -> SyntaxNode {
        SyntaxNode::new_root(self.green_node.clone())
    }

    #[cfg(test)]
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

    fn skip(&mut self, kinds: &'static [SyntaxKind]) {
        loop {
            if self.at_end() {
                break;
            }

            if kinds.contains(&self.peek().unwrap()) {
                self.bump();
            } else {
                break;
            }
        }
    }

    fn skip_ws(&mut self) {
        self.skip(&[SyntaxKind::Whitespace]);
    }

    fn skip_ws_and_eol(&mut self) {
        self.skip(&[SyntaxKind::Whitespace, SyntaxKind::Eol]);
    }

    fn error(&mut self, message: &'static str) {
        self.errors.push(message);

        match self.peek() {
            Some(SyntaxKind::Eol) | None => {}
            Some(_) => {
                let (_, text) = self.lexer.next().unwrap();
                self.builder.token(SyntaxKind::Error.into(), text);
            }
        }
    }

    /// Parses the input the `Parser` was constructed with.
    pub fn parse(mut self) -> ParseOutput {
        self.builder.start_node(SyntaxKind::Root.into());

        self.skip_ws_and_eol();

        loop {
            if self.at_end() {
                break;
            }

            parse_item(&mut self);
            self.skip_ws();

            match self.peek() {
                Some(SyntaxKind::Eol) => self.bump(),
                None => break,
                _ => self.error("expected end of line"),
            }
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
    use pretty_assertions::assert_eq;

    fn test(input: &'static str, expected_output: &'static str) {
        let parser = Parser::new(input);
        let parse_output = parser.parse();

        assert_eq!(parse_output.debug_tree(), expected_output.trim());
    }

    #[test]
    fn parse_nothing() {
        test("", "Root@0..0");
    }

    #[test]
    fn parse_multiple_items() {
        test(
            r#"
let a = "dir"
let b = $a
ls $b"#,
            r#"
Root@0..31
  Eol@0..1 "\n"
  BindingDef@1..14
    Let@1..4 "let"
    Whitespace@4..5 " "
    Atom@5..6 "a"
    Whitespace@6..7 " "
    Equals@7..8 "="
    Whitespace@8..9 " "
    StringLiteral@9..14 "\"dir\""
  Eol@14..15 "\n"
  BindingDef@15..25
    Let@15..18 "let"
    Whitespace@18..19 " "
    Atom@19..20 "b"
    Whitespace@20..21 " "
    Equals@21..22 "="
    Whitespace@22..23 " "
    BindingUsage@23..25
      Dollar@23..24 "$"
      Atom@24..25 "a"
  Eol@25..26 "\n"
  FunctionCall@26..31
    Atom@26..28 "ls"
    Whitespace@28..29 " "
    FunctionCallParams@29..31
      BindingUsage@29..31
        Dollar@29..30 "$"
        Atom@30..31 "b""#,
        );
    }
}
