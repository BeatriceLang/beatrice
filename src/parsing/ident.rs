use crate::{lexing::token::Token, parsing::parsing_rule};

parsing_rule! {
    ident -> String {
        chumsky::select! {
            Token::Ident(name) => name.clone()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_ident() {
        use chumsky::Parser as _;

        let tokens = [Token::Ident("main".into())];

        assert_eq!(ident().parse(&tokens).unwrap(), "main");
    }
}
