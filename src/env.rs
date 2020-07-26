use crate::val::Val;
use smol_str::SmolStr;
use std::collections::HashMap;

pub(crate) struct Env {
    bindings: HashMap<SmolStr, Val>,
}

impl Env {
    pub(crate) fn store_binding(&mut self, name: SmolStr, val: Val) {
        self.bindings.insert(name, val);
    }
}
