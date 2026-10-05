// Path: i_core::conditons::*

use crate::{executers::{ExcuterOutput, executer::execute_closure}, i_core::value::Value, symbolresolver::{localstate::LocalState, symbol_resolver::{Closure, SharedExpression}}};

pub fn i_if(mut args: Vec<SharedExpression>, local_state: &mut LocalState) -> ExcuterOutput {
    if args.len() != 2 {
        return ExcuterOutput::Error("`if` requires a condition and a closure".to_string());
    }

    let condition = args.remove(0);
    let condition = match condition.lock() {
        Ok(condition) => condition,
        Err(_) => return ExcuterOutput::Error("Could not lock `if` condition".to_string()),
    };

    let condition = match condition.as_any().downcast_ref::<Value>() {
        Some(Value::Boolean(condition)) => *condition,
        _ => return ExcuterOutput::Error("`if` condition has to be `true` or `false`".to_string()),
    };

    let closure = args.remove(0);
    let mut closure = match closure.lock() {
        Ok(closure) => closure,
        Err(_) => return ExcuterOutput::Error("Could not lock `if` closure".to_string()),
    };

    let Some(closure) = closure.as_any_mut().downcast_mut::<Closure>() else {
        return ExcuterOutput::Error("`if` requires a closure as its second argument".to_string());
    };

    if condition {
        return execute_closure(closure, local_state);
    }

    ExcuterOutput::ValidNone
}
