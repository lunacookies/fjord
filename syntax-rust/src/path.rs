use {
    crate::ParseResult,
    nom::{bytes::complete::tag, combinator::opt, multi::many0},
};

pub(crate) fn parse(s: &str) -> ParseResult<'_> {
    let (s, leading_colons) = opt(tag("::"))(s)?;

    let (s, modules) = many0(|s| {
        let (s, module_name) = crate::module_name(s)?;
        let (s, double_colon) = tag("::")(s)?;

        let mut output = module_name;

        output.push(syntax::HighlightedSpan {
            text: double_colon,
            group: Some(syntax::HighlightGroup::Separator),
        });

        Ok((s, output))
    })(s)?;

    let (s, associated_tys) = many0(|s| {
        let (s, ty_name) = crate::ty_name(s)?;
        let (s, double_colon) = tag("::")(s)?;

        let (s, turbofish) = opt(|s| {
            let (s, generics) = crate::generics::usage(s)?;
            let (s, double_colon) = tag("::")(s)?;

            let mut output = generics;

            output.push(syntax::HighlightedSpan {
                text: double_colon,
                group: Some(syntax::HighlightGroup::Separator),
            });

            Ok((s, output))
        })(s)?;

        let mut output = ty_name;

        output.push(syntax::HighlightedSpan {
            text: double_colon,
            group: Some(syntax::HighlightGroup::Separator),
        });

        if let Some(mut turbofish) = turbofish {
            output.append(&mut turbofish);
        }

        Ok((s, output))
    })(s)?;

    let mut output = if let Some(leading_colons) = leading_colons {
        vec![syntax::HighlightedSpan {
            text: leading_colons,
            group: Some(syntax::HighlightGroup::Separator),
        }]
    } else {
        vec![]
    };

    output.append(&mut modules.concat());
    output.append(&mut associated_tys.concat());

    Ok((s, output))
}
