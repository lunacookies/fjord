use text_size::TextRange;

/// A syntax error encountered during parsing.
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct SyntaxError {
    pub(super) message: &'static str,
    pub(super) range: TextRange,
}
