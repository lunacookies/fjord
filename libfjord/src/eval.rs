pub(crate) trait Eval {
    fn eval(self) -> Result<OutputExpr, Error>;
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("could not find function")]
    FuncNotFound,
}

pub enum OutputExpr {
    Number(crate::Number),
}
