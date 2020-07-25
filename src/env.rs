use crate::val::Val;
use smol_str::SmolStr;
use std::collections::HashMap;

pub(crate) struct Env {
    bindings: HashMap<SmolStr, Val>,
}
