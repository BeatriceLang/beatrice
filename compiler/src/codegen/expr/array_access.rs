use crate::{
    ast::{expression::Expression, ty::Type},
    codegen::{Codegen, utils::TypedValue},
};

impl<'a> Codegen<'a> {
    pub(super) fn compile_array_access(
        &self,
        array: &Expression,
        index: &Expression,
    ) -> TypedValue<'a> {
        let array = self.compile_expr(array).unwrap();
        let index = self.compile_expr(index).unwrap();

        let Type::Array { element_ty, .. } = &array.ty else {
            panic!()
        };

        let element_llvm_ty = self.to_llvm_type(element_ty);

        let array_ty = self.to_llvm_type(&array.ty);
        let array_ptr = self.builder.build_alloca(array_ty, "_").unwrap();

        self.builder.build_store(array_ptr, array.inner).unwrap();

        let element_ptr = self.gep_ptr(array_ty, array_ptr, index.inner.into_int_value());

        let element = self
            .builder
            .build_load(element_llvm_ty, element_ptr, "_")
            .unwrap();

        TypedValue {
            inner: element,
            ty: *element_ty.clone(),
        }
    }
}
