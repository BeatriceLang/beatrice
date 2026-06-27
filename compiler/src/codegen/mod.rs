use std::collections::HashMap;

use anyhow::{Context as _, Result};
use inkwell::{builder::Builder, context::Context, module::Module};

use crate::{
    ast::{Program, ty::Type},
    codegen::{declare_struct::ResolvedStruct, ident::local::Local, utils::TypedValue},
    state::{Compiler, CompilerState},
};

mod declare_struct;
mod emit_obj;
mod expr;
mod function;
mod generate;
mod ident;
mod statement;
mod ty;
mod utils;

pub struct Codegen<'a> {
    ctx: &'a Context,
    module: Module<'a>,
    builder: Builder<'a>,
    program: Program,
    locals: HashMap<String, Local<'a>>,
    constants: HashMap<String, TypedValue<'a>>,
    struct_types: HashMap<String, ResolvedStruct<'a>>,
    function_return_types: HashMap<String, Option<Type>>,
}

impl<'a> Codegen<'a> {
    pub fn new(ctx: &'a Context, module_name: &str, program: Program) -> Self {
        Self {
            ctx,
            locals: HashMap::new(),
            module: ctx.create_module(module_name),
            builder: ctx.create_builder(),
            struct_types: HashMap::new(),
            constants: HashMap::new(),
            program,
            function_return_types: HashMap::new(),
        }
    }
}

impl Compiler {
    pub fn codegen(&self) -> Result<()> {
        let CompilerState::Codegen(program) = &self.state else {
            panic!("Unexpected compiler state")
        };

        let context = Context::create();
        let mut codegen = Codegen::new(
            &context,
            self.output_path
                .file_name()
                .context("Failed to parse output file name")?
                .to_str()
                .context("Failed to parse output file name")?,
            program.clone(),
        );

        codegen.generate();
        codegen
            .emit_object(&self.output_path)
            .context("Failed to emit object")?;

        Ok(())
    }
}
