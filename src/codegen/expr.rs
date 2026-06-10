use inkwell::values::BasicValueEnum;

use crate::{ast::expression::Expression, codegen::Codegen};

impl<'a> Codegen<'a> {
    pub(super) fn compile_expr(&self, expr: &Expression) -> BasicValueEnum<'a> {
        match expr {
            Expression::Number(number) => self.ctx.i32_type().const_int(*number as u64, false),
            _ => todo!(),
        }
        .into()
    }
}
