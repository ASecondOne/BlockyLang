// Path: i_core::conditons::*

use crate::{executers::{ExecuterOutput, executer::execute_closure}, i_core::value::Value, symbolresolver::{localstate::LocalState, symbol_resolver::{Closure, SharedExpression}}};

pub fn i_if(mut args: Vec<SharedExpression>, local_state: &mut LocalState) -> ExecuterOutput {
    if args.len() != 2 {
        return ExecuterOutput::Error("`if` requires a condition and a closure".to_string());
    }

    let condition = args.remove(0);
    let condition = match condition.lock() {
        Ok(condition) => condition,
        Err(_) => return ExecuterOutput::Error("Could not lock `if` condition".to_string()),
    };

    let condition = match condition.as_any().downcast_ref::<Value>() {
        Some(Value::Boolean(condition)) => *condition,
        _ => return ExecuterOutput::Error("`if` condition has to be `true` or `false`".to_string()),
    };

    let closure = args.remove(0);
    let mut closure = match closure.lock() {
        Ok(closure) => closure,
        Err(_) => return ExecuterOutput::Error("Could not lock `if` closure".to_string()),
    };

    let Some(closure) = closure.as_any_mut().downcast_mut::<Closure>() else {
        return ExecuterOutput::Error("`if` requires a closure as its second argument".to_string());
    };

    if condition.is_none() {
        return ExecuterOutput::Error("Value has to be set for this operation".to_string());
    }

    if condition.unwrap() {
        return execute_closure(closure, local_state);
    }

    ExecuterOutput::ValidNone
}
