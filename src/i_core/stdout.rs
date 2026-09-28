// Path: i_core::stdout::*

use crate::{executers::ExcuterOutput, i_core::{datastore::var::Variable, value::Value}, symbol_resolver::{Expression, SharedExpression}};

pub fn println(mut args: Vec<SharedExpression>) -> ExcuterOutput {
    let arg = args.remove(0);

    if let Some(var) = arg.lock().unwrap().as_any().downcast_ref::<Variable>() {
        if let Some(value) = var.evaluate() {
            println!("{}", value.lock().unwrap().display().unwrap());
        }
    } else if let Some(value) = arg.lock().unwrap().as_any().downcast_ref::<Value>() {
        println!("{}", value.display().unwrap())
    }

    ExcuterOutput::ValidNone
}
