use crate::params::def;

pub trait ForeignFjordFunc: std::fmt::Debug {
    fn params(&self) -> &[def::Param];
    fn run(&self, params: Vec<Param>) -> crate::eval::OutputExpr;
}

#[derive(Clone, Debug)]
pub struct Param {
    name: crate::IdentName,
    val: crate::eval::OutputExpr,
}

impl Param {
    pub fn val(&self) -> &crate::eval::OutputExpr {
        &self.val
    }

    pub(crate) fn from_complete_param(
        complete_param: crate::params::CompleteParam,
        state: &crate::eval::State,
    ) -> Result<Self, crate::eval::Error> {
        Ok(Self {
            name: complete_param.name().clone(),
            val: complete_param.val().clone().eval(state)?,
        })
    }
}
