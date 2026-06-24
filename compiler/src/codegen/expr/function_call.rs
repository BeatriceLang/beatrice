use inkwell::values::BasicMetadataValueEnum;

use crate::{
    ast::{Ident, expression::Expression},
    codegen::{Codegen, utils::TypedValue},
};

impl<'a> Codegen<'a> {
    pub(super) fn compile_function_call(
        &self,
        name: &Ident,
        args: &[Expression],
    ) -> Option<TypedValue<'a>> {
        let function = self.module.get_function(name.as_str()).unwrap();
        let return_type = self
            .function_return_types
            .get(name.as_str())
            .unwrap()
            .clone();
        let args: Vec<BasicMetadataValueEnum<'a>> = args
            .iter()
            .map(|arg| self.compile_expr(arg).unwrap().into())
            .collect();

        let call = self.builder.build_call(function, &args, "_").unwrap();
        let return_type = return_type?;
        let value = call.try_as_basic_value().basic()?;

        Some(TypedValue {
            inner: value,
            ty: return_type,
        })
    }
}
