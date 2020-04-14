//! Types for working with the evaluation of Fjord code.

use {crate::ffi::ForeignFjordFunc, std::collections::HashMap};

/// A structure that contains all the variables and functions available at a given location in a
/// Fjord program.
#[derive(Debug)]
pub struct State<'a> {
    vars: HashMap<crate::IdentName, OutputExpr>,
    funcs: HashMap<crate::IdentName, crate::Func>,
    foreign_funcs: HashMap<crate::IdentName, Box<dyn ForeignFjordFunc>>,
    parent: Option<&'a Self>,
}

impl<'a> State<'a> {
    /// This creates a new ‘root’ state (meaning that it does not have a parent state) from a
    /// vector of foreign functions (plus their names).
    pub fn new_root(foreign_funcs: Vec<(crate::IdentName, Box<dyn ForeignFjordFunc>)>) -> Self {
        use std::iter::FromIterator;

        Self {
            vars: HashMap::new(),
            funcs: HashMap::new(),
            foreign_funcs: HashMap::from_iter(foreign_funcs),
            parent: None,
        }
    }

    pub(crate) fn new_child(&'a self) -> Self {
        Self {
            vars: HashMap::new(),
            funcs: HashMap::new(),
            foreign_funcs: HashMap::new(),
            parent: Some(self),
        }
    }

    pub(crate) fn get_var(&self, name: crate::IdentName) -> Option<&OutputExpr> {
        self.vars.get(&name).or_else(|| match self.parent {
            Some(parent_state) => parent_state.get_var(name),
            _ => None,
        })
    }

    pub(crate) fn get_func(&self, name: crate::IdentName) -> Option<&crate::Func> {
        self.funcs.get(&name).or_else(|| match self.parent {
            Some(parent_state) => parent_state.get_func(name),
            _ => None,
        })
    }

    #[allow(clippy::borrowed_box)]
    pub(crate) fn get_foreign_func(
        &self,
        name: crate::IdentName,
    ) -> Option<&Box<dyn ForeignFjordFunc>> {
        self.foreign_funcs.get(&name).or_else(|| match self.parent {
            Some(parent_state) => parent_state.get_foreign_func(name),
            _ => None,
        })
    }

    pub(crate) fn set_var(&mut self, name: crate::IdentName, val: OutputExpr) {
        self.vars.insert(name, val);
    }

    pub(crate) fn set_func(&mut self, name: crate::IdentName, func: crate::Func) {
        self.funcs.insert(name, func);
    }
}

/// All kinds of errors that can occur while evaluating code.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// when a variable is used that does not exist
    #[error("could not find variable")]
    VarNotFound,
    /// when a function is called that does not exist
    #[error("could not find function")]
    FuncNotFound,
    /// when some kind of error occurs while matching up function call parameters with function
    /// definition parameters
    #[error("failed evaluating function parameters")]
    FuncParamError(#[from] crate::params::Error),
}

/// The output of evaluating something.
#[derive(Clone, Debug)]
pub enum OutputExpr {
    /// a number
    Number(crate::Number),
    /// a string
    Str(String),
    /// the ‘unit type’, equivalent to Rust’s `()`
    Unit,
}

pub(crate) type EvalResult = Result<OutputExpr, Error>;
