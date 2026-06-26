use inkwell::{types::BasicType, values::BasicValue};

use crate::{
    ast::{Ident, Type, expression::Expression},
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
            let field_index = *struct_type.indexes.get(field_name).unwrap();
            let field_ptr = self
                .builder
                .build_struct_gep(
                    struct_llvm_ty,
                    struct_ptr,
                    field_index.try_into().unwrap(),
                    "_",
                )
                .unwrap();
            let field_value = self.compile_expr(field_value).unwrap();

            self.builder
                .build_store(field_ptr, field_value.inner)
                .unwrap();
        }

        TypedValue {
            inner: struct_ptr.as_basic_value_enum(),
            ty: Type::Struct(name.as_str().to_owned()),
        }
    }
}
