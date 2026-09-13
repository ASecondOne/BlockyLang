use crate::psycho_parser::PsychoBlock;

pub trait Expression {
    fn evaluate(self) -> String;
}

pub struct ExpressionParser {
    origin: &'static str,
    parse: fn(&str) -> Option<Box<dyn Expression>>,
}

pub fn resolve_psycho_blocks(psycho_blocks: Vec<PsychoBlock>) -> i32 {
    0
}