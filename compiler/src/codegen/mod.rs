use std::{collections::HashMap, mem::take};

use anyhow::{Context as _, Result};
use inkwell::{builder::Builder, context::Context, module::Module};

use crate::{
    ast::{Item, Program, Type},
    codegen::ident::Local,
    state::{Compiler, CompilerState},
};

mod emit_obj;
mod expr;
mod function;
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
    function_return_types: HashMap<String, Option<Type>>,
}

impl<'a> Codegen<'a> {
    pub fn new(ctx: &'a Context, module_name: &str, program: Program) -> Self {
        Self {
            ctx,
            locals: HashMap::new(),
            module: ctx.create_module(module_name),
            builder: ctx.create_builder(),
            program,
            function_return_types: HashMap::new(),
        }
    }

    pub fn generate(&mut self) {
        let items = take(&mut self.program.items);

        for item in &items {
            match item {
                Item::Function(function) => self.declare_function(
                    function.name.as_str(),
                    &function.params,
                    function.return_type.clone(),
                ),
                Item::ExternFunction(function) => self.declare_function(
                    function.name.as_str(),
                    &function.params,
                    function.return_type.clone(),
                ),
                Item::Import(_) => {}
            }
        }

        for item in &items {
            if let Item::Function(function) = item {
                self.compile_function(function);
            }
        }
        self.program.items = items;
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
