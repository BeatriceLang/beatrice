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
        let struct_type = self
            .struct_types
            .get(name.as_str())
            .unwrap()
            .as_basic_type_enum();
        let struct_ptr = self.builder.build_alloca(struct_type, "_").unwrap();

        // TODO: resolve the proper order from the ident
        for (i, (_, expr)) in fields.iter().enumerate() {
            let field_ptr = self
                .builder
                .build_struct_gep(struct_type, struct_ptr, i.try_into().unwrap(), "_")
                .unwrap();
            let value = self.compile_expr(expr).unwrap();

            self.builder.build_store(field_ptr, value.inner).unwrap();
        }

        TypedValue {
            inner: struct_ptr.as_basic_value_enum(),
            ty: Type::Struct(name.as_str().to_owned()),
        }
    }
}
