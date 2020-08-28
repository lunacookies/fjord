use super::expr::parse_expr;
use super::statement::parse_statement;
use super::Parser;
use crate::lexer::SyntaxKind;

pub(crate) fn parse_item(p: &mut Parser) {
    match p.peek() {
        Some(SyntaxKind::Let) | Some(SyntaxKind::Return) => parse_statement(p),
        _ => parse_expr(p),
    }
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
    fn parse_binding_def() {
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
    fn parse_return_statement() {
        test(
            "return $x",
            expect![[r#"
Root@0..9
  ReturnStatement@0..9
    Return@0..6 "return"
    Whitespace@6..7 " "
    BindingUsage@7..9
      Dollar@7..8 "$"
      Atom@8..9 "x""#]],
        );
    }
}
