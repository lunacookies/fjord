use logos::Logos;
use num_enum::{IntoPrimitive, TryFromPrimitive};

#[derive(Debug, Logos, Copy, Clone, PartialEq, IntoPrimitive, TryFromPrimitive)]
#[repr(u16)]
pub(crate) enum SyntaxKind {
    #[regex(r"([^\n ]|\\ )+")]
    Atom,

    #[regex("[0-9]+", priority = 2)]
    Digits,

    #[regex("\"[^\"]*\"")]
    StringLiteral,

    #[token("=")]
    Equals,

    #[token("$")]
    Dollar,

    #[token("::")]
    DoubleColon,

    #[regex("[\n ]+")]
    Whitespace,

    #[error]
    Error,

    // Compound variants
    Root,
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

    #[test]
    fn lex_atom() {
        test("/bin/åbç123défg456", SyntaxKind::Atom);
    }

    #[test]
    fn lex_atom_that_contains_space_escaped_by_backslash() {
        test("escaped\\ space", SyntaxKind::Atom);
    }

    #[test]
    fn lex_digits() {
        test("1234567890", SyntaxKind::Digits);
    }

    #[test]
    fn lex_string_literal() {
        test("\"hello\"", SyntaxKind::StringLiteral);
    }

    #[test]
    fn lex_equals_sign() {
        test("=", SyntaxKind::Equals);
    }

    #[test]
    fn lex_dollar_sign() {
        test("$", SyntaxKind::Dollar);
    }

    #[test]
    fn lex_double_colon() {
        test("::", SyntaxKind::DoubleColon);
    }

    #[test]
    fn lex_spaces() {
        test("  ", SyntaxKind::Whitespace);
    }

    #[test]
    fn lex_line_feeds() {
        test("\n\n\n", SyntaxKind::Whitespace);
    }
}
