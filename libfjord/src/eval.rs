use {crate::ffi::ForeignFjordFunc, std::collections::HashMap};

#[derive(Debug)]
pub struct State<'a> {
    vars: HashMap<crate::IdentName, OutputExpr>,
    funcs: HashMap<crate::IdentName, crate::Func>,
    foreign_funcs: HashMap<crate::IdentName, Box<dyn ForeignFjordFunc>>,
    parent: Option<&'a Self>,
}

impl<'a> State<'a> {
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

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("could not find variable")]
    VarNotFound,
    #[error("could not find function")]
    FuncNotFound,
    #[error("failed evaluating function parameters")]
    FuncParamError(#[from] crate::params::Error),
}

#[derive(Clone, Debug)]
pub enum OutputExpr {
    Number(crate::Number),
    Str(String),
    Unit,
}

pub(crate) type EvalResult = Result<OutputExpr, Error>;
