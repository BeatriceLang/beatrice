use logos::Logos;

#[derive(Logos, Debug, Clone, PartialEq, Eq)]
#[logos(skip r"[ \t\n\f]+")]
pub enum Token {
    #[token("import")]
    Import,

    #[token("if")]
    If,

    #[token("while")]
    While,

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

    #[token("var")]
    Var,

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

    // TODO: use a better name cuz it also repersents pointers
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

    #[token("string")]
    String,

    #[regex(r#""([^"\\]|\\["\\nrt])*""#, parse_string_literal)]
    StringLiteral(String),
}

fn parse_string_literal(lex: &logos::Lexer<Token>) -> String {
    let slice = lex.slice();
    let inner = &slice[1..slice.len() - 1];
    let mut text = String::new();
    let mut chars = inner.chars();

    while let Some(char) = chars.next() {
        if char == '\\' {
            let escaped = match chars.next() {
                Some('"') => '"',
                Some('\\') => '\\',
                Some('n') => '\n',
                Some('r') => '\r',
                Some('t') => '\t',
                Some(char) => char,
                None => break,
            };
            text.push(escaped);
        } else {
            text.push(char);
        }
    }

    text
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

    #[test]
    fn lexes_string_type_and_text() {
        let input = r#"string "hello\nworld" "quote: \"" "slash: \\""#;

        let tokens: Vec<_> = Token::lexer(input).map(|token| token.unwrap()).collect();

        assert_eq!(
            tokens,
            vec![
                Token::String,
                Token::StringLiteral("hello\nworld".into()),
                Token::StringLiteral("quote: \"".into()),
                Token::StringLiteral("slash: \\".into()),
            ]
        );
    }
}
