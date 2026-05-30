use crate::{lexing::token::Token, parsing_rule};

parsing_rule! {
    ident -> String {
        chumsky::select! {
            Token::Ident(name) => name
        }
    }
}
