use std::{collections::HashSet, path::PathBuf};

use anyhow::{Context, Ok, Result};

use crate::{
    ast::{Item, Program, function::ExternFunction},
    diagnostic::{Diagnostic, DiagnosticKind, Diagnostics},
    import::visit_check::VisitState,
    state::{Compiler, CompilerState},
};

mod visit_check;

impl Compiler {
    pub fn import(&mut self) -> Result<()> {
        let program = {
            let CompilerState::Import(program) = &mut self.state else {
                panic!("Unexpected compiler state")
            };

            let import_paths = imports_of(program);

            let mut import_processor = ImportProcessor::new(program, &mut self.diagnostics);

            for path in import_paths {
                import_processor.process(path)?;
            }

            program.clone()
        };

        self.advance_to(CompilerState::Check(program))
    }
}

struct ImportProcessor<'a> {
    original_program: &'a mut Program,
    visited: HashSet<PathBuf>,
    visiting: Vec<PathBuf>,
    diagnostics: &'a mut Diagnostics,
}

impl<'a> ImportProcessor<'a> {
    fn new(original_program: &'a mut Program, diagnostics: &'a mut Diagnostics) -> Self {
        Self {
            original_program,
            diagnostics,
            visited: HashSet::new(),
            visiting: vec![],
        }
    }

    fn process(&mut self, path: PathBuf) -> Result<()> {
        match self.visit_state(&path) {
            VisitState::Process => {}
            VisitState::Skip => return Ok(()),
        }

        self.visiting.push(path.clone());

        let mut compiler = Compiler::new(path.clone(), PathBuf::new())
            .context("Failed to create compiler when processing imports")?;

        compiler.lex()?;
        compiler.parse()?;

        let CompilerState::Import(imported_program) = &compiler.state else {
            panic!("Unexected compiler state")
        };

        for imported_item in &imported_program.items {
            self.process_imported_item(imported_item)?;
        }

        self.visiting.pop();
        self.visited.insert(path);

        Ok(())
    }

    fn process_imported_item(&mut self, imported_item: &Item) -> Result<()> {
        match imported_item {
            Item::Function(function) => {
                self.original_program
                    .items
                    .push(Item::ExternFunction(ExternFunction {
                        name: function.name.clone(),
                        params: function.params.clone(),
                        return_type: function.return_type,
                    }))
            }
            Item::ExternFunction(_) => self.original_program.items.push(imported_item.clone()),
            Item::Import(path) => {
                self.process(path.clone())?;
            }
        }

        Ok(())
    }
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
