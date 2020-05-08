use {
    super::{comma_separated, expr, ParseResult},
    crate::utils::take_whitespace0,
    nom::bytes::complete::tag,
};

const FIELDS_START: &str = "(";
const FIELDS_END: &str = ")";

pub(super) fn fields(s: &str) -> ParseResult<'_> {
    let (s, open_paren) = tag(FIELDS_START)(s)?;
    let (s, open_paren_space) = take_whitespace0(s)?;

    // Fields are just expressions.
    let (s, mut fields) = comma_separated(&expr, FIELDS_END)(s)?;

    let (s, close_paren_space) = take_whitespace0(s)?;
    let (s, close_paren) = tag(FIELDS_END)(s)?;

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
            text: close_paren_space,
            group: None,
        },
        syntax::HighlightedSpan {
            text: close_paren,
            group: Some(syntax::HighlightGroup::Delimiter),
        },
    ]);

    Ok((s, output))
}
