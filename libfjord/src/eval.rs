pub(crate) trait Eval {
    fn eval<'a>(self) -> Result<OutputExpr<'a>, Error>;
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
