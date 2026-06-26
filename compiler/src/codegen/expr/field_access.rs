use crate::{
    ast::{Ident, Type},
    codegen::{Codegen, utils::TypedValue},
};

impl<'a> Codegen<'a> {
    pub(super) fn compile_field_access(&mut self, base: &Ident, field: &Ident) -> TypedValue<'a> {
        let structure = self.resolve_ident(base).unwrap();
        let Type::Struct(ref struct_ty_name) = structure.ty else {
            panic!()
        };

        let indexed_struct_ty = self.struct_types.get(struct_ty_name).unwrap();

        let field_ptr = self.struct_field_ptr(
            indexed_struct_ty,
            field,
            structure.inner.into_pointer_value(),
        );
        let field_val = self
            .builder
            .build_load(indexed_struct_ty.inner, field_ptr, "_")
            .unwrap();

        TypedValue {
            inner: field_val,
            ty: Type::I32, // TODO: add proper types
        }
    }
}
