//! A foreign function interface to Fjord.

use crate::params::def;

/// Implement this trait to make arbitrary Rust code available for calling from within
/// Fjord, as if it was a function implemented in Fjord.
pub trait ForeignFjordFunc: std::fmt::Debug {
    /// All the parameters that the foreign function accepts.
    fn params(&self) -> &[def::Param];

    /// This is called when the foreign function is called. Like any other function in Fjord, the
    /// foreign function takes in parameters that consist of names and values, and outputs an
    /// expression. The input vector of parameters is guaranteed to be in the order that the
    /// parameters from [`params`](#method.params) are.
    fn run(&self, params: Vec<Param>) -> crate::eval::OutputExpr;
}

/// A parameter of a call to a foreign function. This is separate to a normal function call
/// parameter\*, because those contain [unevaluated expressions](../enum.Expr.html), rather than the
/// [pre-evaluated ones](../eval/enum.OutputExpr.html) we want.
///
/// \*you won’t find those, they aren’t in Fjord’s public interface.
#[derive(Clone, Debug)]
pub struct Param {
    /// its name
    pub name: crate::IdentName,
    /// its value
    pub val: crate::eval::OutputExpr,
}

impl Param {
    pub(crate) fn from_complete_param(
        complete_param: crate::params::CompleteParam,
        state: &crate::eval::State,
    ) -> Result<Self, crate::eval::Error> {
        Ok(Self {
            name: complete_param.name,
            val: complete_param.val.eval(state)?,
        })
    }
}
