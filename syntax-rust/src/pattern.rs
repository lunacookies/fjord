use nom::combinator::map;

// TODO: implement patterns beyond just a single variable.
#[derive(Debug, PartialEq)]
pub(crate) struct Pattern<'text> {
    name: crate::Ident<'text>,
}

impl<'text> Pattern<'text> {
    pub(crate) fn new(s: &'text str) -> nom::IResult<&'text str, Self> {
        map(crate::Ident::new, |name| Self { name })(s)
    }
}

impl<'p> From<Pattern<'p>> for Vec<syntax::HighlightedSpan<'p>> {
    fn from(pattern: Pattern<'p>) -> Self {
        vec![syntax::HighlightedSpan {
            text: pattern.name.name,
            group: Some(syntax::HighlightGroup::VariableDef),
        }]
    }
}
