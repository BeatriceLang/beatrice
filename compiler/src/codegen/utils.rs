use inkwell::{
    basic_block::BasicBlock,
    types::BasicTypeEnum,
    values::{BasicMetadataValueEnum, BasicValueEnum, IntValue, PointerValue},
};

use crate::{ast::ty::Type, codegen::Codegen};

impl<'a> Codegen<'a> {
    pub(super) fn current_block(&self) -> BasicBlock<'a> {
        self.builder.get_insert_block().unwrap()
    }

    pub(super) fn gep_ptr(
        &self,
        ty: BasicTypeEnum<'a>,
        ptr: PointerValue<'a>,
        index: IntValue<'a>,
    ) -> PointerValue<'a> {
        let zero = self.ctx.i32_type().const_zero();

        unsafe {
            self.builder
                .build_gep(ty, ptr, &[zero, index], "_")
                .unwrap()
        }
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
