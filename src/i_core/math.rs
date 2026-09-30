// Path: i_core::stdout::*

use std::sync::{Arc, Mutex};

use crate::{executers::ExcuterOutput, i_core::value::Value, symbolresolver::symbol_resolver::SharedExpression};

pub fn i_inc_one(mut args: Vec<SharedExpression>) -> ExcuterOutput {
    if args.is_empty() {
        return ExcuterOutput::Error("`inc_one` requires one argument".to_string());
    }

    let arg = args.remove(0);

    let arg = match arg.lock() {
        Ok(arg) => arg,
        Err(_) => return ExcuterOutput::Error("Could not lock `inc_one` argument".to_string()),
    };

    if let Some(Value::Number(n)) = arg.as_any().downcast_ref::<Value>() {
        return ExcuterOutput::ValidSome(Arc::new(Mutex::new(Value::Number(n + 1))));
    }

    ExcuterOutput::Error("`inc_one` requires a number".to_string())
}
