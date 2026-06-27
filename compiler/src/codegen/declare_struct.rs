use std::collections::HashMap;

use inkwell::{types::StructType, values::PointerValue};

use crate::{
    ast::{DeclareStruct, Ident, ty::Type},
    codegen::Codegen,
};

pub(super) struct ResolvedStruct<'a> {
    pub inner: StructType<'a>,
    pub info: HashMap<Ident, FieldInfo>,
}

pub(super) struct FieldInfo {
    pub index: usize,
    pub ty: Type,
}

impl<'a> Codegen<'a> {
    pub(super) fn struct_field_ptr(
        &self,
        struct_type: &ResolvedStruct<'a>,
        field_name: &Ident,
        struct_ptr: PointerValue<'a>,
    ) -> PointerValue<'a> {
        let struct_llvm_ty = struct_type.inner;
        let field_index = struct_type.info.get(field_name).unwrap().index;

        self.builder
            .build_struct_gep(
                struct_llvm_ty,
                struct_ptr,
                field_index.try_into().unwrap(),
                "_",
            )
            .unwrap()
    }

    pub(super) fn declare_struct(&mut self, declare_struct: &DeclareStruct) {
        let struct_name = declare_struct.name.as_str();
        let llvm_struct_ty = self.ctx.opaque_struct_type(struct_name);

        let mut field_infos = HashMap::new();

        for (index, (field_name, field_ty)) in declare_struct.fields.iter().enumerate() {
            let info = FieldInfo {
                index,
                ty: field_ty.clone(),
            };

            field_infos.insert(field_name.clone(), info);
        }

        let resolved_struct = ResolvedStruct {
            inner: llvm_struct_ty,
            info: field_infos,
        };

        self.struct_types
            .insert(struct_name.into(), resolved_struct);
    }

    pub(super) fn define_struct(&mut self, declare_struct: &DeclareStruct) {
        let struct_name = declare_struct.name.as_str();
        let field_types: Vec<_> = declare_struct
            .fields
            .iter()
            .map(|(_, ty)| self.to_llvm_type(ty))
            .collect();
        let llvm_struct = self.struct_types.get_mut(struct_name).unwrap().inner;

        llvm_struct.set_body(&field_types, false);
    }
}
