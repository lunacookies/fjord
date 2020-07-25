//! A library that parses and evaluates Fjord code.

#![warn(missing_docs, rust_2018_idioms)]

mod ast;
mod env;
mod lang;
mod lexer;
mod parser;
mod val;

type SyntaxNode = rowan::SyntaxNode<lang::Lang>;

pub use parser::{ParseOutput, Parser};
