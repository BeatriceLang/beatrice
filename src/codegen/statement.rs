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
            Statement::If { cond, body } => {
                let cond = self.compile_expr(cond).into_int_value();

                let current_block = self.builder.get_insert_block().unwrap();
                let current_function = current_block.get_parent().unwrap();

                let then_block = self.ctx.append_basic_block(current_function, "if_then");
                let end_block = self.ctx.append_basic_block(current_function, "if_end");

                self.builder
                    .build_conditional_branch(cond, then_block, end_block)
                    .unwrap();

                self.builder.position_at_end(then_block);

                for stmt in &body.statements {
                    self.compile_statement(stmt);
                }

                // Force jump to end_block if then_block dont have a return
                if then_block.get_terminator().is_none() {
                    self.builder.build_unconditional_branch(end_block).unwrap();
                }

                self.builder.position_at_end(end_block);
            }
        };
    }
}
