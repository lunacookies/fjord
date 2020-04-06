use nom::{
    bytes::complete::{take_till, take_while1},
    character::complete::char,
    multi::{many0, separated_list},
    sequence::delimited,
};

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
        params: Vec<Param>,
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
            Param::new(s)
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
    let foobar \"Hello, World!\"
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
                Expr::new("{let myVar 5}"),
                Ok((
                    "",
                    Expr {
                        kind: ExprKind::Block(vec![crate::Item::new("let myVar 5").unwrap().1])
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
                            Param::new("1").unwrap().1,
                            Param::new("7").unwrap().1,
                            Param::new("4").unwrap().1
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
                        params: vec![Param::new("5").unwrap().1]
                    }
                }
            ))
        )
    }
}

impl Expr {
    pub(crate) fn eval(self, state: &crate::eval::State) -> crate::eval::EvalResult {
        use std::cmp::Ordering;

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

                let mut def_params = func.params().to_vec();
                let mut func_state = state.new_child();

                // FIXME: these two bindings could use ‘.partition()’, but then we’d have to turn
                // each back into an iterator and back to a Vec in order to unwrap the Param enum.
                // For now we’ll just loop twice.
                let named_params = call_params.iter().filter_map(|p| {
                    if let Param::Named(np) = p {
                        Some(np)
                    } else {
                        None
                    }
                });
                let positional_params = call_params.iter().filter_map(|p| {
                    if let Param::Positional(pp) = p {
                        Some(pp)
                    } else {
                        None
                    }
                });

                // Loop through all named function parameters. If the parameter actually exists,
                // add it to the state and remove it from the list of function definition params
                // (as we don’t need it anymore. If it doesn’t exist, then return an error.
                for named_param in named_params {
                    if let Some(def_param_idx) = def_params
                        .iter()
                        .position(|p| p.name() == &named_param.name)
                    {
                        func_state.set_var(
                            named_param.name.clone(),
                            named_param.val.clone().eval(&func_state)?,
                        );
                        def_params.remove(def_param_idx);
                    } else {
                        return Err(crate::eval::Error::FuncParamNotFound);
                    }
                }

                let def_params_len = def_params.len();
                let positional_params_len = positional_params.clone().count();

                let ord = positional_params_len.cmp(&def_params_len);
                match ord {
                    // In these cases we have the same or greater number of remaining definition
                    // arguments, compared to the number of input arguments.
                    Ordering::Less | Ordering::Equal => {
                        // Match up all the call parameters with as many definition parameters as
                        // possible.
                        for (call_param, def_param) in positional_params.zip(&def_params) {
                            func_state.set_var(
                                def_param.name().clone(),
                                call_param.val.clone().eval(&func_state)?,
                            );
                        }

                        // This branch also has the possibility that there are some definition
                        // parameters left over.

                        // In this case we have less parameters than required, so we apply all
                        // parameters with default values.
                        if ord == Ordering::Less {
                            // Remove all the function definition parameters we just used as they
                            // aren’t needed anymore. We only do this in this branch because it
                            // doesn’t affect anything if all parameters have a value.
                            (0..positional_params_len).for_each(|_| {
                                def_params.remove(0);
                            });

                            // Use up as many of the definition parameters we have by using all
                            // those that have default values.

                            // Isolate.
                            let default_params = def_params.iter().filter_map(|p| {
                                if let crate::func::Param::WithDefault(d) = p {
                                    Some(d)
                                } else {
                                    None
                                }
                            });

                            // Apply.
                            for default_param in default_params {
                                func_state.set_var(
                                    default_param.name().clone(),
                                    default_param.val().clone().eval(&func_state)?,
                                );
                            }

                            // Prune.
                            def_params.retain(|p| {
                                if let crate::func::Param::WithDefault(_) = p {
                                    false
                                } else {
                                    true
                                }
                            });

                            // If everything has gone well, there should be no definition
                            // parameters left. However, if the caller has not specified enough
                            // parameters, there will be some left over.
                            if !def_params.is_empty() {
                                return Err(crate::eval::Error::NotEnoughFuncParams);
                            }
                        }
                    }
                    // In this case we have more input arguments than are defined on the function.
                    Ordering::Greater => {
                        return Err(crate::eval::Error::TooManyFuncParams);
                    }
                }

                func.body().clone().eval(&func_state)
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) enum Param {
    Named(NamedParam),
    Positional(PositionalParam),
}

impl Param {
    fn new(s: &str) -> nom::IResult<&str, Self> {
        Self::new_named(s).or_else(|_| Self::new_positional(s))
    }

    fn new_named(s: &str) -> nom::IResult<&str, Self> {
        NamedParam::new(s).map(|(s, p)| (s, Self::Named(p)))
    }

    fn new_positional(s: &str) -> nom::IResult<&str, Self> {
        PositionalParam::new(s).map(|(s, p)| (s, Self::Positional(p)))
    }
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct NamedParam {
    val: Expr,
    name: crate::IdentName,
}

impl NamedParam {
    fn new(s: &str) -> nom::IResult<&str, Self> {
        let (s, name) = crate::IdentName::new(s)?;
        let (s, _) = char('=')(s)?;
        let (s, val) = Expr::new(s)?;

        Ok((s, Self { name, val }))
    }
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct PositionalParam {
    val: Expr,
}

impl PositionalParam {
    fn new(s: &str) -> nom::IResult<&str, Self> {
        let (s, val) = Expr::new(s)?;
        Ok((s, Self { val }))
    }
}

#[cfg(test)]
mod param_tests {
    use super::*;

    #[test]
    fn named() {
        assert_eq!(
            Param::new("paramName=10"),
            Ok(("", Param::Named(NamedParam::new("paramName=10").unwrap().1)))
        );

        assert_eq!(
            NamedParam::new("foobar=100"),
            Ok((
                "",
                NamedParam {
                    name: crate::IdentName::new("foobar").unwrap().1,
                    val: Expr::new("100").unwrap().1
                }
            ))
        )
    }
}
