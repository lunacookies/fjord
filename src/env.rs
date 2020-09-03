//! The evaluation environment, which holds all state needed to evaluate Fjord code.

mod commands;

use crate::val::Val;
use commands::Commands;
use smol_str::SmolStr;
use std::collections::HashMap;
use std::io;
use std::path::PathBuf;
use std::rc::Rc;

/// See the module-level documentation.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Env<'parent> {
    bindings: HashMap<SmolStr, Val>,
    parent: Option<&'parent Self>,
    commands: Rc<Commands>,
}

impl<'parent> Env<'parent> {
    /// Constructs a new evaluation environment given a vector of paths to search for executables.
    pub fn new(search_path: Vec<PathBuf>) -> io::Result<Self> {
        Ok(Self {
            bindings: HashMap::new(),
            parent: None,
            commands: Rc::new(Commands::new(search_path)?),
        })
    }

    pub(crate) fn create_child(&'parent self) -> Self {
        Self {
            bindings: HashMap::new(),
            parent: Some(self),
            commands: Rc::clone(&self.commands),
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
