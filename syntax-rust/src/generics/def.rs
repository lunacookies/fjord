use {
    crate::{utils::take_whitespace0, ParseResult},
    nom::{branch::alt, combinator::opt},
};

pub(crate) fn parse(s: &str) -> ParseResult<'_> {
    super::parse(&param)(s)
}

fn param(s: &str) -> ParseResult<'_> {
    let (s, param) = alt((crate::lifetime::def, crate::ty))(s)?;
    let (s, param_space) = take_whitespace0(s)?;

    let (s, bounds) = opt(crate::bounds)(s)?;

    let mut output = param;

    output.push(syntax::HighlightedSpan {
        text: param_space,
        group: None,
    });

    if let Some(mut bounds) = bounds {
        output.append(&mut bounds);
    }

    Ok((s, output))
}
