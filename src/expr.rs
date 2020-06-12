use nom::{
    branch::alt,
    bytes::complete::{tag, take_till, take_while1},
    character::complete::char,
    multi::{many0, separated_list},
    sequence::delimited,
};

/// An expression.
#[derive(Clone, Debug, PartialEq)]
pub enum Expr {
    /// a boolean literal
    Bool(bool),
    /// a number literal
    Number(crate::Number),
    /// a string literal
    Str(String),
    /// a format string
    FStr(String, Vec<(Self, String)>),
    /// a [block expression](https://doc.rust-lang.org/reference/expressions/block-expr.html)
    Block(Vec<crate::Item>),
    /// a variable usage (not definition)
    Var(crate::IdentName),
    /// an if expression
    If {
        /// the condition for the true case to take place
        condition: Box<Self>,
        /// the if expression evaluates to this if the condition is true
        true_case: Box<Self>,
        /// the if expression evaluates to this if the condition is false
        false_case: Box<Self>,
    },
    /// a function call
    FuncCall {
        /// the name of the function being called
        name: crate::IdentName,
        /// the parameters given to the function
        params: Vec<Self>,
    },
    /// an expression surrounded by parentheses
    Parens(Box<Self>),
}

impl Expr {
    pub(crate) fn new(s: &str) -> nom::IResult<&str, Self> {
        Self::new_bool(s)
            .or_else(|_| Self::new_number(s))
            .or_else(|_| Self::new_str(s))
            .or_else(|_| Self::new_fstr(s))
            .or_else(|_| Self::new_block(s))
            .or_else(|_| Self::new_var(s))
            .or_else(|_| Self::new_if(s))
            .or_else(|_| Self::new_func_call(s))
            .or_else(|_| Self::new_parens(s))
    }

    fn new_bool(s: &str) -> nom::IResult<&str, Self> {
        let (s, boolean) = alt((tag("true"), tag("false")))(s)?;

        let boolean = match boolean {
            "true" => true,
            _ => false,
        };

        Ok((s, Self::Bool(boolean)))
    }

    fn new_number(s: &str) -> nom::IResult<&str, Self> {
        let (s, n) = take_while1(|c: char| c.is_ascii_digit())(s)?;

        // This cannot fail because we know that n is all digits.
        let n = crate::Number::from_str_radix(n, 10).unwrap();

        Ok((s, Self::Number(n)))
    }

    fn new_str(s: &str) -> nom::IResult<&str, Self> {
        let (s, text) = delimited(char('"'), take_till(|c| c == '"'), char('"'))(s)?;

        Ok((s, Self::Str(text.into())))
    }

