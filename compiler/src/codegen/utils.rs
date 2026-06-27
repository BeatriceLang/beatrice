use inkwell::{
    basic_block::BasicBlock,
    values::{BasicMetadataValueEnum, BasicValueEnum},
};

use crate::{ast::ty::Type, codegen::Codegen};

impl<'a> Codegen<'a> {
    pub(super) fn current_block(&self) -> BasicBlock<'a> {
        self.builder.get_insert_block().unwrap()
    }
}

#[derive(Clone, Debug)]
pub struct TypedValue<'a> {
    pub inner: BasicValueEnum<'a>,
    pub ty: Type,
}

impl<'a> From<TypedValue<'a>> for BasicValueEnum<'a> {
    fn from(value: TypedValue<'a>) -> Self {
        value.inner
    }
}

impl<'a> From<TypedValue<'a>> for BasicMetadataValueEnum<'a> {
    fn from(value: TypedValue<'a>) -> Self {
        value.inner.into()
    }
}
