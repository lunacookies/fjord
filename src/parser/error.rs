use text_size::TextRange;

#[derive(Debug)]
pub(super) struct SyntaxError {
    pub(super) message: &'static str,
    pub(super) range: TextRange,
}
