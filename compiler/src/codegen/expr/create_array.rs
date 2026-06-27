use inkwell::{types::BasicType, values::BasicValue};

use crate::{
    ast::{expression::Expression, ty::Type},
    codegen::{Codegen, utils::TypedValue},
};

impl<'a> Codegen<'a> {
    pub(super) fn compile_create_array(&self, array: &[Expression]) -> TypedValue<'a> {
        let array = array
            .iter()
            .map(|f| self.compile_expr(f).unwrap())
            .collect::<Vec<_>>();

        let element_ty = &array[0].ty;
        let element_llvm_ty = self.to_llvm_type(element_ty);

        let array_ty = element_llvm_ty.array_type(array.len().try_into().unwrap());

        let array_ptr = self.builder.build_alloca(array_ty, "_").unwrap();

        for (i, element) in array.iter().enumerate() {
            let element_ptr = self
                .builder
                .build_struct_gep(array_ty, array_ptr, i.try_into().unwrap(), "_")
                .unwrap();

            self.builder
                .build_store(element_ptr, element.inner)
                .unwrap();
        }

        let array_value = self.builder.build_load(array_ty, array_ptr, "_").unwrap();

        TypedValue {
            inner: array_value,
            ty: Type::Array {
                element_ty: Box::new(element_ty.clone()),
                size: array.len().try_into().unwrap(),
            },
        }
    }
}
