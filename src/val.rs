#[derive(Clone)]
pub(crate) enum Val {
    Number(i64),
    Str(String),
    Nil,
}
