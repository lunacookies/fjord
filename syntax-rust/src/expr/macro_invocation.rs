use {
    crate::{
        utils::{comma_separated, snake_case, take_whitespace0},
        ParseResult,
    },
    nom::{branch::alt, bytes::complete::tag},
};

pub(super) fn parse(s: &str) -> ParseResult<'_> {
    let open_delim = alt((tag("("), tag("["), tag("{")));

    let convert_open_delim_to_close = |s| match s {
        "(" => ")",
        "[" => "]",
        "{" => "}",
        _ => unreachable!(),
    };

    let (s, path) = crate::path(s)?;
    let (s, name) = snake_case(s)?;
    let (s, bang) = tag("!")(s)?;
    let (s, bang_space) = take_whitespace0(s)?;

    let (s, open_delim) = open_delim(s)?;
    let (s, open_delim_space) = take_whitespace0(s)?;

    // This is the delimiter we’re looking for to close the macro invocation.
    let close_delim = convert_open_delim_to_close(open_delim);

    // This isn’t 100% correct, as macros can take any valid token tree as input. However, it’s
    // close enough for most macros.
    let (s, mut params) =
        comma_separated(&|s| alt((crate::item, crate::statement))(s), close_delim)(s)?;

    let (s, params_space) = take_whitespace0(s)?;

    let (s, close_delim) = tag(close_delim)(s)?;

    let mut output = path;

    output.extend_from_slice(&[
        syntax::HighlightedSpan {
            text: name,
            group: Some(syntax::HighlightGroup::MacroUse),
        },
        syntax::HighlightedSpan {
            text: bang,
            group: Some(syntax::HighlightGroup::MacroUse),
        },
        syntax::HighlightedSpan {
            text: bang_space,
            group: None,
        },
        syntax::HighlightedSpan {
            text: open_delim,
            group: Some(syntax::HighlightGroup::Delimiter),
        },
        syntax::HighlightedSpan {
            text: open_delim_space,
            group: None,
        },
    ]);

    output.append(&mut params);

    output.extend_from_slice(&[
        syntax::HighlightedSpan {
            text: params_space,
            group: None,
        },
        syntax::HighlightedSpan {
            text: close_delim,
            group: Some(syntax::HighlightGroup::Delimiter),
        },
    ]);

    Ok((s, output))
}
