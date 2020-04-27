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
        rhs: Option<LetRhs<'text>>,
        rhs_space: &'text str,
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

        let (s, rhs) = opt(LetRhs::new)(s)?;
        let (s, rhs_space) = crate::take_whitespace0(s)?;

        let (s, semicolon) = opt(tag(";"))(s)?;

        Ok((
            s,
            Self::Let {
                keyword,
                keyword_space,
                pattern,
                pattern_space,
                rhs,
                rhs_space,
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
                rhs,
                rhs_space,
                semicolon,
            } => {
                let mut output: Vec<_> = std::iter::once(syntax::HighlightedSpan {
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
                .collect();

                if let Some(rhs) = rhs {
                    output.append(&mut Vec::from(rhs));
                }

                output.push(syntax::HighlightedSpan {
                    text: rhs_space,
                    group: None,
                });

                if let Some(semicolon) = semicolon {
                    output.push(syntax::HighlightedSpan {
                        text: semicolon,
                        group: Some(syntax::HighlightGroup::Terminator),
                    });
                }

                output
            }
        }
    }
}

#[derive(Debug, PartialEq)]
pub(crate) struct LetRhs<'text> {
    equals: &'text str,
    equals_space: &'text str,
    value: crate::Expr<'text>,
}

impl<'text> LetRhs<'text> {
    fn new(s: &'text str) -> nom::IResult<&'text str, Self> {
        let (s, equals) = tag("=")(s)?;
        let (s, equals_space) = crate::take_whitespace0(s)?;

        let (s, value) = crate::Expr::new(s)?;

        Ok((
            s,
            Self {
                equals,
                equals_space,
                value,
            },
        ))
    }
}

impl<'rhs> From<LetRhs<'rhs>> for Vec<syntax::HighlightedSpan<'rhs>> {
    fn from(rhs: LetRhs<'rhs>) -> Self {
        std::iter::once(syntax::HighlightedSpan {
            text: rhs.equals,
            group: Some(syntax::HighlightGroup::AssignOper),
        })
        .chain(std::iter::once(syntax::HighlightedSpan {
            text: rhs.equals_space,
            group: None,
        }))
        .chain(Vec::from(rhs.value).into_iter())
        .collect()
    }
}
