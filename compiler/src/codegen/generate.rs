use std::mem::take;

use crate::{ast::item::Item, codegen::Codegen};

impl Codegen<'_> {
    pub fn generate(&mut self) {
        let items = take(&mut self.program.items);

        for item in &items {
            if let Item::DeclareStruct { name, fields } = item {
                self.declare_struct(name, fields);
            }
        }

        for item in &items {
            match item {
                Item::Function {
                    name,
                    params,
                    return_type,
                    ..
                }
                | Item::ExternFunction {
                    name,
                    params,
                    return_type,
                } => self.declare_function(name.as_str(), params, return_type.clone()),
                Item::Const { name, val, .. } => {
                    let value = self.compile_expr(val).unwrap();
                    self.constants.insert(name.as_str().to_string(), value);
                }
                Item::TypeAlias { .. } | Item::Import(_) | Item::DeclareStruct { .. } => (),
            }
        }

        for item in &items {
            if let Item::DeclareStruct { name, fields } = item {
                self.define_struct(name, fields);
            }
        }

        for item in &items {
            if let Item::Function {
                name,
                params,
                return_type,
                body,
            } = item
            {
                self.compile_function(name, params, return_type.as_ref(), body);
            }
        }
        self.program.items = items;
    }
}
