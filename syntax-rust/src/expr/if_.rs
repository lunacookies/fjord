use {
    crate::{
        utils::{take_whitespace0, take_whitespace1},
        ParseResult,
    },
    nom::{bytes::complete::tag, combinator::opt, multi::many0},
};

pub(super) fn parse(s: &str) -> ParseResult<'_> {
    let (s, keyword) = tag("if")(s)?;
    let (s, keyword_space) = take_whitespace1(s)?;

    let (s, mut cond_and_body) = parse_cond_and_body(s)?;

    let (s, else_if_clauses) = many0(|s| {
        let (s, else_keyword) = tag("else")(s)?;
        let (s, else_keyword_space) = take_whitespace1(s)?;

        let (s, if_keyword) = tag("if")(s)?;
        let (s, if_keyword_space) = take_whitespace1(s)?;

        let (s, mut cond_and_body) = parse_cond_and_body(s)?;

        let mut output = vec![
            syntax::HighlightedSpan {
                text: else_keyword,
                group: Some(syntax::HighlightGroup::CtrlFlowKeyword),
            },
            syntax::HighlightedSpan {
                text: else_keyword_space,
                group: None,
            },
            syntax::HighlightedSpan {
                text: if_keyword,
                group: Some(syntax::HighlightGroup::CtrlFlowKeyword),
            },
            syntax::HighlightedSpan {
                text: if_keyword_space,
                group: None,
            },
        ];

        output.append(&mut cond_and_body);

        Ok((s, output))
    })(s)?;

    let (s, else_clause) = opt(|s| {
        let (s, keyword) = tag("else")(s)?;
        let (s, keyword_space) = take_whitespace1(s)?;

        let (s, mut body) = crate::block(s)?;
        let (s, body_space) = take_whitespace0(s)?;

        let mut output = vec![
            syntax::HighlightedSpan {
                text: keyword,
                group: Some(syntax::HighlightGroup::CtrlFlowKeyword),
            },
            syntax::HighlightedSpan {
                text: keyword_space,
                group: None,
            },
        ];

        output.append(&mut body);

        output.push(syntax::HighlightedSpan {
            text: body_space,
            group: None,
        });

        Ok((s, output))
    })(s)?;

    let mut output = vec![
        syntax::HighlightedSpan {
            text: keyword,
            group: Some(syntax::HighlightGroup::CtrlFlowKeyword),
        },
        syntax::HighlightedSpan {
            text: keyword_space,
            group: None,
        },
    ];

    output.append(&mut cond_and_body);

    output.append(&mut else_if_clauses.concat());

    if let Some(mut else_clause) = else_clause {
        output.append(&mut else_clause);
    }

    Ok((s, output))
}

fn parse_cond_and_body(s: &str) -> ParseResult<'_> {
    let (s, cond) = crate::expr(s)?;
    let (s, cond_space) = take_whitespace0(s)?;

    let (s, mut body) = crate::block(s)?;
    let (s, body_space) = take_whitespace0(s)?;

    let mut output = cond;

    output.push(syntax::HighlightedSpan {
        text: cond_space,
        group: None,
    });

    output.append(&mut body);

    output.push(syntax::HighlightedSpan {
        text: body_space,
        group: None,
    });

    Ok((s, output))
}
