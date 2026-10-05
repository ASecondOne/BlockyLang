// Path: i_core::stdout::*

use std::sync::{Arc, Mutex};

use crate::{executers::ExcuterOutput, i_core::{datastore::var::Variable, value::Value}, symbolresolver::{localstate::LocalState, symbol_resolver::SharedExpression}};

pub fn i_inc_one(mut args: Vec<SharedExpression>, _local_state: &mut LocalState) -> ExcuterOutput {
    if args.is_empty() {
        return ExcuterOutput::Error("`inc_one` requires one argument".to_string());
    }

    let arg = args.remove(0);

    let arg = match arg.lock() {
        Ok(arg) => arg,
        Err(_) => return ExcuterOutput::Error("Could not lock `inc_one` argument".to_string()),
    };

    //? Variables get their value read first, values are used directly
    let value = if let Some(var) = arg.as_any().downcast_ref::<Variable>() {
        &var.value
    } else if let Some(value) = arg.as_any().downcast_ref::<Value>() {
        value
    } else {
        return ExcuterOutput::Error("`inc_one` requires a number or a variable".to_string());
    };

    if let Value::Number(n) = value {
        return ExcuterOutput::ValidSome(Arc::new(Mutex::new(Value::Number(n + 1))));
    }

    ExcuterOutput::Error("`inc_one` requires a number".to_string())
}
