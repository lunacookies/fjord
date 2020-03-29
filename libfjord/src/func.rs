use nom::{
    bytes::complete::tag,
    character::complete::char,
    multi::{many0, separated_list},
};

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct Func {
    params: Vec<crate::IdentName>,
    body: Vec<crate::Item>,
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

        let (s, body) = separated_list(
            |s| {
                // Items in a function are separated by newlines, plus zero or more whitespace (for
                // indentation).
                let (s, newline) = char('\n')(s)?;
                let (s, _) = crate::take_whitespace(s)?;

                Ok((s, newline))
            },
            crate::Item::new,
        )(s)?;

        let (s, _) = crate::take_whitespace(s)?;
        let (s, _) = char('}')(s)?;

        Ok((s, Self { params, body }))
    }

    pub(crate) fn params(&self) -> &[crate::IdentName] {
        &self.params
    }

    pub(crate) fn body(&self) -> &[crate::Item] {
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
                    body: vec![crate::Item::new("123").unwrap().1],
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
                    body: vec![crate::Item::new("\"Hello, World!\"").unwrap().1]
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
                    body: vec![]
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
                    body: vec![
                        crate::Item::new("let otherName #x").unwrap().1,
                        crate::Item::new("#otherName").unwrap().1,
                    ]
                }
            ))
        )
    }
}
