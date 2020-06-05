use nom::bytes::complete::{take_while, take_while1};

fn is_whitespace(c: char) -> bool {
    c.is_ascii_whitespace()
}

pub(crate) fn take_whitespace1(s: &str) -> nom::IResult<&str, &str> {
    take_while1(is_whitespace)(s)
}

pub(crate) fn take_whitespace(s: &str) -> nom::IResult<&str, &str> {
    take_while(is_whitespace)(s)
}

pub(crate) type Number = i32;
