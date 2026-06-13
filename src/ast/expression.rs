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
    FunctionCall {
        name: String,
        args: Vec<Expression>,
    },
    Condition {
        lhs: Box<Expression>,
        kind: ConditionOpKind,
        rhs: Box<Expression>,
    },
}

#[derive(PartialEq, Eq, Debug)]
pub enum ConditionOpKind {
    MoreThen,
    LessThen,
    EqualTo,
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum MathOpKind {
    Add,
    Subtract,
    Divide,
    Multiply,
}
