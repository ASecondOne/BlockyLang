// PATH: i_core::datastore::*

use std::{collections::HashSet, sync::{Arc, LazyLock, Mutex}};

use crate::{i_core::datastore::var::Variable, symbol_resolver::SharedExpression};

pub mod var;

pub static ASSUMEND_VARIABLES: LazyLock<Mutex<HashSet<String>>> = LazyLock::new(|| Mutex::new(HashSet::new()));

pub static VARIABLES: LazyLock<Mutex<Vec<SharedExpression>>> = LazyLock::new(|| Mutex::new(Vec::new()));

pub fn variable_parse(input: &str) -> Option<SharedExpression> {

    let var = Variable {
        name: input.to_string(),
        value: super::value::Value::Undefined,
    };

    ASSUMEND_VARIABLES.lock().unwrap().insert(input.to_string());

    Some(Arc::new(Mutex::new(var)))
}
