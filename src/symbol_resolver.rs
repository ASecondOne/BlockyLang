use crate::{i_core::gather_expression_parsers, psycho_parser::{PsychoBlock, PsychoLine}};

//? Used for Values and Variables
pub trait Expression {
    fn evaluate(self: Box<Self>) -> Option<Box<dyn Expression>>;
    fn display(self: Box<Self>) -> Option<String>;
}

//? Used to Parse everything that implements Expression
pub struct ExpressionParser {
    pub origin: &'static str,
    pub parse: fn(&str) -> Option<Box<dyn Expression>>,
}

pub enum ResolvedExpression {
    Expression(Box<dyn Expression>),
    KeywordCall(KeywordCall),
}

pub struct KeywordCall {
    pub keyword: Keyword,
    pub args: Vec<ResolvedExpression>,
}

pub struct Keyword {
    pub name: String,
    pub execute: fn(Box<dyn Expression>) -> Option<Box<dyn Expression>>, //? Alter result into an accual Result, and make OK the Option<...>
}

pub fn resolve_psycho_blocks(psycho_blocks: Vec<PsychoBlock>) {
    let parsers = gather_expression_parsers();

    for psycho_block in psycho_blocks {
        let block_type = psycho_block.block_type; // //! currently does nothing, later hast to be resolved into an concreate block type
        let contents = psycho_block.contents;

        for content in contents {
             let out = resolve_individual_line(content);
        }
    }
}

fn resolve_individual_line(input: Vec<PsychoLine>) -> Option<ResolvedExpression> {
    None
}