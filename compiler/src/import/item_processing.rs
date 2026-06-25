use anyhow::Result;

use crate::{
    ast::{Item, function::ExternFunction},
    import::ImportProcessor,
};

impl ImportProcessor<'_> {
    pub(super) fn process_imported_item(&mut self, imported_item: &Item) -> Result<()> {
        match imported_item {
            Item::Function(function) => {
                self.original_program
                    .items
                    .push(Item::ExternFunction(ExternFunction {
                        name: function.name.clone(),
                        params: function.params.clone(),
                        return_type: function.return_type.clone(),
                    }));
            }
            Item::ExternFunction(_) | Item::Const(_) => {
                self.original_program.items.push(imported_item.clone())
            }
            Item::Import(path) => {
                self.process(path.clone())?;
            }
            Item::Struct(_) => todo!(),
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use crate::{
        ast::{Block, Function, Ident, Item, Program, Type, function::ExternFunction},
        diagnostic::Diagnostics,
        import::ImportProcessor,
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

    fn extern_function(name: &str) -> ExternFunction {
        ExternFunction {
            name: ident(name),
            params: vec![(ident("value"), Type::String)],
            return_type: Some(Type::I32),
        }
    }

    fn diagnostics() -> Diagnostics {
        Diagnostics::new(String::new(), PathBuf::from("main.bt"))
    }

    fn source_path() -> PathBuf {
        PathBuf::from("main.bt")
    }

    #[test]
    fn process_imported_function_adds_extern_function() {
        let mut program = Program { items: vec![] };
        let mut diagnostics = diagnostics();
        let imported = function("imported_value");

        let mut processor = ImportProcessor::new(&mut program, &mut diagnostics, source_path());

        processor
            .process_imported_item(&Item::Function(imported.clone()))
            .unwrap();

        assert_eq!(
            program.items,
            vec![Item::ExternFunction(ExternFunction {
                name: imported.name,
                params: imported.params,
                return_type: imported.return_type,
            })]
        );
        assert!(diagnostics.inner.is_empty());
    }

    #[test]
    fn process_imported_extern_function_copies_item() {
        let mut program = Program { items: vec![] };
        let mut diagnostics = diagnostics();
        let imported = Item::ExternFunction(extern_function("puts"));

        let mut processor = ImportProcessor::new(&mut program, &mut diagnostics, source_path());

        processor.process_imported_item(&imported).unwrap();

        assert_eq!(program.items, vec![imported]);
        assert!(diagnostics.inner.is_empty());
    }
}
