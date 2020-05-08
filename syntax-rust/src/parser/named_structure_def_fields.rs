use {
    super::{comma_separated, expect, ty, ParseResult},
    crate::utils::{snake_case, take_whitespace0},
    nom::bytes::complete::tag,
};

const FIELDS_START: &str = "{";
const FIELDS_END: &str = "}";

pub(super) fn fields(s: &str) -> ParseResult<'_> {
    let (s, open_brace) = tag(FIELDS_START)(s)?;
    let (s, open_brace_space) = take_whitespace0(s)?;

    let (s, mut fields) = comma_separated(&field, FIELDS_END)(s)?;

    let (s, close_brace_space) = take_whitespace0(s)?;
    let (s, close_brace) = tag(FIELDS_END)(s)?;

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

    output.append(&mut fields);

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

fn field(s: &str) -> ParseResult<'_> {
    let (s, name) = snake_case(s)?;
    let (s, name_space) = take_whitespace0(s)?;

    let (s, colon) = tag(":")(s)?;
    let (s, colon_space) = take_whitespace0(s)?;

    let (s, mut ty) = expect(ty, None)(s)?;

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
