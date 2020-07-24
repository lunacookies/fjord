mod syntax_kind;

use smol_str::SmolStr;
use syntax_kind::SyntaxKind;

struct Lexer<'a> {
    inner: logos::Lexer<'a, SyntaxKind>,
}

impl Iterator for Lexer<'_> {
    type Item = (SyntaxKind, SmolStr);

    fn next(&mut self) -> Option<Self::Item> {
        let kind = self.inner.next()?;
        let text = self.inner.slice().into();

        Some((kind, text))
    }
}
