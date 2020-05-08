use {
    crate::{
        utils::{float_ty, int_ty, pascal_case},
        ParseResult,
    },
    nom::{branch::alt, bytes::complete::tag, combinator::map},
};

pub(crate) fn parse(s: &str) -> ParseResult<'_> {
    let user_ty = map(pascal_case, |s| {
        vec![syntax::HighlightedSpan {
            text: s,
            group: Some(syntax::HighlightGroup::TyUse),
        }]
    });

    alt((user_ty, primitive_ty))(s)
}

fn primitive_ty(s: &str) -> ParseResult<'_> {
    map(
        alt((int_ty, float_ty, tag("str"), tag("bool"), tag("!"))),
        |s| {
            vec![syntax::HighlightedSpan {
                text: s,
                group: Some(syntax::HighlightGroup::PrimitiveTy),
            }]
        },
    )(s)
}
