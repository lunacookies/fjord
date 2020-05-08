use {
    crate::{
        utils::{comma_separated, take_whitespace0},
        ParseResult,
    },
    nom::bytes::complete::tag,
};

mod def;
mod usage;

pub(crate) use {def::parse as def, usage::parse as usage};

const GENERICS_START: &str = "<";
const GENERICS_END: &str = ">";

fn parse<'input, P: Fn(&'input str) -> ParseResult<'input> + Copy + 'input>(
    param: &'input P,
) -> impl Fn(&'input str) -> ParseResult<'input> + 'input {
    move |s| {
        let (s, open_bracket) = tag(GENERICS_START)(s)?;
        let (s, open_bracket_space) = take_whitespace0(s)?;

        let (s, mut params) = comma_separated(param, GENERICS_END)(s)?;

        let (s, close_bracket_space) = take_whitespace0(s)?;
        let (s, close_bracket) = tag(GENERICS_END)(s)?;

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
