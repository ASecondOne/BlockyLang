// PATH: i_core::datastore::var::*

use std::sync::{Arc, Mutex};

use crate::{executers::ExcuterOutput, i_core::{datastore::VARIABLES, value::Value}, symbol_resolver::{Expression, SharedExpression}};

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
        ExcuterOutput::ValidNone
    }
}

pub fn i_let(mut args: Vec<SharedExpression>) -> ExcuterOutput {
    let arg = args.remove(0);

    if let Some(var) = arg.lock().unwrap().as_any().downcast_ref::<Variable>() {
        let var = Variable {
            name: var.name.to_string(),
            value: Value::Undefined
        };

        let var: SharedExpression = Arc::new(Mutex::new(var));

        VARIABLES.lock().unwrap().push(Arc::clone(&var));

        return ExcuterOutput::ValidSome(var);
    }

    ExcuterOutput::ValidNone
}
