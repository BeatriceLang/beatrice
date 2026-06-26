use std::path::PathBuf;

use crate::ast::{expression::Expression, function::ExternFunction, statement::Statement};

pub mod expression;
pub mod function;
mod ident;
pub mod statement;

pub use function::Function;
pub use ident::Ident;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Program {
    pub items: Vec<Item>,
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Item {
    Function(Function),
    ExternFunction(ExternFunction),
    Import(PathBuf),
    Const(Const),
    DeclareStruct(DeclareStruct),
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct DeclareStruct {
    pub name: Ident,
    pub fields: Vec<(Ident, Type)>,
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Const {
    pub name: Ident,
    pub ty: Type,
    pub val: Expression,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Block {
    pub statements: Vec<Statement>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Type {
    I32,
    U32,
    String,
    Bool,
    Ptr(Box<Self>),
    Struct(String),
}

impl Type {
    pub(crate) const fn signed(&self) -> Option<bool> {
        match self {
            Self::U32 => Some(false),
            Self::I32 => Some(true),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Type;

    #[test]
    fn reports_integer_signedness() {
        assert_eq!(Type::I32.signed(), Some(true));
        assert_eq!(Type::U32.signed(), Some(false));
    }

    #[test]
    fn reports_no_signedness_for_non_integer_types() {
        assert_eq!(Type::String.signed(), None);
        assert_eq!(Type::Ptr(Box::new(Type::I32)).signed(), None);
        assert_eq!(Type::Struct("Point".into()).signed(), None);
    }
}
