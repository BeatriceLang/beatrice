use chumsky::Parser;

use crate::{
    ast::expression::Expression,
    parsing::expr::{atom::atom, postfix::array_access::array_access},
};

mod array_access;

#[derive(Debug, PartialEq, Eq)]
enum Postfix {
    ArrayAccess(Expression),
}

pub(super) fn postfix_expr<'a>(expr: parser_type!(Expression)) -> parser_type!(Expression) {
    let postfix = array_access(expr.clone());

    atom(expr).foldl(postfix.repeated(), merge_postfix)
}

fn merge_postfix(prefix: Expression, postfix: Postfix) -> Expression {
    match postfix {
        Postfix::ArrayAccess(index) => Expression::ArrayAccess {
            array: Box::new(prefix),
            index: Box::new(index),
        },
    }
}
