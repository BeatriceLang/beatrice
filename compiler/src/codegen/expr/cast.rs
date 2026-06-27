use inkwell::values::BasicValue;

use crate::{
    ast::{expression::Expression, ty::Type},
    codegen::{Codegen, utils::TypedValue},
};

impl<'a> Codegen<'a> {
    pub(super) fn compile_cast(&self, value: &Expression, to: &Type) -> TypedValue<'a> {
        let from = self.compile_expr(value).unwrap();
        let llvm_to = self.to_llvm_type(to);

        let casted = match (&from.ty, to) {
            // bool to int
            (Type::Bool, Type::U32 | Type::I32) => self
                .builder
                .build_int_z_extend(from.inner.into_int_value(), llvm_to.into_int_type(), "_")
                .unwrap()
                .as_basic_value_enum(),
            // int to int, or bool to bool
            (Type::U32 | Type::I32 | Type::Bool, Type::U32 | Type::I32 | Type::Bool) => self
                .builder
                .build_int_cast(from.inner.into_int_value(), llvm_to.into_int_type(), "_")
                .unwrap()
                .as_basic_value_enum(),
            // ptr to int
            (Type::Ptr(_), Type::I32 | Type::U32) => self
                .builder
                .build_ptr_to_int(
                    from.inner.into_pointer_value(),
                    llvm_to.into_int_type(),
                    "_",
                )
                .unwrap()
                .as_basic_value_enum(),
            // int to ptr
            (Type::I32 | Type::U32, Type::Ptr(_)) => self
                .builder
                .build_int_to_ptr(
                    from.inner.into_int_value(),
                    llvm_to.into_pointer_type(),
                    "_",
                )
                .unwrap()
                .as_basic_value_enum(),
            _ => panic!("Unsupported cast"),
        };

        TypedValue {
            inner: casted,
            ty: to.clone(),
        }
    }
}
