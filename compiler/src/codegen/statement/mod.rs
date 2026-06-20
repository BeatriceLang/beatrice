use inkwell::values::BasicValue;

use crate::{ast::statement::Statement, codegen::Codegen};

mod r#if;
mod r#while;

impl Codegen<'_> {
    pub(super) fn compile_statement(&mut self, statement: &Statement) {
        match statement {
            Statement::Return(expr) => {
                let value = self.compile_expr(expr).unwrap();

                _ = self
                    .builder
                    .build_return(Some(&value.inner as &dyn BasicValue))
                    .unwrap();
            }
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
