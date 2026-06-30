use crate::ast::{Block, expression::Expression, ident::Ident, ty::Type};

// Statements will proform an action (aka do something)
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Statement {
    Return(Option<Expression>),
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
