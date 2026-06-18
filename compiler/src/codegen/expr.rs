use inkwell::{IntPredicate, values::BasicValueEnum};

use crate::{
    ast::expression::{BinaryOpKind, Expression},
    codegen::Codegen,
};

impl<'a> Codegen<'a> {
    pub(super) fn compile_expr(&self, expr: &Expression) -> Option<BasicValueEnum<'a>> {
        match expr {
            Expression::Number(number) => Some(
                self.ctx
                    .i32_type()
                    .const_int(number.cast_unsigned(), false)
                    .into(),
            ),
            Expression::BinaryOp { lhs, kind, rhs } => {
                let lhs = self.compile_expr(lhs).unwrap().into_int_value();
                let rhs = self.compile_expr(rhs).unwrap().into_int_value();

                Some(
                    match kind {
                        BinaryOpKind::Add => self.builder.build_int_add(lhs, rhs, "_").unwrap(),
                        BinaryOpKind::Subtract => {
                            self.builder.build_int_sub(lhs, rhs, "_").unwrap()
                        }
                        BinaryOpKind::Divide => {
                            self.builder.build_int_signed_div(lhs, rhs, "_").unwrap()
                        }
                        BinaryOpKind::Multiply => {
                            self.builder.build_int_mul(lhs, rhs, "_").unwrap()
                        }
                        BinaryOpKind::GreaterThan => self
                            .builder
                            .build_int_compare(IntPredicate::SGT, lhs, rhs, "_")
                            .unwrap(),
                        BinaryOpKind::LessThan => self
                            .builder
                            .build_int_compare(IntPredicate::SLT, lhs, rhs, "_")
                            .unwrap(),
                        BinaryOpKind::EqualTo => self
                            .builder
                            .build_int_compare(IntPredicate::EQ, lhs, rhs, "_")
                            .unwrap(),
                    }
                    .into(),
                )
            }
            Expression::FunctionCall { name, args } => {
                let function = self.module.get_function(name.as_str()).unwrap();
                let args: Vec<_> = args
                    .iter()
                    .map(|arg| self.compile_expr(arg).unwrap().into())
                    .collect();

                self.builder
                    .build_call(function, &args, "_")
                    .unwrap()
                    .try_as_basic_value()
                    .basic()
            }
            Expression::Ident(ident) => Some(*self.idents.get(ident.as_str()).unwrap()),
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
