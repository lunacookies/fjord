use nom::{bytes::complete::tag, multi::many0};

pub(crate) struct Generics<'text> {
    open_angle_bracket: &'text str,
    open_angle_bracket_space: &'text str,
    lifetimes: Vec<crate::Lifetime<'text>>,
    tys: Vec<crate::Ty<'text>>,
    close_angle_bracket_space: &'text str,
    close_angle_bracket: &'text str,
}

impl<'text> Generics<'text> {
    pub(crate) fn new(s: &'text str) -> nom::IResult<&'text str, Self> {
        let (s, open_angle_bracket) = tag("<")(s)?;
        let (s, open_angle_bracket_space) = crate::take_whitespace0(s)?;
        let (s, lifetimes) = many0(crate::Lifetime::new)(s)?;
        let (s, tys) = many0(crate::Ty::new)(s)?;
        let (s, close_angle_bracket_space) = crate::take_whitespace0(s)?;
        let (s, close_angle_bracket) = tag(">")(s)?;

        Ok((
            s,
            Self {
                open_angle_bracket,
                open_angle_bracket_space,
                lifetimes,
                tys,
                close_angle_bracket_space,
                close_angle_bracket,
            },
        ))
    }
}
