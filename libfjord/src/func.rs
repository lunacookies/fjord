use nom::{bytes::complete::tag, character::complete::char, multi::many0};

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct Func {
    params: Vec<Param>,
    body: crate::Expr,
}

impl Func {
    pub(crate) fn new(s: &str) -> nom::IResult<&str, Self> {
        let (s, _) = tag("fn")(s)?;
        let (s, _) = crate::take_whitespace1(s)?;

        let (s, params) = many0(|s| {
            let (s, param) = Param::new(s)?;
            let (s, _) = crate::take_whitespace1(s)?;
            Ok((s, param))
        })(s)?;

        let (s, body) = crate::Expr::new(s)?;

        Ok((s, Self { params, body }))
    }

    pub(crate) fn params(&self) -> &[Param] {
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
                        Param::new("param1").unwrap().1,
                        Param::new("param2").unwrap().1
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
    let otherName .x
    .otherName
}"
            ),
            Ok((
                "",
                Func {
                    params: vec![Param::new("x").unwrap().1],
                    body: crate::Expr::Block(vec![
                        crate::Item::new("let otherName .x").unwrap().1,
                        crate::Item::new(".otherName").unwrap().1,
                    ])
                }
            ))
        )
    }
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) enum Param {
    WithDefault(ParamWithDefault),
    WithoutDefault(ParamWithoutDefault),
}

impl Param {
    fn new(s: &str) -> nom::IResult<&str, Self> {
        Self::new_with_default(s).or_else(|_| Self::new_without_default(s))
    }

    fn new_with_default(s: &str) -> nom::IResult<&str, Self> {
        ParamWithDefault::new(s).map(|(s, p)| (s, Self::WithDefault(p)))
    }

    fn new_without_default(s: &str) -> nom::IResult<&str, Self> {
        ParamWithoutDefault::new(s).map(|(s, p)| (s, Self::WithoutDefault(p)))
    }

    pub(crate) fn name(&self) -> &crate::IdentName {
        match self {
            Self::WithDefault(p) => &p.name,
            Self::WithoutDefault(p) => &p.name,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct ParamWithDefault {
    name: crate::IdentName,
    val: crate::Expr,
}

impl ParamWithDefault {
    fn new(s: &str) -> nom::IResult<&str, Self> {
        let (s, name) = crate::IdentName::new(s)?;
        let (s, _) = char('=')(s)?;
        let (s, val) = crate::Expr::new(s)?;

        Ok((s, Self { name, val }))
    }

    pub(crate) fn name(&self) -> &crate::IdentName {
        &self.name
    }

    pub(crate) fn val(&self) -> &crate::Expr {
        &self.val
    }
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct ParamWithoutDefault {
    name: crate::IdentName,
}

impl ParamWithoutDefault {
    fn new(s: &str) -> nom::IResult<&str, Self> {
        let (s, name) = crate::IdentName::new(s)?;
        Ok((s, Self { name }))
    }
}

#[cfg(test)]
mod param_tests {
    use super::*;

    #[test]
    fn with_default() {
        assert_eq!(
            Param::new("paramName=5"),
            Ok((
                "",
                Param::WithDefault(ParamWithDefault::new("paramName=5").unwrap().1)
            ))
        );

        assert_eq!(
            ParamWithDefault::new("foobar=\"test\""),
            Ok((
                "",
                ParamWithDefault {
                    name: crate::IdentName::new("foobar").unwrap().1,
                    val: crate::Expr::new("\"test\"").unwrap().1,
                }
            ))
        );
    }

    #[test]
    fn without_default() {
        assert_eq!(
            Param::new("paramName"),
            Ok((
                "",
                Param::WithoutDefault(ParamWithoutDefault::new("paramName").unwrap().1)
            ))
        );

        assert_eq!(
            ParamWithoutDefault::new("foobar"),
            Ok((
                "",
                ParamWithoutDefault {
                    name: crate::IdentName::new("foobar").unwrap().1,
                }
            ))
        );
    }
}
