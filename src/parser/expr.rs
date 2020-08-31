use super::Parser;
use crate::lexer::SyntaxKind;
use crate::Op;

#[derive(Copy, Clone, PartialEq)]
enum VirtualOp {
    Op(Op),
    Application,
}

pub(crate) fn parse_expr(p: &mut Parser) {
    parse_expr_bp(p, 0, false);
}

fn parse_expr_bp(p: &mut Parser, min_bp: u8, in_func_call_params: bool) {
    let checkpoint = p.builder.checkpoint();

    parse_one_expr(p, in_func_call_params);

    p.skip_ws();

    loop {
        let op = loop {
            match p.peek() {
                Some(SyntaxKind::Plus) => break VirtualOp::Op(Op::Add),
                Some(SyntaxKind::Minus) => break VirtualOp::Op(Op::Sub),
                Some(SyntaxKind::Star) => break VirtualOp::Op(Op::Mul),
                Some(SyntaxKind::Slash) => break VirtualOp::Op(Op::Div),
                Some(SyntaxKind::Atom)
                | Some(SyntaxKind::Digits)
                | Some(SyntaxKind::StringLiteral)
                | Some(SyntaxKind::Dollar)
                | Some(SyntaxKind::Pipe) => break VirtualOp::Application,
                Some(SyntaxKind::RParen) | Some(SyntaxKind::Eol) | None => return,
                Some(_) => p.error("expected operator"),
            }
        };

        let (left_bp, right_bp) = infix_bp(op);

        if left_bp < min_bp {
            break;
        }

        if op == VirtualOp::Application {
            if !in_func_call_params {
                p.builder
                    .start_node_at(checkpoint, SyntaxKind::FunctionCall.into());

                p.builder.start_node(SyntaxKind::FunctionCallParams.into());
            }

            parse_expr_bp(p, right_bp, true);
            p.skip_ws();

            if !in_func_call_params {
                p.builder.finish_node();
                p.builder.finish_node();
            }
        } else {
            p.builder
                .start_node_at(checkpoint, SyntaxKind::BinOp.into());

            // Eat the operator’s token.
            p.bump();
            p.skip_ws();

            parse_expr_bp(p, right_bp, in_func_call_params);

            p.builder.finish_node();
        }
    }
}

fn parse_one_expr(p: &mut Parser, in_func_call_params: bool) {
    match p.peek() {
        Some(SyntaxKind::Atom) => parse_atom(p, in_func_call_params),
        Some(SyntaxKind::Digits) | Some(SyntaxKind::StringLiteral) => p.bump(),
        Some(SyntaxKind::Dollar) => parse_binding_usage(p),
        Some(SyntaxKind::Pipe) => parse_lambda(p),
        Some(SyntaxKind::LParen) => {
            p.bump();
            parse_expr_bp(p, 0, false);

            if p.peek() == Some(SyntaxKind::RParen) {
                p.bump();
            } else {
                p.error("expected right parenthesis");
            }
        }
        _ => p.error("expected expression"),
    }
}

fn parse_atom(p: &mut Parser, in_func_call_params: bool) {
    assert_eq!(p.peek(), Some(SyntaxKind::Atom));

    // If we’re in the parameters of a function call, then atoms are parsed as bare words, and we
    // can immediately return.
    if in_func_call_params {
        p.bump();
        return;
    }

    let idx_of_next_non_whitespace_token = {
        // We know we’re at an atom, so we don’t need to look at that token.
        let mut idx = 1;

        loop {
            // If we’re at whitespace, then we increment idx so we can see the next token.
            if p.lookahead(idx) == Some(SyntaxKind::Whitespace) {
                idx += 1;
            } else {
                break idx;
            }
        }
    };

    let at_expr = match p.lookahead(idx_of_next_non_whitespace_token) {
        Some(SyntaxKind::Atom)
        | Some(SyntaxKind::Digits)
        | Some(SyntaxKind::StringLiteral)
        | Some(SyntaxKind::Dollar)
        | Some(SyntaxKind::Pipe) => true,
        _ => false,
    };

    // Being at an expression means that we’re at the start of a function call (i.e. we’re at the
    // name of the function being called) that has one or more parameters.
    if at_expr {
        p.bump();
    } else {
        // In this case we’re not at an expression and we’re not in a function call (see the early
        // return further up), and we’re at an atom. The only thing this could be is a function call
        // with no parameters.
        p.builder.start_node(SyntaxKind::FunctionCall.into());
        p.bump();
        p.builder.start_node(SyntaxKind::FunctionCallParams.into());

        p.builder.finish_node();
        p.builder.finish_node();
    }
}

