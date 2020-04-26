use nom::bytes::complete::take_while1;

#[derive(Debug, PartialEq)]
pub(crate) struct Ident<'name> {
    pub(crate) name: &'name str,
}

impl<'name> Ident<'name> {
    pub(crate) fn new(s: &'name str) -> nom::IResult<&'name str, Self> {
        // Identifier names must start with lowercase letters.
        let _ = take_while1(|c: char| c.is_ascii_lowercase())(s)?;

        // They can follow, however, with lowercase, numbers, or underscores.
        let (s, name) =
            take_while1(|c: char| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_')(s)?;

        Ok((s, Self { name }))
    }
}
