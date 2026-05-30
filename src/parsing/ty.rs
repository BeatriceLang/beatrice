use chumsky::select;

use crate::{ast::Type, lexing::token::Token, parsing_rule};

parsing_rule! {
    ty -> Type {
        select! {
            Token::I32 => Type::I32
        }
    }
}
