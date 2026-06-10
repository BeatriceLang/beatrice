use crate::ast::expression::Expression;

// Statements will proform an action (aka do something)
#[derive(Debug, PartialEq, Eq)]
pub enum Statement {
    Return(Expression),
    Expression(Expression),
}
