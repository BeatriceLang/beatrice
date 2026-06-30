use anyhow::Result;

use crate::{
    ast::item::Item,
    import::ImportProcessor,
};

impl ImportProcessor<'_> {
    pub(super) fn process_imported_item(&mut self, imported_item: &Item) -> Result<()> {
        match imported_item {
            Item::Function {
                name,
                params,
                return_type,
                ..
            } => {
                self.original_program
                    .items
                    .push(Item::ExternFunction {
                        name: name.clone(),
                        params: params.clone(),
                        return_type: return_type.clone(),
                    });
            }
            Item::ExternFunction { .. } | Item::Const { .. } | Item::DeclareStruct { .. } => {
                self.original_program.items.push(imported_item.clone());
            }
            Item::Import(path) => {
                self.process(path.clone())?;
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use crate::{
        ast::{
            Block, Program,
            ident::Ident,
            item::Item,
            ty::Type,
        },
        diagnostic::Diagnostics,
        import::ImportProcessor,
    };

    fn ident(name: &str) -> Ident {
        Ident::new(name.into(), 0..name.len())
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
        let imported = Item::Function {
            name: ident("imported_value"),
            params: vec![(ident("value"), Type::I32)],
            return_type: Some(Type::I32),
            body: Block { statements: vec![] },
        };

        let mut processor = ImportProcessor::new(&mut program, &mut diagnostics, source_path());

        processor.process_imported_item(&imported).unwrap();

        assert_eq!(
            program.items,
            vec![Item::ExternFunction {
                name: ident("imported_value"),
                params: vec![(ident("value"), Type::I32)],
                return_type: Some(Type::I32),
            }]
        );
        assert!(diagnostics.inner.is_empty());
    }

    #[test]
    fn process_imported_extern_function_copies_item() {
        let mut program = Program { items: vec![] };
        let mut diagnostics = diagnostics();
        let imported = Item::ExternFunction {
            name: ident("puts"),
            params: vec![(ident("value"), Type::String)],
            return_type: Some(Type::I32),
        };

        let mut processor = ImportProcessor::new(&mut program, &mut diagnostics, source_path());

        processor.process_imported_item(&imported).unwrap();

        assert_eq!(program.items, vec![imported]);
        assert!(diagnostics.inner.is_empty());
    }
}
