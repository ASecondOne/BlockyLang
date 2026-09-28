// Path: i_core::stdout::*

use std::sync::{Arc, Mutex};

use crate::{executers::ExcuterOutput, i_core::value::Value, symbol_resolver::SharedExpression};

pub fn inc_one(mut args: Vec<SharedExpression>) -> ExcuterOutput {
    let arg = args.remove(0);

    if let Some(Value::Number(n)) = arg.lock().unwrap().as_any().downcast_ref::<Value>() {
        return ExcuterOutput::ValidSome(Arc::new(Mutex::new(Value::Number(n + 1))));
    }

    ExcuterOutput::ValidNone
}
