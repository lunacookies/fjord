//! A library that parses and evaluates Fjord code.

#![warn(missing_docs, rust_2018_idioms)]

mod lang;
mod lexer;
mod parser;

type SyntaxNode = rowan::SyntaxNode<lang::Lang>;

pub use parser::Parser;
