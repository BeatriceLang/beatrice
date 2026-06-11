// Expressions can be evaluated into a value
#[derive(Debug, PartialEq, Eq)]
pub enum Expression {
    Ident(String),
    Number(i64),
    MathOp {
        kind: MathOpKind,
        lhs: Box<Expression>,
        rhs: Box<Expression>,
    },
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum MathOpKind {
    Add,
    Subtract,
    Divide,
    Multiply,
}
