use std::collections::HashMap;

pub(crate) trait Eval {
    fn eval(self, state: &State) -> EvalResult;
}

pub struct State {
    vars: HashMap<crate::IdentName, crate::Expr>,
    funcs: HashMap<crate::IdentName, crate::Func>,
}

impl State {
    pub fn new() -> Self {
        Self {
            vars: HashMap::new(),
            funcs: HashMap::new(),
        }
    }

    pub(crate) fn get_var(&self, name: crate::IdentName) -> Option<&crate::Func> {
        self.funcs.get(&name)
    }

    pub(crate) fn get_func(&self, name: crate::IdentName) -> Option<&crate::Func> {
        self.funcs.get(&name)
    }

    pub(crate) fn set_var(&mut self, name: crate::IdentName, val: crate::Expr) {
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
}

#[derive(Debug)]
pub enum OutputExpr {
    Number(crate::Number),
    Str(String),
    Unit,
}

pub(crate) type EvalResult = Result<OutputExpr, Error>;
