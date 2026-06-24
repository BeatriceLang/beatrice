use crate::{ast::Ident, codegen::Codegen};

mod local;
pub(super) use local::Local;

pub(super) enum ResolvedIdent<'ctx, 'cg> {
    Local(&'cg Local<'ctx>),
}

impl<'a> Codegen<'a> {
    pub(super) fn resolve_ident(&self, ident: &Ident) -> Option<ResolvedIdent<'a, '_>> {
        if let Some(local) = self.locals.get(ident.as_str()) {
            return Some(ResolvedIdent::Local(local));
        }

        None
    }
}
