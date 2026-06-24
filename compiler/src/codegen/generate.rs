use std::mem::take;

use crate::{
    ast::Item,
    codegen::{Codegen, utils::TypedValue},
};

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
                Item::Const(constant) => {
                    let value = self.compile_expr(&constant.val).unwrap();
                    self.constants
                        .insert(constant.name.as_str().to_string(), value);
                }
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
