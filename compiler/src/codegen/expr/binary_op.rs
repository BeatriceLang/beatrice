use inkwell::{IntPredicate, values::BasicValue};

use crate::{
    ast::{
        expression::{BinaryOpKind, Expression},
        ty::Type,
    },
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

        let (value, ty) = match kind {
            BinaryOpKind::Add => (
                self.builder.build_int_add(llvm_lhs, llvm_rhs, "_").unwrap(),
                lhs.ty,
            ),
            BinaryOpKind::Subtract => (
                self.builder.build_int_sub(llvm_lhs, llvm_rhs, "_").unwrap(),
                lhs.ty,
            ),
            BinaryOpKind::Divide => (
                if signed {
                    self.builder
                        .build_int_signed_div(llvm_lhs, llvm_rhs, "_")
                        .unwrap()
                } else {
                    self.builder
                        .build_int_unsigned_div(llvm_lhs, llvm_rhs, "_")
                        .unwrap()
                },
                lhs.ty,
            ),
            BinaryOpKind::Multiply => (
                self.builder.build_int_mul(llvm_lhs, llvm_rhs, "_").unwrap(),
                lhs.ty,
            ),
            BinaryOpKind::GreaterThan => {
                let predicate = if signed {
                    IntPredicate::SGT
                } else {
                    IntPredicate::UGT
                };

                (
                    self.builder
                        .build_int_compare(predicate, llvm_lhs, llvm_rhs, "_")
                        .unwrap(),
                    Type::Bool,
                )
            }
            BinaryOpKind::LessThan => {
                let predicate = if signed {
                    IntPredicate::SLT
                } else {
                    IntPredicate::ULT
                };

                (
                    self.builder
                        .build_int_compare(predicate, llvm_lhs, llvm_rhs, "_")
                        .unwrap(),
                    Type::Bool,
                )
            }
            BinaryOpKind::EqualTo => (
                self.builder
                    .build_int_compare(IntPredicate::EQ, llvm_lhs, llvm_rhs, "_")
                    .unwrap(),
                Type::Bool,
            ),
        };

        TypedValue {
            inner: value.as_basic_value_enum(),
            ty,
        }
    }
}
