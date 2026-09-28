use std::any::Any;
use std::fmt::Debug;
use std::sync::{Arc, Mutex};

use crate::executers::ExcuterOutput;
use crate::i_core::{gather_blocktypes, gather_expression_parsers, gather_keywords};
use crate::psychoparser::psycho_parser::{PsychoBlock, PsychoCall, PsychoExpression};

//? Used for Values and Variables
pub trait Expression: Debug + Send {
    fn evaluate(&self) -> Option<SharedExpression>;
    fn display(&self) -> Option<String>;
    fn as_any(&self) -> &dyn Any;
    fn redirect(&mut self, new: SharedExpression) -> ExcuterOutput;
}

pub type SharedExpression = Arc<Mutex<dyn Expression>>;

//? Used to Parse everything that implements Expression
#[derive(Debug)]
pub struct ExpressionParser {
    pub origin: &'static str,
    pub parse: fn(&str) -> Option<SharedExpression>,
}

#[derive(Debug)]
pub struct BlockType {
    pub name: String,
    pub symbol_blacklist: Vec<String>,
    pub symbol_whitelist: Vec<String>,
    pub execution_order: usize,
}

#[derive(Debug)]
pub struct ResolvedBlock {
    pub block_type: Arc<BlockType>,
    pub resolved_lines: Vec<ResolvedExpression>,
}

#[derive(Debug)]
pub enum ResolvedExpression {
    Expression(SharedExpression),
    KeywordCall(KeywordCall),
    Redirect(Box<ResolvedExpression>, Box<ResolvedExpression>),
}

#[derive(Debug)]
pub struct KeywordCall {
    pub keyword: Keyword,
    pub args: Vec<ResolvedExpression>,
}

#[derive(Clone, Debug)]
pub struct Keyword {
    pub origin: String,
    pub execute: fn(Vec<SharedExpression>) -> ExcuterOutput,
}

pub fn resolve_psycho_blocks(psycho_blocks: Vec<PsychoBlock>) -> Vec<ResolvedBlock> {
    let block_types = gather_blocktypes();

    let mut out = Vec::new();

    for psycho_block in psycho_blocks {
        let block_type = psycho_block.block_type;
        let contents = psycho_block.contents;

        let resolved_block_type = block_types
            .iter()
            .find(|b| b.name == block_type)
            .unwrap()
            .clone();

        let resolved_contents: Vec<ResolvedExpression> = contents
            .iter()
            .filter_map(|content| {
                resolve_individual_line(content, &resolved_block_type)
            })
            .collect();

        out.push(ResolvedBlock {
            block_type: resolved_block_type,
            resolved_lines: resolved_contents,
        });
    }

    out
}

fn resolve_individual_line(input: &PsychoExpression, block_type: &BlockType) -> Option<ResolvedExpression> {
    match input {
        PsychoExpression::Expression(e) => {
            resolve_expressions(e)
        }

        PsychoExpression::Call(c) => {
            resolve_call(c, block_type)
        }

        PsychoExpression::Redirect(into, from) => {
            Some(ResolvedExpression::Redirect(
                Box::new(resolve_individual_line(into, block_type)?),
                Box::new(resolve_individual_line(from, block_type)?),
            ))
        }
    }
}

fn resolve_call(input: &PsychoCall, block_type: &BlockType) -> Option<ResolvedExpression> {
    let available_keywords = gather_keywords();

    let resolved_expressions: Vec<ResolvedExpression> = input
        .expressions
        .iter()
        .map(|expression| resolve_individual_line(expression, block_type))
        .collect::<Option<Vec<_>>>()?;

    let keyword = available_keywords
        .into_iter()
        .find(|k| {
            let matches = k.origin.contains(&input.keyword);

            let whitelisted = block_type
                .symbol_whitelist
                .iter()
                .any(|s| k.origin.contains(s));

            let blacklisted = block_type
                .symbol_blacklist
                .iter()
                .any(|s| k.origin.contains(s));

            matches && whitelisted && !blacklisted
        })?;

    Some(ResolvedExpression::KeywordCall(KeywordCall {
        keyword,
        args: resolved_expressions,
    }))
}

fn resolve_expressions(input: &str) -> Option<ResolvedExpression> {
    let expression_parsers = gather_expression_parsers();

    let mut possible_expressions: Vec<SharedExpression> = Vec::new();

    for parser in expression_parsers {
        if let Some(exp) = (parser.parse)(input) {
            possible_expressions.push(exp);
        }
    }

    if !possible_expressions.is_empty() {
        // //! Only takes first possible expression, needs to be changed.
        return Some(ResolvedExpression::Expression(
            possible_expressions.remove(0),
        ));
    }

    None
}
