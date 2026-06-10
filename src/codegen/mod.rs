use inkwell::{builder::Builder, context::Context, module::Module};

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
}

impl<'a> Codegen<'a> {
    pub fn new(ctx: &'a Context, module_name: &str, program: Program) -> Self {
        Self {
            ctx,
            module: ctx.create_module(module_name),
            builder: ctx.create_builder(),
            program,
        }
    }

    pub fn generate(&self) {
        for function in &self.program.functions {
            self.compile_function(function);
        }
    }
}
