use nom::{bytes::complete::tag, combinator::opt};

#[derive(Debug, PartialEq)]
pub(crate) struct FunctionParam<'text> {
    name: crate::Ident<'text>,
    name_space: &'text str,
    colon: &'text str,
    colon_space: &'text str,
    ty: crate::Ty<'text>,
    ty_space: &'text str,
    comma: Option<&'text str>,
    comma_space: &'text str,
}

impl<'text> FunctionParam<'text> {
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

impl<'fp> From<FunctionParam<'fp>> for Vec<syntax::HighlightedSpan<'fp>> {
    fn from(fp: FunctionParam<'fp>) -> Self {
        let mut output = vec![
            syntax::HighlightedSpan {
                text: fp.name.name,
                group: Some(syntax::HighlightGroup::FunctionParam),
            },
            syntax::HighlightedSpan {
                text: fp.name_space,
                group: None,
            },
            syntax::HighlightedSpan {
                text: fp.colon,
                group: Some(syntax::HighlightGroup::Separator),
            },
            syntax::HighlightedSpan {
                text: fp.colon_space,
                group: None,
            },
        ];

        output.extend(Vec::from(fp.ty).into_iter().chain(std::iter::once(
            syntax::HighlightedSpan {
                text: fp.ty_space,
                group: None,
            },
        )));

        if let Some(comma) = fp.comma {
            output.push(syntax::HighlightedSpan {
                text: comma,
                group: Some(syntax::HighlightGroup::Separator),
            });
        }

        output.push(syntax::HighlightedSpan {
            text: fp.comma_space,
            group: None,
        });

        output
    }
}
