use nom::{
    branch::alt,
    bytes::complete::{tag, take_while, take_while1},
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

        // This can be folloewd by digits as well as underscores.
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
