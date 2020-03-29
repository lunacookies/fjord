use nom::{bytes::complete::tag, character::complete::char, multi::many0};

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct Func {
    params: Vec<crate::IdentName>,
    body: crate::Expr,
}

impl Func {
    pub(crate) fn new(s: &str) -> nom::IResult<&str, Self> {
        let (s, _) = tag("fn")(s)?;
        let (s, _) = crate::take_whitespace1(s)?;

        let (s, params) = many0(|s| {
            let (s, param) = crate::IdentName::new(s)?;
            let (s, _) = crate::take_whitespace1(s)?;
            Ok((s, param))
        })(s)?;

        let (s, _) = char('{')(s)?;
        let (s, _) = crate::take_whitespace(s)?;

        let (s, body) = crate::Expr::new(s)?;

        let (s, _) = crate::take_whitespace(s)?;
        let (s, _) = char('}')(s)?;

        Ok((s, Self { params, body }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_params() {
        assert_eq!(
            Func::new("fn { 123 }",),
            Ok((
                "",
                Func {
                    params: vec![],
                    body: crate::Expr::Number(123),
                }
            )),
        )
    }

    #[test]
    fn some_params() {
        assert_eq!(
            Func::new(
                "\
fn param1 param2 {
    \"Hello, World!\"
}"
            ),
            Ok((
                "",
                Func {
                    params: vec![
                        crate::IdentName::new("param1").unwrap().1,
                        crate::IdentName::new("param2").unwrap().1
                    ],
                    body: crate::Expr::Str("Hello, World!".into())
                }
            ))
        )
    }
}

impl crate::eval::Eval for Func {
    fn eval(self, state: &crate::eval::State) -> crate::eval::EvalResult {
        self.body.eval(state)
    }
}
