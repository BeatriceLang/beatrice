use inkwell::types::{BasicMetadataTypeEnum, BasicType, FunctionType};

use crate::{
    ast::{ident::Ident, ty::Type},
    codegen::Codegen,
};

impl<'a> Codegen<'a> {
    pub(super) fn declare_function(
        &mut self,
        name: &str,
        params: &[(Ident, Type)],
        return_type: Option<Type>,
    ) {
        let function_type = self.function_type(params, return_type.as_ref());
        self.module.add_function(name, function_type, None);
        self.function_return_types
            .insert(name.to_string(), return_type);
    }

    pub(super) fn compile_function(
        &mut self,
        name: &Ident,
        params: &[(Ident, Type)],
        return_type: &Option<Type>,
        body: &crate::ast::Block,
    ) {
        self.locals.clear();

        let llvm_function = self.module.get_function(name.as_str()).unwrap();

        let entry_block = self.ctx.append_basic_block(llvm_function, "entry");
        self.builder.position_at_end(entry_block);

        for (i, (param_name, param_ty)) in params.iter().enumerate() {
            let llvm_param = llvm_function
                .get_nth_param(u32::try_from(i).unwrap())
                .unwrap();
            self.insert_local(param_name, param_ty.clone(), llvm_param, false);
        }

        self.compile_block(body);

        if return_type.is_none() && self.current_block().get_terminator().is_none() {
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
