use std::sync::Arc;

use crate::{i_core::{math::inc_one, stdout::println, value::expression_parser}, symbol_resolver::{BlockType, ExpressionParser, Keyword}};

mod value;
mod math;
mod stdout;

pub fn gather_expression_parsers() -> Vec<ExpressionParser> {
    let mut out = Vec::new();

    out.push(
        ExpressionParser {
            origin: "i_core::value",
            parse: expression_parser
        }
    );

    out
}

pub fn gather_keywords() -> Vec<Keyword> {
    let mut out = Vec::new();

    out.push(Keyword { 
        origin: "i_core::stdout::println".to_string(), 
        execute: println 
    });

    out.push(Keyword { 
        origin: "i_core::math::inc_one".to_string(), 
        execute: inc_one 
    });

    out
}

pub fn gather_blocktypes() -> Vec<Arc<BlockType>> {
    let mut out= Vec::new();

    //* 1 means first, 2 means second and so on, 0 means not at all

    out.push(Arc::new(BlockType {
        name: "execute".to_string(),
        execution_order: 1,
    }));

    out
}