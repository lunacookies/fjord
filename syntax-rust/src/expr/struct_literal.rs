use {
    crate::{utils::take_whitespace0, ParseResult},
    nom::branch::alt,
};

mod named_fields;
mod tuple_fields;

use {named_fields::parse as named_fields, tuple_fields::parse as tuple_fields};

pub(super) fn parse(s: &str) -> ParseResult<'_> {
    let (s, path) = crate::path(s)?;

    let (s, mut name) = crate::ty_name(s)?;
    let (s, name_space) = take_whitespace0(s)?;

    let (s, mut fields) = fields(s)?;

    let mut output = path;

    output.append(&mut name);
    output.push(syntax::HighlightedSpan {
        text: name_space,
        group: None,
    });

    output.append(&mut fields);

    Ok((s, output))
}

fn fields(s: &str) -> ParseResult<'_> {
    alt((named_fields, tuple_fields, |s| Ok((s, vec![]))))(s)
}
