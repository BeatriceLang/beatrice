use crate::ast::{Block, Ident, Type};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Function {
    pub name: Ident,
    pub params: Vec<(Ident, Type)>,
    pub return_type: Type,
    pub body: Block,
}
