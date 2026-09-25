use crate::symbol_resolver::Expression;

pub mod dirty_executer;

pub enum ExcuterOutput {
    ValidNone,
    ValidSome(Box<dyn Expression>),
    Error(String)
}