//! Parsing and the results thereof.

mod error;
pub use error::SyntaxError;

pub(crate) mod expr;
pub(crate) mod item;
pub(crate) mod statement;

use crate::ast::Root;
use crate::env::Env;
use crate::eval::EvalError;
use crate::lexer::{Lexeme, Lexer, SyntaxKind};
use crate::val::Val;
use crate::SyntaxNode;
use rowan::{GreenNode, GreenNodeBuilder};
use std::iter::Peekable;
use text_size::TextRange;

/// A type representing the state of a `ParseOutput` containing no errors.
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct NoErrors;

/// A type representing the state of a `ParseOutput` containing some number of errors.
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct ContainsErrors(Vec<SyntaxError>);

/// The state of a `ParseOutput`. This trait is sealed, and as such cannot be implemented outside of
/// this crate.
///
/// Due to its sealed nature, `ParseOutputState` functions somewhat similarly to an enum, because
/// all its implementors (variants to continue the enum analogy) are a fixed set (i.e. all the
/// possible implementors are known statically).
///
/// See `ParseOutput`’s documentation for the usage and purpose of this trait.
pub trait ParseOutputState: crate::private::Sealed {}
impl ParseOutputState for NoErrors {}
impl crate::private::Sealed for NoErrors {}
impl ParseOutputState for ContainsErrors {}
impl crate::private::Sealed for ContainsErrors {}

/// The output of parsing Fjord code.
///
/// The `State` type parameter is used to restrict the use of the `eval` method to when no syntax
/// errors are present through the type system. This is a usage of [the typestate
/// pattern](http://cliffle.com/blog/rust-typestate/).
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct ParseOutput<State: ParseOutputState> {
    green_node: GreenNode,
    state: State,
}

impl<State: ParseOutputState> ParseOutput<State> {
    pub(crate) fn syntax(&self) -> SyntaxNode {
        SyntaxNode::new_root(self.green_node.clone())
    }

    /// Returns a representation of the underlying syntax tree suitable for debugging purposes.
    pub fn debug_tree(&self) -> String {
        format!("{:#?}", self.syntax()).trim().to_string()
    }
}

impl ParseOutput<NoErrors> {
    /// Evaluates the parsed syntax tree with the given environment. This is only implemented for
    /// the case in which the `ParseOutput` contains no errors, because evaluating a `ParseOutput`
    /// with syntax errors is likely to both lead to confusing errors, and because this adds a lot
    /// of complexity to the interpreter.
    pub fn eval(&self, env: &mut Env<'_>) -> Result<Val, EvalError> {
        // The parser always emits a syntax tree with a Root node at the top, so we can safely
        // unwrap.
        let root = Root::cast(self.syntax()).unwrap();

        root.eval(env)
    }
}

impl ParseOutput<ContainsErrors> {
    /// Converts a `ParseOutput` that contains errors into one that does not.
    ///
    /// This method returns `None` if errors are present.
    pub fn into_no_errors(self) -> Option<ParseOutput<NoErrors>> {
        if self.state.0.is_empty() {
            Some(ParseOutput {
                green_node: self.green_node,
                state: NoErrors,
            })
        } else {
            None
        }
    }

    /// Returns all the syntax errors found throughout the parsing process.
    pub fn errors(&self) -> &[SyntaxError] {
        &self.state.0
    }
}

/// Parses Fjord code.
pub struct Parser<'a> {
    lexer: Peekable<Lexer<'a>>,
    builder: GreenNodeBuilder<'static>,
    errors: Vec<SyntaxError>,
    last_lexeme_range: TextRange,
}

impl<'a> Parser<'a> {
    /// Creates a new Parser given the input.
    pub fn new(input: &'a str) -> Self {
        Self {
            lexer: Lexer::new(input).peekable(),
            builder: GreenNodeBuilder::new(),
            errors: Vec::new(),
            last_lexeme_range: TextRange::default(),
        }
    }

    fn peek(&mut self) -> Option<SyntaxKind> {
        self.lexer.peek().map(|Lexeme { kind, .. }| *kind)
    }

    fn at_end(&mut self) -> bool {
        self.lexer.peek().is_none()
    }

    fn at_end_or_eol(&mut self) -> bool {
        self.at_end() || self.peek() == Some(SyntaxKind::Eol)
    }

    fn bump(&mut self) {
        let lexeme = self.lexer.next().unwrap();
        self.builder.token(lexeme.kind.into(), lexeme.text);

        self.last_lexeme_range = lexeme.range;
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
        match self.peek() {
            Some(SyntaxKind::Eol) | None => {}
            Some(_) => {
                let lexeme = self.lexer.next().unwrap();
                self.builder.token(SyntaxKind::Error.into(), lexeme.text);

                self.last_lexeme_range = lexeme.range;
            }
        }

        self.errors.push(SyntaxError {
            message,
            range: self.last_lexeme_range,
        });
    }

    #[cfg(test)]
    pub(crate) fn finish_and_get_syntax(self) -> SyntaxNode {
        let green = self.builder.finish();
        SyntaxNode::new_root(green)
    }

    /// Parses the input the `Parser` was constructed with.
    pub fn parse(mut self) -> ParseOutput<ContainsErrors> {
        self.builder.start_node(SyntaxKind::Root.into());

        self.skip_ws_and_eol();

        loop {
            if self.at_end() {
                break;
            }

            item::parse_item(&mut self);
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
            state: ContainsErrors(self.errors),
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
            state: ContainsErrors(p.errors),
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
