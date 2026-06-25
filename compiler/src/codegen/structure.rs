use crate::{ast::Struct, codegen::Codegen};

impl Codegen<'_> {
    pub(super) fn declare_struct(&mut self, structure: &Struct) {
        let struct_name = structure.name.as_str();
        let structure = self.ctx.opaque_struct_type(struct_name);
        self.structs.push(structure);
    }
}
