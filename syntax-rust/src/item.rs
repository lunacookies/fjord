use nom::{branch::alt, bytes::complete::tag, combinator::opt, multi::many0};

// HACK: Rust mistakenly doesn’t realise that the variants of this enum are actually used.
#[allow(dead_code)]
#[derive(Debug, PartialEq)]
pub(crate) enum Item<'text> {
    Use {
        keyword: &'text str,
        keyword_space: &'text str,
        path: crate::Path<'text>,
        semicolon: Option<&'text str>,
    },
    Function {
        keyword: &'text str,
        keyword_space: &'text str,
        name: crate::Ident<'text>,
        name_space: &'text str,
        open_paren: &'text str,
        open_paren_space: &'text str,
        params: Vec<crate::FunctionDefParam<'text>>,
        params_space: &'text str,
        close_paren_space: &'text str,
        close_paren: &'text str,
        return_type: Option<crate::FunctionReturnType<'text>>,
        return_type_space: &'text str,
        body: Option<crate::Block<'text>>,
    },
}

impl<'text> Item<'text> {
    pub(crate) fn new(s: &'text str) -> nom::IResult<&'text str, Self> {
        alt((Self::new_use, Self::new_function))(s)
    }

    fn new_use(s: &'text str) -> nom::IResult<&'text str, Self> {
        let (s, keyword) = tag("use")(s)?;
        let (s, keyword_space) = crate::take_whitespace1(s)?;

        let (s, path) = crate::Path::new(s)?;
        let (s, semicolon) = opt(tag(";"))(s)?;

        Ok((
            s,
            Self::Use {
                keyword,
                keyword_space,
                path,
                semicolon,
            },
        ))
    }

    fn new_function(s: &'text str) -> nom::IResult<&'text str, Self> {
        let (s, keyword) = tag("fn")(s)?;
        let (s, keyword_space) = crate::take_whitespace1(s)?;

        let (s, name) = crate::Ident::new(s)?;
        let (s, name_space) = crate::take_whitespace0(s)?;

        let (s, open_paren) = tag("(")(s)?;
        let (s, open_paren_space) = crate::take_whitespace0(s)?;

        let (s, params) = many0(crate::FunctionDefParam::new)(s)?;
        let (s, params_space) = crate::take_whitespace0(s)?;

        let (s, close_paren) = tag(")")(s)?;
        let (s, close_paren_space) = crate::take_whitespace0(s)?;

        let (s, return_type) = opt(crate::FunctionReturnType::new)(s)?;
        let (s, return_type_space) = crate::take_whitespace0(s)?;

        let (s, body) = opt(crate::Block::new)(s)?;

        Ok((
            s,
            Self::Function {
                keyword,
                keyword_space,
                name,
                name_space,
                open_paren,
                open_paren_space,
                params,
                params_space,
                close_paren_space,
                close_paren,
                return_type,
                return_type_space,
                body,
            },
        ))
    }
}

impl<'text> From<Item<'text>> for Vec<syntax::HighlightedSpan<'text>> {
    fn from(item: Item<'text>) -> Self {
        match item {
            Item::Use {
                keyword,
                keyword_space,
                path,
                semicolon,
            } => {
                let mut output = vec![
                    syntax::HighlightedSpan {
                        text: keyword,
                        group: Some(syntax::HighlightGroup::OtherKeyword),
                    },
                    syntax::HighlightedSpan {
                        text: keyword_space,
                        group: None,
                    },
                ];

                output.append(&mut path.into());

                if let Some(semicolon) = semicolon {
                    output.push(syntax::HighlightedSpan {
                        text: semicolon,
                        group: Some(syntax::HighlightGroup::Terminator),
                    });
                }

                output
            }
            Item::Function {
                keyword,
                keyword_space,
                name,
                name_space,
                open_paren,
                open_paren_space,
                params,
                params_space,
                close_paren_space,
                close_paren,
                return_type,
                return_type_space,
                body,
            } => {
                let mut output = vec![
                    syntax::HighlightedSpan {
                        text: keyword,
                        group: Some(syntax::HighlightGroup::OtherKeyword),
                    },
                    syntax::HighlightedSpan {
                        text: keyword_space,
                        group: None,
                    },
                    syntax::HighlightedSpan {
                        text: name.name,
                        group: Some(syntax::HighlightGroup::FunctionDef),
                    },
                    syntax::HighlightedSpan {
                        text: name_space,
                        group: None,
                    },
                    syntax::HighlightedSpan {
                        text: open_paren,
                        group: Some(syntax::HighlightGroup::Delimiter),
                    },
                    syntax::HighlightedSpan {
                        text: open_paren_space,
                        group: None,
                    },
                ];

                output.extend(
                    params
                        .into_iter()
                        .map(Vec::from)
                        .flatten()
                        .chain(std::iter::once(syntax::HighlightedSpan {
                            text: params_space,
                            group: None,
                        }))
                        .chain(std::iter::once(syntax::HighlightedSpan {
                            text: close_paren,
                            group: Some(syntax::HighlightGroup::Delimiter),
                        }))
                        .chain(std::iter::once(syntax::HighlightedSpan {
                            text: close_paren_space,
                            group: None,
                        })),
                );

                if let Some(return_type) = return_type {
                    output.append(&mut Vec::from(return_type));
                }

                output.push(syntax::HighlightedSpan {
                    text: return_type_space,
                    group: None,
                });

                if let Some(body) = body {
                    output.append(&mut Vec::from(body));
                }

                output
            }
        }
    }
}
