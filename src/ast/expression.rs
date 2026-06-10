// Expressions can be evaluated into a value
#[derive(Debug, PartialEq, Eq)]
pub enum Expression {
    Ident(String),
    Number(i64),
}
