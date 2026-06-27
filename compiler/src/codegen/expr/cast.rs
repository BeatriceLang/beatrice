use inkwell::values::BasicValue;

use crate::{
    ast::{Type, expression::Expression},
    codegen::{Codegen, utils::TypedValue},
};

impl<'a> Codegen<'a> {
    pub(super) fn compile_cast(&self, value: &Expression, to: &Type) -> TypedValue<'a> {
        let value = self.compile_expr(value).unwrap();
        let llvm_to = self.to_llvm_type(to);

        let casted = match to {
            Type::U32 | Type::I32 => self
                .builder
                .build_int_cast(value.inner.into_int_value(), llvm_to.into_int_type(), "_")
                .unwrap()
                .as_basic_value_enum(),
            Type::Bool => self
                .builder
                .build_bit_cast(value.inner.into_int_value(), llvm_to.into_int_type(), "_")
                .unwrap()
                .as_basic_value_enum(),
            Type::String | Type::Ptr(_) => self
                .builder
                .build_pointer_cast(
                    value.inner.into_pointer_value(),
                    llvm_to.into_pointer_type(),
                    "_",
                )
                .unwrap()
                .as_basic_value_enum(),
            Type::Struct(_) => self
                .builder
                .build_bit_cast(value.inner, llvm_to, "_")
                .unwrap()
                .as_basic_value_enum(),
        };

        TypedValue {
            inner: casted,
            ty: to.clone(),
        }
    }
}
