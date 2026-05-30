// Expressions can be evaluated into a value
#[derive(Debug)]
pub enum Expression {
    Ident(String),
    Number(i64),
}
