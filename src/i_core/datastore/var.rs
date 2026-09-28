// PATH: i_core::datastore::var::*

use std::sync::{Arc, Mutex};

use crate::{executers::ExcuterOutput, i_core::value::Value, symbol_resolver::{Expression, SharedExpression}};

// //? Later needs access modifirers, traits and so on
#[derive(Debug, Clone)]
pub struct Variable {
    pub name: String,
    pub value: Value,
}

impl Expression for Variable {
    fn evaluate(&self) -> Option<SharedExpression> {
        Some(Arc::new(Mutex::new(self.value.clone())))
    }

    fn display(&self) -> Option<String> {
        Some(self.name.clone())
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn redirect(&mut self, new: SharedExpression) -> ExcuterOutput {
        let new = match new.lock() {
            Ok(new) => new,
            Err(_) => return ExcuterOutput::Error("Could not lock redirect value".to_string()),
        };

        if let Some(new_value) = new.as_any().downcast_ref::<Value>() {
            self.value = new_value.clone(); // //! Somehow get rid of clone
            return ExcuterOutput::ValidNone;
        }
        
        ExcuterOutput::Error("A variable can only receive a value".to_string())
    }
}

pub fn i_let(mut args: Vec<SharedExpression>) -> ExcuterOutput {
    if args.is_empty() {
        return ExcuterOutput::Error("`let` requires a variable name".to_string());
    }

    let arg = args.remove(0);

    let is_variable = arg
        .lock()
        .ok()
        .is_some_and(|arg| arg.as_any().downcast_ref::<Variable>().is_some());

    if is_variable {
        return ExcuterOutput::ValidSome(arg);
    }

    ExcuterOutput::Error("`let` requires a variable name".to_string())
}
