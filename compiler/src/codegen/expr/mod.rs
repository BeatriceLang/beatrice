use inkwell::values::BasicValue;

use crate::{
    ast::{Type, expression::Expression},
    codegen::{Codegen, utils::TypedValue},
};

mod binary_op;
mod deref;
mod function_call;

impl<'a> Codegen<'a> {
    pub(super) fn compile_expr(&self, expr: &Expression) -> Option<TypedValue<'a>> {
        match expr {
            Expression::Number(number) => Some(TypedValue {
                inner: self
                    .ctx
                    .i32_type()
                    .const_int(number.cast_unsigned(), false)
                    .into(),
                ty: Type::I32,
            }),
            Expression::BinaryOp { lhs, kind, rhs } => {
                Some(self.compile_binary_op(lhs, *kind, rhs))
            }
            Expression::FunctionCall { name, args } => self.compile_function_call(name, args),
            Expression::Ident(ident) => self.resolve_ident(ident),
            Expression::Deref { ptr } => Some(self.compile_deref(ptr)),
            Expression::StringLiteral(string) => Some(TypedValue {
                inner: self
                    .builder
                    .build_global_string_ptr(string.as_str(), "_")
                    .unwrap()
                    .as_pointer_value()
                    .into(),
                ty: Type::String,
            }),
            Expression::AddressOf { value } => {
                let Expression::Ident(ident) = &**value else {
                    panic!("Expected ident")
                };

                let local = self.locals.get(ident.as_str()).unwrap();

                Some(TypedValue {
                    inner: local.ptr.as_basic_value_enum(),
                    ty: Type::Ptr(Box::new(local.ty.clone())),
                })
            }
        }
    }
}
