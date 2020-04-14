use {
    crate::params::{call, def},
    nom::{bytes::complete::tag, multi::many0},
};

/// A function literal. Note that this isn’t a ‘real’ function literal, as it is not part of
/// [`Expr`](enum.Expr.html), and therefore can only be used when defining a function.
#[derive(Clone, Debug, PartialEq)]
pub struct Func {
    /// its parameters
    pub params: Vec<def::Param>,
    /// its body is simply an expression
    pub body: crate::Expr,
}

impl Func {
    pub(crate) fn new(s: &str) -> nom::IResult<&str, Self> {
        let (s, params) = many0(|s| {
            let (s, param) = def::Param::new(s)?;
            let (s, _) = crate::take_whitespace1(s)?;
            Ok((s, param))
        })(s)?;

        let (s, _) = tag("::")(s)?;
        let (s, _) = crate::take_whitespace(s)?;

        let (s, body) = crate::Expr::new(s)?;

        Ok((s, Self { params, body }))
    }

    pub(crate) fn eval(
        self,
        call_params: Vec<call::Param>,
        state: &crate::eval::State,
    ) -> crate::eval::EvalResult {
        let def_params = self.params;
        let complete_params = crate::params::eval(call_params, def_params)?;

        let mut func_state = state.new_child();

        for param in complete_params {
            func_state.set_var(param.name().clone(), param.val().clone().eval(&func_state)?)
        }

        self.body.eval(&func_state)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_params() {
        assert_eq!(
            Func::new(":: 123",),
            Ok((
                "",
                Func {
                    params: vec![],
                    body: crate::Expr::new("123").unwrap().1,
                }
            )),
        )
    }

    #[test]
    fn some_params() {
        assert_eq!(
            Func::new("param1 param2 :: \"Hello, World!\""),
            Ok((
                "",
                Func {
                    params: vec![
                        def::Param::new("param1").unwrap().1,
                        def::Param::new("param2").unwrap().1
                    ],
                    body: crate::Expr::new("\"Hello, World!\"").unwrap().1
                }
            ))
        )
    }

    #[test]
    fn no_body() {
        assert_eq!(
            Func::new(":: {}"),
            Ok((
                "",
                Func {
                    params: vec![],
                    body: crate::Expr::new("{}").unwrap().1
                }
            ))
        )
    }

    #[test]
    fn multiple_body_lines() {
        assert_eq!(
            Func::new(
                "\
x :: {
    otherName = .x
    .otherName
}"
            ),
            Ok((
                "",
                Func {
                    params: vec![def::Param::new("x").unwrap().1],
                    body: crate::Expr::new(
                        "\
{
    otherName = .x
    .otherName
}"
                    )
                    .unwrap()
                    .1
                }
            ))
        )
    }
}
