use {
    super::{bounds, lifetime_def, ty, ParseResult},
    crate::utils::take_whitespace0,
    nom::{branch::alt, combinator::opt},
};

pub(super) fn parse(s: &str) -> ParseResult<'_> {
    super::generics(&param)(s)
}

fn param(s: &str) -> ParseResult<'_> {
    let (s, param) = alt((lifetime_def, ty))(s)?;
    let (s, param_space) = take_whitespace0(s)?;

    let (s, bounds) = opt(bounds)(s)?;

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
