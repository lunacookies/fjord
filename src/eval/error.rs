/// An enum representing all the possible ways evaluation can fail.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum EvalError {
    /// when a binding is used that has not been defined
    BindingDoesNotExist,
    /// when too many parameters are supplied to a function
    TooManyParams,
}
