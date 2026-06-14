use crate::ast::{Block, Type, expression::Expression};

// Statements will proform an action (aka do something)
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Statement {
    Return(Expression),
    Expression(Expression),
    If {
        cond: Expression,
        body: Block,
    },
    Let {
        name: String,
        ty: Type,
        value: Expression,
    },
}
