use inkwell::values::BasicValueEnum;

use crate::{
    ast::{Ident, expression::Expression},
    codegen::Codegen,
};

impl<'a> Codegen<'a> {
    pub(super) fn compile_function_call(
        &self,
        name: &Ident,
        args: &[Expression],
    ) -> Option<BasicValueEnum<'a>> {
        let function = self.module.get_function(name.as_str()).unwrap();
        let args: Vec<_> = args
            .iter()
            .map(|arg| self.compile_expr(arg).unwrap().into())
            .collect();

        self.builder
            .build_call(function, &args, "_")
            .unwrap()
            .try_as_basic_value()
            .basic()
    }
}
