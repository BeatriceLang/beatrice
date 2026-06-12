use inkwell::values::BasicValueEnum;

use crate::{
    ast::expression::{Expression, MathOpKind},
    codegen::Codegen,
};

impl<'a> Codegen<'a> {
    pub(super) fn compile_expr(&self, expr: &Expression) -> BasicValueEnum<'a> {
        match expr {
            Expression::Number(number) => {
                self.ctx.i32_type().const_int(*number as u64, false).into()
            }
            Expression::MathOp { lhs, kind, rhs } => {
                let lhs = self.compile_expr(lhs).into_int_value();
                let rhs = self.compile_expr(rhs).into_int_value();

                match kind {
                    MathOpKind::Add => self.builder.build_int_add(lhs, rhs, "_").unwrap(),
                    MathOpKind::Subtract => self.builder.build_int_sub(lhs, rhs, "_").unwrap(),
                    MathOpKind::Divide => self.builder.build_int_signed_div(lhs, rhs, "_").unwrap(),
                    MathOpKind::Multiply => self.builder.build_int_mul(lhs, rhs, "_").unwrap(),
                }
                .into()
            }
            Expression::FunctionCall { name, args } => {
                let function = self.module.get_function(name).unwrap();
                let args: Vec<_> = args.iter().map(|f| self.compile_expr(f).into()).collect();

                self.builder
                    .build_call(function, &args, "_")
                    .unwrap()
                    .try_as_basic_value()
                    .unwrap_basic()
            }
            _ => todo!(),
        }
    }
}
