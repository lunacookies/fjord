//! Data structures for representing function parameters at their definitions.

use nom::character::complete::char;

/// Any kind of parameter on a function definition.
#[derive(Clone, Debug, PartialEq)]
pub struct Param {
    /// the parameter’s name
    pub name: crate::IdentName,
    /// the parameter’s default value (not all parameters have default values, necessarily)
    pub val: Option<crate::Expr>,
}

impl Param {
    pub(crate) fn new(s: &str) -> nom::IResult<&str, Self> {
        Self::new_with_default(s).or_else(|_| Self::new_without_default(s))
    }

    fn new_with_default(s: &str) -> nom::IResult<&str, Self> {
        let (s, name) = crate::IdentName::new(s)?;
        let (s, _) = char('=')(s)?;
        let (s, val) = crate::Expr::new(s)?;

        Ok((
            s,
            Self {
                name,
                val: Some(val),
            },
        ))
    }

    fn new_without_default(s: &str) -> nom::IResult<&str, Self> {
        let (s, name) = crate::IdentName::new(s)?;
        Ok((s, Self { name, val: None }))
    }

    pub(crate) fn has_default(&self) -> bool {
        self.val.is_some()
    }
}

#[cfg(test)]
mod param_tests {
    use super::*;

    #[test]
    fn with_default() {
        assert_eq!(
            Param::new_with_default("paramName=5"),
            Ok((
                "",
                Param {
                    name: crate::IdentName::new("paramName=5").unwrap().1,
                    val: Some(crate::Expr::Number(5))
                }
            ))
        );

        assert_eq!(
            Param::new("foobar=\"test\""),
            Ok((
                "",
                Param {
                    name: crate::IdentName::new("foobar").unwrap().1,
                    val: Some(crate::Expr::Str("test".into())),
                }
            ))
        );
    }

    #[test]
    fn without_default() {
        assert_eq!(
            Param::new_without_default("paramName"),
            Ok((
                "",
                Param {
                    name: crate::IdentName::new("paramName").unwrap().1,
                    val: None
                }
            ))
        );

        assert_eq!(
            Param::new("foobar"),
            Ok((
                "",
                Param {
                    name: crate::IdentName::new("foobar").unwrap().1,
                    val: None,
                }
            ))
        );
    }
}
