use nom::bytes::complete::take_while1;

pub(crate) struct Ident<'name> {
    pub(crate) name: &'name str,
}

impl<'name> Ident<'name> {
    pub(crate) fn new(s: &'name str) -> nom::IResult<&'name str, Self> {
        let _ = take_while1(|c: char| c.is_ascii_lowercase())(s)?;
        let (s, name) =
            take_while1(|c: char| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_')(s)?;

        Ok((s, Self { name }))
    }
}
