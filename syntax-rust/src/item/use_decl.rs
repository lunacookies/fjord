use {
    crate::{
        utils::{take_whitespace0, take_whitespace1},
        ParseResult,
    },
    nom::{branch::alt, bytes::complete::tag},
};

pub(super) fn parse(s: &str) -> ParseResult<'_> {
    let (s, keyword) = tag("use")(s)?;
    let (s, keyword_space) = take_whitespace1(s)?;

    let (s, mut path) = crate::path(s)?;
    let (s, path_space) = take_whitespace0(s)?;

    let (s, mut ident) = alt((crate::ty_name, crate::module_name))(s)?;
    let (s, ident_space) = take_whitespace0(s)?;

    let (s, semicolon) = tag(";")(s)?;

    let mut output = vec![
        syntax::HighlightedSpan {
            text: keyword,
            group: Some(syntax::HighlightGroup::OtherKeyword),
        },
        syntax::HighlightedSpan {
            text: keyword_space,
            group: None,
        },
    ];

    output.append(&mut path);

    output.push(syntax::HighlightedSpan {
        text: path_space,
        group: None,
    });

    output.append(&mut ident);

    output.extend_from_slice(&[
        syntax::HighlightedSpan {
            text: ident_space,
            group: None,
        },
        syntax::HighlightedSpan {
            text: semicolon,
            group: Some(syntax::HighlightGroup::Terminator),
        },
    ]);

    Ok((s, output))
}
