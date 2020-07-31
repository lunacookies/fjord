use text_size::TextRange;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub(super) struct SyntaxError {
    pub(super) message: &'static str,
    pub(super) range: TextRange,
}
