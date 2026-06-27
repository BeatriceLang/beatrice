use inkwell::types::BasicType;

use crate::{
    ast::{Ident, expression::Expression, ty::Type},
    codegen::{Codegen, utils::TypedValue},
};

impl<'a> Codegen<'a> {
    pub(super) fn compile_create_struct(
        &self,
        name: &Ident,
        fields: &[(Ident, Box<Expression>)],
    ) -> TypedValue<'a> {
        let struct_type = self.struct_types.get(name.as_str()).unwrap();

        let struct_llvm_ty = struct_type.inner.as_basic_type_enum();
        let struct_ptr = self.builder.build_alloca(struct_llvm_ty, "_").unwrap();

        for (field_name, field_value) in fields {
            let field_ptr = self.struct_field_ptr(struct_type, field_name, struct_ptr);
            let field_value = self.compile_expr(field_value).unwrap();

            self.builder
                .build_store(field_ptr, field_value.inner)
                .unwrap();
        }

        let struct_value = self
            .builder
            .build_load(struct_llvm_ty, struct_ptr, "_")
            .unwrap();

        TypedValue {
            inner: struct_value,
            ty: Type::Struct(name.as_str().to_owned()),
        }
    }
}
