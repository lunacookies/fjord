use {
    crate::{
        utils::{expect, take_whitespace0},
        ParseResult,
    },
    nom::{bytes::complete::tag, multi::many0},
};

const BLOCK_START: &str = "{";
const BLOCK_END: &str = "}";

pub(crate) fn parse(s: &str) -> ParseResult<'_> {
    let (s, open_brace) = tag(BLOCK_START)(s)?;
    let (s, open_brace_space) = take_whitespace0(s)?;

    let (s, statements) = many0(|s| {
        let (s, statement) = expect(crate::statement, Some(BLOCK_END))(s)?;
        let (s, space) = take_whitespace0(s)?;

        let mut output = statement;
        output.push(syntax::HighlightedSpan {
            text: space,
            group: None,
        });

        Ok((s, output))
    })(s)?;

    let (s, close_brace_space) = take_whitespace0(s)?;
    let (s, close_brace) = tag(BLOCK_END)(s)?;

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

    output.append(&mut statements.concat());

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
