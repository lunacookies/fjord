use nom::{bytes::complete::tag, combinator::opt};

#[derive(Debug, PartialEq)]
pub(crate) struct FunctionDefParam<'text> {
    name: crate::Ident<'text>,
    name_space: &'text str,
    colon: &'text str,
    colon_space: &'text str,
    ty: crate::Ty<'text>,
    ty_space: &'text str,
    comma: Option<&'text str>,
    comma_space: &'text str,
}

impl<'text> FunctionDefParam<'text> {
    pub(crate) fn new(s: &'text str) -> nom::IResult<&'text str, Self> {
        let (s, name) = crate::Ident::new(s)?;
        let (s, name_space) = crate::take_whitespace0(s)?;

        let (s, colon) = tag(":")(s)?;
        let (s, colon_space) = crate::take_whitespace0(s)?;

        let (s, ty) = crate::Ty::new(s)?;
        let (s, ty_space) = crate::take_whitespace0(s)?;

        let (s, comma) = opt(tag(","))(s)?;
        let (s, comma_space) = crate::take_whitespace0(s)?;

        Ok((
            s,
            Self {
                name,
                name_space,
                colon,
                colon_space,
                ty,
                ty_space,
                comma,
                comma_space,
            },
        ))
    }
}

impl<'param> From<FunctionDefParam<'param>> for Vec<syntax::HighlightedSpan<'param>> {
    fn from(param: FunctionDefParam<'param>) -> Self {
        let mut output = vec![
            syntax::HighlightedSpan {
                text: param.name.name,
                group: Some(syntax::HighlightGroup::FunctionParam),
            },
            syntax::HighlightedSpan {
                text: param.name_space,
                group: None,
            },
            syntax::HighlightedSpan {
                text: param.colon,
                group: Some(syntax::HighlightGroup::Separator),
            },
            syntax::HighlightedSpan {
                text: param.colon_space,
                group: None,
            },
        ];

        output.extend(Vec::from(param.ty).into_iter().chain(std::iter::once(
            syntax::HighlightedSpan {
                text: param.ty_space,
                group: None,
            },
        )));

        if let Some(comma) = param.comma {
            output.push(syntax::HighlightedSpan {
                text: comma,
                group: Some(syntax::HighlightGroup::Separator),
            });
        }

        output.push(syntax::HighlightedSpan {
            text: param.comma_space,
            group: None,
        });

        output
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(
            FunctionDefParam::new("buf: &mut String, "),
            Ok((
                "",
                FunctionDefParam {
                    name: crate::Ident { name: "buf" },
                    name_space: "",
                    colon: ":",
                    colon_space: " ",
                    ty: crate::Ty::new("&mut String").unwrap().1,
                    ty_space: "",
                    comma: Some(","),
                    comma_space: " ",
                }
            ))
        )
    }
}
