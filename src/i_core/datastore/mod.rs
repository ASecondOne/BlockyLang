// PATH: i_core::datastore::*

use std::{collections::HashSet, sync::{Arc, LazyLock, Mutex}};

use crate::{i_core::datastore::var::Variable, symbolresolver::symbol_resolver::SharedExpression};

pub mod var;
pub mod access_modifires;
pub mod helpers;

pub static ASSUMEND_VARIABLES: LazyLock<Mutex<HashSet<String>>> = LazyLock::new(|| Mutex::new(HashSet::new()));

pub static VARIABLES: LazyLock<Mutex<Vec<SharedExpression>>> = LazyLock::new(|| Mutex::new(Vec::new()));

pub fn variable_parse(input: &str) -> Option<SharedExpression> {
    let mut variables = VARIABLES.lock().unwrap();

    for variable in variables.iter() {
        let is_match = variable
            .lock()
            .ok()
            .and_then(|variable| {
                variable
                    .as_any()
                    .downcast_ref::<Variable>()
                    .map(|variable| variable.name == input)
            })
            .unwrap_or(false);

        if is_match {
            return Some(Arc::clone(variable));
        }
    }

    let mut var = Variable {
        name: input.to_string(),
        origin: String::new(),
        root: String::new(),
        identity: String::new(),
        access_modifires: HashSet::new(),
        value: super::value::Value::Undefined,
    };

    // * Needed so the first redirect can cautomatically set the value type
    var.access_modifires.insert(access_modifires::AccessModifires::OneTimeMutabl);

    let var: SharedExpression = Arc::new(Mutex::new(var));

    ASSUMEND_VARIABLES.lock().unwrap().insert(input.to_string());
    variables.push(Arc::clone(&var));

    Some(var)
}
