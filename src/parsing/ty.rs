use chumsky::select;

use crate::{ast::Type, lexing::token::Token, parsing::parsing_rule};

parsing_rule! {
    ty -> Type {
        select! {
            Token::I32 => Type::I32
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_ty() {
        use chumsky::Parser as _;

        let tokens = [Token::I32];

        assert_eq!(ty().parse(&tokens).unwrap(), Type::I32);
    }
}
