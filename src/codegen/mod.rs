use std::collections::HashMap;

use inkwell::{builder::Builder, context::Context, module::Module, values::BasicValueEnum};

use crate::ast::Program;

mod emit_obj;
mod expr;
mod function;
mod statement;
mod ty;

pub struct Codegen<'a> {
    ctx: &'a Context,
    module: Module<'a>,
    builder: Builder<'a>,
    program: Program,
    values: HashMap<String, BasicValueEnum<'a>>,
}

impl<'a> Codegen<'a> {
    pub fn new(ctx: &'a Context, module_name: &str, program: Program) -> Self {
        Self {
            ctx,
            values: HashMap::new(),
            module: ctx.create_module(module_name),
            builder: ctx.create_builder(),
            program,
        }
    }

    pub fn generate(&self) {
        for function in &self.program.functions {
            self.declare_function(function);
        }

        for function in &self.program.functions {
            self.compile_function(function);
        }
    }
}
