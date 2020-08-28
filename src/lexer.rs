mod syntax_kind;
pub(crate) use syntax_kind::SyntaxKind;

use logos::Logos;
use smol_str::SmolStr;
use std::{convert::TryFrom, ops::Range};
use text_size::{TextRange, TextSize};

#[derive(Debug)]
pub(crate) struct Lexeme {
    pub(crate) kind: SyntaxKind,
    pub(crate) text: SmolStr,
    pub(crate) range: TextRange,
}

pub(crate) struct Lexer<'a> {
    inner: logos::Lexer<'a, SyntaxKind>,
}

impl<'a> Lexer<'a> {
    pub(crate) fn new(input: &'a str) -> Self {
        Self {
            inner: SyntaxKind::lexer(input),
        }
    }
}

impl Iterator for Lexer<'_> {
    type Item = Lexeme;

    fn next(&mut self) -> Option<Self::Item> {
        let kind = self.inner.next()?;
        let text = self.inner.slice().into();

        let Range { start, end } = self.inner.span();
        let start = TextSize::try_from(start).unwrap();
        let end = TextSize::try_from(end).unwrap();
        let range = TextRange::new(start, end);

        Some(Lexeme { kind, text, range })
    }
}
