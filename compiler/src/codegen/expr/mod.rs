use chumsky::primitive::todo;
use inkwell::values::BasicValueEnum;

use crate::{ast::expression::Expression, codegen::Codegen};

mod binary_op;
mod function_call;

impl<'a> Codegen<'a> {
    pub(super) fn compile_expr(&self, expr: &Expression) -> Option<BasicValueEnum<'a>> {
        match expr {
            Expression::Number(number) => Some(
                self.ctx
                    .i32_type()
                    .const_int(number.cast_unsigned(), false)
                    .into(),
            ),
            Expression::BinaryOp { lhs, kind, rhs } => self.compile_binary_op(lhs, *kind, rhs),
            Expression::FunctionCall { name, args } => self.compile_function_call(name, args),
            Expression::Ident(ident) => {
                let local = self.locals.get(ident.as_str()).unwrap();
                let ty = self.to_llvm_type(&local.ty);

                self.builder.build_load(ty, local.ptr, ident.as_str()).ok()
            }
            Expression::Deref { ptr } => todo!(),
            Expression::StringLiteral(string) => Some(
                self.builder
                    .build_global_string_ptr(string.as_str(), "_")
                    .unwrap()
                    .as_pointer_value()
                    .into(),
            ),
        }
    }
}
