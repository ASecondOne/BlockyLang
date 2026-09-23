use std::fmt::Debug;
use std::sync::Arc;

use crate::i_core::gather_blocktypes;
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
pub struct BlockType {
    pub name: String,
    pub execution_order: usize,
}

#[derive(Debug)]
pub struct ResolvedBlock {
    pub block_type: Arc<BlockType>,
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
    pub origin: String,
    pub execute: fn(Box<dyn Expression>) -> Option<Box<dyn Expression>>, //? Alter result into an actual Result, and make OK the Option<...>
}

pub fn resolve_psycho_blocks(psycho_blocks: Vec<PsychoBlock>) -> Vec<ResolvedBlock> {
    let block_types = gather_blocktypes();

    let mut out = Vec::new();

    for psycho_block in psycho_blocks {
        let block_type = psycho_block.block_type;
        let contents = psycho_block.contents;

        let resolved_block_type = block_types.iter().find(|b| b.name == block_type).unwrap().clone();

        let resolved_contents: Vec<ResolvedExpression> = contents
            .iter()
            .map(|content| resolve_individual_line(content))
            .filter_map(|option|
                match option {
                    Some(s) => Some(s),
                    None => None
                }
            )
            .collect();

        let resolved_block = ResolvedBlock {
            block_type: resolved_block_type,
            resolved_lines: resolved_contents
        };

        println!("{:#?}", resolved_block);

        out.push(resolved_block);
    }

    out
}

fn resolve_individual_line(input: &PsychoCall) -> Option<ResolvedExpression> {
    let available_keywords = gather_keywords();
    let mut resolved_expressions = Vec::new();

    for expression in &input.expressions {
        match expression {
            PsychoExpression::Call(c) => {
                resolved_expressions.push(resolve_individual_line(&c)?);
            }

            PsychoExpression::Expression(e) => {
                resolved_expressions.push(resolve_expressions(&e)?);
            }
        }
    }

    let keyword = available_keywords
        .into_iter()
        .find(|k| k.origin.contains(&input.keyword))?;

    Some(ResolvedExpression::KeywordCall(KeywordCall {
        keyword,
        args: resolved_expressions,
    }))
}

fn resolve_expressions(input: &str) -> Option<ResolvedExpression> {
    let expression_parsers = gather_expression_parsers();

    let mut possible_expressions: Vec<Box<dyn Expression>> = Vec::new();

    for parser in expression_parsers {
        if let Some(exp) = (parser.parse)(input) {
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