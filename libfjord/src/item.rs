use nom::bytes::complete::tag;

#[derive(Debug, PartialEq)]
pub(crate) struct Item {
    kind: ItemKind,
}

#[derive(Debug, PartialEq)]
enum ItemKind {
    Expr(crate::Expr),
    Binding {
        name: crate::IdentName,
        val: BindingVal,
    },
}

impl Item {
    pub(crate) fn new(s: &str) -> nom::IResult<&str, Self> {
        Self::new_binding(s).or_else(|_| Self::new_expr(s))
    }

    fn new_expr(s: &str) -> nom::IResult<&str, Self> {
        crate::Expr::new(s).map(|(s, e)| {
            (
                s,
                Self {
                    kind: ItemKind::Expr(e),
                },
            )
        })
    }

    fn new_binding(s: &str) -> nom::IResult<&str, Self> {
        let (s, _) = tag("let")(s)?;
        let (s, _) = crate::take_whitespace1(s)?;

        let (s, name) = crate::IdentName::new(s)?;
        let (s, _) = crate::take_whitespace1(s)?;

        let (s, val) = BindingVal::new(s)?;

        Ok((
            s,
            Self {
                kind: ItemKind::Binding { name, val },
            },
        ))
    }

    fn eval(self, state: &mut crate::eval::State) -> crate::eval::EvalResult {
        use crate::eval::Eval;

        match self.kind {
            ItemKind::Expr(e) => e.eval(state),
            ItemKind::Binding { name, val } => {
                match val {
                    BindingVal::Var(e) => state.set_var(name, e),
                    BindingVal::Func(f) => state.set_func(name, f),
                };
                Ok(crate::eval::OutputExpr::Unit)
            }
        }
    }
}

#[cfg(test)]
mod item_tests {
    use super::*;

    #[test]
    fn expr() {
        assert_eq!(
            Item::new("123"),
            Ok((
                "",
                Item {
                    kind: ItemKind::Expr(crate::Expr::Number(123))
                }
            ))
        )
    }

    #[test]
    fn expr_binding() {
        assert_eq!(
            Item::new("let myVar 25"),
            Ok((
                "",
                Item {
                    kind: ItemKind::Binding {
                        name: crate::IdentName::new("myVar").unwrap().1,
                        val: BindingVal::Var(crate::Expr::Number(25))
                    }
                }
            ))
        )
    }

    fn func_binding() {
        assert_eq!(
            Item::new("let myFunc fn param1 { 4321 }"),
            Ok((
                "",
                Item {
                    kind: ItemKind::Binding {
                        name: crate::IdentName::new("myFunc").unwrap().1,
                        val: BindingVal::Func(crate::Func::new("fn param1 { 4321 }").unwrap().1)
                    }
                }
            ))
        )
    }
}

#[derive(Debug, PartialEq)]
enum BindingVal {
    Var(crate::Expr),
    Func(crate::Func),
}

impl BindingVal {
    fn new(s: &str) -> nom::IResult<&str, Self> {
        Self::new_func(s).or_else(|_| Self::new_var(s))
    }

    fn new_func(s: &str) -> nom::IResult<&str, Self> {
        crate::Func::new(s).map(|(s, f)| (s, Self::Func(f)))
    }

    fn new_var(s: &str) -> nom::IResult<&str, Self> {
        crate::Expr::new(s).map(|(s, e)| (s, Self::Var(e)))
    }
}

#[cfg(test)]
mod binding_val_tests {
    use super::*;

    #[test]
    fn expr() {
        assert_eq!(
            BindingVal::new("123"),
            Ok(("", BindingVal::Var(crate::Expr::Number(123))))
        );
        assert_eq!(
            BindingVal::new("\"foobar\""),
            Ok(("", BindingVal::Var(crate::Expr::Str("foobar".into()))))
        );
    }

    #[test]
    fn func() {
        assert_eq!(
            BindingVal::new("fn { 9876 }"),
            Ok((
                "",
                BindingVal::Func(crate::Func::new("fn { 9876 }").unwrap().1)
            ))
        );
        assert_eq!(
            BindingVal::new("fn param1 param2 { \"some text\" }"),
            Ok((
                "",
                BindingVal::Func(
                    crate::Func::new("fn param1 param2 { \"some text\" }")
                        .unwrap()
                        .1
                )
            ))
        );
    }
}
