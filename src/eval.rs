//! Types for working with the evaluation of Fjord code.

use std::{collections::HashMap, path::PathBuf};

/// A structure that contains all the variables, functions and commands available at a given
/// location in a Fjord program.
#[derive(Debug)]
pub struct State<'a> {
    env: Environment<'a>,
    kind: StateKind<'a>,
}

impl<'a> State<'a> {
    /// This creates a new ‘root’ state (meaning that it does not have a parent state).
    pub fn new_root(commands: &'a crate::Commands) -> Self {
        Self {
            env: Environment::new(commands),
            kind: StateKind::Root,
        }
    }

    pub(crate) fn new_child(&'a self) -> Self {
        Self {
            env: Environment::new(self.env.commands),
            kind: StateKind::Child { parent: self },
        }
    }

    pub(crate) fn get_var(&self, name: &crate::IdentName) -> Option<&OutputExpr> {
        self.env.vars.get(name).or_else(|| match self.kind {
            StateKind::Child {
                parent: parent_state,
            } => parent_state.get_var(name),
            _ => None,
        })
    }

    pub(crate) fn get_func(&self, name: &crate::IdentName) -> Option<&crate::Func> {
        self.env.funcs.get(name).or_else(|| match self.kind {
            StateKind::Child {
                parent: parent_state,
            } => parent_state.get_func(name),
            _ => None,
        })
    }

    pub(crate) fn get_command(&self, name: &str) -> Option<PathBuf> {
        self.env.commands.get(name)
    }

    pub(crate) fn set_var(&mut self, name: crate::IdentName, val: OutputExpr) {
        self.env.vars.insert(name, val);
    }

    pub(crate) fn set_func(&mut self, name: crate::IdentName, func: crate::Func) {
        self.env.funcs.insert(name, func);
    }
}

#[derive(Debug)]
enum StateKind<'a> {
    Root,
    Child { parent: &'a State<'a> },
}

#[derive(Debug)]
struct Environment<'a> {
    vars: HashMap<crate::IdentName, OutputExpr>,
    funcs: HashMap<crate::IdentName, crate::Func>,
    commands: &'a crate::Commands,
}

impl<'a> Environment<'a> {
    fn new(commands: &'a crate::Commands) -> Self {
        Self {
            vars: HashMap::new(),
            funcs: HashMap::new(),
            commands,
        }
    }
}

/// All kinds of errors that can occur while evaluating code.
#[derive(Debug, thiserror::Error, PartialEq)]
pub enum Error {
    /// when a variable is used that does not exist
    #[error("could not find variable ‘.{0}’")]
    VarNotFound(crate::IdentName),
    /// when a function or command is called that does not exist
    #[error("could not find function or command ‘{0}’")]
    FuncOrCommandNotFound(crate::IdentName),
    /// when some kind of error occurs while matching up function call parameters with function
    /// definition parameters
    #[error("failed evaluating function parameters")]
    FuncParamError(#[from] crate::params::Error),
    /// when a non-boolean expression is used as the condition of an if expression
    #[error("a non-boolean expression was used as the condition of an if expression")]
    NonBoolCond,
    /// when the execution of a command fails
    #[error("executing command failed")]
    CommandFailure,
    /// when a command exits with a non-zero exit code
    #[error("command exited with a non-zero exit code: {0}")]
    NonZeroExitCode(i32),
}

/// The output of evaluating something.
#[derive(Clone, Debug, PartialEq)]
pub enum OutputExpr {
    /// a number
    Number(crate::Number),
    /// a string
    Str(String),
    /// a boolean,
    Bool(bool),
    /// the ‘unit type’, equivalent to Rust’s `()`
    Unit,
}

impl OutputExpr {
    pub(crate) fn format(self) -> String {
        match self {
            OutputExpr::Bool(true) => "true".into(),
            OutputExpr::Bool(false) => "false".into(),
            OutputExpr::Number(n) => n.to_string(),
            OutputExpr::Str(s) => s,
            OutputExpr::Unit => "()".into(),
        }
    }
}

impl std::fmt::Display for OutputExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OutputExpr::Bool(true) => f.write_str("true")?,
            OutputExpr::Bool(false) => f.write_str("false")?,
            OutputExpr::Number(n) => write!(f, "{}", n)?,
            OutputExpr::Str(s) => write!(f, "\"{}\"", s)?,
            OutputExpr::Unit => (),
        }

        Ok(())
    }
}

/// This error occurs when a conversion from an OutputExpr to a type that isn’t contained by the
/// OutputExpr is attempted.
#[derive(Debug)]
pub struct TypeError;

macro_rules! impl_try_from {
    ($variant:ident, $contained_ty:ty) => {
        impl std::convert::TryFrom<OutputExpr> for $contained_ty {
            type Error = TypeError;

            fn try_from(e: OutputExpr) -> Result<Self, Self::Error> {
                match e {
                    OutputExpr::$variant(x) => Ok(x),
                    _ => Err(TypeError),
                }
            }
        }
    };
}

impl_try_from!(Number, i32);
impl_try_from!(Str, String);
impl_try_from!(Bool, bool);

pub(crate) type EvalResult = Result<OutputExpr, Error>;
