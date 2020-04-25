use nom::{bytes::complete::tag, combinator::opt, multi::many0, sequence::pair};

pub(crate) struct Ty<'text> {
    // The second item in this tuple is whitespace.
    refs: Vec<(Ref<'text>, &'text str)>,
    refs_space: &'text str,
    name: crate::TyIdent<'text>,
    name_space: &'text str,
    generics: crate::Generics<'text>,
}

impl<'text> Ty<'text> {
    pub(crate) fn new(s: &'text str) -> nom::IResult<&'text str, Self> {
        let (s, refs) = many0(pair(Ref::new, crate::take_whitespace0))(s)?;
        let (s, refs_space) = crate::take_whitespace0(s)?;
        let (s, name) = crate::TyIdent::new(s)?;
        let (s, name_space) = crate::take_whitespace0(s)?;
        let (s, generics) = crate::Generics::new(s)?;

        Ok((
            s,
            Self {
                refs,
                refs_space,
                name,
                name_space,
                generics,
            },
        ))
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
