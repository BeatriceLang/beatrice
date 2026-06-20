use inkwell::types::{BasicMetadataTypeEnum, BasicType, FunctionType};

use crate::{
    ast::{Function, Ident, Type},
    codegen::Codegen,
};

impl<'a> Codegen<'a> {
    pub(super) fn declare_function(
        &self,
        name: &str,
        params: &[(Ident, Type)],
        return_type: Option<Type>,
    ) {
        let function_type = self.function_type(params, return_type.as_ref());
        self.module.add_function(name, function_type, None);
    }

    pub(super) fn compile_function(&mut self, function: &Function) {
        self.locals.clear();

        let llvm_function = self.module.get_function(function.name.as_str()).unwrap();

        let entry_block = self.ctx.append_basic_block(llvm_function, "entry");
        self.builder.position_at_end(entry_block);

        for (i, (param_name, param_ty)) in function.params.iter().enumerate() {
            let llvm_param = llvm_function
                .get_nth_param(u32::try_from(i).unwrap())
                .unwrap();
            self.insert_local(param_name, param_ty.clone(), llvm_param, false);
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
        params: &[(Ident, Type)],
        return_type: Option<&Type>,
    ) -> FunctionType<'a> {
        let params = self.function_params(params);

        return_type.map_or_else(
            || self.ctx.void_type().fn_type(&params, false),
            |ty| self.to_llvm_type(ty).fn_type(&params, false),
        )
    }

    fn function_params(&self, params: &[(Ident, Type)]) -> Vec<BasicMetadataTypeEnum<'a>> {
        params
            .iter()
            .map(|(_, ty)| self.to_llvm_type(ty).into())
            .collect()
    }
}
