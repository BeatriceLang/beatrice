use inkwell::values::BasicValue;

use crate::{
    ast::{expression::Expression, ty::Type},
    codegen::{Codegen, utils::TypedValue},
};

mod binary_op;
mod cast;
mod create_struct;
mod deref;
mod field_access;
mod function_call;
mod intrinsic;

impl<'a> Codegen<'a> {
    pub(super) fn compile_expr(&self, expr: &Expression) -> Option<TypedValue<'a>> {
        match expr {
            Expression::Invert(value) => {
                let value = self.compile_expr(value).unwrap();
                let true_val = self.ctx.bool_type().const_int(1, false);

                let inverted = self
                    .builder
                    .build_xor(value.inner.into_int_value(), true_val, "_")
                    .unwrap();

                Some(TypedValue {
                    inner: inverted.as_basic_value_enum(),
                    ty: Type::Bool,
                })
            }
            Expression::Cast { value, to } => Some(self.compile_cast(value, to)),
            Expression::Number(number) => Some(TypedValue {
                inner: self
                    .ctx
                    .i32_type()
                    .const_int(number.cast_unsigned(), false)
                    .into(),
                ty: Type::I32,
            }),
            Expression::Bool(value) => Some(TypedValue {
                inner: self
                    .ctx
                    .bool_type()
                    .const_int(u64::from(*value), false)
                    .into(),
                ty: Type::Bool,
            }),
            Expression::TypedNumber { value, ty } => {
                let llvm_ty = self.to_llvm_type(ty).into_int_type();

                let value = llvm_ty.const_int(u64::try_from(*value).unwrap(), false);

                Some(TypedValue {
                    inner: value.into(),
                    ty: ty.clone(),
                })
            }
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
            Expression::CreateStruct { name, fields } => {
                Some(self.compile_create_struct(name, fields))
            }
            Expression::FieldAccess { base, field } => Some(self.compile_field_access(base, field)),
        }
    }
}
