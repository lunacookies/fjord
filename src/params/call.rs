//! Various data structures for representing function call parameters.

use nom::character::complete::char;

/// Any kind of parameter of a function call.
#[derive(Clone, Debug, PartialEq)]
pub struct Param {
    /// the parameter’s value
    pub val: crate::Expr,
    /// the parameter’s name (optional because not all parameters in a function call have to be
    /// named)
    pub name: Option<crate::IdentName>,
}

impl Param {
    pub(crate) fn new(s: &str) -> nom::IResult<&str, Self> {
        Self::new_named(s).or_else(|_| Self::new_positional(s))
    }

    fn new_named(s: &str) -> nom::IResult<&str, Self> {
        let (s, name) = crate::IdentName::new(s)?;
        let (s, _) = char('=')(s)?;
        let (s, val) = crate::Expr::new(s)?;

        Ok((
            s,
            Self {
                val,
                name: Some(name),
            },
        ))
    }

    fn new_positional(s: &str) -> nom::IResult<&str, Self> {
        let (s, val) = crate::Expr::new(s)?;
        Ok((s, Self { val, name: None }))
    }

    pub(crate) fn is_named(&self) -> bool {
        self.name.is_some()
    }
}

#[cfg(test)]
mod param_tests {
    use super::*;

    #[test]
    fn named() {
        assert_eq!(
            Param::new_named("paramName=10"),
            Ok((
                "",
                Param {
                    val: crate::Expr::Number(10),
                    name: Some(crate::IdentName::new("paramName=10").unwrap().1),
                }
            ))
        );

        assert_eq!(
            Param::new("foobar=100"),
            Ok((
                "",
                Param {
                    val: crate::Expr::Number(100),
                    name: Some(crate::IdentName::new("foobar").unwrap().1),
                }
            ))
        )
    }

    #[test]
    fn positional() {
        assert_eq!(
            Param::new_positional("123"),
            Ok((
                "",
                Param {
                    val: crate::Expr::Number(123),
                    name: None,
                }
            ))
        );

        assert_eq!(
            Param::new("\"Test\""),
            Ok((
                "",
                Param {
                    val: crate::Expr::Str("Test".into()),
                    name: None,
                }
            ))
        )
    }
}
