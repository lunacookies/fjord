use {
    super::{trait_, ParseResult},
    crate::utils::take_whitespace0,
    nom::{bytes::complete::tag, multi::many0},
};

pub(super) fn parse(s: &str) -> ParseResult<'_> {
    let (s, colon) = tag(":")(s)?;
    let (s, colon_space) = take_whitespace0(s)?;

    let (s, mut first) = trait_(s)?;

    let (s, rest) = many0(|s| {
        let (s, space) = take_whitespace0(s)?;

        let (s, plus) = tag("+")(s)?;
        let (s, plus_space) = take_whitespace0(s)?;

        let (s, mut trait_) = trait_(s)?;

        let mut output = vec![
            syntax::HighlightedSpan {
                text: space,
                group: None,
            },
            syntax::HighlightedSpan {
                text: plus,
                group: Some(syntax::HighlightGroup::BinaryOper),
            },
            syntax::HighlightedSpan {
                text: plus_space,
                group: None,
            },
        ];

        output.append(&mut trait_);

        Ok((s, output))
    })(s)?;

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

    output.append(&mut first);
    output.append(&mut rest.concat());

    Ok((s, output))
}
