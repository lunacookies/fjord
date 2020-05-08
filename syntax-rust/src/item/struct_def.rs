use {
    crate::{
        utils::{pascal_case, take_whitespace0, take_whitespace1},
        ParseResult,
    },
    nom::{
        branch::alt,
        bytes::complete::tag,
        combinator::{map, opt},
    },
};

mod named_fields;
mod tuple_fields;

use {named_fields::parse as named_fields, tuple_fields::parse as tuple_fields};

pub(super) fn parse(s: &str) -> ParseResult<'_> {
    let (s, keyword) = tag("struct")(s)?;
    let (s, keyword_space) = take_whitespace1(s)?;

    let (s, name) = pascal_case(s)?;
    let (s, name_space) = take_whitespace0(s)?;

    let (s, generics) = opt(crate::generics::def)(s)?;
    let (s, generics_space) = take_whitespace0(s)?;

    let (s, mut fields) = fields(s)?;

    let mut output = vec![
        syntax::HighlightedSpan {
            text: keyword,
            group: Some(syntax::HighlightGroup::OtherKeyword),
        },
        syntax::HighlightedSpan {
            text: keyword_space,
            group: None,
        },
        syntax::HighlightedSpan {
            text: name,
            group: Some(syntax::HighlightGroup::TyDef),
        },
        syntax::HighlightedSpan {
            text: name_space,
            group: None,
        },
    ];

    if let Some(mut generics) = generics {
        output.append(&mut generics);
    }

    output.push(syntax::HighlightedSpan {
        text: generics_space,
        group: None,
    });

    output.append(&mut fields);

    Ok((s, output))
}

fn fields(s: &str) -> ParseResult<'_> {
    alt((named_fields, tuple_fields, unnamed))(s)
}

fn unnamed(s: &str) -> ParseResult<'_> {
    map(tag(";"), |semicolon| {
        vec![syntax::HighlightedSpan {
            text: semicolon,
            group: Some(syntax::HighlightGroup::Terminator),
        }]
    })(s)
}
