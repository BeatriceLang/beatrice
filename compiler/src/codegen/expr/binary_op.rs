use inkwell::IntPredicate;

use crate::{
    ast::{
        Type,
        expression::{BinaryOpKind, Expression},
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

        let value = match kind {
            BinaryOpKind::Add => self.builder.build_int_add(llvm_lhs, llvm_rhs, "_").unwrap(),
            BinaryOpKind::Subtract => self.builder.build_int_sub(llvm_lhs, llvm_rhs, "_").unwrap(),
            BinaryOpKind::Divide => self
                .builder
                .build_int_signed_div(llvm_lhs, llvm_rhs, "_")
                .unwrap(),
            BinaryOpKind::Multiply => self.builder.build_int_mul(llvm_lhs, llvm_rhs, "_").unwrap(),
            BinaryOpKind::GreaterThan => self
                .builder
                .build_int_compare(IntPredicate::SGT, llvm_lhs, llvm_rhs, "_")
                .unwrap(),
            BinaryOpKind::LessThan => self
                .builder
                .build_int_compare(IntPredicate::SLT, llvm_lhs, llvm_rhs, "_")
                .unwrap(),
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
