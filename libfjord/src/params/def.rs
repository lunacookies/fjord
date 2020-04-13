use nom::character::complete::char;

#[derive(Clone, Debug, PartialEq)]
pub enum Param {
    WithDefault(ParamWithDefault),
    WithoutDefault(ParamWithoutDefault),
}

impl Param {
    pub(crate) fn new(s: &str) -> nom::IResult<&str, Self> {
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
pub struct ParamWithDefault {
    pub name: crate::IdentName,
    pub val: crate::Expr,
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
pub struct ParamWithoutDefault {
    pub name: crate::IdentName,
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
