use crate::ast::Ident;

// Expressions can be evaluated into a value
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Expression {
    Ident(Ident),
    Number(i64),
    StringLiteral(String),
    BinaryOp {
        lhs: Box<Expression>,
        kind: BinaryOpKind,
        rhs: Box<Expression>,
    },
    FunctionCall {
        name: Ident,
        args: Vec<Expression>,
    },
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum BinaryOpKind {
    Add,
    Subtract,
    Divide,
    Multiply,
    GreaterThan,
    LessThan,
    EqualTo,
}
