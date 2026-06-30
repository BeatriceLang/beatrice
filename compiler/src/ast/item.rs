use std::path::PathBuf;

use super::{expression::Expression, ident::Ident, Block, ty::Type};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Item {
    Function {
        name: Ident,
        params: Vec<(Ident, Type)>,
        return_type: Option<Type>,
        body: Block,
    },
    ExternFunction {
        name: Ident,
        params: Vec<(Ident, Type)>,
        return_type: Option<Type>,
    },
    Import(PathBuf),
    Const {
        name: Ident,
        ty: Type,
        val: Expression,
    },
    DeclareStruct {
        name: Ident,
        fields: Vec<(Ident, Type)>,
    },
}
