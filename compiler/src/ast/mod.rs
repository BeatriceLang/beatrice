use crate::ast::{item::Item, statement::Statement};

pub mod expression;
pub mod ident;
pub mod item;
pub mod statement;
pub mod ty;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Program {
    pub items: Vec<Item>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Block {
    pub statements: Vec<Statement>,
}
