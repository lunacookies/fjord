mod syntax_kind;

use logos::Logos;
use smol_str::SmolStr;
use syntax_kind::SyntaxKind;

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
    type Item = (SyntaxKind, SmolStr);

    fn next(&mut self) -> Option<Self::Item> {
        let kind = self.inner.next()?;
        let text = self.inner.slice().into();

        Some((kind, text))
    }
}
