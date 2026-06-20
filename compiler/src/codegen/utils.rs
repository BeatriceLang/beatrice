use inkwell::{
    basic_block::BasicBlock,
    values::{BasicMetadataValueEnum, BasicValueEnum},
};

use crate::{ast::Type, codegen::Codegen};

impl<'a> Codegen<'a> {
    pub(super) fn current_block(&self) -> BasicBlock<'a> {
        self.builder.get_insert_block().unwrap()
    }
}

pub struct TypedValue<'a> {
    pub value: BasicValueEnum<'a>,
    pub ty: Type,
}

impl<'a> From<TypedValue<'a>> for BasicValueEnum<'a> {
    fn from(value: TypedValue<'a>) -> Self {
        value.value
    }
}

impl<'a> From<TypedValue<'a>> for BasicMetadataValueEnum<'a> {
    fn from(value: TypedValue<'a>) -> Self {
        value.value.into()
    }
}
