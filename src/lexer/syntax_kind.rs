use logos::Logos;
use num_enum::{IntoPrimitive, TryFromPrimitive};

#[derive(Debug, Logos, Copy, Clone, PartialEq, IntoPrimitive, TryFromPrimitive)]
#[repr(u16)]
pub(crate) enum SyntaxKind {
    #[token("let")]
    LetKw,

    #[token("if")]
    IfKw,

    #[regex(r"([^\n\r =$|*(){}]|\\ )+")]
    Atom,

    #[regex("[0-9]+", priority = 2)]
    Digits,

    #[regex("\"[^\"]*\"")]
    StringLiteral,

    #[token("true")]
    True,

    #[token("false")]
    False,

    #[token("=")]
    Equals,

    #[token("$")]
    Dollar,

    #[token("|")]
    Pipe,

    #[token("+")]
    Plus,

    #[token("-")]
    Minus,

    #[token("*")]
    Star,

    #[token("/")]
    Slash,

    #[token("(")]
    LParen,

    #[token(")")]
    RParen,

    #[token("{")]
    LBrace,

    #[token("}")]
    RBrace,

    #[regex(" +")]
    Whitespace,

    #[regex("[\n\r]+")]
    Eol,

    #[error]
    Error,

    // Compound variants
    Root,
    BindingDef,
    BinOp,
    FunctionCall,
    FunctionCallParams,
    Lambda,
    LambdaParams,
    BindingUsage,
    Block,
}

impl From<SyntaxKind> for rowan::SyntaxKind {
    fn from(kind: SyntaxKind) -> Self {
        Self(kind.into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lex_nothing() {
        assert_eq!(SyntaxKind::lexer("").next(), None);
    }

    fn test(input: &str, expected_kind: SyntaxKind) {
        let mut lexer = SyntaxKind::lexer(input);

        assert_eq!(lexer.next(), Some(expected_kind));
        assert_eq!(lexer.slice(), input);
    }

    fn test_join_to_atom(input: &str, expected_kind: SyntaxKind) {
        test(input, expected_kind);

        let lexer_input = format!("a{}b", input);
        let mut lexer = SyntaxKind::lexer(&lexer_input);

        assert_eq!(lexer.next(), Some(SyntaxKind::Atom));
        assert_eq!(lexer.slice(), lexer_input);
    }

    fn test_separate_from_atom(input: &str, expected_kind: SyntaxKind) {
        test(input, expected_kind);

        let lexer_input = format!("a{}b", input);
        let mut lexer = SyntaxKind::lexer(&lexer_input);

        assert_eq!(lexer.next(), Some(SyntaxKind::Atom));
        assert_eq!(lexer.slice(), "a");
        assert_eq!(lexer.next(), Some(expected_kind));
        assert_eq!(lexer.slice(), input);
        assert_eq!(lexer.next(), Some(SyntaxKind::Atom));
        assert_eq!(lexer.slice(), "b");
    }

    #[test]
    fn lex_let_keyword() {
        test_join_to_atom("let", SyntaxKind::LetKw);
    }

    #[test]
    fn lex_if_keyword() {
        test_join_to_atom("if", SyntaxKind::IfKw);
    }

    #[test]
    fn lex_atom() {
        test_join_to_atom("/bin/åbç123défg456", SyntaxKind::Atom);
    }

    #[test]
    fn lex_atom_that_contains_space_escaped_by_backslash() {
        test_join_to_atom("escaped\\ space", SyntaxKind::Atom);
    }

    #[test]
    fn lex_digits() {
        test_join_to_atom("1234567890", SyntaxKind::Digits);
    }

    #[test]
    fn lex_string_literal() {
        test_join_to_atom("\"hello\"", SyntaxKind::StringLiteral);
    }

    #[test]
    fn lex_true() {
        test_join_to_atom("true", SyntaxKind::True);
    }

    #[test]
    fn lex_false() {
        test_join_to_atom("false", SyntaxKind::False);
    }

    #[test]
    fn lex_equals_sign() {
        test_separate_from_atom("=", SyntaxKind::Equals);
    }

    #[test]
    fn lex_dollar_sign() {
        test_separate_from_atom("$", SyntaxKind::Dollar);
    }

    #[test]
    fn lex_pipe() {
        test_separate_from_atom("|", SyntaxKind::Pipe);
    }

    #[test]
    fn lex_plus() {
        test_join_to_atom("+", SyntaxKind::Plus);
    }

    #[test]
    fn lex_minus() {
        test_join_to_atom("-", SyntaxKind::Minus);
    }

    #[test]
    fn lex_star() {
        // Stars must be kept separate from Atoms because it allows them to be used for globbing.
        test_separate_from_atom("*", SyntaxKind::Star);
    }

    #[test]
    fn lex_slash() {
        test_join_to_atom("/", SyntaxKind::Slash);
    }

    #[test]
    fn lex_l_paren() {
        test_separate_from_atom("(", SyntaxKind::LParen);
    }

    #[test]
    fn lex_r_paren() {
        test_separate_from_atom(")", SyntaxKind::RParen);
    }

    #[test]
    fn lex_l_brace() {
        test_separate_from_atom("{", SyntaxKind::LBrace);
    }

    #[test]
    fn lex_r_brace() {
        test_separate_from_atom("}", SyntaxKind::RBrace);
    }

    #[test]
    fn lex_spaces() {
        test_separate_from_atom("  ", SyntaxKind::Whitespace);
    }

    #[test]
    fn lex_line_feeds() {
        test_separate_from_atom("\n\n\n", SyntaxKind::Eol);
    }

    #[test]
    fn lex_mixed_carriage_returns_and_line_feeds() {
        test_separate_from_atom("\r\r\n\r\n\n", SyntaxKind::Eol);
    }
}

impl SyntaxKind {
    pub(crate) fn can_start_expr(self) -> bool {
        match self {
            Self::Atom
            | Self::Digits
            | Self::StringLiteral
            | Self::True
            | Self::False
            | Self::Dollar
            | Self::Pipe
            | Self::LParen
            | Self::LBrace => true,
            _ => false,
        }
    }
}
