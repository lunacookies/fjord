use {
    crate::{
        utils::{digits, int_ty},
        ParseResult,
    },
    nom::{
        branch::alt,
        bytes::complete::tag,
        combinator::{map, opt},
    },
};

pub(super) fn parse(s: &str) -> ParseResult<'_> {
    let (s, number) = alt((binary, octal, hex, decimal))(s)?;
    let (s, suffix) = opt(int_ty)(s)?;

    let mut output = number;

    if let Some(suffix) = suffix {
        output.push(syntax::HighlightedSpan {
            text: suffix,
            group: Some(syntax::HighlightGroup::Number),
        });
    }

    Ok((s, output))
}

fn decimal(s: &str) -> ParseResult<'_> {
    map(digits(|c| c.is_ascii_digit()), |s| {
        vec![syntax::HighlightedSpan {
            text: s,
            group: Some(syntax::HighlightGroup::Number),
        }]
    })(s)
}

fn binary(s: &str) -> ParseResult<'_> {
    let (s, leader) = tag("0b")(s)?;
    let (s, underscore) = opt(tag("_"))(s)?;
    let (s, digits) = digits(|c| c == '0' || c == '1')(s)?;

    let mut output = vec![syntax::HighlightedSpan {
        text: leader,
        group: Some(syntax::HighlightGroup::Number),
    }];

    if let Some(underscore) = underscore {
        output.push(syntax::HighlightedSpan {
            text: underscore,
            group: Some(syntax::HighlightGroup::Number),
        });
    }

    output.push(syntax::HighlightedSpan {
        text: digits,
        group: Some(syntax::HighlightGroup::Number),
    });

    Ok((s, output))
}

fn octal(s: &str) -> ParseResult<'_> {
    let (s, leader) = tag("0o")(s)?;
    let (s, underscore) = opt(tag("_"))(s)?;
    let (s, digits) = digits(|c| c >= '0' && c <= '7')(s)?;

    let mut output = vec![syntax::HighlightedSpan {
        text: leader,
        group: Some(syntax::HighlightGroup::Number),
    }];

    if let Some(underscore) = underscore {
        output.push(syntax::HighlightedSpan {
            text: underscore,
            group: Some(syntax::HighlightGroup::Number),
        });
    }

    output.push(syntax::HighlightedSpan {
        text: digits,
        group: Some(syntax::HighlightGroup::Number),
    });

    Ok((s, output))
}

fn hex(s: &str) -> ParseResult<'_> {
    let (s, leader) = tag("0x")(s)?;
    let (s, underscore) = opt(tag("_"))(s)?;
    let (s, digits) =
        digits(|c| c.is_ascii_digit() || (c >= 'a' && c <= 'f') || (c >= 'A' && c <= 'F'))(s)?;

    let mut output = vec![syntax::HighlightedSpan {
        text: leader,
        group: Some(syntax::HighlightGroup::Number),
    }];

    if let Some(underscore) = underscore {
        output.push(syntax::HighlightedSpan {
            text: underscore,
            group: Some(syntax::HighlightGroup::Number),
        });
    }

    output.push(syntax::HighlightedSpan {
        text: digits,
        group: Some(syntax::HighlightGroup::Number),
    });

    Ok((s, output))
}
