use {
    crate::{utils::take_whitespace0, ParseResult},
    nom::combinator::opt,
};

pub(crate) fn parse(s: &str) -> ParseResult<'_> {
    let (s, path) = crate::path(s)?;

    let (s, mut name) = crate::ty_name(s)?;
    let (s, name_space) = take_whitespace0(s)?;

    let (s, generics) = opt(crate::generics::usage)(s)?;

    let mut output = path;

    output.append(&mut name);
    output.push(syntax::HighlightedSpan {
        text: name_space,
        group: None,
    });

    if let Some(mut generics) = generics {
        output.append(&mut generics);
    }

    Ok((s, output))
}
