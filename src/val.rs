use crate::ast::Lambda;

#[derive(Clone)]
pub(crate) enum Val {
    Number(i64),
    Str(String),
    Lambda(Lambda),
    Nil,
}
