use std::mem::take;

use crate::{ast::Item, codegen::Codegen};

impl Codegen<'_> {
    pub fn generate(&mut self) {
        let items = take(&mut self.program.items);

        for item in &items {
            if let Item::DeclareStruct(declare_struct) = item {
                self.declare_struct(declare_struct);
            }
        }

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
                Item::DeclareStruct(_) => {}
            }
        }

        for item in &items {
            match item {
                Item::Function(function) => {
                    self.compile_function(function);
                }
                Item::DeclareStruct(declare_struct) => self.define_struct(declare_struct),
                _ => (),
            }
        }
        self.program.items = items;
    }
}
