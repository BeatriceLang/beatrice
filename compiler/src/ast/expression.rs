use crate::ast::{Ident, ty::Type};

// Expressions can be evaluated into a value
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Expression {
    Ident(Ident),
    Bool(bool),
    Number(i64),
    StringLiteral(String),
    TypedNumber {
        value: i64,
        ty: Type,
    },
    BinaryOp {
        lhs: Box<Self>,
        kind: BinaryOpKind,
        rhs: Box<Self>,
    },
    FunctionCall {
        name: Ident,
        args: Vec<Self>,
    },
    Deref {
        ptr: Box<Self>,
    },
    AddressOf {
        value: Box<Self>,
    },
    CreateStruct {
        name: Ident,
        fields: Vec<(Ident, Box<Self>)>,
    },
    CreateArray(Vec<Self>),
    FieldAccess {
        base: Ident,
        field: Ident,
    },
    ArrayAccess {
        array: Box<Self>,
        index: Box<Self>,
    },
    Cast {
        value: Box<Self>,
        to: Type,
    },
    Invert(Box<Self>),
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
