use crate::{lexing::token::Token, parsing::parsing_rule};

parsing_rule! {
    ident -> String {
        chumsky::select! {
            Token::Ident(name) => name.clone()
        }
    }
}
