use crate::ast::{function::ExternFunction, statement::Statement};

pub mod expression;
pub mod function;
mod ident;
pub mod statement;

pub use function::Function;
pub use ident::Ident;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Program {
    pub items: Vec<Item>,
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Item {
    Function(Function),
    ExternFunction(ExternFunction),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Block {
    pub statements: Vec<Statement>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Type {
    I32,
}
