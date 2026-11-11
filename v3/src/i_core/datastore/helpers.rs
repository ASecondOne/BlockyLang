use std::sync::{Arc, Mutex};

use crate::{
    i_core::{datastore::var::Variable, value::Value},
    symbolresolver::symbol_resolver::SharedExpression,
};

pub fn take_first_argument(args: &mut Vec<SharedExpression>) -> Option<SharedExpression> {
    if args.is_empty() {
        None
    } else {
        Some(args.remove(0))
    }
}

pub fn get_variable(expression: &SharedExpression) -> Result<Option<Variable>, String> {
    let expression = expression
        .lock()
        .map_err(|_| "Could not lock expression".to_string())?;
    Ok(expression.as_any().downcast_ref::<Variable>().cloned())
}

pub fn update_variable(expression: &SharedExpression, update: impl FnOnce(&mut Variable)) -> Result<bool, String> {
    let mut expression = expression
        .lock()
        .map_err(|_| "Could not lock variable".to_string())?;

    let Some(variable) = expression.as_any_mut().downcast_mut::<Variable>() else {
        return Ok(false);
    };
    update(variable);
    Ok(true)
}

pub fn get_value(expression: &SharedExpression) -> Result<Option<Value>, String> {
    let expression = expression
        .lock()
        .map_err(|_| "Could not lock expression".to_string())?;
    Ok(expression.as_any().downcast_ref::<Value>().cloned())
}

pub fn string_expression(value: impl Into<String>) -> SharedExpression {
    Arc::new(Mutex::new(Value::String(Some(value.into()))))
}
