use inkwell::{
    AddressSpace,
    types::{BasicType, BasicTypeEnum},
};

use crate::{ast::ty::Type, codegen::Codegen};

impl<'a> Codegen<'a> {
    pub(super) fn to_llvm_type(&self, ty: &Type) -> BasicTypeEnum<'a> {
        match ty {
            Type::I32 | Type::U32 => self.ctx.i32_type().into(),
            Type::Named(ty_name) => self.resolve_named_type(ty_name).unwrap(),
            Type::Bool => self.ctx.bool_type().into(),
            Type::String | Type::Ptr(_) => self.ctx.ptr_type(AddressSpace::default()).into(),
            Type::Array { element_ty, size } => {
                let element_ty = self.to_llvm_type(element_ty);

                element_ty.array_type(*size).as_basic_type_enum()
            }
        }
    }

    fn resolve_named_type(&self, name: &str) -> Option<BasicTypeEnum<'a>> {
        if let Some(struct_type) = self.struct_types.get(name) {
            return Some(struct_type.inner.as_basic_type_enum());
        }

        if let Some(alias) = self.type_alias.get(name) {
            return Some(self.to_llvm_type(alias));
        }

        None
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
