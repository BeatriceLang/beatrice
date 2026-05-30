use crate::ast::expresion::Expression;

// Statements will proform an action (aka do something)
pub enum Statement {
    Return(Expression),
}
