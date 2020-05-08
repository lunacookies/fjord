use {
    super::{lifetime_use, ty, ParseResult},
    nom::branch::alt,
};

pub(super) fn parse(s: &str) -> ParseResult<'_> {
    super::generics(&param)(s)
}

fn param(s: &str) -> ParseResult<'_> {
    alt((lifetime_use, ty))(s)
}
