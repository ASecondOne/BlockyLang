use crate::psycho_parser::PsychoBlock;

pub struct Symbol {

}

pub struct ResolvedBlock {

}

pub enum ResolvedLine {
    Keyword(Symbol),
    Expression(Expression)
}

pub enum Expression {
    Value(Option<i32>), //? Option<i32> will be changed into an module path to the Value Handler (Required)
}

pub fn resolve_psycho_blocks(psycho_blocks: Vec<PsychoBlock>) -> i32 {
    0
}