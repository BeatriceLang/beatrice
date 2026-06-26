use inkwell::{
    AddressSpace,
    types::{BasicType, BasicTypeEnum},
};

use crate::{ast::Type, codegen::Codegen};

impl<'a> Codegen<'a> {
    pub(super) fn to_llvm_type(&self, ty: &Type) -> BasicTypeEnum<'a> {
        match ty {
            Type::I32 => self.ctx.i32_type().into(),
            Type::U32 => todo!(),
            Type::Struct(struct_name) => self
                .struct_types
                .get(struct_name)
                .unwrap()
                .inner
                .as_basic_type_enum(),
            Type::String | Type::Ptr(_) => self.ctx.ptr_type(AddressSpace::default()).into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use inkwell::context::Context;

    use crate::{ast::Program, codegen::Codegen};

    use super::Type;

    #[test]
    fn lowers_string_type_to_pointer_type() {
        let context = Context::create();
        let codegen = Codegen::new(&context, "test", Program { items: vec![] });

        assert!(codegen.to_llvm_type(&Type::String).is_pointer_type());
    }
}
