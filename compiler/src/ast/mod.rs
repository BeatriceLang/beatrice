use std::{collections::HashMap, path::PathBuf};

use crate::ast::{expression::Expression, function::ExternFunction, statement::Statement};

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
    Import(PathBuf),
    Const(Const),
    Struct(Struct),
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Struct {
    pub name: Ident,
    pub fields: HashMap<Ident, Type>,
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Const {
    pub name: Ident,
    pub ty: Type,
    pub val: Expression,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Block {
    pub statements: Vec<Statement>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Type {
    I32,
    String,
    Ptr(Box<Self>),
}
