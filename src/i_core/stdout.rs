// Path: i_core::stdout::*

use crate::{executers::ExcuterOutput, i_core::{datastore::var::Variable, value::Value}, symbolresolver::symbol_resolver::{Expression, SharedExpression}};

pub fn i_println(mut args: Vec<SharedExpression>) -> ExcuterOutput {
    if args.is_empty() {
        return ExcuterOutput::Error("`println` requires one argument".to_string());
    }

    let arg = args.remove(0);
    let arg = match arg.lock() {
        Ok(arg) => arg,
        Err(_) => return ExcuterOutput::Error("Could not lock `println` argument".to_string()),
    };

    if let Some(var) = arg.as_any().downcast_ref::<Variable>() {
        if let Some(value) = var.evaluate() {
            let value = match value.lock() {
                Ok(value) => value,
                Err(_) => return ExcuterOutput::Error("Could not lock variable value".to_string()),
            };

            if let Some(value) = value.display() {
                println!("{value}");
                return ExcuterOutput::ValidNone;
            }
        }
    } else if let Some(value) = arg.as_any().downcast_ref::<Value>() {
        if let Some(value) = value.display() {
            println!("{value}");
            return ExcuterOutput::ValidNone;
        }
    }

    ExcuterOutput::Error("`println` could not display its argument".to_string())
}
