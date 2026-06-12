use inkwell::types::IntType;

use crate::{ast::Type, codegen::Codegen};

impl<'a> Codegen<'a> {
    pub(super) fn into_llvm_type(&self, ty: &Type) -> IntType<'a> {
        match ty {
            Type::I32 => self.ctx.i32_type(),
        }
    }
}
