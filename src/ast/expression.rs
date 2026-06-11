// Expressions can be evaluated into a value
#[derive(Debug, PartialEq, Eq)]
pub enum Expression {
    Ident(String),
    Number(i64),
    MathOp {
        lhs: Box<Expression>,
        kind: MathOpKind,
        rhs: Box<Expression>,
    },
    FunctionCall(String),
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum MathOpKind {
    Add,
    Subtract,
    Divide,
    Multiply,
}
