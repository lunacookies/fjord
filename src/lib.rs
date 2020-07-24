//! A library that parses and evaluates Fjord code.

#![warn(missing_docs, rust_2018_idioms)]

mod lexer;
mod parser;

pub use parser::Parser;
