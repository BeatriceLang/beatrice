use std::path::PathBuf;

use anyhow::{Context, Ok, Result};

use crate::{
    ast::{Item, Program, function::ExternFunction},
    state::{Compiler, CompilerState},
};

impl Compiler {
    pub fn import(&mut self) -> Result<()> {
        let program = {
            let CompilerState::Import(program) = &mut self.state else {
                panic!("Unexpected compiler state")
            };

            let import_paths = imports_of(program);

            for path in import_paths {
                process_import(program, path)?;
            }

            program.clone()
        };

        self.advance_to(CompilerState::Check(program))
    }
}

fn process_import(original_program: &mut Program, path: PathBuf) -> Result<()> {
    let mut compiler = Compiler::new(path, PathBuf::new())
        .context("Failed to create compiler when processing imports")?;

    compiler.lex()?;
    compiler.parse()?;

    let CompilerState::Import(imported_program) = &compiler.state else {
        panic!("Unexected compiler state")
    };

    for imported_item in &imported_program.items {
        process_imported_item(original_program, imported_item)?;
    }

    Ok(())
}

fn process_imported_item(original_program: &mut Program, imported_item: &Item) -> Result<()> {
    match imported_item {
        Item::Function(function) => {
            original_program
                .items
                .push(Item::ExternFunction(ExternFunction {
                    name: function.name.clone(),
                    params: function.params.clone(),
                    return_type: function.return_type,
                }))
        }
        Item::ExternFunction(_) => original_program.items.push(imported_item.clone()),
        Item::Import(path) => {
            process_import(original_program, path.clone())?;
        }
    }

    Ok(())
}

// Returns all the imports of `program`
fn imports_of(program: &Program) -> Vec<PathBuf> {
    program
        .items
        .iter()
        .filter_map(|item| match item {
            Item::Import(path) => Some(path.clone()),
            _ => None,
        })
        .collect()
}
