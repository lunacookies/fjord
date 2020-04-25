use nom::{bytes::complete::tag, combinator::map, sequence::pair};

pub(crate) struct Lifetime<'text> {
    tick: &'text str,
    name: crate::Ident<'text>,
}

impl<'text> Lifetime<'text> {
    pub(crate) fn new(s: &'text str) -> nom::IResult<&'text str, Self> {
        let lifetime = pair(tag("'"), crate::Ident::new);
        map(lifetime, |(tick, name)| Self { tick, name })(s)
    }
}