fn infix_bp(op: VirtualOp) -> (u8, u8) {
    match op {
        VirtualOp::Op(op) => match op {
            Op::Add | Op::Sub => (1, 2),
            Op::Mul | Op::Div => (3, 4),
        },
        VirtualOp::Application => (5, 5),
    }
}

pub(crate) fn parse_lambda(p: &mut Parser) {
    assert_eq!(p.peek(), Some(SyntaxKind::Pipe));

    p.builder.start_node(SyntaxKind::Lambda.into());

    p.builder.start_node(SyntaxKind::LambdaParams.into());

    p.bump();
    p.skip_ws();

    loop {
        if p.at_end() {
            break;
        }

        match p.peek() {
            Some(SyntaxKind::Atom) => p.bump(),
            Some(SyntaxKind::Pipe) => {
                p.bump();
                break;
            }
            None => break,
            _ => p.error("expected atom or pipe"),
        }

        p.skip_ws();
    }

    p.builder.finish_node();

    p.skip_ws();
    parse_expr(p);

    p.builder.finish_node();
}

pub(crate) fn parse_binding_usage(p: &mut Parser) {
    assert_eq!(p.peek(), Some(SyntaxKind::Dollar));

    p.builder.start_node(SyntaxKind::BindingUsage.into());
    p.bump();

    match p.peek() {
        Some(SyntaxKind::Atom) => p.bump(),
        _ => p.error("expected atom"),
    }

    p.builder.finish_node();
}

#[cfg(test)]
mod tests {
    use super::*;
    use expect_test::{expect, Expect};

    fn test(input: &'static str, expected_output: Expect) {
        Parser::test(parse_expr, input, expected_output);
    }

