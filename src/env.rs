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

    pub(crate) fn get_binding(&self, name: &SmolStr) -> Option<Val> {
        self.bindings.get(name).cloned()
    }
}
