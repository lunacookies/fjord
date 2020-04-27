use nom::{bytes::complete::tag, multi::many0, sequence::pair};

#[derive(Debug, PartialEq)]
pub(crate) struct Block<'text> {
    open_brace: &'text str,
    open_brace_space: &'text str,
    // The second item of this tuple is whitespace.
    statements: Vec<(crate::Statement<'text>, &'text str)>,
    close_brace: &'text str,
}

impl<'text> Block<'text> {
    pub(crate) fn new(s: &'text str) -> nom::IResult<&'text str, Self> {
        let (s, open_brace) = tag("{")(s)?;
        let (s, open_brace_space) = crate::take_whitespace0(s)?;

        let (s, statements) = many0(pair(crate::Statement::new, crate::take_whitespace0))(s)?;

        let (s, close_brace) = tag("}")(s)?;

        Ok((
            s,
            Self {
                open_brace,
                open_brace_space,
                statements,
                close_brace,
            },
        ))
    }
}

impl<'b> From<Block<'b>> for Vec<syntax::HighlightedSpan<'b>> {
    fn from(block: Block<'b>) -> Self {
        std::iter::once(syntax::HighlightedSpan {
            text: block.open_brace,
            group: Some(syntax::HighlightGroup::Delimiter),
        })
        .chain(std::iter::once(syntax::HighlightedSpan {
            text: block.open_brace_space,
            group: None,
        }))
        .chain(
            block
                .statements
                .into_iter()
                .map(|(statement, whitespace)| {
                    Vec::from(statement).into_iter().chain(std::iter::once(
                        syntax::HighlightedSpan {
                            text: whitespace,
                            group: None,
                        },
                    ))
                })
                .flatten(),
        )
        .chain(std::iter::once(syntax::HighlightedSpan {
            text: block.close_brace,
            group: Some(syntax::HighlightGroup::Delimiter),
        }))
        .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(
            Block::new("{}"),
            Ok((
                "",
                Block {
                    open_brace: "{",
                    open_brace_space: "",
                    statements: vec![],
                    close_brace: "}"
                }
            ))
        )
    }
}
