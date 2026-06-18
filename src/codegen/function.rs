use inkwell::types::{BasicMetadataTypeEnum, BasicType, FunctionType};

use crate::{
    ast::{Function, Ident, Type},
    codegen::Codegen,
};

impl<'a> Codegen<'a> {
    pub(super) fn declare_function(
        &self,
        name: &str,
        params: &Vec<(Ident, Type)>,
        return_type: Option<Type>,
    ) {
        let function_type = self.function_type(params, return_type);
        self.module.add_function(name, function_type, None);
    }

    pub(super) fn compile_function(&mut self, function: &Function) {
        self.idents.clear();

        let llvm_function = self.module.get_function(function.name.as_str()).unwrap();

        let entry_block = self.ctx.append_basic_block(llvm_function, "entry");
        self.builder.position_at_end(entry_block);

        for (i, (param_name, _)) in function.params.iter().enumerate() {
            let llvm_param = llvm_function.get_nth_param(i as u32).unwrap();
            self.idents.insert(param_name.as_str().into(), llvm_param);
        }

        for statement in &function.body.statements {
            self.compile_statement(statement);
        }

        if function.return_type.is_none() && self.current_block().get_terminator().is_none() {
            self.builder.build_return(None).unwrap();
        }
    }

    fn function_type(
        &self,
        params: &Vec<(Ident, Type)>,
        return_type: Option<Type>,
    ) -> FunctionType<'a> {
        let params = self.function_params(params);

        match return_type {
            Some(ty) => self.to_llvm_type(&ty).fn_type(&params, false),
            None => self.ctx.void_type().fn_type(&params, false),
        }
    }

    fn function_params(&self, params: &Vec<(Ident, Type)>) -> Vec<BasicMetadataTypeEnum<'a>> {
        params
            .iter()
            .map(|(_, ty)| self.to_llvm_type(ty).into())
            .collect()
    }
}
