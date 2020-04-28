use nom::bytes::complete::{take_while, take_while1};

pub(crate) fn take_whitespace0(s: &str) -> nom::IResult<&str, &str> {
    take_while(is_whitespace)(s)
}

pub(crate) fn take_whitespace1(s: &str) -> nom::IResult<&str, &str> {
    take_while1(is_whitespace)(s)
}

fn is_whitespace(c: char) -> bool {
    c.is_ascii_whitespace()
}

pub(crate) fn ident(s: &str) -> nom::IResult<&str, &str> {
    // Identifiers must start with an uppercase or lowercase letter.
    let _ = take_while1(|c: char| c.is_ascii_alphabetic())(s)?;

    // After this, however, they can contain alphanumeric characters or underscores.
    take_while1(|c: char| c.is_ascii_alphanumeric() || c == '_')(s)
}
