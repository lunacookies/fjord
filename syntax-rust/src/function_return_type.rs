use nom::bytes::complete::tag;

pub(crate) struct FunctionReturnType<'text> {
    arrow: &'text str,
    arrow_space: &'text str,
    ty: crate::Ty<'text>,
}

impl<'text> FunctionReturnType<'text> {
    pub(crate) fn new(s: &'text str) -> nom::IResult<&'text str, Self> {
        let (s, arrow) = tag("->")(s)?;
        let (s, arrow_space) = crate::take_whitespace0(s)?;
        let (s, ty) = crate::Ty::new(s)?;

        Ok((
            s,
            Self {
                arrow,
                arrow_space,
                ty,
            },
        ))
    }
}

impl<'frt> From<FunctionReturnType<'frt>> for Vec<syntax::HighlightedSpan<'frt>> {
    fn from(frt: FunctionReturnType<'frt>) -> Self {
        let mut output = vec![
            syntax::HighlightedSpan {
                text: frt.arrow,
                group: Some(syntax::HighlightGroup::Separator),
            },
            syntax::HighlightedSpan {
                text: frt.arrow_space,
                group: None,
            },
        ];

        // The type may expand to several HighlightedSpans, so we need to extend here.
        output.extend(Vec::from(frt.ty));

        output
    }
}
