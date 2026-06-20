use std::ops::Deref;

use crate::{
    ast::{Type, expression::Expression},
    codegen::{Codegen, utils::TypedValue},
};

mod binary_op;
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
            Expression::Ident(ident) => {
                let local = self.locals.get(ident.as_str()).unwrap();
                let ty = local.ty.clone();
                let llvm_ty = self.to_llvm_type(&ty);

                let value = self
                    .builder
                    .build_load(llvm_ty, local.ptr, ident.as_str())
                    .ok()?;
                Some(TypedValue { inner: value, ty })
            }
            Expression::Deref { ptr } => {
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
            Expression::StringLiteral(string) => Some(TypedValue {
                inner: self
                    .builder
                    .build_global_string_ptr(string.as_str(), "_")
                    .unwrap()
                    .as_pointer_value()
                    .into(),
                ty: Type::String,
            }),
        }
    }
}
