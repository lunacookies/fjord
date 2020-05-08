use {
    crate::{
        utils::{snake_case, take_whitespace1},
        ParseResult,
    },
    nom::{bytes::complete::tag, combinator::opt},
};

pub(crate) fn parse(s: &str) -> ParseResult<'_> {
    let (s, ref_) = opt(|s| {
        let (s, ref_) = tag("ref")(s)?;
        let (s, ref_space) = take_whitespace1(s)?;

        let output = vec![
            syntax::HighlightedSpan {
                text: ref_,
                group: Some(syntax::HighlightGroup::OtherKeyword),
            },
            syntax::HighlightedSpan {
                text: ref_space,
                group: None,
            },
        ];

        Ok((s, output))
    })(s)?;

    let (s, mut_) = opt(|s| {
        let (s, mut_) = tag("mut")(s)?;
        let (s, mut_space) = take_whitespace1(s)?;

        let output = vec![
            syntax::HighlightedSpan {
                text: mut_,
                group: Some(syntax::HighlightGroup::OtherKeyword),
            },
            syntax::HighlightedSpan {
                text: mut_space,
                group: None,
            },
        ];

        Ok((s, output))
    })(s)?;

    let (s, name) = snake_case(s)?;

    let mut output = ref_.unwrap_or_else(|| vec![]);

    if let Some(mut mut_) = mut_ {
        output.append(&mut mut_);
    }

    output.push(syntax::HighlightedSpan {
        text: name,
        group: Some(syntax::HighlightGroup::VariableDef),
    });

    Ok((s, output))
}
