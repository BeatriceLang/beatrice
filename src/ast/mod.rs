use crate::ast::statement::Statement;

pub mod expresion;
pub mod statement;

pub struct Program {
    pub functions: Vec<Function>,
}

pub struct Function {
    pub name: String,
    pub return_type: Type,
    pub body: Block,
}

pub struct Block {
    pub statements: Vec<Statement>,
}

pub enum Type {
    I32,
}
