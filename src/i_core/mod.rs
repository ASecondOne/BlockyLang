use std::sync::Arc;

use crate::{i_core::{conditions::i_if, datastore::{access_modifires::{i_get_acmods, i_set_acmods}, var::{i_let, i_origin, i_transfer, i_type}, variable_parse}, math::i_inc_one, stdout::{i_print, i_println}, value::{i_new, value_parser}}, symbolresolver::symbol_resolver::{BlockType, ExpressionParser, Keyword}};

pub(crate) mod value;
mod math;
mod stdout;
mod conditions;

pub(crate) mod datastore;

pub fn gather_expression_parsers() -> Vec<ExpressionParser> {
    let mut out = Vec::new();

    out.push(
        ExpressionParser {
            origin: "i_core::value",
            parse: value_parser
        }
    );

    out.push(
        ExpressionParser {
            origin: "i_core::datastore",
            parse: variable_parse
        }
    );

    out
}

pub fn gather_keywords() -> Vec<Keyword> {
    let mut out = Vec::new();

    out.push(Keyword {
        origin: "i_core::conditons::if".to_string(),
        execute: i_if
    });

    out.push(Keyword { 
        origin: "i_core::stdout::println".to_string(), 
        execute: i_println 
    });

    out.push(Keyword { 
        origin: "i_core::stdout::print".to_string(), 
        execute: i_print
    });

    out.push(Keyword { 
        origin: "i_core::math::inc_one".to_string(), 
        execute: i_inc_one 
    });

    out.push(Keyword { 
        origin: "i_core::datastore::var::let".to_string(), 
        execute: i_let 
    });

    out.push(Keyword { 
        origin: "i_core::datastore::var::type".to_string(), 
        execute: i_type 
    });

    out.push(Keyword { 
        origin: "i_core::datastore::var::origin".to_string(), 
        execute: i_origin 
    });

    out.push(Keyword { 
        origin: "i_core::datastore::var::transfer".to_string(), 
        execute: i_transfer 
    });

    out.push(Keyword { 
        origin: "i_core::datastore::access_modifires::get_AcMods".to_string(), 
        execute: i_get_acmods 
    });

    out.push(Keyword { 
        origin: "i_core::datastore::access_modifires::set_AcMods".to_string(), 
        execute: i_set_acmods 
    });

    out.push(Keyword { 
        origin: "i_core::value::new".to_string(), 
        execute: i_new 
    });

    out
}

pub fn gather_blocktypes() -> Vec<Arc<BlockType>> {
    let mut out= Vec::new();

    //* 1 means first, 2 means second and so on, 0 means not at all

    out.push(Arc::new(BlockType {
        name: "execute".to_string(),
        symbol_blacklist: vec!["i_core::datastore::var::let".to_string()],
        symbol_whitelist: vec![],
        execution_order: 2,
    }));

    out.push(Arc::new(BlockType {
        name: "define".to_string(),
        symbol_blacklist: vec![],
        symbol_whitelist: vec!["i_core::datastore::var::let".to_string(), "i_core::value::new".to_string()],
        execution_order: 1,
    }));

    out
}
