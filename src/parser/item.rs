use super::expr::parse_expr;
use super::statement::parse_statement;
use super::Parser;
use crate::lexer::SyntaxKind;

pub(super) fn parse_item(p: &mut Parser<'_>) {
    if p.peek() == Some(SyntaxKind::Let) {
        parse_statement(p);
    } else {
        parse_expr(p);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(input: &'static str, expected_output: &'static str) {
        Parser::test(parse_item, input, expected_output);
    }

    #[test]
    fn parse_expr() {
        test(
            "5",
            r#"
Root@0..1
  Digits@0..1 "5""#,
        );
    }

    #[test]
    fn parse_statement() {
        test(
            "let x = $y",
            r#"
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
      Atom@9..10 "y""#,
        );
    }
}
