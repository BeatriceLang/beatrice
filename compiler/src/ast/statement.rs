use crate::ast::{Block, Ident, Type, expression::Expression};

// Statements will proform an action (aka do something)
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Statement {
    Return(Expression),
    Expression(Expression),
    If {
        cond: Expression,
        body: Block,
    },
    While {
        cond: Expression,
        body: Block,
    },
    Var {
        name: Ident,
        ty: Type,
        value: Expression,
    },
    Let {
        name: Ident,
        ty: Type,
        value: Expression,
    },
    Assign {
        ident: Ident,
        value: Expression,
    },
}
