use crate::{ast::Struct, codegen::Codegen};

impl Codegen<'_> {
    pub(super) fn declare_struct(&mut self, structure: &Struct) {
        let struct_name = structure.name.as_str();
        let structure = self.ctx.opaque_struct_type(struct_name);
        self.structs.insert(struct_name.into(), structure);
    }

    pub(super) fn define_struct(&mut self, structure: &Struct) {
        let struct_name = structure.name.as_str();
        let field_types: Vec<_> = structure
            .fields
            .iter()
            .map(|(_, ty)| self.to_llvm_type(ty))
            .collect();
        let llvm_struct = self.structs.get_mut(struct_name).unwrap();

        llvm_struct.set_body(&field_types, false);
    }
}
