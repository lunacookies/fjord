use nom::bytes::complete::take_while1;

#[derive(Debug, PartialEq)]
pub(crate) struct TyIdent<'text> {
    pub(crate) name: &'text str,
}

impl<'text> TyIdent<'text> {
    pub(crate) fn new(s: &'text str) -> nom::IResult<&'text str, Self> {
        // Type identifiers must start with an uppercase letter.
        let _ = take_while1(|c: char| c.is_ascii_uppercase())(s)?;

        // They can follow, however, with uppercase letters, lowercase letters, or numbers.
        let (s, name) = take_while1(|c: char| {
            c.is_ascii_uppercase() || c.is_ascii_lowercase() || c.is_ascii_digit()
        })(s)?;

        Ok((s, Self { name }))
    }
}
