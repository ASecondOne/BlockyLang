// PATH: i_core::datastore::var::*

use std::{collections::HashSet, sync::{Arc, Mutex}};

use crate::{executers::ExcuterOutput, i_core::{datastore::{access_modifires::AccessModifires, helpers::{get_value, get_variable, string_expression, take_first_argument}}, value::Value}, symbolresolver::{localstate::LocalState, symbol_resolver::{Expression, SharedExpression}}};

// //? Later needs access modifirers, traits and so on
#[derive(Debug, Clone)]
pub struct Variable {
    pub name: String,
    /// The immediate variable copy this value originated from.
    pub origin: String,
    /// Stable identity of the original declaration, used to find it through nested scopes.
    pub root: String,
    /// Identity of this particular copy in its current scope.
    pub identity: String,
    pub access_modifires: HashSet<AccessModifires>,
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

    fn redirect(&mut self, new: SharedExpression) -> ExcuterOutput {
        let mut new = match new.lock() {
            Ok(new) => new,
            Err(_) => return ExcuterOutput::Error("Could not lock redirect value".to_string()),
        };

        if !self.access_modifires.contains(&AccessModifires::Mutabl) {

            if self.access_modifires.contains(&AccessModifires::OneTimeMutabl) {
                self.access_modifires.remove(&AccessModifires::OneTimeMutabl);
                
            } else {
                return ExcuterOutput::Error("Variable requires Mutabl access modifier to be changed".to_string());
            }
        } 

        if let Some(new_value) = new.as_any_mut().downcast_mut::<Value>() {
            if std::mem::discriminant(new_value) == std::mem::discriminant(&self.value) ||
                &self.value == &Value::default()
            {
                self.value = std::mem::take(new_value);
                return ExcuterOutput::ValidNone;
            } else {
                return ExcuterOutput::Error("The new value must match the old one".to_string());
            }
        }
        
        ExcuterOutput::Error("A variable can only receive a value".to_string())
    }
}

pub fn i_let(mut args: Vec<SharedExpression>, _local_state: &mut LocalState) -> ExcuterOutput {
    let Some(arg) = take_first_argument(&mut args) else {
        return ExcuterOutput::Error("`let` requires a variable name".to_string());
    };

    match get_variable(&arg) {
        Ok(Some(_)) => ExcuterOutput::ValidSome(arg),
        Ok(None) => ExcuterOutput::Error("`let` requires a variable name".to_string()),
        Err(message) => ExcuterOutput::Error(message),
    }
}

pub fn i_type(mut args: Vec<SharedExpression>, _local_state: &mut LocalState) -> ExcuterOutput {
    let Some(arg) = take_first_argument(&mut args) else {
        return ExcuterOutput::Error("`type` requires a variable name".to_string());
    };

    match get_variable(&arg) {
        Ok(Some(variable)) => type_matcher(&variable.value),
        Ok(None) => match get_value(&arg) {
            Ok(Some(value)) => type_matcher(&value),
            Ok(None) => ExcuterOutput::Error("`type` requires a variable or value".to_string()),
            Err(message) => ExcuterOutput::Error(message),
        },
        Err(message) => ExcuterOutput::Error(message),
    }
}

pub fn i_origin(mut args: Vec<SharedExpression>, _local_state: &mut LocalState) -> ExcuterOutput {
    let Some(arg) = take_first_argument(&mut args) else {
        return ExcuterOutput::Error("`origin` requires a variable name".to_string());
    };

    match get_variable(&arg) {
        Ok(Some(variable)) => ExcuterOutput::ValidSome(string_expression(variable.origin)),
        Ok(None) => ExcuterOutput::Error("`origin` requires a variable name".to_string()),
        Err(message) => ExcuterOutput::Error(message),
    }
}

pub fn i_transfer(mut args: Vec<SharedExpression>, local_state: &mut LocalState) -> ExcuterOutput {
    if args.len() != 1 {
        if args.is_empty() {
            return ExcuterOutput::Error("`transfer` requires one variable".to_string());
        }
        return ExcuterOutput::Error("`transfer` accepts one variable".to_string());
    }

    let arg = take_first_argument(&mut args).unwrap();
    match local_state.transfer(arg) {
        Ok(()) => ExcuterOutput::ValidNone,
        Err(message) => ExcuterOutput::Error(message),
    }
}

fn type_matcher(value: &Value) -> ExcuterOutput {
    match value {
            &Value::Boolean(_) => return ExcuterOutput::ValidSome(
                Arc::new(
                    Mutex::new(
                            Value::String(Some("Boolean".to_string()))
                        )
                    )
                ),

            &Value::String(_) => return ExcuterOutput::ValidSome(
                Arc::new(
                    Mutex::new(
                            Value::String(Some("String".to_string()))
                        )
                    )
                ),

            &Value::Number(_) => return ExcuterOutput::ValidSome(
                Arc::new(
                    Mutex::new(
                            Value::String(Some("Number".to_string()))
                        )
                    )
                ),

            &Value::Undefined => return ExcuterOutput::ValidSome(
                Arc::new(
                    Mutex::new(
                            Value::String(Some("Undefined".to_string()))
                        )
                    )
                ),
        }
}
