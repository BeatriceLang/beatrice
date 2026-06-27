use chumsky::prelude::{choice, recursive};

use crate::{
    ast::ty::Type,
    parsing::ty::{array::array_ty, atom::atom_ty, ptr::ptr_ty},
};

mod array;
mod atom;
mod ptr;

pub fn ty<'a>() -> parser_type!(Type) {
    recursive(|ty| choice((array_ty(ty.clone()), atom_ty(), ptr_ty(ty))))
}
