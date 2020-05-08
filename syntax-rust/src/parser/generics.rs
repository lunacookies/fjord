use {
    super::{comma_separated, ParseResult},
    crate::utils::take_whitespace0,
    nom::bytes::complete::tag,
};

const START_GENERICS: &str = "<";
const END_GENERICS: &str = ">";

pub(super) fn parse<'input, P: Fn(&'input str) -> ParseResult<'input> + Copy + 'input>(
    param: &'input P,
) -> impl Fn(&'input str) -> ParseResult<'input> + 'input {
    move |s| {
        let (s, open_bracket) = tag(START_GENERICS)(s)?;
        let (s, open_bracket_space) = take_whitespace0(s)?;

        let (s, mut params) = comma_separated(param, END_GENERICS)(s)?;

        let (s, close_bracket_space) = take_whitespace0(s)?;
        let (s, close_bracket) = tag(END_GENERICS)(s)?;

        let mut output = vec![
            syntax::HighlightedSpan {
                text: open_bracket,
                group: Some(syntax::HighlightGroup::Delimiter),
            },
            syntax::HighlightedSpan {
                text: open_bracket_space,
                group: None,
            },
        ];

        output.append(&mut params);

        output.extend_from_slice(&[
            syntax::HighlightedSpan {
                text: close_bracket_space,
                group: None,
            },
            syntax::HighlightedSpan {
                text: close_bracket,
                group: Some(syntax::HighlightGroup::Delimiter),
            },
        ]);

        Ok((s, output))
    }
}
