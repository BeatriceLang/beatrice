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
        let lhs = self.compile_expr(lhs).unwrap().value.into_int_value();
        let rhs = self.compile_expr(rhs).unwrap().value.into_int_value();

        let value = match kind {
            BinaryOpKind::Add => self.builder.build_int_add(lhs, rhs, "_").unwrap(),
            BinaryOpKind::Subtract => self.builder.build_int_sub(lhs, rhs, "_").unwrap(),
            BinaryOpKind::Divide => self.builder.build_int_signed_div(lhs, rhs, "_").unwrap(),
            BinaryOpKind::Multiply => self.builder.build_int_mul(lhs, rhs, "_").unwrap(),
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
        .into();

        TypedValue {
            value,
            ty: Type::I32,
        }
    }
}
