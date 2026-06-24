

use crate::{
    ast::Ident,
    codegen::{Codegen, utils::TypedValue},
};

pub mod local;

impl<'a> Codegen<'a> {
    pub(super) fn resolve_ident(&self, ident: &Ident) -> Option<TypedValue<'a>> {
        let ident = ident.as_str();

        if let Some(local) = self.locals.get(ident) {
            return self.resolve_local(local, ident);
        }

        if let Some(constant) = self.constants.get(ident) {
            return Some(constant.clone());
        }

        None
    }
}
