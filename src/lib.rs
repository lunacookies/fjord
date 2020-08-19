//! A library that parses and evaluates Fjord code.

#![warn(missing_docs, rust_2018_idioms)]

mod lang;
mod lexer;

pub mod ast;
pub mod env;
pub mod eval;
pub mod parser;
pub mod val;

type SyntaxNode = rowan::SyntaxNode<lang::Lang>;
type SyntaxToken = rowan::SyntaxToken<lang::Lang>;
type SyntaxElement = rowan::NodeOrToken<SyntaxNode, SyntaxToken>;

#[derive(Debug)]
enum Op {
    Add,
    Sub,
    Mul,
    Div,
}

mod private {
    pub trait Sealed {}
}
