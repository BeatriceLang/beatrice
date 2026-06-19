use std::fmt::Display;

use crate::lexing::token::Token;

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Import => f.write_str("import"),
            Self::If => f.write_str("if"),
            Self::Fn => f.write_str("fn"),
            Self::LeftParen => f.write_str("("),
            Self::RightParen => f.write_str(")"),
            Self::LeftBrace => f.write_str("{"),
            Self::RightBrace => f.write_str("}"),
            Self::Colon => f.write_str(":"),
            Self::Return => f.write_str("return"),
            Self::Semicolon => f.write_str(";"),
            Self::Comma => f.write_str(","),
            Self::RetArrow => f.write_str("->"),
            Self::I32 => f.write_str("i32"),
            Self::Ident(name) => write!(f, "{name}"),
            Self::Number(value) => write!(f, "{value}"),
            Self::Add => f.write_str("+"),
            Self::Minus => f.write_str("-"),
            Self::Divide => f.write_str("/"),
            Self::Multiply => f.write_str("*"),
            Self::GreaterThan => f.write_str(">"),
            Self::LessThan => f.write_str("<"),
            Self::Equal => f.write_str("=="),
            Self::Assign => f.write_str("="),
            Self::Let => f.write_str("let"),
            Self::Extern => f.write_str("extern"),
            Self::String => f.write_str("string"),
            Self::StringLiteral(text) => write!(f, "\"{text}\""),
            Self::Var => f.write_str("var"),
        }
    }
}
