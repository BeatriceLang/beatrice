use crate::{
    ast::{expression::Expression, ident::Ident},
    codegen::{Codegen, utils::TypedValue},
};

mod is_nullptr;

impl<'a> Codegen<'a> {
    pub(super) fn compile_intrinsic(&self, name: &Ident, args: &[Expression]) -> TypedValue<'a> {
        match name.as_str() {
            "_is_nullptr" => self.compile_is_nullptr(args),
            _ => unreachable!(),
        }
    }
}

#[allow(clippy::match_like_matches_macro)]
pub(super) fn is_intrinsic(name: &Ident) -> bool {
    match name.as_str() {
        "_is_nullptr" => true,
        _ => false,
    }
}
