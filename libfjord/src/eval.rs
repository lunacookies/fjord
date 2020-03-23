use std::collections::HashMap;

pub(crate) trait Eval<'a> {
    fn eval(self, state: &'a State<'a>) -> Result<OutputExpr<'a>, Error>;
}

pub struct State<'a> {
    funcs: HashMap<crate::IdentName<'a>, crate::Func<'a>>,
}

impl<'a> State<'a> {
    pub(crate) fn new() -> Self {
        Self {
            funcs: HashMap::new(),
        }
    }

    pub(crate) fn get_func(&self, name: crate::IdentName<'a>) -> Option<&'a crate::Func> {
        self.funcs.get(&name)
    }
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("could not find function")]
    FuncNotFound,
}

pub enum OutputExpr<'a> {
    Number(crate::Number),
    Str(&'a str),
}
