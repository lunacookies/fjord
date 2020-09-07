//! A representation of what a value in Fjord can be.

use crate::ast::Lambda;
use std::path::PathBuf;

/// See the module-level documentation.
#[allow(missing_docs)]
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum Val {
    Number(i64),
    Str(String),
    Bool(bool),
    Lambda(Lambda),
    Nil,
}

impl Val {
    pub(crate) fn ty(&self) -> Ty {
        match self {
            Self::Number(_) => Ty::Number,
            Self::Str(_) => Ty::Str,
            Self::Bool(_) => Ty::Bool,
            Self::Lambda(_) => Ty::Lambda,
            Self::Nil => Ty::Nil,
        }
    }
}

impl Val {
    pub(crate) fn display_repr(&self) -> Option<String> {
        match self {
            Self::Number(n) => Some(n.to_string()),
            Self::Str(s) => Some(s.clone()),
            Self::Bool(b) => {
                if *b {
                    Some("true".to_string())
                } else {
                    Some("false".to_string())
                }
            }
            Self::Lambda(_) => None,
            Self::Nil => Some("nil".to_string()),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub(crate) enum Ty {
    Number,
    Str,
    Bool,
    Lambda,
    Nil,
}

pub(crate) enum FuncOrCommand {
    Func(Lambda),
    Command(PathBuf),
}
