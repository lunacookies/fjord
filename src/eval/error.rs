use crate::val::Ty;
use text_size::TextRange;

/// A struct representing all the possible ways evaluation can fail. This includes both the kind of
/// error that ocurred and the text range at which it is located.
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct EvalError {
    kind: EvalErrorKind,
    range: TextRange,
}

impl EvalError {
    pub(super) fn new(kind: EvalErrorKind, range: TextRange) -> Self {
        Self { kind, range }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub(super) enum EvalErrorKind {
    /// when a binding is used that has not been defined
    BindingDoesNotExist,
    /// when too many parameters are supplied to a function
    TooManyParams,
    /// when too few parameters are supplied to a function
    TooFewParams,
    /// when something that is not a lambda is called
    CallNonLambda,
    /// when a binary operation is applied to two types that are not numbers
    BinOpOnNonNumbers { lhs_ty: Ty, rhs_ty: Ty },
}
