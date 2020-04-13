pub mod eval;
mod expr;
pub mod ffi;
mod func;
mod ident_name;
mod item;
mod misc;
pub mod params;

use misc::*;
pub use {
    expr::Expr,
    func::Func,
    ident_name::IdentName,
    item::{BindingVal, Item},
};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("evaluation error")]
    Eval(#[from] eval::Error),
    #[error("parsing error")]
    Parse,
}

pub fn eval(s: &str, state: &mut eval::State) -> Result<eval::OutputExpr, Error> {
    let (_, expr) = match Item::new(s) {
        Ok(e) => e,
        _ => return Err(Error::Parse),
    };

    Ok(expr.eval(state)?)
}
