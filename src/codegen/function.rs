use chumsky::container::Seq;
use inkwell::{types::FunctionType, values::FunctionValue};

use crate::{ast::Function, codegen::Codegen};

impl<'a> Codegen<'a> {
    pub(super) fn declare_function(&self, function: &Function) {
        self.module
            .add_function(&function.name, self.stub_function_args(), None);
    }

    pub(super) fn compile_function(&self, function: &Function) {
        let llvm_function = self.module.get_function(&function.name).unwrap();

        let entry_block = self.ctx.append_basic_block(llvm_function, "entry");
        self.builder.position_at_end(entry_block);

        for statement in &function.body.statements {
            self.compile_statement(statement);
        }
    }

    fn stub_function_args(&self) -> FunctionType<'a> {
        self.ctx.i32_type().fn_type(&[], false)
    }
}
