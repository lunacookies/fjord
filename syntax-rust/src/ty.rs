use nom::{bytes::complete::tag, combinator::opt, multi::many0, sequence::pair};

struct Ty<'text> {
    // The second item in this tuple is whitespace.
    refs: Vec<(Ref<'text>, &'text str)>,
    space: &'text str,
    name: crate::TyIdent<'text>,
}

impl<'text> Ty<'text> {
    fn new(s: &'text str) -> nom::IResult<&'text str, Self> {
        let (s, refs) = many0(pair(Ref::new, crate::take_whitespace0))(s)?;
        let (s, space) = crate::take_whitespace0(s)?;
        let (s, name) = crate::TyIdent::new(s)?;

        Ok((s, Self { refs, space, name }))
    }
}

struct Ref<'text> {
    ampersand: &'text str,
    ampersand_space: &'text str,
    lifetime: Option<crate::Lifetime<'text>>,
    lifetime_space: &'text str,
    mutable: Option<&'text str>,
}

impl<'text> Ref<'text> {
    fn new(s: &'text str) -> nom::IResult<&'text str, Self> {
        let (s, ampersand) = tag("&")(s)?;
        let (s, ampersand_space) = crate::take_whitespace0(s)?;
        let (s, lifetime) = opt(crate::Lifetime::new)(s)?;
        let (s, lifetime_space) = crate::take_whitespace0(s)?;
        let (s, mutable) = opt(tag("mut"))(s)?;

        Ok((
            s,
            Self {
                ampersand,
                ampersand_space,
                lifetime,
                lifetime_space,
                mutable,
            },
        ))
    }
}
