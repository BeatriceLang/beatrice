use std::mem::take;

use crate::{ast::Item, codegen::Codegen};

impl Codegen<'_> {
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
                Item::Const(_) => todo!(),
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
