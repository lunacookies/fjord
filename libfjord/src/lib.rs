mod eval;
mod expr;
mod ident_name;
mod misc;

use {expr::Expr, ident_name::IdentName, misc::*};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("evaluation error")]
    Eval(#[from] eval::Error),
    #[error("parsing error")]
    Parse,
}

pub fn eval(s: &str) -> Result<eval::OutputExpr, Error> {
    use eval::Eval;

    let (_, expr) = match Expr::new(s) {
        Ok(e) => e,
        _ => return Err(Error::Parse),
    };
    Ok(expr.eval()?)
}
