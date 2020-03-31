use nom::{bytes::complete::tag, multi::many0};

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

        let (s, body) = crate::Expr::new(s)?;

        Ok((s, Self { params, body }))
    }

    pub(crate) fn params(&self) -> &[crate::IdentName] {
        &self.params
    }

    pub(crate) fn body(&self) -> &crate::Expr {
        &self.body
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
                    body: crate::Expr::Block(vec![crate::Item::new("123").unwrap().1]),
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
                    body: crate::Expr::Block(vec![
                        crate::Item::new("\"Hello, World!\"").unwrap().1
                    ]),
                }
            ))
        )
    }

    #[test]
    fn no_body() {
        assert_eq!(
            Func::new("fn {}"),
            Ok((
                "",
                Func {
                    params: vec![],
                    body: crate::Expr::Block(vec![])
                }
            ))
        )
    }

    #[test]
    fn multiple_body_lines() {
        assert_eq!(
            Func::new(
                "\
fn x {
    let otherName #x
    #otherName
}"
            ),
            Ok((
                "",
                Func {
                    params: vec![crate::IdentName::new("x").unwrap().1],
                    body: crate::Expr::Block(vec![
                        crate::Item::new("let otherName #x").unwrap().1,
                        crate::Item::new("#otherName").unwrap().1,
                    ])
                }
            ))
        )
    }
}
