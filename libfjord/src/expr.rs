use nom::{
    bytes::complete::{take_till, take_while1},
    character::complete::char,
    multi::{many0, separated_list},
    sequence::delimited,
};

use crate::params::call;

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct Expr {
    kind: ExprKind,
}

#[derive(Clone, Debug, PartialEq)]
enum ExprKind {
    Number(crate::Number),
    Str(String),
    Block(Vec<crate::Item>),
    Var(crate::IdentName),
    FuncCall {
        name: crate::IdentName,
        params: Vec<call::Param>,
    },
}

impl Expr {
    pub(crate) fn new(s: &str) -> nom::IResult<&str, Self> {
        Self::new_number(s)
            .or_else(|_| Self::new_str(s))
            .or_else(|_| Self::new_block(s))
            .or_else(|_| Self::new_var(s))
            .or_else(|_| Self::new_func_call(s))
    }

    fn new_number(s: &str) -> nom::IResult<&str, Self> {
        let (s, n) = take_while1(|c: char| c.is_ascii_digit())(s)?;

        // This cannot fail because we know that n is all digits.
        let n = crate::Number::from_str_radix(n, 10).unwrap();

        Ok((
            s,
            Self {
                kind: ExprKind::Number(n),
            },
        ))
    }

    fn new_str(s: &str) -> nom::IResult<&str, Self> {
        let (s, text) = delimited(char('"'), take_till(|c| c == '"'), char('"'))(s)?;

        Ok((
            s,
            Self {
                kind: ExprKind::Str(text.into()),
            },
        ))
    }

    fn new_block(s: &str) -> nom::IResult<&str, Self> {
        let (s, _) = char('{')(s)?;
        let (s, _) = crate::take_whitespace(s)?;

        let (s, items) = separated_list(
            |s| {
                // Items in a block are separated by newlines, plus zero or more whitespace (for
                // indentation).
                let (s, newline) = char('\n')(s)?;
                let (s, _) = crate::take_whitespace(s)?;

                Ok((s, newline))
            },
            crate::Item::new,
        )(s)?;

        let (s, _) = crate::take_whitespace(s)?;
        let (s, _) = char('}')(s)?;

        Ok((
            s,
            Self {
                kind: ExprKind::Block(items),
            },
        ))
    }

    fn new_var(s: &str) -> nom::IResult<&str, Self> {
        let (s, _) = char('.')(s)?;
        let (s, name) = crate::IdentName::new(s)?;

        Ok((
            s,
            Self {
                kind: ExprKind::Var(name),
            },
        ))
    }

