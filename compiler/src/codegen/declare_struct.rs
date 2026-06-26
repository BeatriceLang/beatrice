use std::collections::HashMap;

use inkwell::types::StructType;

use crate::{
    ast::{DeclareStruct, Ident},
    codegen::Codegen,
};

pub(super) struct IndexedStruct<'a> {
    pub inner: StructType<'a>,
    pub indexes: HashMap<Ident, usize>,
}

impl Codegen<'_> {
    pub(super) fn declare_struct(&mut self, declare_struct: &DeclareStruct) {
        let struct_name = declare_struct.name.as_str();
        let llvm_struct_ty = self.ctx.opaque_struct_type(struct_name);

        let mut indexes = HashMap::new();

        for (i, (field_name, _)) in declare_struct.fields.iter().enumerate() {
            indexes.insert(field_name.clone(), i);
        }

        let indexed_struct = IndexedStruct {
            inner: llvm_struct_ty,
            indexes,
        };

        self.struct_types.insert(struct_name.into(), indexed_struct);
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
