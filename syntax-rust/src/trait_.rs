use {
    crate::{
        utils::{pascal_case, take_whitespace0},
        ParseResult,
    },
    nom::combinator::opt,
};

pub(crate) fn parse(s: &str) -> ParseResult<'_> {
    let (s, path) = crate::path(s)?;

    let (s, name) = pascal_case(s)?;
    let (s, name_space) = take_whitespace0(s)?;

    let (s, generics) = opt(crate::generics::usage)(s)?;

    let mut output = path;

    output.extend_from_slice(&[
        syntax::HighlightedSpan {
            text: name,
            group: Some(syntax::HighlightGroup::InterfaceUse),
        },
        syntax::HighlightedSpan {
            text: name_space,
            group: None,
        },
    ]);

    if let Some(mut generics) = generics {
        output.append(&mut generics);
    }

    Ok((s, output))
}
