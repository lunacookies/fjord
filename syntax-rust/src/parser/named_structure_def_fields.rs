use {
    super::{ty, ParseResult},
    crate::utils::{ident, take_whitespace0},
    nom::{bytes::complete::tag, combinator::opt, multi::many0},
};

pub(super) fn fields(s: &str) -> ParseResult<'_> {
    let (s, open_brace) = tag("{")(s)?;
    let (s, open_brace_space) = take_whitespace0(s)?;

    // The first field in a struct never has a comma preceding it.
    let (s, mut first_field) = field_without_comma(s)?;

    // All the other fields have a comma preceding them, though.
    let (s, other_fields) = many0(field_preceded_by_comma)(s)?;
    let (s, other_fields_space) = take_whitespace0(s)?;

    // Optional trailing comma.
    let (s, trailing_comma) = opt(tag(","))(s)?;

    let (s, close_brace_space) = take_whitespace0(s)?;
    let (s, close_brace) = tag("}")(s)?;

    let mut output = vec![
        syntax::HighlightedSpan {
            text: open_brace,
            group: Some(syntax::HighlightGroup::Delimiter),
        },
        syntax::HighlightedSpan {
            text: open_brace_space,
            group: None,
        },
    ];

    output.append(&mut first_field);
    output.append(&mut other_fields.concat());

    output.push(syntax::HighlightedSpan {
        text: other_fields_space,
        group: None,
    });

    if let Some(trailing_comma) = trailing_comma {
        output.push(syntax::HighlightedSpan {
            text: trailing_comma,
            group: Some(syntax::HighlightGroup::Separator),
        })
    }

    output.extend_from_slice(&[
        syntax::HighlightedSpan {
            text: close_brace_space,
            group: None,
        },
        syntax::HighlightedSpan {
            text: close_brace,
            group: Some(syntax::HighlightGroup::Delimiter),
        },
    ]);

    Ok((s, output))
}

fn field_without_comma(s: &str) -> ParseResult<'_> {
    let (s, name) = ident(s)?;
    let (s, name_space) = take_whitespace0(s)?;

    let (s, colon) = tag(":")(s)?;
    let (s, colon_space) = take_whitespace0(s)?;

    let (s, mut ty) = ty(s)?;

    let mut output = vec![
        syntax::HighlightedSpan {
            text: name,
            group: Some(syntax::HighlightGroup::MemberDef),
        },
        syntax::HighlightedSpan {
            text: name_space,
            group: None,
        },
        syntax::HighlightedSpan {
            text: colon,
            group: Some(syntax::HighlightGroup::Separator),
        },
        syntax::HighlightedSpan {
            text: colon_space,
            group: None,
        },
    ];

    output.append(&mut ty);

    Ok((s, output))
}

fn field_preceded_by_comma(s: &str) -> ParseResult<'_> {
    let (s, initial_space) = take_whitespace0(s)?;
    let (s, comma) = tag(",")(s)?;
    let (s, comma_space) = take_whitespace0(s)?;

    let (s, mut field) = field_without_comma(s)?;

    let mut output = vec![
        syntax::HighlightedSpan {
            text: initial_space,
            group: None,
        },
        syntax::HighlightedSpan {
            text: comma,
            group: Some(syntax::HighlightGroup::Separator),
        },
        syntax::HighlightedSpan {
            text: comma_space,
            group: None,
        },
    ];

    output.append(&mut field);

    Ok((s, output))
}
