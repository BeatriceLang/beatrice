use std::path::PathBuf;

use crate::ast::{
    expression::Expression, function::ExternFunction, statement::Statement, ty::Type,
};

pub mod expression;
pub mod function;
mod ident;
pub mod statement;
pub mod ty;

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
    DeclareStruct(DeclareStruct),
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct DeclareStruct {
    pub name: Ident,
    pub fields: Vec<(Ident, Type)>,
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
