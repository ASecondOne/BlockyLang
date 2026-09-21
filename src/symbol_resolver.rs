use crate::psycho_parser::{PsychoBlock, PsychoCall};
use crate::{i_core::{gather_expression_parsers, gather_keywords}};

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

pub struct ResolvedBlock {
    pub block_type: String, // //! currently does nothing, later hast to be resolved into an concrete block type
    pub resolved_lines: Vec<ResolvedExpression>
}

pub enum ResolvedExpression {
    Expression(Box<dyn Expression>),
    KeywordCall(KeywordCall),
}

pub struct KeywordCall {
    pub keyword: Keyword,
    pub args: Vec<ResolvedExpression>,
}

#[derive(Clone)]
pub struct Keyword {
    pub name: String,
    pub origin: String,
    pub execute: fn(Box<dyn Expression>) -> Option<Box<dyn Expression>>, //? Alter result into an actual Result, and make OK the Option<...>
}

pub fn resolve_psycho_blocks(psycho_blocks: Vec<PsychoBlock>) {
    for psycho_block in psycho_blocks {
        let _block_type = psycho_block.block_type; // //! currently does nothing, later hast to be resolved into an concrete block type
        let contents = psycho_block.contents;

        for content in contents {
            if let Some(resolved_line) = resolve_individual_line(content) {
                
            }
        }
    }
}

fn resolve_individual_line(input: PsychoCall) -> Option<ResolvedExpression> {
    let available_keywords = gather_keywords();


    None
}

fn attempt_expression_parsers(input: String) -> Option<Box<dyn Expression>> {
    let expression_parsers = gather_expression_parsers();

    let possible_expressions: Vec<Box<dyn Expression>> = Vec::new();

    for parser in expression_parsers {
        let possible = (parser.parse)(&input);
    }

    None
}