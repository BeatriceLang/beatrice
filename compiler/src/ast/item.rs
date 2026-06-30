use std::path::PathBuf;

use super::{expression::Expression, ident::Ident, Block, ty::Type};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Item {
    Function(Function),
    ExternFunction(ExternFunction),
    Import(PathBuf),
    Const(Const),
    DeclareStruct(DeclareStruct),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DeclareStruct {
    pub name: Ident,
    pub fields: Vec<(Ident, Type)>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Const {
    pub name: Ident,
    pub ty: Type,
    pub val: Expression,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Function {
    pub name: Ident,
    pub params: Vec<(Ident, Type)>,
    pub return_type: Option<Type>,
    pub body: Block,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ExternFunction {
    pub name: Ident,
    pub params: Vec<(Ident, Type)>,
    pub return_type: Option<Type>,
}
