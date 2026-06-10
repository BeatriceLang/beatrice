use inkwell::values::BasicValue;

use crate::{ast::statement::Statement, codegen::Codegen};

impl<'a> Codegen<'a> {
    pub(super) fn compile_statement(&self, statement: &Statement) {
        match statement {
            Statement::Return(expr) => {
                let value = self.compile_expr(expr);

                _ = self
                    .builder
                    .build_return(Some(&value as &dyn BasicValue))
                    .unwrap()
            }
            Statement::Expression(expr) => {
                _ = self.compile_expr(expr);
            }
        };
    }
}
