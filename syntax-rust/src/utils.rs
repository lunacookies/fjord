use {
    crate::ParseResult,
    nom::{
        branch::alt,
        bytes::complete::{tag, take, take_while, take_while1},
        combinator::{map, not, opt},
        multi::many0,
    },
};

pub(crate) fn take_whitespace0(s: &str) -> nom::IResult<&str, &str> {
    take_while(is_whitespace)(s)
}

pub(crate) fn take_whitespace1(s: &str) -> nom::IResult<&str, &str> {
    take_while1(is_whitespace)(s)
}

fn is_whitespace(c: char) -> bool {
    c.is_ascii_whitespace()
}

fn ident(
    starts_with: impl Fn(char) -> bool + Copy,
    rest: impl Fn(char) -> bool + Copy,
) -> impl Fn(&str) -> nom::IResult<&str, &str> {
    move |s| {
        let _ = take_while1(starts_with)(s)?;
        take_while1(rest)(s)
    }
}

pub(crate) fn snake_case(s: &str) -> nom::IResult<&str, &str> {
    ident(
        |c| c.is_ascii_lowercase(),
        |c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_',
    )(s)
}

pub(crate) fn shouting_snake_case(s: &str) -> nom::IResult<&str, &str> {
    ident(
        |c| c.is_ascii_uppercase(),
        |c| c.is_ascii_uppercase() || c.is_ascii_digit() || c == '_',
    )(s)
}

pub(crate) fn pascal_case(s: &str) -> nom::IResult<&str, &str> {
    ident(|c| c.is_ascii_uppercase(), |c| c.is_ascii_alphanumeric())(s)
}

pub(crate) fn digits(
    is_digit: impl Fn(char) -> bool + Copy + 'static,
) -> impl Fn(&str) -> nom::IResult<&str, &str> {
    move |s| {
        // Digit literals must start with at least one digit.
        let _ = take_while1(is_digit)(s)?;

        // This can be followed by digits as well as underscores.
        take_while1(|c| is_digit(c) || c == '_')(s)
    }
}

pub(crate) fn int_ty(s: &str) -> nom::IResult<&str, &str> {
    alt((
        tag("u8"),
        tag("u16"),
        tag("u32"),
        tag("u64"),
        tag("u128"),
        tag("usize"),
        tag("i8"),
        tag("i16"),
        tag("i32"),
        tag("i64"),
        tag("i128"),
        tag("isize"),
    ))(s)
}

pub(crate) fn float_ty(s: &str) -> nom::IResult<&str, &str> {
    alt((tag("f32"), tag("f64")))(s)
}

pub(crate) fn expect<'input>(
    parser: impl Fn(&'input str) -> ParseResult<'input> + Copy,
    ending_sequence: Option<&'input str>,
) -> impl Fn(&'input str) -> ParseResult<'input> {
    move |s| {
        if let Ok((s, spans)) = parser(s) {
            Ok((s, spans))
        } else {
            // Stop parsing if the user has chosen an ending sequence and if we’ve reached the end
            // of this sequence.
            if let Some(ending_sequence) = ending_sequence {
                let _ = not(tag(ending_sequence))(s)?;
            }

            // If the input parser fails, then take a single character as an error and attempt
            // parsing again.
            let (s, error) = error_1_char(s)?;
            let (s, mut parser_output) = expect(parser, ending_sequence)(s)?;

            let mut output = error;
            output.append(&mut parser_output);

            Ok((s, output))
        }
    }
}

pub(crate) fn comma_separated<'input, P: Fn(&'input str) -> ParseResult<'input> + Copy + 'input>(
    parser: &'input P,
    ending_sequence: &'input str,
) -> impl Fn(&'input str) -> ParseResult<'input> + 'input {
    let parser_stop_at_end = move |s| expect(parser, Some(ending_sequence))(s);

    let comma_stop_at_end = move |s| {
        expect(
            |s| {
                map(tag(","), |s| {
                    vec![syntax::HighlightedSpan {
                        text: s,
                        group: Some(syntax::HighlightGroup::Separator),
                    }]
                })(s)
            },
            Some(ending_sequence),
        )(s)
    };

    let followed_by_comma = move |s| {
        let (s, parser_output) = parser_stop_at_end(s)?;
        let (s, parser_output_space) = take_whitespace0(s)?;

        let (s, mut comma) = comma_stop_at_end(s)?;
        let (s, comma_space) = take_whitespace0(s)?;

        let mut output = parser_output;

        output.push(syntax::HighlightedSpan {
            text: parser_output_space,
            group: None,
        });

        output.append(&mut comma);

        output.push(syntax::HighlightedSpan {
            text: comma_space,
            group: None,
        });

        Ok((s, output))
    };

    move |s| {
        let (s, followed_by_commas) = many0(followed_by_comma)(s)?;
        let (s, last_without_comma) = opt(parser_stop_at_end)(s)?;

        let (s, space) = take_whitespace0(s)?;

        let mut output = followed_by_commas.concat();

        if let Some(mut last_without_comma) = last_without_comma {
            output.append(&mut last_without_comma);
        }

        output.push(syntax::HighlightedSpan {
            text: space,
            group: None,
        });

        Ok((s, output))
    }
}

fn error_1_char(s: &str) -> ParseResult<'_> {
    map(take(1usize), |s| {
        vec![syntax::HighlightedSpan {
            text: s,
            group: Some(syntax::HighlightGroup::Error),
        }]
    })(s)
}
