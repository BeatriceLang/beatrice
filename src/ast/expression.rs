// Expressions can be evaluated into a value
#[derive(Debug, PartialEq, Eq)]
pub enum Expression {
    Ident(String),
    Number(i64),
    BinaryOp {
        lhs: Box<Expression>,
        kind: BinaryOpKind,
        rhs: Box<Expression>,
    },
    FunctionCall {
        name: String,
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
