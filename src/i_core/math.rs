// Path: i_core::stdout::*

use crate::{executers::ExcuterOutput, i_core::value::Value, symbol_resolver::Expression};

pub fn inc_one(mut args: Vec<Box<dyn Expression>>) -> ExcuterOutput {
    let arg = args.remove(0);

    if let Some(Value::Number(n)) = arg.as_any().downcast_ref::<Value>() {
        return ExcuterOutput::ValidSome(Box::new(Value::Number(n + 1)));
    }

    ExcuterOutput::ValidNone
}