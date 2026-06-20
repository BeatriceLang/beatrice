use crate::{
    ast::{Type, expression::Expression},
    codegen::{Codegen, utils::TypedValue},
};

impl<'a> Codegen<'a> {
    pub(super) fn compile_deref(&self, ptr: &Expression) -> Option<TypedValue<'a>> {
        let ptr = self.compile_expr(ptr).unwrap();
        let Type::Ptr(pointed_ty) = ptr.ty else {
            panic!("Expected pointer");
        };

        let llvm_pointed_ty = self.to_llvm_type(&pointed_ty);

        let value = self
            .builder
            .build_load(llvm_pointed_ty, ptr.inner.into_pointer_value(), "_")
            .unwrap();

        Some(TypedValue {
            inner: value,
            ty: *pointed_ty,
        })
    }
}
