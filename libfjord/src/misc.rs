pub(crate) fn take_whitespace1(s: &str) -> nom::IResult<&str, &str> {
    nom::bytes::complete::take_while1(|c: char| c.is_ascii_whitespace())(s)
}

pub(crate) type Number = i32;
