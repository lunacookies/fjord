use super::expr::parse_expr;
use super::Parser;
use crate::lexer::SyntaxKind;

pub(crate) fn parse_statement(p: &mut Parser) {
    match p.peek() {
        Some(SyntaxKind::Let) => parse_binding_def(p),
        Some(SyntaxKind::Return) => parse_return_statement(p),
        _ => p.error("expected let or return"),
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

pub(crate) fn parse_return_statement(p: &mut Parser) {
    assert_eq!(p.peek(), Some(SyntaxKind::Return));

    p.builder.start_node(SyntaxKind::ReturnStatement.into());
    p.bump();
    p.skip_ws();

    parse_expr(p);

    p.builder.finish_node();
}

#[cfg(test)]
mod tests {
    use super::*;
    use expect_test::{expect, Expect};

    fn test(input: &'static str, expected_output: Expect) {
        Parser::test(parse_statement, input, expected_output);
    }

    #[test]
    fn parse_binding_definition() {
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
    fn recover_from_junk_binding_name_in_binding_definition() {
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
    fn recover_from_junk_equals_sign_in_binding_definition() {
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
    fn recover_from_junk_rhs_of_binding_definition() {
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

    #[test]
    fn parse_return_statement() {
        test(
            "return ls ~/Documents",
            expect![[r#"
Root@0..21
  ReturnStatement@0..21
    Return@0..6 "return"
    Whitespace@6..7 " "
    FunctionCall@7..21
      Atom@7..9 "ls"
      Whitespace@9..10 " "
      FunctionCallParams@10..21
        Atom@10..21 "~/Documents""#]],
        );
    }

    #[test]
    fn parse_return_statement_without_val() {
        test(
            "return",
            expect![[r#"
Root@0..6
  ReturnStatement@0..6
    Return@0..6 "return""#]],
        );
    }

    #[test]
    fn parse_return_statement_without_val_followed_by_eol() {
        test(
            "return\nblah",
            expect![[r#"
Root@0..6
  ReturnStatement@0..6
    Return@0..6 "return""#]],
        );
    }

    #[test]
    fn parse_return_statement_without_val_followed_by_whitespace_then_eol() {
        test(
            "return   \nfoobar",
            expect![[r#"
Root@0..9
  ReturnStatement@0..9
    Return@0..6 "return"
    Whitespace@6..9 "   ""#]],
        );
    }
}
