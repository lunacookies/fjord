use super::expr::parse_expr;
use super::Parser;
use crate::lexer::SyntaxKind;

pub(crate) fn parse_item(p: &mut Parser) {
    match p.peek() {
        Some(SyntaxKind::Let) => parse_binding_def(p),
        _ => parse_expr(p),
    }
}

pub(crate) fn parse_binding_def(p: &mut Parser) {
    assert_eq!(p.peek(), Some(SyntaxKind::Let));

    p.builder.start_node(SyntaxKind::BindingDef.into());
    p.bump();
    p.skip_ws();

    if let Some(SyntaxKind::Atom) = p.peek() {
        p.bump();
    } else {
        p.error("expected binding name");
    }

    p.skip_ws();

    if let Some(SyntaxKind::Equals) = p.peek() {
        p.bump();
    } else {
        p.error("expected equals sign");
    }

    p.skip_ws();

    parse_expr(p);

    p.builder.finish_node();
}

#[cfg(test)]
mod tests {
    use super::*;
    use expect_test::{expect, Expect};

    fn test(input: &'static str, expected_output: Expect) {
        Parser::test(parse_item, input, expected_output);
    }

    #[test]
    fn parse_expr() {
        test(
            "5",
            expect![[r#"
            Root@0..1
              Digits@0..1 "5""#]],
        );
    }

    #[test]
    fn parse_simple_binding_def() {
        test(
            "let x = $y",
            expect![[r#"
            Root@0..10
              BindingDef@0..10
                Let@0..3 "let"
                Whitespace@3..4 " "
                Atom@4..5 "x"
                Whitespace@5..6 " "
                Equals@6..7 "="
                Whitespace@7..8 " "
                BindingUsage@8..10
                  Dollar@8..9 "$"
                  Atom@9..10 "y""#]],
        );
    }

    #[test]
    fn parse_complex_binding_def() {
        test(
            r#"let foo = bar "baz" $quux 5"#,
            expect![[r#"
            Root@0..27
              BindingDef@0..27
                Let@0..3 "let"
                Whitespace@3..4 " "
                Atom@4..7 "foo"
                Whitespace@7..8 " "
                Equals@8..9 "="
                Whitespace@9..10 " "
                FunctionCall@10..27
                  Atom@10..13 "bar"
                  Whitespace@13..14 " "
                  FunctionCallParams@14..27
                    StringLiteral@14..19 "\"baz\""
                    Whitespace@19..20 " "
                    BindingUsage@20..25
                      Dollar@20..21 "$"
                      Atom@21..25 "quux"
                    Whitespace@25..26 " "
                    Digits@26..27 "5""#]],
        );
    }

    #[test]
    fn recover_from_junk_binding_name_in_binding_def() {
        test(
            "let 5 = 10",
            expect![[r#"
            Root@0..10
              BindingDef@0..10
                Let@0..3 "let"
                Whitespace@3..4 " "
                Error@4..5 "5"
                Whitespace@5..6 " "
                Equals@6..7 "="
                Whitespace@7..8 " "
                Digits@8..10 "10""#]],
        );
    }

    #[test]
    fn recover_from_junk_equals_sign_in_binding_def() {
        test(
            "let x _ 10",
            expect![[r#"
            Root@0..10
              BindingDef@0..10
                Let@0..3 "let"
                Whitespace@3..4 " "
                Atom@4..5 "x"
                Whitespace@5..6 " "
                Error@6..7 "_"
                Whitespace@7..8 " "
                Digits@8..10 "10""#]],
        );
    }

    #[test]
    fn recover_from_junk_rhs_of_binding_def() {
        test(
            "let a = =",
            expect![[r#"
            Root@0..9
              BindingDef@0..9
                Let@0..3 "let"
                Whitespace@3..4 " "
                Atom@4..5 "a"
                Whitespace@5..6 " "
                Equals@6..7 "="
                Whitespace@7..8 " "
                Error@8..9 "=""#]],
        );
    }
}
