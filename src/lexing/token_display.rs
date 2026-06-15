use std::fmt::Display;

use crate::lexing::token::Token;

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Token::Import => f.write_str("import"),
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
            Token::String => f.write_str("string"),
            Token::StringLiteral(text) => write!(f, "\"{text}\""),
        }
    }
}
