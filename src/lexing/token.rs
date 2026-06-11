use logos::Logos;

#[derive(Logos, Debug, Clone, PartialEq, Eq)]
#[logos(skip r"[ \t\n\f]+")]
pub enum Token {
    #[token("fn")]
    Fn,

    #[token("(")]
    LeftParen,

    #[token(")")]
    RightParen,

    #[token("{")]
    LeftBrace,

    #[token("}")]
    RightBrace,

    #[token("return")]
    Return,

    #[token(";")]
    Semicolon,

    #[token("->")]
    RetArrow,

    #[token("i32")]
    I32,

    #[regex(r"[a-zA-Z_][a-zA-Z0-9_]*", |lex| lex.slice().to_string())]
    Ident(String),

    #[regex(r"[0-9]+", |lex| lex.slice().parse::<i64>().ok())]
    Number(i64),

    #[token("+")]
    Add,

    #[token("-")]
    Minus,

    #[token("/")]
    Divide,

    #[token("*")]
    Multiply,
}

#[cfg(test)]
mod tests {
    use logos::Logos;

    use super::Token;

    #[test]
    fn lexes_return_number_function() {
        let input = "fn main() -> i32 { return 42; }";

        let tokens: Vec<_> = Token::lexer(input).map(|token| token.unwrap()).collect();

        assert_eq!(
            tokens,
            vec![
                Token::Fn,
                Token::Ident("main".into()),
                Token::LeftParen,
                Token::RightParen,
                Token::RetArrow,
                Token::I32,
                Token::LeftBrace,
                Token::Return,
                Token::Number(42),
                Token::Semicolon,
                Token::RightBrace,
            ]
        );
    }
}
