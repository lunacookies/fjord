use nom::{
    branch::alt,
    bytes::complete::tag,
    combinator::{map, opt},
};

// HACK: Rust mistakenly doesn’t realise that the variants of this enum are actually used.
#[allow(dead_code)]
#[derive(Debug, PartialEq)]
pub(crate) enum Statement<'text> {
    Item(crate::Item<'text>),
    Expr(crate::Expr<'text>),
    Let {
        keyword: &'text str,
        keyword_space: &'text str,
        pattern: crate::Pattern<'text>,
        pattern_space: &'text str,
        equals: &'text str,
        equals_space: &'text str,
        value: crate::Expr<'text>,
        semicolon: Option<&'text str>,
    },
}

impl<'text> Statement<'text> {
    pub(crate) fn new(s: &'text str) -> nom::IResult<&'text str, Self> {
        alt((Self::new_item, Self::new_expr, Self::new_let))(s)
    }

    fn new_item(s: &'text str) -> nom::IResult<&'text str, Self> {
        map(crate::Item::new, Self::Item)(s)
    }

    fn new_expr(s: &'text str) -> nom::IResult<&'text str, Self> {
        map(crate::Expr::new, Self::Expr)(s)
    }

    fn new_let(s: &'text str) -> nom::IResult<&'text str, Self> {
        let (s, keyword) = tag("let")(s)?;
        let (s, keyword_space) = crate::take_whitespace1(s)?;

        let (s, pattern) = crate::Pattern::new(s)?;
        let (s, pattern_space) = crate::take_whitespace0(s)?;

        let (s, equals) = tag("=")(s)?;
        let (s, equals_space) = crate::take_whitespace0(s)?;

        let (s, value) = crate::Expr::new(s)?;

        let (s, semicolon) = opt(tag(";"))(s)?;

        Ok((
            s,
            Self::Let {
                keyword,
                keyword_space,
                pattern,
                pattern_space,
                equals,
                equals_space,
                value,
                semicolon,
            },
        ))
    }
}

impl<'s> From<Statement<'s>> for Vec<syntax::HighlightedSpan<'s>> {
    fn from(statement: Statement<'s>) -> Self {
        match statement {
            Statement::Item(item) => Vec::from(item),
            Statement::Expr(expr) => Vec::from(expr),
            Statement::Let {
                keyword,
                keyword_space,
                pattern,
                pattern_space,
                equals,
                equals_space,
                value,
                semicolon,
            } => {
                let output = std::iter::once(syntax::HighlightedSpan {
                    text: keyword,
                    group: Some(syntax::HighlightGroup::OtherKeyword),
                })
                .chain(std::iter::once(syntax::HighlightedSpan {
                    text: keyword_space,
                    group: None,
                }))
                .chain(Vec::from(pattern))
                .chain(std::iter::once(syntax::HighlightedSpan {
                    text: pattern_space,
                    group: None,
                }))
                .chain(std::iter::once(syntax::HighlightedSpan {
                    text: equals,
                    group: Some(syntax::HighlightGroup::AssignOper),
                }))
                .chain(std::iter::once(syntax::HighlightedSpan {
                    text: equals_space,
                    group: None,
                }))
                .chain(Vec::from(value));

                if let Some(semicolon) = semicolon {
                    output
                        .chain(std::iter::once(syntax::HighlightedSpan {
                            text: semicolon,
                            group: Some(syntax::HighlightGroup::Terminator),
                        }))
                        .collect()
                } else {
                    output.collect()
                }
            }
        }
    }
}
