use inkwell::values::{BasicValueEnum, PointerValue};

use crate::{
    ast::{Ident, Type},
    codegen::{Codegen, utils::TypedValue},
};

pub struct Local<'a> {
    pub ptr: PointerValue<'a>,
    pub ty: Type,
    pub mutable: bool,
}

impl<'a> Codegen<'a> {
    pub(super) fn compile_local(
        &self,
        name: &Ident,
        ty: Type,
        value: BasicValueEnum<'a>,
        mutable: bool,
    ) -> Local<'a> {
        let llvm_ty = self.to_llvm_type(&ty);
        let ptr = self.builder.build_alloca(llvm_ty, name.as_str()).unwrap();

        self.builder.build_store(ptr, value).unwrap();

        Local { ptr, ty, mutable }
    }

    pub(crate) fn insert_local(
        &mut self,
        name: &Ident,
        ty: Type,
        value: BasicValueEnum<'a>,
        mutable: bool,
    ) {
        let local = self.compile_local(name, ty, value, mutable);
        self.locals.insert(name.as_str().to_string(), local);
    }

    pub(super) fn resolve_local(&self, local: &Local<'a>, name: &str) -> Option<TypedValue<'a>> {
        let ty = local.ty.clone();
        let llvm_ty = self.to_llvm_type(&ty);

        let value = self.builder.build_load(llvm_ty, local.ptr, name).ok()?;
        Some(TypedValue { inner: value, ty })
    }
}
