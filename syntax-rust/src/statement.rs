use {
    crate::{
        utils::{expect, take_whitespace0, take_whitespace1},
        ParseResult,
    },
    nom::{
        branch::alt,
        bytes::complete::tag,
        combinator::{map, opt},
    },
};

pub(crate) fn parse(s: &str) -> ParseResult<'_> {
    alt((crate::item, let_, expr_in_statement))(s)
}

fn let_(s: &str) -> ParseResult<'_> {
    let (s, keyword) = tag("let")(s)?;
    let (s, keyword_space) = take_whitespace1(s)?;

    let (s, mut pattern) = crate::pattern(s)?;
    let (s, pattern_space) = take_whitespace0(s)?;

    let (s, ty_annotation) = opt(|s| {
        let (s, colon) = tag(":")(s)?;
        let (s, colon_space) = take_whitespace0(s)?;

        let (s, mut ty) = crate::ty(s)?;
        let (s, ty_space) = take_whitespace0(s)?;

        let mut output = vec![
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

        output.push(syntax::HighlightedSpan {
            text: ty_space,
            group: None,
        });

        Ok((s, output))
    })(s)?;

    let (s, rhs) = opt(|s| {
        let (s, equals) = tag("=")(s)?;
        let (s, equals_space) = take_whitespace0(s)?;

        let (s, mut expr) = expect(crate::expr, None)(s)?;
        let (s, expr_space) = take_whitespace0(s)?;

        let mut output = vec![
            syntax::HighlightedSpan {
                text: equals,
                group: Some(syntax::HighlightGroup::AssignOper),
            },
            syntax::HighlightedSpan {
                text: equals_space,
                group: None,
            },
        ];

        output.append(&mut expr);
        output.push(syntax::HighlightedSpan {
            text: expr_space,
            group: None,
        });

        Ok((s, output))
    })(s)?;

    let (s, mut semicolon) = expect(
        |s| {
            map(tag(";"), |s| {
                vec![syntax::HighlightedSpan {
                    text: s,
                    group: Some(syntax::HighlightGroup::Terminator),
                }]
            })(s)
        },
        None,
    )(s)?;

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

    output.append(&mut pattern);
    output.push(syntax::HighlightedSpan {
        text: pattern_space,
        group: None,
    });

    if let Some(mut ty_annotation) = ty_annotation {
        output.append(&mut ty_annotation);
    }

    if let Some(mut rhs) = rhs {
        output.append(&mut rhs);
    }

    output.append(&mut semicolon);

    Ok((s, output))
}

fn expr_in_statement(s: &str) -> ParseResult<'_> {
    let (s, expr) = crate::expr(s)?;
    let (s, expr_space) = take_whitespace0(s)?;

    let (s, semicolon) = opt(tag(";"))(s)?;

    let mut output = expr;
    output.push(syntax::HighlightedSpan {
        text: expr_space,
        group: None,
    });

    if let Some(semicolon) = semicolon {
        output.push(syntax::HighlightedSpan {
            text: semicolon,
            group: Some(syntax::HighlightGroup::Terminator),
        });
    }

    Ok((s, output))
}
