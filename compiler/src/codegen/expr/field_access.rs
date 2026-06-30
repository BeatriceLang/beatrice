use crate::{
    ast::{ident::Ident, ty::Type},
    codegen::{Codegen, utils::TypedValue},
};

impl<'a> Codegen<'a> {
    pub(super) fn compile_field_access(&self, base: &Ident, field: &Ident) -> TypedValue<'a> {
        let structure = self.locals.get(base.as_str()).unwrap();
        let Type::Struct(ref struct_ty_name) = structure.ty else {
            panic!()
        };

        let indexed_struct_ty = self.struct_types.get(struct_ty_name).unwrap();
        let field_info = indexed_struct_ty.info.get(field).unwrap();

        let field_ptr = self.struct_field_ptr(indexed_struct_ty, field, structure.ptr);
        let field_val = self
            .builder
            .build_load(self.to_llvm_type(&field_info.ty), field_ptr, "_")
            .unwrap();

        TypedValue {
            inner: field_val,
            ty: field_info.ty.clone(),
        }
    }
}
