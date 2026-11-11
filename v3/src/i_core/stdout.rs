// Path: i_core::stdout::*

use crate::{executers::ExecuterOutput, i_core::{datastore::var::Variable, value::Value}, symbolresolver::{localstate::LocalState, symbol_resolver::{Expression, SharedExpression}}};

pub fn i_println(mut args: Vec<SharedExpression>, _local_state: &mut LocalState) -> ExecuterOutput {
    if args.is_empty() {
        return ExecuterOutput::Error("`println` requires one argument".to_string());
    }

    let arg = args.remove(0);
    let arg = match arg.lock() {
        Ok(arg) => arg,
        Err(_) => return ExecuterOutput::Error("Could not lock `println` argument".to_string()),
    };

    if let Some(var) = arg.as_any().downcast_ref::<Variable>() {
        if let Some(value) = var.evaluate() {
            let value = match value.lock() {
                Ok(value) => value,
                Err(_) => return ExecuterOutput::Error("Could not lock variable value".to_string()),
            };

            if let Some(value) = value.display() {
                println!("{value}");
                return ExecuterOutput::ValidNone;
            }
        }
    } else if let Some(value) = arg.as_any().downcast_ref::<Value>() {
        if let Some(value) = value.display() {
            println!("{value}");
            return ExecuterOutput::ValidNone;
        }
    }

    ExecuterOutput::Error("`println` could not display its argument".to_string())
}

pub fn i_print(mut args: Vec<SharedExpression>, _local_state: &mut LocalState) -> ExecuterOutput {
    if args.is_empty() {
        return ExecuterOutput::Error("`println` requires one argument".to_string());
    }

    let arg = args.remove(0);
    let arg = match arg.lock() {
        Ok(arg) => arg,
        Err(_) => return ExecuterOutput::Error("Could not lock `println` argument".to_string()),
    };

    if let Some(var) = arg.as_any().downcast_ref::<Variable>() {
        if let Some(value) = var.evaluate() {
            let value = match value.lock() {
                Ok(value) => value,
                Err(_) => return ExecuterOutput::Error("Could not lock variable value".to_string()),
            };

            if let Some(value) = value.display() {
                print!("{value}");
                return ExecuterOutput::ValidNone;
            }
        }
    } else if let Some(value) = arg.as_any().downcast_ref::<Value>() {
        if let Some(value) = value.display() {
            print!("{value}");
            return ExecuterOutput::ValidNone;
        }
    }

    ExecuterOutput::Error("`println` could not display its argument".to_string())
}
