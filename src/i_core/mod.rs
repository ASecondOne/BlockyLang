use crate::{i_core::{stdout::println, value::expression_parser}, symbol_resolver::{ExpressionParser, Keyword}};

mod value;
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
        name: "i_core::stdout::println".to_string(), 
        execute: println 
    });

    out
}