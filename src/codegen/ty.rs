use inkwell::{
    AddressSpace,
    types::{BasicTypeEnum, IntType},
};

use crate::{ast::Type, codegen::Codegen};

impl<'a> Codegen<'a> {
    pub(super) fn into_llvm_type(&self, ty: &Type) -> BasicTypeEnum<'a> {
        match ty {
            Type::I32 => self.ctx.i32_type().into(),
            Type::String => self.ctx.ptr_type(AddressSpace::default()).into(),
        }
    }
}
