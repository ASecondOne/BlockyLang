use crate::symbol_resolver::SharedExpression;

pub mod dirty_executer;

pub enum ExcuterOutput {
    ValidNone,
    ValidSome(SharedExpression),
    Error(String)
}