    fn new_fstr(s: &str) -> nom::IResult<&str, Self> {
        let (s, _) = tag("f\"")(s)?;

        let literal_parser = take_till(|c| c == '{' || c == '"');

        let (s, before_first_interpolation) = literal_parser(s)?;

        let (s, interpolations_and_literals) = many0(|s| {
            let (s, interpolation) = delimited(char('{'), Self::new, char('}'))(s)?;
            let (s, literal) = literal_parser(s)?;

            Ok((s, (interpolation, literal.into())))
        })(s)?;

        let (s, _) = char('"')(s)?;

        Ok((
            s,
            Self::FStr(
                before_first_interpolation.into(),
                interpolations_and_literals,
            ),
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

        Ok((s, Self::Block(items)))
    }

    fn new_var(s: &str) -> nom::IResult<&str, Self> {
        let (s, _) = char('.')(s)?;
        let (s, name) = crate::IdentName::new(s)?;

        Ok((s, Self::Var(name)))
    }

    fn new_if(s: &str) -> nom::IResult<&str, Self> {
        let (s, _) = tag("if")(s)?;
        let (s, _) = crate::take_whitespace1(s)?;

        let (s, condition) = Self::new(s)?;
        let (s, _) = crate::take_whitespace1(s)?;

        let (s, _) = tag("then")(s)?;
        let (s, _) = crate::take_whitespace1(s)?;

        let (s, true_case) = Self::new(s)?;
        let (s, _) = crate::take_whitespace1(s)?;

        let (s, _) = tag("else")(s)?;
        let (s, _) = crate::take_whitespace1(s)?;

        let (s, false_case) = Self::new(s)?;

        Ok((
            s,
            Self::If {
                condition: Box::new(condition),
                true_case: Box::new(true_case),
                false_case: Box::new(false_case),
            },
        ))
    }

    fn new_func_call(s: &str) -> nom::IResult<&str, Self> {
        let (s, name) = crate::IdentName::new(s)?;

        let (s, params) = many0(|s| {
            let (s, _) = crate::take_whitespace1(s)?;
            Self::new(s)
        })(s)?;

        Ok((s, Self::FuncCall { name, params }))
    }

    fn new_parens(s: &str) -> nom::IResult<&str, Self> {
        let (s, expr) = delimited(char('('), Self::new, char(')'))(s)?;

        Ok((s, Self::Parens(Box::new(expr))))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bool() {
        assert_eq!(Expr::new_bool("true"), Ok(("", Expr::Bool(true))));
        assert_eq!(Expr::new("false"), Ok(("", Expr::Bool(false))));
    }

    #[test]
    fn number() {
        assert_eq!(Expr::new_number("123"), Ok(("", Expr::Number(123))));
        assert_eq!(Expr::new("123"), Ok(("", Expr::Number(123))));
    }

    #[test]
    fn str() {
        assert_eq!(
            Expr::new_str("\"Hello, World!\""),
            Ok(("", Expr::Str("Hello, World!".into())))
        );

        assert_eq!(Expr::new_str("\"🦀\""), Ok(("", Expr::Str("🦀".into()))));

        assert_eq!(
            Expr::new("\"foobar\""),
            Ok(("", Expr::Str("foobar".into())))
        );
    }

    mod fstr {
        use super::*;

        #[test]
        fn no_interpolations() {
            assert_eq!(
                Expr::new_fstr("f\"some text\""),
                Ok(("", Expr::FStr("some text".into(), vec![])))
            );

            assert_eq!(
                Expr::new("f\"test\""),
                Ok(("", Expr::FStr("test".into(), vec![])))
            );
        }

        #[test]
        fn interpolation_surrounded_by_literals() {
            assert_eq!(
                Expr::new_fstr("f\"Hello, {.person}!\""),
                Ok((
                    "",
                    Expr::FStr(
                        "Hello, ".into(),
                        vec![(
                            Expr::Var(crate::IdentName::new("person").unwrap().1),
                            "!".into()
                        )]
                    )
                ))
            );

            assert_eq!(
                Expr::new("f\"Your user, {.username}, has {.remainingDays} free days left.\""),
                Ok((
                    "",
                    Expr::FStr(
                        "Your user, ".into(),
                        vec![
                            (
                                Expr::Var(crate::IdentName::new("username").unwrap().1),
                                ", has ".into()
                            ),
                            (
                                Expr::Var(crate::IdentName::new("remainingDays").unwrap().1),
                                " free days left.".into()
                            )
                        ]
                    )
                ))
            );
        }

        #[test]
        fn interpolation_followed_by_literal() {
            assert_eq!(
                Expr::new_fstr("f\"{.randWord} is the word of the day\""),
                Ok((
                    "",
                    Expr::FStr(
                        "".into(),
                        vec![(
                            Expr::Var(crate::IdentName::new("randWord").unwrap().1),
                            " is the word of the day".into()
                        )]
                    )
                ))
            );

            assert_eq!(
                Expr::new("f\"{.latestMovie}: in cinemas now\""),
                Ok((
                    "",
                    Expr::FStr(
                        "".into(),
                        vec![(
                            Expr::Var(crate::IdentName::new("latestMovie").unwrap().1),
                            ": in cinemas now".into()
                        )]
                    )
                ))
            );
        }

        #[test]
        fn interpolation_preceded_by_literal() {
            assert_eq!(
                Expr::new_fstr("f\"Good day, {.user}\""),
                Ok((
                    "",
                    Expr::FStr(
                        "Good day, ".into(),
                        vec![(
                            Expr::Var(crate::IdentName::new("user").unwrap().1),
                            "".into()
                        )]
                    )
                ))
            );

            assert_eq!(
                Expr::new_fstr("f\"Error in module {.moduleName}: {.error}\""),
                Ok((
                    "",
                    Expr::FStr(
                        "Error in module ".into(),
                        vec![
                            (
                                Expr::Var(crate::IdentName::new("moduleName").unwrap().1),
                                ": ".into()
                            ),
                            (
                                Expr::Var(crate::IdentName::new("error").unwrap().1),
                                "".into()
                            )
                        ]
                    )
                ))
            );
        }
    }

    mod block {
        use super::*;

        #[test]
        fn basic() {
            assert_eq!(
                Expr::new_block("{ 25 }"),
                Ok(("", Expr::Block(vec![crate::Item::new("25").unwrap().1])))
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
                    Expr::Block(vec![
                        crate::Item::new("foobar = \"Hello, World!\"").unwrap().1,
                        crate::Item::new(".foobar").unwrap().1,
                    ])
                ))
            );
        }

        #[test]
        fn only_variable() {
            assert_eq!(
                Expr::new("{myVar = 5}"),
                Ok((
                    "",
                    Expr::Block(vec![crate::Item::new("myVar = 5").unwrap().1])
                ))
            )
        }
    }

    #[test]
    fn var() {
        assert_eq!(
            Expr::new_var(".myVar"),
            Ok(("", Expr::Var(crate::IdentName::new("myVar").unwrap().1)))
        );
        assert_eq!(
            Expr::new(".foobar"),
            Ok(("", Expr::Var(crate::IdentName::new("foobar").unwrap().1)))
        );
    }

    #[test]
    fn if_expr() {
        assert_eq!(
            Expr::new_if("if true then 1 else 0"),
            Ok((
                "",
                Expr::If {
                    condition: Box::new(Expr::Bool(true)),
                    true_case: Box::new(Expr::Number(1)),
                    false_case: Box::new(Expr::Number(0))
                }
            ))
        );

        assert_eq!(
            Expr::new("if .b then false else true"),
            Ok((
                "",
                Expr::If {
                    condition: Box::new(Expr::Var(crate::IdentName::new("b").unwrap().1)),
                    true_case: Box::new(Expr::Bool(false)),
                    false_case: Box::new(Expr::Bool(true))
                }
            ))
        )
    }

    #[test]
    fn no_args() {
        assert_eq!(
            Expr::new_func_call("funcName"),
            Ok((
                "",
                Expr::FuncCall {
                    name: crate::IdentName::new("funcName").unwrap().1,
                    params: vec![]
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
                Expr::FuncCall {
                    name: crate::IdentName::new("addThree").unwrap().1,
                    params: vec![
                        Expr::new("1").unwrap().1,
                        Expr::new("7").unwrap().1,
                        Expr::new("4").unwrap().1
                    ]
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
                Expr::FuncCall {
                    name: crate::IdentName::new("sqrt").unwrap().1,
                    params: vec![Expr::new("5").unwrap().1]
                }
            ))
        )
    }

    #[test]
    fn parens() {
        assert_eq!(
            Expr::new_parens("(1000)"),
            Ok(("", Expr::Parens(Box::new(Expr::Number(1000)))))
        );

        assert_eq!(
            Expr::new("(getUserInput .stdout)"),
            Ok((
                "",
                Expr::Parens(Box::new(Expr::FuncCall {
                    name: crate::IdentName::new("getUserInput").unwrap().1,
                    params: vec![Expr::Var(crate::IdentName::new("stdout").unwrap().1),]
                }))
            ))
        );
    }
}

impl Expr {
    pub(crate) fn eval(self, state: &crate::eval::State<'_>) -> crate::eval::EvalResult {
        match self {
            Self::Bool(b) => Ok(crate::eval::OutputExpr::Bool(b)),
            Self::Number(n) => Ok(crate::eval::OutputExpr::Number(n)),
            Self::Str(s) => Ok(crate::eval::OutputExpr::Str(s)),
            Self::FStr(before_first_interpolation, interpolations_and_literals) => {
                let mut len = before_first_interpolation.len();

                // Evaluate each of the interpolations, and turn the result of these interpolations
                // into Strings.
                let interpolations_and_literals: Vec<_> = interpolations_and_literals
                    .into_iter()
                    .map::<Result<_, crate::eval::Error>, _>(|(interpolation, s)| {
                        let interpolation = interpolation.eval(state)?.format();

                        // HACK: It’s kind of hacky to mutate state inside of a call to .map, but
                        // this is the easiest way.
                        len += interpolation.len();
                        len += s.len();

                        Ok((interpolation, s))
                    })
                    .collect::<Result<_, _>>()?;

                // Create a string to hold the f-string’s output with the length we’ve kept track
                // of.
                let mut output = String::with_capacity(len);

                // Push all of the strings we now have onto the output String.

                output.push_str(&before_first_interpolation);

                for (interpolation, s) in &interpolations_and_literals {
                    output.push_str(interpolation);
                    output.push_str(s);
                }

                Ok(crate::eval::OutputExpr::Str(output))
            }
            Self::Block(b) => {
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
            Self::Var(name) => match state.get_var(&name) {
                Some(val) => Ok(val.clone()),
                None => Err(crate::eval::Error::VarNotFound),
            },
            Self::If {
                condition,
                true_case,
                false_case,
            } => match condition.eval(state)? {
                crate::eval::OutputExpr::Bool(true) => true_case.eval(state),
                crate::eval::OutputExpr::Bool(false) => false_case.eval(state),
                _ => Err(crate::eval::Error::NonBoolCond),
            },
            Self::FuncCall {
                name,
                params: call_params,
            } => {
                // If a function has been defined with this name, then evaluate it and return.
                if let Some(func) = state.get_func(&name) {
                    return func.clone().eval(call_params, state);
                }

                // If a command exists with the name of the function call, then execute the
                // command.
                if let Some(command_name) = state.get_command(&name) {
                    let status = std::process::Command::new(command_name)
                        .status()
                        .map_err(|_| crate::eval::Error::CommandFailure)?;

                    // Return an error if the exit code isn’t 0; otherwise return Unit.
                    return match status.code() {
                        Some(code) if code != 0 => Err(crate::eval::Error::NonZeroExitCode(code)),
                        _ => Ok(crate::eval::OutputExpr::Unit),
                    };
                }

                // In this case no function or command exists with the name of the function call,
                // so we return an error saying as such.
                Err(crate::eval::Error::FuncOrCommandNotFound(name))
            }
            Self::Parens(e) => e.eval(state),
        }
    }
}

#[cfg(test)]
mod eval_tests {
    use super::*;

    #[test]
    fn bool() {
        let commands = crate::Commands::default();
        let state = crate::eval::State::new_root(&commands);

        assert_eq!(
            Expr::Bool(false).eval(&state),
            Ok(crate::eval::OutputExpr::Bool(false))
        );
    }

    #[test]
    fn number() {
        let commands = crate::Commands::default();
        let state = crate::eval::State::new_root(&commands);

        assert_eq!(
            Expr::Number(100).eval(&state),
            Ok(crate::eval::OutputExpr::Number(100))
        );
    }

    #[test]
    fn str() {
        let commands = crate::Commands::default();
        let state = crate::eval::State::new_root(&commands);

        assert_eq!(
            Expr::Str("Hello, World!".into()).eval(&state),
            Ok(crate::eval::OutputExpr::Str("Hello, World!".into()))
        );
    }

    #[test]
    fn fstr() {
        let commands = crate::Commands::default();
        let state = crate::eval::State::new_root(&commands);

        assert_eq!(
            Expr::FStr(
                "The number is ".into(),
                vec![(Expr::Number(100), "!".into())]
            )
            .eval(&state),
            Ok(crate::eval::OutputExpr::Str("The number is 100!".into()))
        );
    }

    #[test]
    fn block() {
        let commands = crate::Commands::default();
        let state = crate::eval::State::new_root(&commands);

        assert_eq!(
            Expr::Block(vec![
                crate::Item::Binding {
                    name: crate::IdentName::new("foo").unwrap().1,
                    val: crate::BindingVal::Var(Expr::Number(5))
                },
                crate::Item::Expr(Expr::Var(crate::IdentName::new("foo").unwrap().1))
            ])
            .eval(&state),
            Ok(crate::eval::OutputExpr::Number(5))
        );
    }

    #[test]
    fn var() {
        let commands = crate::Commands::default();

        let mut state = crate::eval::State::new_root(&commands);
        state.set_var(
            crate::IdentName::new("name").unwrap().1,
            crate::eval::OutputExpr::Str("John Smith".into()),
        );

        assert_eq!(
            Expr::Var(crate::IdentName::new("name").unwrap().1).eval(&state),
            Ok(crate::eval::OutputExpr::Str("John Smith".into()))
        );
    }

    #[test]
    fn if_expr() {
        let commands = crate::Commands::default();
        let state = crate::eval::State::new_root(&commands);

        assert_eq!(
            Expr::If {
                condition: Box::new(Expr::Bool(true)),
                true_case: Box::new(Expr::Number(25)),
                false_case: Box::new(Expr::Number(50)),
            }
            .eval(&state),
            Ok(crate::eval::OutputExpr::Number(25))
        );
    }

    #[test]
    fn func_call() {
        let commands = crate::Commands::default();

        let mut state = crate::eval::State::new_root(&commands);
        state.set_func(
            crate::IdentName::new("identity").unwrap().1,
            crate::Func {
                params: vec![crate::IdentName::new("x").unwrap().1],
                body: Expr::Var(crate::IdentName::new("x").unwrap().1),
            },
        );

        assert_eq!(
            Expr::FuncCall {
                name: crate::IdentName::new("identity").unwrap().1,
                params: vec![Expr::Str("✅".into()),]
            }
            .eval(&state),
            Ok(crate::eval::OutputExpr::Str("✅".into()))
        );
    }

    #[test]
    fn parens() {
        let commands = crate::Commands::default();
        let state = crate::eval::State::new_root(&commands);

        assert_eq!(
            Expr::Parens(Box::new(Expr::Number(100))).eval(&state),
            Ok(crate::eval::OutputExpr::Number(100))
        );
    }
}
