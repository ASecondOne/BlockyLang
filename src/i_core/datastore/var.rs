// PATH: i_core::datastore::var::*

use std::{collections::HashSet, sync::{Arc, Mutex}};

use crate::{executers::ExecuterOutput, i_core::{datastore::{access_modifiers::AccessModifiers, helpers::{get_value, get_variable, string_expression, take_first_argument}}, value::Value}, symbolresolver::{localstate::LocalState, symbol_resolver::{Expression, SharedExpression}}};

// //? Later needs access modifiers, traits and so on
#[derive(Debug, Clone)]
pub struct Variable {
    pub name: String,
    /// The immediate variable copy this value originated from.
    pub origin: String,
    /// Stable identity of the original declaration, used to find it through nested scopes.
    pub root: String,
    /// Identity of this particular copy in its current scope.
    pub identity: String,
    pub access_modifiers: HashSet<AccessModifiers>,
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

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }

    fn redirect(&mut self, new: SharedExpression) -> ExecuterOutput {
        let mut new = match new.lock() {
            Ok(new) => new,
            Err(_) => return ExecuterOutput::Error("Could not lock redirect value".to_string()),
        };

        if !self.access_modifiers.contains(&AccessModifiers::Mutabl) {

            if self.access_modifiers.contains(&AccessModifiers::OneTimeMutabl) {
                self.access_modifiers.remove(&AccessModifiers::OneTimeMutabl);
                
            } else {
                return ExecuterOutput::Error("Variable requires Mutabl access modifier to be changed".to_string());
            }
        } 

        if let Some(new_value) = new.as_any_mut().downcast_mut::<Value>() {
            if std::mem::discriminant(new_value) == std::mem::discriminant(&self.value) ||
                &self.value == &Value::default()
            {
                self.value = std::mem::take(new_value);
                return ExecuterOutput::ValidNone;
            } else {
                return ExecuterOutput::Error("The new value must match the old one".to_string());
            }
        }
        
        ExecuterOutput::Error("A variable can only receive a value".to_string())
    }
}

pub fn i_let(mut args: Vec<SharedExpression>, _local_state: &mut LocalState) -> ExecuterOutput {
    let Some(arg) = take_first_argument(&mut args) else {
        return ExecuterOutput::Error("`let` requires a variable name".to_string());
    };

    match get_variable(&arg) {
        Ok(Some(_)) => ExecuterOutput::ValidSome(arg),
        Ok(None) => ExecuterOutput::Error("`let` requires a variable name".to_string()),
        Err(message) => ExecuterOutput::Error(message),
    }
}

pub fn i_type(mut args: Vec<SharedExpression>, _local_state: &mut LocalState) -> ExecuterOutput {
    let Some(arg) = take_first_argument(&mut args) else {
        return ExecuterOutput::Error("`type` requires a variable name".to_string());
    };

    match get_variable(&arg) {
        Ok(Some(variable)) => type_matcher(&variable.value),
        Ok(None) => match get_value(&arg) {
            Ok(Some(value)) => type_matcher(&value),
            Ok(None) => ExecuterOutput::Error("`type` requires a variable or value".to_string()),
            Err(message) => ExecuterOutput::Error(message),
        },
        Err(message) => ExecuterOutput::Error(message),
    }
}

pub fn i_origin(mut args: Vec<SharedExpression>, _local_state: &mut LocalState) -> ExecuterOutput {
    let Some(arg) = take_first_argument(&mut args) else {
        return ExecuterOutput::Error("`origin` requires a variable name".to_string());
    };

    match get_variable(&arg) {
        Ok(Some(variable)) => ExecuterOutput::ValidSome(string_expression(variable.origin)),
        Ok(None) => ExecuterOutput::Error("`origin` requires a variable name".to_string()),
        Err(message) => ExecuterOutput::Error(message),
    }
}

pub fn i_transfer(mut args: Vec<SharedExpression>, local_state: &mut LocalState) -> ExecuterOutput {
    if args.len() != 1 {
        if args.is_empty() {
            return ExecuterOutput::Error("`transfer` requires one variable".to_string());
        }
        return ExecuterOutput::Error("`transfer` accepts one variable".to_string());
    }

    let arg = take_first_argument(&mut args).unwrap();

    match get_variable(&arg) {
        Ok(Some(variable)) => {
            if !variable.access_modifiers.contains(&AccessModifiers::Transferabl) {
                return ExecuterOutput::Error("Variable requires Transferabl access modifier to be transferred".to_string());
            }
        },
        Ok(None) => {
            return ExecuterOutput::Error("`transfer` requires a variable name".to_string());
        },
        Err(message) => {
            return ExecuterOutput::Error(message);
        },
    };

    match local_state.transfer(arg) {
        Ok(()) => ExecuterOutput::ValidNone,
        Err(message) => ExecuterOutput::Error(message),
    }
}

fn type_matcher(value: &Value) -> ExecuterOutput {
    match value {
            &Value::Boolean(_) => return ExecuterOutput::ValidSome(
                Arc::new(
                    Mutex::new(
                            Value::String(Some("Boolean".to_string()))
                        )
                    )
                ),

            &Value::String(_) => return ExecuterOutput::ValidSome(
                Arc::new(
                    Mutex::new(
                            Value::String(Some("String".to_string()))
                        )
                    )
                ),

            &Value::Number(_) => return ExecuterOutput::ValidSome(
                Arc::new(
                    Mutex::new(
                            Value::String(Some("Number".to_string()))
                        )
                    )
                ),

            &Value::Undefined => return ExecuterOutput::ValidSome(
                Arc::new(
                    Mutex::new(
                            Value::String(Some("Undefined".to_string()))
                        )
                    )
                ),
        }
}