    fn new_func_call(s: &str) -> nom::IResult<&str, Self> {
        let (s, name) = crate::IdentName::new(s)?;

        let (s, params) = many0(|s| {
            let (s, _) = crate::take_whitespace1(s)?;
            call::Param::new(s)
        })(s)?;

        Ok((
            s,
            Self {
                kind: ExprKind::FuncCall { name, params },
            },
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn number() {
        assert_eq!(
            Expr::new_number("123"),
            Ok((
                "",
                Expr {
                    kind: ExprKind::Number(123)
                }
            ))
        );

        assert_eq!(
            Expr::new("123"),
            Ok((
                "",
                Expr {
                    kind: ExprKind::Number(123)
                }
            ))
        );
    }

    #[test]
    fn str() {
        assert_eq!(
            Expr::new_str("\"Hello, World!\""),
            Ok((
                "",
                Expr {
                    kind: ExprKind::Str("Hello, World!".into())
                }
            ))
        );

        assert_eq!(
            Expr::new_str("\"🦀\""),
            Ok((
                "",
                Expr {
                    kind: ExprKind::Str("🦀".into())
                }
            ))
        );

        assert_eq!(
            Expr::new("\"foobar\""),
            Ok((
                "",
                Expr {
                    kind: ExprKind::Str("foobar".into())
                }
            ))
        );
    }

    mod block {
        use super::*;

        #[test]
        fn basic() {
            assert_eq!(
                Expr::new_block("{ 25 }"),
                Ok((
                    "",
                    Expr {
                        kind: ExprKind::Block(vec![crate::Item::new("25").unwrap().1])
                    }
                ))
            )
        }

        #[test]
        fn variable_and_return() {
            assert_eq!(
                Expr::new(
                    "\
{
    foobar = \"Hello, World!\"
    .foobar
}"
                ),
                Ok((
                    "",
                    Expr {
                        kind: ExprKind::Block(vec![
                            crate::Item::new("foobar = \"Hello, World!\"").unwrap().1,
                            crate::Item::new(".foobar").unwrap().1,
                        ])
                    }
                ))
            );
        }

        #[test]
        fn only_variable() {
            assert_eq!(
                Expr::new("{myVar = 5}"),
                Ok((
                    "",
                    Expr {
                        kind: ExprKind::Block(vec![crate::Item::new("myVar = 5").unwrap().1])
                    }
                ))
            )
        }
    }

    #[test]
    fn var() {
        assert_eq!(
            Expr::new_var(".myVar"),
            Ok((
                "",
                Expr {
                    kind: ExprKind::Var(crate::IdentName::new("myVar").unwrap().1)
                }
            ))
        );
        assert_eq!(
            Expr::new(".foobar"),
            Ok((
                "",
                Expr {
                    kind: ExprKind::Var(crate::IdentName::new("foobar").unwrap().1)
                }
            ))
        );
    }

    #[test]
    fn no_args() {
        assert_eq!(
            Expr::new_func_call("funcName"),
            Ok((
                "",
                Expr {
                    kind: ExprKind::FuncCall {
                        name: crate::IdentName::new("funcName").unwrap().1,
                        params: vec![]
                    }
                }
            ))
        )
    }

    #[test]
    fn some_args() {
        assert_eq!(
            Expr::new_func_call("addThree 1 7 4"),
            Ok((
                "",
                Expr {
                    kind: ExprKind::FuncCall {
                        name: crate::IdentName::new("addThree").unwrap().1,
                        params: vec![
                            call::Param::new("1").unwrap().1,
                            call::Param::new("7").unwrap().1,
                            call::Param::new("4").unwrap().1
                        ]
                    }
                }
            ))
        )
    }

    #[test]
    fn func_call() {
        assert_eq!(
            Expr::new("sqrt 5"),
            Ok((
                "",
                Expr {
                    kind: ExprKind::FuncCall {
                        name: crate::IdentName::new("sqrt").unwrap().1,
                        params: vec![call::Param::new("5").unwrap().1]
                    }
                }
            ))
        )
    }
}

impl Expr {
    pub(crate) fn eval(self, state: &crate::eval::State) -> crate::eval::EvalResult {
        match self.kind {
            ExprKind::Number(n) => Ok(crate::eval::OutputExpr::Number(n)),
            ExprKind::Str(s) => Ok(crate::eval::OutputExpr::Str(s)),
            ExprKind::Block(b) => {
                // The block gets a scope of its own to isolate its contents from the parent scope.
                let mut block_scope = state.new_child();

                for item in &b {
                    // Early return on any free expression that isn’t the unit.
                    match item.clone().eval(&mut block_scope)? {
                        crate::eval::OutputExpr::Unit => (),
                        expr => return Ok(expr),
                    }
                }

                // At this point all items in the block have evaluated to the unit, so we return
                // the unit.
                Ok(crate::eval::OutputExpr::Unit)
            }
            ExprKind::Var(name) => match state.get_var(name) {
                Some(val) => Ok(val.clone()),
                None => Err(crate::eval::Error::VarNotFound),
            },
            ExprKind::FuncCall {
                name,
                params: call_params,
            } => {
                let func = state
                    .get_func(name)
                    .ok_or(crate::eval::Error::FuncNotFound)?;

                func.clone().eval(call_params, state)
            }
        }
    }
}
