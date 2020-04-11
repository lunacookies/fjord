use nom::character::complete::char;

#[derive(Clone, Debug, PartialEq)]
pub(crate) enum Param {
    Named(NamedParam),
    Positional(PositionalParam),
}

impl Param {
    pub(crate) fn new(s: &str) -> nom::IResult<&str, Self> {
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
    val: crate::Expr,
    name: crate::IdentName,
}

impl NamedParam {
    fn new(s: &str) -> nom::IResult<&str, Self> {
        let (s, name) = crate::IdentName::new(s)?;
        let (s, _) = char('=')(s)?;
        let (s, val) = crate::Expr::new(s)?;

        Ok((s, Self { name, val }))
    }

    pub(crate) fn val(&self) -> &crate::Expr {
        &self.val
    }

    pub(crate) fn name(&self) -> &crate::IdentName {
        &self.name
    }
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct PositionalParam {
    val: crate::Expr,
}

impl PositionalParam {
    fn new(s: &str) -> nom::IResult<&str, Self> {
        let (s, val) = crate::Expr::new(s)?;
        Ok((s, Self { val }))
    }

    pub(crate) fn val(&self) -> &crate::Expr {
        &self.val
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
                    val: crate::Expr::new("100").unwrap().1
                }
            ))
        )
    }

    #[test]
    fn positional() {
        assert_eq!(
            Param::new("123"),
            Ok((
                "",
                Param::Positional(PositionalParam {
                    val: crate::Expr::new("123").unwrap().1
                })
            ))
        );

        assert_eq!(
            PositionalParam::new("\"Test\""),
            Ok((
                "",
                PositionalParam {
                    val: crate::Expr::new("\"Test\"").unwrap().1,
                }
            ))
        )
    }
}
