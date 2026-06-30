pub mod expression;
pub mod ident;
pub mod item;
pub mod statement;
pub mod ty;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Program {
    pub items: Vec<item::Item>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Block {
    pub statements: Vec<statement::Statement>,
}
