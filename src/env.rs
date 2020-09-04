//! The evaluation environment, which holds all state needed to evaluate Fjord code.

mod commands;

use crate::eval::EvalErrorKind;
use crate::val::{FuncOrCommand, Val};
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

    pub(crate) fn get_func_or_command(
        &self,
        name: &SmolStr,
    ) -> Result<FuncOrCommand, EvalErrorKind> {
        match (self.get_binding(name), self.commands.get(name.as_str())) {
            // If we have a lambda, then we use that over a command.
            (Some(Val::Lambda(lambda)), _) => Ok(FuncOrCommand::Func(lambda)),

            // In this case we either don’t have a binding with that name, or we do have a binding
            // but it isn’t a lambda, and we have a command with the name requested.
            (_, Some(path)) => Ok(FuncOrCommand::Command(path.to_path_buf())),

            // Here we have a binding with the name, but it isn’t a lambda.
            (Some(_), None) => Err(EvalErrorKind::CallNonLambda),

            // In this case nothing exists with the specified name.
            (None, None) => Err(EvalErrorKind::FuncOrCommandDoesNotExist),
        }
    }
}
