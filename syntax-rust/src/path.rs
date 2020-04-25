use nom::{bytes::complete::tag, combinator::opt, multi::many1};

// TODO: Still need to add final component of path.
pub(crate) struct Path<'text> {
    leading_colons: Option<&'text str>,
    components: Vec<PathComponent<'text>>,
}

impl<'text> Path<'text> {
    pub(crate) fn new(s: &'text str) -> nom::IResult<&'text str, Self> {
        let (s, leading_colons) = opt(tag("::"))(s)?;
        let (s, components) = many1(PathComponent::new)(s)?;

        Ok((
            s,
            Self {
                leading_colons,
                components,
            },
        ))
    }
}

impl<'p> From<Path<'p>> for Vec<syntax::HighlightedSpan<'p>> {
    fn from(path: Path<'p>) -> Self {
        let mut output = Vec::new();

        if let Some(c) = path.leading_colons {
            output.push(syntax::HighlightedSpan {
                text: c,
                group: None,
            });
        }

        output.extend(path.components.into_iter().map(Vec::from).flatten());

        output
    }
}

struct PathComponent<'text> {
    module_name: crate::Ident<'text>,
    colons: &'text str,
}

impl<'text> PathComponent<'text> {
    fn new(s: &'text str) -> nom::IResult<&'text str, Self> {
        let (s, module_name) = crate::Ident::new(s)?;
        let (s, colons) = tag("::")(s)?;

        Ok((
            s,
            Self {
                colons,
                module_name,
            },
        ))
    }
}

impl<'pc> From<PathComponent<'pc>> for Vec<syntax::HighlightedSpan<'pc>> {
    fn from(path_component: PathComponent<'pc>) -> Self {
        vec![
            syntax::HighlightedSpan {
                text: path_component.module_name.name,
                group: Some(syntax::HighlightGroup::Module),
            },
            syntax::HighlightedSpan {
                text: path_component.colons,
                group: Some(syntax::HighlightGroup::MemberOper),
            },
        ]
    }
}
