use crate::symbolresolver::symbol_resolver::SharedExpression;

pub mod dirty_executer;
pub mod executer;

pub enum ExcuterOutput {
    ValidNone,
    ValidSome(SharedExpression),
    Error(String)
}
