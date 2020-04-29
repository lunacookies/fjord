use {
    super::{comma_separated, ty, ParseResult},
    crate::utils::take_whitespace0,
    nom::bytes::complete::tag,
};

pub(super) fn fields(s: &str) -> ParseResult<'_> {
    let (s, open_paren) = tag("(")(s)?;
    let (s, open_paren_space) = take_whitespace0(s)?;

    // Fields of a tuple struct are simply types.
    let (s, mut fields) = comma_separated(&ty)(s)?;
    let (s, fields_space) = take_whitespace0(s)?;

    let (s, close_paren) = tag(")")(s)?;
    let (s, close_paren_space) = take_whitespace0(s)?;

    let (s, semicolon) = tag(";")(s)?;

    let mut output = vec![
        syntax::HighlightedSpan {
            text: open_paren,
            group: Some(syntax::HighlightGroup::Delimiter),
        },
        syntax::HighlightedSpan {
            text: open_paren_space,
            group: None,
        },
    ];

    output.append(&mut fields);

    output.extend_from_slice(&[
        syntax::HighlightedSpan {
            text: fields_space,
            group: None,
        },
        syntax::HighlightedSpan {
            text: close_paren,
            group: Some(syntax::HighlightGroup::Delimiter),
        },
        syntax::HighlightedSpan {
            text: close_paren_space,
            group: None,
        },
        syntax::HighlightedSpan {
            text: semicolon,
            group: Some(syntax::HighlightGroup::Terminator),
        },
    ]);

    Ok((s, output))
}
