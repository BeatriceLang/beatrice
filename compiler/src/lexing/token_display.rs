use std::fmt::Display;

use crate::lexing::token::Token;

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Import => f.write_str("import"),
            Self::If => f.write_str("if"),
            Self::Fn => f.write_str("fn"),
            Self::New => f.write_str("new"),
            Self::ExclamationMark => f.write_str("!"),
            Self::As => f.write_str("as"),
            Self::Bool => f.write_str("bool"),
            Self::BoolLiteral(value) => write!(f, "{value}"),
            Self::LeftParen => f.write_str("("),
            Self::U32 => f.write_str("u32"),
            Self::Dot => f.write_str("."),
            Self::Struct => f.write_str("struct"),
            Self::Const => f.write_str("const"),
            Self::RightParen => f.write_str(")"),
            Self::LeftBrace => f.write_str("{"),
            Self::RightBrace => f.write_str("}"),
            Self::Colon => f.write_str(":"),
            Self::Return => f.write_str("return"),
            Self::Semicolon => f.write_str(";"),
            Self::Comma => f.write_str(","),
            Self::RetArrow => f.write_str("->"),
            Self::I32 => f.write_str("i32"),
            Self::I32Number(num) => write!(f, "{num}i32"),
            Self::U32Number(num) => write!(f, "{num}u32"),
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
            Self::While => f.write_str("while"),
            Self::Var => f.write_str("var"),
            Self::AddressOf => f.write_str("&"),
        }
    }
}
