use crate::{
    ast::{Ident, expression::Expression},
    codegen::{Codegen, utils::TypedValue},
};

impl<'a> Codegen<'a> {
    pub(super) fn compile_struct(
        &self,
        name: &Ident,
        fields: &[(Ident, Box<Expression>)],
    ) -> TypedValue<'a> {
        todo!()
    }
}
