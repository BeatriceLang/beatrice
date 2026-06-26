use inkwell::IntPredicate;

use crate::{
    ast::expression::{BinaryOpKind, Expression},
    codegen::{Codegen, utils::TypedValue},
};

impl<'a> Codegen<'a> {
    pub(super) fn compile_binary_op(
        &self,
        lhs: &Expression,
        kind: BinaryOpKind,
        rhs: &Expression,
    ) -> TypedValue<'a> {
        let lhs = self.compile_expr(lhs).unwrap();
        let rhs = self.compile_expr(rhs).unwrap();
        let llvm_lhs = lhs.inner.into_int_value();
        let llvm_rhs = rhs.inner.into_int_value();

        assert_eq!(lhs.ty, rhs.ty);

        let signed = lhs.ty.signed().unwrap();

        let value = match kind {
            BinaryOpKind::Add => self.builder.build_int_add(llvm_lhs, llvm_rhs, "_").unwrap(),
            BinaryOpKind::Subtract => self.builder.build_int_sub(llvm_lhs, llvm_rhs, "_").unwrap(),
            BinaryOpKind::Divide => {
                if signed {
                    self.builder
                        .build_int_signed_div(llvm_lhs, llvm_rhs, "_")
                        .unwrap()
                } else {
                    self.builder
                        .build_int_unsigned_div(llvm_lhs, llvm_rhs, "_")
                        .unwrap()
                }
            }
            BinaryOpKind::Multiply => self.builder.build_int_mul(llvm_lhs, llvm_rhs, "_").unwrap(),
            BinaryOpKind::GreaterThan => {
                let predicate = if signed {
                    IntPredicate::SGT
                } else {
                    IntPredicate::UGT
                };

                self.builder
                    .build_int_compare(predicate, llvm_lhs, llvm_rhs, "_")
                    .unwrap()
            }
            BinaryOpKind::LessThan => {
                let predicate = if signed {
                    IntPredicate::SLT
                } else {
                    IntPredicate::ULT
                };

                self.builder
                    .build_int_compare(predicate, llvm_lhs, llvm_rhs, "_")
                    .unwrap()
            }
            BinaryOpKind::EqualTo => self
                .builder
                .build_int_compare(IntPredicate::EQ, llvm_lhs, llvm_rhs, "_")
                .unwrap(),
        }
        .into();

        TypedValue {
            inner: value,
            ty: lhs.ty,
        }
    }
}
