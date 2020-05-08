use {crate::ParseResult, nom::branch::alt};

pub(crate) fn parse(s: &str) -> ParseResult<'_> {
    super::parse(&param)(s)
}

fn param(s: &str) -> ParseResult<'_> {
    alt((crate::lifetime::usage, crate::ty))(s)
}
