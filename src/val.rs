//! A representation of what a value in Fjord can be.

use crate::ast::Lambda;

/// See the module-level documentation.
#[allow(missing_docs)]
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum Val {
    Number(i64),
    Str(String),
    Lambda(Lambda),
    Nil,
}
