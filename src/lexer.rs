use logos::Logos;

#[derive(Debug, Logos, PartialEq)]
enum SyntaxKind {
    #[regex("[A-Za-z][A-Za-z0-9]*")]
    Identifier,

    #[error]
    Error,
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

    fn neg_test(input: &str, expected_kind: SyntaxKind) {
        let mut lexer = SyntaxKind::lexer(input);

        assert_ne!(lexer.next(), Some(expected_kind));
        assert_ne!(lexer.slice(), input);
    }

    #[test]
    fn lex_all_lowercase_identifier() {
        test("abcdefg", SyntaxKind::Identifier);
    }

    #[test]
    fn lex_all_caps_identifier() {
        test("ABCDEFG", SyntaxKind::Identifier);
    }

    #[test]
    fn lex_identifer_with_digits_at_the_end() {
        test("abc123", SyntaxKind::Identifier);
    }

    #[test]
    fn lex_identifier_with_digits_in_the_middle() {
        test("abc123def", SyntaxKind::Identifier);
    }

    #[test]
    fn dont_lex_identifier_with_digits_at_the_start() {
        neg_test("123abc", SyntaxKind::Identifier);
    }

    #[test]
    fn lex_one_char_identifier() {
        test("a", SyntaxKind::Identifier);
    }
}
