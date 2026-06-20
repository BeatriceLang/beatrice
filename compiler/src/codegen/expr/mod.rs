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
                value: self
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
                Some(TypedValue { value, ty })
            }
            Expression::Deref { ptr } => todo!(),
            Expression::StringLiteral(string) => Some(TypedValue {
                value: self
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
