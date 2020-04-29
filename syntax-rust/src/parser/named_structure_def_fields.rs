use {
    super::{comma_separated, expect, ty, ParseResult},
    crate::utils::{ident, take_whitespace0},
    nom::bytes::complete::tag,
};

pub(super) fn fields(s: &str) -> ParseResult<'_> {
    let (s, open_brace) = tag("{")(s)?;
    let (s, open_brace_space) = take_whitespace0(s)?;

    let (s, mut fields) = comma_separated(&field)(s)?;

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
    let (s, name) = ident(s)?;
    let (s, name_space) = take_whitespace0(s)?;

    let (s, colon) = tag(":")(s)?;
    let (s, colon_space) = take_whitespace0(s)?;

    let (s, mut ty) = expect(ty)(s)?;

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
