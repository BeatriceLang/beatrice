use logos::Logos;

#[derive(Logos, Debug, Clone, PartialEq, Eq)]
#[logos(skip r"[ \t\n\f]+")]
pub enum Token {
    #[token("if")]
    If,

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

    #[token(":")]
    Colon,

    #[token("return")]
    Return,

    #[token(";")]
    Semicolon,

    #[token(",")]
    Comma,

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

    #[token(">")]
    GreaterThan,

    #[token("<")]
    LessThan,

    #[token("==")]
    Equal,

    #[token("let")]
    Let,
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

    #[test]
    fn lexes_arithmetic_operators() {
        let input = "1 + 2 - 3 * 4 / 5";

        let tokens: Vec<_> = Token::lexer(input).map(|token| token.unwrap()).collect();

        assert_eq!(
            tokens,
            vec![
                Token::Number(1),
                Token::Add,
                Token::Number(2),
                Token::Minus,
                Token::Number(3),
                Token::Multiply,
                Token::Number(4),
                Token::Divide,
                Token::Number(5),
            ]
        );
    }
}
