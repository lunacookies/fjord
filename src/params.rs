//! Data structures for representing function parameters, both at definition and call sites of
//! functions.

#[derive(Debug, PartialEq)]
pub(crate) struct CompleteParam {
    pub name: crate::IdentName,
    pub val: crate::Expr,
}

/// The errors that can occur when matching up function definition parameters with function call
/// parameters.
#[derive(Debug, thiserror::Error, PartialEq)]
pub enum Error {
    /// when too many function parameters are specified
    #[error("too many function parameters were provided")]
    TooManyFuncParams,
    /// when not enough function parameters are specified
    #[error("not enough function parameters were provided")]
    NotEnoughFuncParams,
}

pub(crate) fn eval(
    call_params: impl ExactSizeIterator<Item = crate::Expr>,
    def_params: impl ExactSizeIterator<Item = crate::IdentName>,
) -> Result<Vec<CompleteParam>, Error> {
    use std::cmp::Ordering;

    match call_params.len().cmp(&def_params.len()) {
        Ordering::Equal => Ok(def_params
            .zip(call_params)
            .map(|(name, val)| CompleteParam { name, val })
            .collect()),
        Ordering::Greater => Err(Error::TooManyFuncParams),
        Ordering::Less => Err(Error::NotEnoughFuncParams),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn too_many_params() {
        assert_eq!(
            eval(
                vec![crate::Expr::Bool(true), crate::Expr::Bool(true)].into_iter(),
                vec![crate::IdentName::new("foo").unwrap().1].into_iter(),
            ),
            Err(Error::TooManyFuncParams)
        );
    }

    #[test]
    fn not_enough_params() {
        assert_eq!(
            eval(
                vec![crate::Expr::Number(50)].into_iter(),
                vec![
                    crate::IdentName::new("param1").unwrap().1,
                    crate::IdentName::new("param2").unwrap().1,
                ]
                .into_iter(),
            ),
            Err(Error::NotEnoughFuncParams)
        )
    }

    #[test]
    fn eval_params() {
        assert_eq!(
            eval(
                vec![
                    crate::Expr::Number(100),
                    crate::Expr::Str("file.txt".into())
                ]
                .into_iter(),
                vec![
                    crate::IdentName::new("count").unwrap().1,
                    crate::IdentName::new("path").unwrap().1,
                ]
                .into_iter()
            ),
            Ok(vec![
                CompleteParam {
                    name: crate::IdentName::new("count").unwrap().1,
                    val: crate::Expr::Number(100)
                },
                CompleteParam {
                    name: crate::IdentName::new("path").unwrap().1,
                    val: crate::Expr::Str("file.txt".into())
                }
            ])
        );
    }
}
