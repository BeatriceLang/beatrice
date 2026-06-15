use std::{collections::HashMap, mem::take};

use anyhow::{Context as _, Result};
use inkwell::{builder::Builder, context::Context, module::Module, values::BasicValueEnum};

use crate::{
    ast::{Item, Program},
    state::{Compiler, CompilerState},
};

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
    idents: HashMap<String, BasicValueEnum<'a>>,
}

impl<'a> Codegen<'a> {
    pub fn new(ctx: &'a Context, module_name: &str, program: Program) -> Self {
        Self {
            ctx,
            idents: HashMap::new(),
            module: ctx.create_module(module_name),
            builder: ctx.create_builder(),
            program,
        }
    }

    pub fn generate(&mut self) {
        for item in &self.program.items {
            if let Item::Function(function) = item {
                self.declare_function(function);
            }
        }

        let items = take(&mut self.program.items);
        for item in &items {
            if let Item::Function(function) = item {
                self.compile_function(function);
            }
        }
        self.program.items = items;
    }
}

impl Compiler {
    pub fn codegen(&mut self) -> Result<()> {
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
