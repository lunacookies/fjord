use nom::bytes::complete::take_while1;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub(crate) struct IdentName<'a>(&'a str);

impl<'a> IdentName<'a> {
    pub(crate) fn new(s: &'a str) -> nom::IResult<&'a str, Self> {
        let _ = take_while1(|c: char| c.is_ascii_lowercase())(s)?;
        let (s, name) = take_while1(|c: char| c.is_ascii_alphanumeric())(s)?;

        Ok((s, Self(name)))
    }
}
