use crate::ast::expresion::Expression;

// Statements will proform an action (aka do something)
#[derive(Debug)]
pub enum Statement {
    Return(Expression),
    Expression(Expression),
}
