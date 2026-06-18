use std::{collections::HashSet, path::PathBuf};

use anyhow::{Context, Ok, Result};

use crate::{
    ast::{Item, Program},
    diagnostic::Diagnostics,
    import::visit_check::VisitState,
    state::{Compiler, CompilerState},
};

mod item_processing;
mod visit_check;

impl Compiler {
    pub fn import(&mut self) -> Result<()> {
        let original_source_path = self.source_path.clone();
        let program = {
            let CompilerState::Import(program) = &mut self.state else {
                panic!("Unexpected compiler state")
            };

            let import_paths = imports_of(program);

            let mut import_processor =
                ImportProcessor::new(program, &mut self.diagnostics, original_source_path);

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
    original_source_path: PathBuf,
    visited: HashSet<PathBuf>,
    visiting: Vec<PathBuf>,
    diagnostics: &'a mut Diagnostics,
}

impl<'a> ImportProcessor<'a> {
    fn new(
        original_program: &'a mut Program,
        diagnostics: &'a mut Diagnostics,
        original_source_path: PathBuf,
    ) -> Self {
        Self {
            original_program,
            original_source_path,
            diagnostics,
            visited: HashSet::new(),
            visiting: vec![],
        }
    }

    fn process(&mut self, path: PathBuf) -> Result<()> {
        let path = self.resolve_import_path(path)?;

        match self.visit_state(&path) {
            VisitState::Process => {}
            VisitState::Skip => return Ok(()),
        }

        self.visiting.push(path.clone());

        let mut compiler = Compiler::new(&path, PathBuf::new())
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

    fn resolve_import_path(&self, path: PathBuf) -> Result<PathBuf> {
        let path = if path.is_absolute() {
            path
        } else {
            self.importing_file()
                .parent()
                .context("Failed to parse importing file parent directory")?
                .join(path)
        };

        path.canonicalize().context("Failed to resolve import path")
    }

    fn importing_file(&self) -> &PathBuf {
        self.visiting.last().unwrap_or(&self.original_source_path)
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

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use crate::{
        ast::{Block, Function, Ident, Item, Program, Type},
        import::imports_of,
    };

    fn ident(name: &str) -> Ident {
        Ident::new(name.into(), 0..name.len())
    }

    fn function(name: &str) -> Function {
        Function {
            name: ident(name),
            params: vec![(ident("value"), Type::I32)],
            return_type: Some(Type::I32),
            body: Block { statements: vec![] },
        }
    }

    #[test]
    fn imports_of_returns_import_paths_in_order() {
        let program = Program {
            items: vec![
                Item::Function(function("main")),
                Item::Import(PathBuf::from("first.bt")),
                Item::Import(PathBuf::from("second.bt")),
            ],
        };

        assert_eq!(
            imports_of(&program),
            vec![PathBuf::from("first.bt"), PathBuf::from("second.bt")]
        );
    }
}
