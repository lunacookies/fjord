mod eval;
mod expr;
mod func;
mod ident_name;
mod misc;

use {expr::Expr, func::Func, ident_name::IdentName, misc::*};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("evaluation error")]
    Eval(#[from] eval::Error),
    #[error("parsing error")]
    Parse,
}

pub fn eval<'a>(s: &'a str, state: &'a eval::State<'a>) -> Result<eval::OutputExpr<'a>, Error> {
    use eval::Eval;

    let (_, expr) = match Expr::new(s) {
        Ok(e) => e,
        _ => return Err(Error::Parse),
    };

    Ok(expr.eval(state)?)
}
