use inkwell::values::BasicValue;

use crate::{
    ast::{Block, statement::Statement},
    codegen::Codegen,
};

mod r#if;
mod r#while;

impl Codegen<'_> {
    pub(super) fn compile_block(&mut self, block: &Block) {
        for statement in &block.statements {
            if self.current_block().get_terminator().is_some() {
                break;
            }

            self.compile_statement(statement);
        }
    }

    pub(super) fn compile_statement(&mut self, statement: &Statement) {
        match statement {
            Statement::Return(expr) => match expr {
                Some(expr) => {
                    let value = self.compile_expr(expr).unwrap();

                    _ = self
                        .builder
                        .build_return(Some(&value.inner as &dyn BasicValue))
                        .unwrap();
                }
                None => _ = self.builder.build_return(None).unwrap(),
            },
            Statement::Expression(expr) => {
                _ = self.compile_expr(expr);
            }
            Statement::If { cond, body } => self.compile_if(cond, body),
            Statement::Let { name, ty, value } => {
                let value = self.compile_expr(value).unwrap();
                self.insert_local(name, ty.clone(), value.inner, false);
            }
            Statement::Var { name, ty, value } => {
                let value = self.compile_expr(value).unwrap();
                self.insert_local(name, ty.clone(), value.inner, true);
            }
            Statement::Assign { ident, value } => {
                let local = self.locals.get(ident.as_str()).unwrap();
                let value = self.compile_expr(value).unwrap();

                assert!(local.mutable);

                self.builder.build_store(local.ptr, value.inner).unwrap();
            }
            Statement::While { cond, body } => self.compile_while(cond, body),
        }
    }
}
