use crate::{ast::DeclareStruct, codegen::Codegen};

impl Codegen<'_> {
    pub(super) fn declare_struct(&mut self, declare_struct: &DeclareStruct) {
        let struct_name = declare_struct.name.as_str();
        let struct_type = self.ctx.opaque_struct_type(struct_name);
        self.struct_types.insert(struct_name.into(), struct_type);
    }

    pub(super) fn define_struct(&mut self, declare_struct: &DeclareStruct) {
        let struct_name = declare_struct.name.as_str();
        let field_types: Vec<_> = declare_struct
            .fields
            .iter()
            .map(|(_, ty)| self.to_llvm_type(ty))
            .collect();
        let llvm_struct = self.struct_types.get_mut(struct_name).unwrap();

        llvm_struct.set_body(&field_types, false);
    }
}
