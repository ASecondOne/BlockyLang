use std::fmt::Debug;

use crate::psycho_parser::{PsychoBlock, PsychoCall, PsychoExpression};
use crate::{i_core::{gather_expression_parsers, gather_keywords}};


//? Used for Values and Variables
pub trait Expression: Debug {
    fn evaluate(self: Box<Self>) -> Option<Box<dyn Expression>>;
    fn display(self: Box<Self>) -> Option<String>;
}

//? Used to Parse everything that implements Expression
#[derive(Debug)]
pub struct ExpressionParser {
    pub origin: &'static str,
    pub parse: fn(&str) -> Option<Box<dyn Expression>>,
}

#[derive(Debug)]
pub struct ResolvedBlock {
    pub block_type: String, // //! currently does nothing, later hast to be resolved into an concrete block type
    pub resolved_lines: Vec<ResolvedExpression>
}

#[derive(Debug)]
pub enum ResolvedExpression {
    Expression(Box<dyn Expression>),
    KeywordCall(KeywordCall),
}

#[derive(Debug)]
pub struct KeywordCall {
    pub keyword: Keyword,
    pub args: Vec<ResolvedExpression>,
}

#[derive(Clone, Debug)]
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
                println!("{:#?}", resolved_line)
            }
        }
    }
}

fn resolve_individual_line(input: PsychoCall) -> Option<ResolvedExpression> {
    let available_keywords = gather_keywords();
    let mut resolved_expressions = Vec::new();

    for expression in input.expressions {
        match expression {
            PsychoExpression::Call(c) => {
                resolved_expressions.push(resolve_individual_line(c)?);
            }

            PsychoExpression::Expression(e) => {
                resolved_expressions.push(resolve_expressions(e)?);
            }
        }
    }

    let keyword = available_keywords
        .into_iter()
        .find(|k| k.name == input.keyword)?;

    Some(ResolvedExpression::KeywordCall(KeywordCall {
        keyword,
        args: resolved_expressions,
    }))
}

fn resolve_expressions(input: String) -> Option<ResolvedExpression> {
    let expression_parsers = gather_expression_parsers();

    let mut possible_expressions: Vec<Box<dyn Expression>> = Vec::new();

    for parser in expression_parsers {
        if let Some(exp) = (parser.parse)(&input) {
            possible_expressions.push(exp);
        }
    }

    if possible_expressions.len() >= 1 {
        // //! only takes first possible expression needs to be changed

        return Some(ResolvedExpression::Expression(
                possible_expressions.remove(0)
        ));
    }

    None
}