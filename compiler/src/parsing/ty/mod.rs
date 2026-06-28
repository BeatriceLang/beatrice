use chumsky::prelude::{choice, recursive};

use crate::{
    ast::ty::Type,
    parsing::ty::{array::array, atom::atom, ptr::ptr},
};

mod array;
mod atom;
mod ptr;

pub fn ty<'a>() -> parser_type!(Type) {
    recursive(|ty| choice((array(ty.clone()), atom(), ptr(ty))))
}
