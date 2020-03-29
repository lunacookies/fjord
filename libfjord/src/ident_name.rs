use nom::bytes::complete::take_while1;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub(crate) struct IdentName(String);

impl IdentName {
    pub(crate) fn new(s: &str) -> nom::IResult<&str, Self> {
        let _ = take_while1(|c: char| c.is_ascii_lowercase())(s)?;
        let (s, name) = take_while1(|c: char| c.is_ascii_alphanumeric())(s)?;

        Ok((s, Self(name.into())))
    }
}