    #[test]
    fn parse_number_literal() {
        test(
            "10",
            expect![[r#"
            Root@0..2
              Digits@0..2 "10""#]],
        );
    }

    #[test]
    fn parse_string_literal() {
        test(
            "\"Hello, world!\"",
            expect![[r#"
            Root@0..15
              StringLiteral@0..15 "\"Hello, world!\"""#]],
        );
    }

    #[test]
    fn parse_function_call() {
        test(
            "func a 1",
            expect![[r#"
            Root@0..8
              FunctionCall@0..8
                Atom@0..4 "func"
                Whitespace@4..5 " "
                FunctionCallParams@5..8
                  Atom@5..6 "a"
                  Whitespace@6..7 " "
                  Digits@7..8 "1""#]],
        );
    }

    #[test]
    fn parse_function_call_with_no_params() {
        test(
            "ls",
            expect![[r#"
            Root@0..2
              FunctionCall@0..2
                Atom@0..2 "ls"
                FunctionCallParams@2..2"#]],
        );
    }

    #[test]
    fn stop_parsing_function_call_at_end_of_line() {
        test(
            "ls $dir\n",
            expect![[r#"
            Root@0..7
              FunctionCall@0..7
                Atom@0..2 "ls"
                Whitespace@2..3 " "
                FunctionCallParams@3..7
                  BindingUsage@3..7
                    Dollar@3..4 "$"
                    Atom@4..7 "dir""#]],
        );
    }

    #[test]
    fn parse_binding_usage() {
        test(
            "$var",
            expect![[r#"
            Root@0..4
              BindingUsage@0..4
                Dollar@0..1 "$"
                Atom@1..4 "var""#]],
        );
    }

    #[test]
    fn recover_from_junk_binding_usage() {
        test(
            "$let",
            expect![[r#"
            Root@0..4
              BindingUsage@0..4
                Dollar@0..1 "$"
                Error@1..4 "let""#]],
        );
    }

    #[test]
    fn parse_lambda() {
        test(
            "|a b| a $b 5",
            expect![[r#"
            Root@0..12
              Lambda@0..12
                LambdaParams@0..5
                  Pipe@0..1 "|"
                  Atom@1..2 "a"
                  Whitespace@2..3 " "
                  Atom@3..4 "b"
                  Pipe@4..5 "|"
                Whitespace@5..6 " "
                FunctionCall@6..12
                  Atom@6..7 "a"
                  Whitespace@7..8 " "
                  FunctionCallParams@8..12
                    BindingUsage@8..10
                      Dollar@8..9 "$"
                      Atom@9..10 "b"
                    Whitespace@10..11 " "
                    Digits@11..12 "5""#]],
        );
    }

    #[test]
    fn parse_simple_bin_op() {
        test(
            "1 + 5",
            expect![[r#"
            Root@0..5
              BinOp@0..5
                Digits@0..1 "1"
                Whitespace@1..2 " "
                Plus@2..3 "+"
                Whitespace@3..4 " "
                Digits@4..5 "5""#]],
        );
    }

    #[test]
    fn parse_bin_op_showing_precedence() {
        test(
            "2 + 3 * 4",
            expect![[r#"
            Root@0..9
              BinOp@0..9
                Digits@0..1 "2"
                Whitespace@1..2 " "
                Plus@2..3 "+"
                Whitespace@3..4 " "
                BinOp@4..9
                  Digits@4..5 "3"
                  Whitespace@5..6 " "
                  Star@6..7 "*"
                  Whitespace@7..8 " "
                  Digits@8..9 "4""#]],
        );
    }

    #[test]
    fn parse_bin_op_showing_associativity() {
        test(
            "10 - 5 - 3 - 2",
            expect![[r#"
            Root@0..14
              BinOp@0..14
                BinOp@0..11
                  BinOp@0..7
                    Digits@0..2 "10"
                    Whitespace@2..3 " "
                    Minus@3..4 "-"
                    Whitespace@4..5 " "
                    Digits@5..6 "5"
                    Whitespace@6..7 " "
                  Minus@7..8 "-"
                  Whitespace@8..9 " "
                  Digits@9..10 "3"
                  Whitespace@10..11 " "
                Minus@11..12 "-"
                Whitespace@12..13 " "
                Digits@13..14 "2""#]],
        );
    }

    #[test]
    fn function_call_has_higher_precedence_than_bin_op() {
        test(
            "sin 90 * 2",
            expect![[r#"
            Root@0..10
              BinOp@0..10
                FunctionCall@0..7
                  Atom@0..3 "sin"
                  Whitespace@3..4 " "
                  FunctionCallParams@4..7
                    Digits@4..6 "90"
                    Whitespace@6..7 " "
                Star@7..8 "*"
                Whitespace@8..9 " "
                Digits@9..10 "2""#]],
        );
    }

    #[test]
    fn parse_bin_op_containing_parens() {
        test(
            "2 * (3 + 5)",
            expect![[r#"
            Root@0..11
              BinOp@0..11
                Digits@0..1 "2"
                Whitespace@1..2 " "
                Star@2..3 "*"
                Whitespace@3..4 " "
                LParen@4..5 "("
                BinOp@5..10
                  Digits@5..6 "3"
                  Whitespace@6..7 " "
                  Plus@7..8 "+"
                  Whitespace@8..9 " "
                  Digits@9..10 "5"
                RParen@10..11 ")""#]],
        );
    }

    #[test]
    fn unneeded_parentheses_are_flattened() {
        test(
            "(((((1 + (1))))))",
            expect![[r#"
            Root@0..17
              LParen@0..1 "("
              LParen@1..2 "("
              LParen@2..3 "("
              LParen@3..4 "("
              LParen@4..5 "("
              BinOp@5..12
                Digits@5..6 "1"
                Whitespace@6..7 " "
                Plus@7..8 "+"
                Whitespace@8..9 " "
                LParen@9..10 "("
                Digits@10..11 "1"
                RParen@11..12 ")"
              RParen@12..13 ")"
              RParen@13..14 ")"
              RParen@14..15 ")"
              RParen@15..16 ")"
              RParen@16..17 ")""#]],
        );
    }
}
