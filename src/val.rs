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

impl Val {
    pub(crate) fn ty(&self) -> Ty {
        match self {
            Self::Number(_) => Ty::Number,
            Self::Str(_) => Ty::Str,
            Self::Lambda(_) => Ty::Lambda,
            Self::Nil => Ty::Nil,
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub(crate) enum Ty {
    Number,
    Str,
    Lambda,
    Nil,
}
