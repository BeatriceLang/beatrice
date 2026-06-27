use inkwell::values::BasicValue;

use crate::{
    ast::{expression::Expression, ty::Type},
    codegen::{Codegen, utils::TypedValue},
};

impl<'a> Codegen<'a> {
    pub(super) fn compile_is_nullptr(&self, args: &[Expression]) -> TypedValue<'a> {
        let value = self
            .compile_expr(&args[0])
            .unwrap()
            .inner
            .into_pointer_value();

        let is_null_val = self.builder.build_is_null(value, "_").unwrap();

        TypedValue {
            inner: is_null_val.as_basic_value_enum(),
            ty: Type::Bool,
        }
    }
}
