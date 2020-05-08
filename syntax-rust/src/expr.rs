use {
    crate::ParseResult,
    nom::{branch::alt, bytes::complete::tag, combinator::map, multi::many0},
};

mod character;
mod fn_call;
mod if_;
mod int;
mod macro_invocation;
mod postfix;
mod prefix;
mod string;
mod struct_literal;
mod variable;

use {
    character::parse as character, fn_call::parse as fn_call, if_::parse as if_, int::parse as int,
    macro_invocation::parse as macro_invocation, postfix::parse as postfix,
    prefix::parse as prefix, string::parse as string, struct_literal::parse as struct_literal,
    variable::parse as variable,
};

pub(crate) fn parse(s: &str) -> ParseResult<'_> {
    let (s, prefixes) = many0(prefix)(s)?;

    let (s, mut expr) = alt((
        crate::block,
        if_,
        fn_call,
        macro_invocation,
        boolean,
        variable,
        struct_literal,
        string,
        character,
        int,
    ))(s)?;

    let (s, postfixes) = many0(postfix)(s)?;

    let mut output = prefixes.concat();
    output.append(&mut expr);
    output.append(&mut postfixes.concat());

    Ok((s, output))
}

fn boolean(s: &str) -> ParseResult<'_> {
    map(alt((tag("true"), tag("false"))), |s| {
        vec![syntax::HighlightedSpan {
            text: s,
            group: Some(syntax::HighlightGroup::Boolean),
        }]
    })(s)
}
