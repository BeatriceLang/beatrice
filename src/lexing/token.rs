use std::fmt::Display;

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

    #[token("extern")]
    Extern,

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

    #[token("=")]
    Assign,

    #[token("let")]
    Let,
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Token::If => f.write_str("if"),
            Token::Fn => f.write_str("fn"),
            Token::LeftParen => f.write_str("("),
            Token::RightParen => f.write_str(")"),
            Token::LeftBrace => f.write_str("{"),
            Token::RightBrace => f.write_str("}"),
            Token::Colon => f.write_str(":"),
            Token::Return => f.write_str("return"),
            Token::Semicolon => f.write_str(";"),
            Token::Comma => f.write_str(","),
            Token::RetArrow => f.write_str("->"),
            Token::I32 => f.write_str("i32"),
            Token::Ident(name) => write!(f, "{name}"),
            Token::Number(value) => write!(f, "{value}"),
            Token::Add => f.write_str("+"),
            Token::Minus => f.write_str("-"),
            Token::Divide => f.write_str("/"),
            Token::Multiply => f.write_str("*"),
            Token::GreaterThan => f.write_str(">"),
            Token::LessThan => f.write_str("<"),
            Token::Equal => f.write_str("=="),
            Token::Assign => f.write_str("="),
            Token::Let => f.write_str("let"),
            Token::Extern => f.write_str("extern"),
        }
    }
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
