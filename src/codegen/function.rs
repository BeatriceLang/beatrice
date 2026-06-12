use chumsky::container::Seq;
use inkwell::{
    types::{BasicMetadataTypeEnum, FunctionType},
    values::{BasicValueEnum, FunctionValue},
};

use crate::{ast::Function, codegen::Codegen};

impl<'a> Codegen<'a> {
    pub(super) fn declare_function(&self, function: &Function) {
        let function_type = self.function_type(function);
        self.module
            .add_function(&function.name, function_type, None);
    }

    pub(super) fn compile_function(&self, function: &Function) {
        let llvm_function = self.module.get_function(&function.name).unwrap();

        let entry_block = self.ctx.append_basic_block(llvm_function, "entry");
        self.builder.position_at_end(entry_block);

        for statement in &function.body.statements {
            self.compile_statement(statement);
        }
    }

    fn function_type(&self, function: &Function) -> FunctionType<'a> {
        let params = self.function_params(function);
        let return_type = self.into_llvm_type(&function.return_type);

        return_type.fn_type(&params, false)
    }

    fn function_params(&self, function: &Function) -> Vec<BasicMetadataTypeEnum<'a>> {
        function
            .params
            .iter()
            .map(|(_, ty)| self.into_llvm_type(ty).into())
            .collect()
    }
}
