use crate::val::Val;
use smol_str::SmolStr;
use std::collections::HashMap;

pub(crate) struct Env<'parent> {
    bindings: HashMap<SmolStr, Val>,
    parent: Option<&'parent Self>,
}

impl<'parent> Env<'parent> {
    pub(crate) fn new() -> Self {
        Self {
            bindings: HashMap::new(),
            parent: None,
        }
    }

    pub(crate) fn create_child(&'parent self) -> Self {
        Self {
            bindings: HashMap::new(),
            parent: Some(self),
        }
    }

    pub(crate) fn store_binding(&mut self, name: SmolStr, val: Val) {
        self.bindings.insert(name, val);
    }

    pub(crate) fn get_binding(&self, name: &SmolStr) -> Option<Val> {
        self.bindings
            .get(name)
            .cloned()
            .or_else(|| self.parent.and_then(|parent| parent.get_binding(name)))
    }
}
