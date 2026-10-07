// path: i_core::value::*

use std::sync::{Arc, Mutex};

use crate::{executers::ExcuterOutput, i_core::datastore::helpers::take_first_argument, symbolresolver::{localstate::LocalState, symbol_resolver::{Expression, SharedExpression}}};

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub enum Value {
    String(Option<String>),
    Number(Option<i32>),
    Boolean(Option<bool>),

    #[default]
    Undefined
}

impl Expression for Value {
    fn evaluate(&self) -> Option<SharedExpression> {
        Some(Arc::new(Mutex::new(self.clone())))
    }

    fn display(&self) -> Option<String> {
        match self {
            Value::String(s) => {
                if s.is_none() { return Some("Undefined".to_string()); }
                return Some(s.clone().unwrap());
            },
            Value::Number(s ) => {
                if s.is_none() { return Some("Undefined".to_string()); }
                return Some(s.unwrap().to_string())
            },
            Value::Boolean(s) => {
                if s.is_none() { return Some("Undefined".to_string()); }
                return Some(s.unwrap().to_string());
            }
            Value::Undefined => {
                return Some("Undefined".to_string());
            }
        }
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }

    fn redirect(&mut self, _: SharedExpression) -> crate::executers::ExcuterOutput {
        crate::executers::ExcuterOutput::Error("Cannot redirect into a value".to_string())
    }
}

pub fn value_parser(input: &str) -> Option<SharedExpression> {
    if input.starts_with('"') && input.ends_with('"') {
        return Some(Arc::new(Mutex::new(Value::String(
            Some(input.strip_prefix('"')
            .unwrap()
            .strip_suffix('"')
            .unwrap()
            .to_string())
        ))));
    } else if let Ok(num) = input.parse::<i32>() {
        return Some(Arc::new(Mutex::new(Value::Number(Some(num)))));
    } else if input == "true" || input == "false" {
        return Some(Arc::new(Mutex::new(Value::Boolean(
            match input {
                "true" => Some(true),
                "false" => Some(false),

                _ => Some(false)
            }
        ))));
    }

    None
}

pub fn i_new(mut args: Vec<SharedExpression>, _local_state: &mut LocalState) -> ExcuterOutput {
    let Some(arg) = take_first_argument(&mut args) else {
        return ExcuterOutput::Error("`new` requires one argument".to_string());
    };

    let guard = arg.lock().unwrap();

    if let Some(value) = guard.as_any().downcast_ref::<Value>() {
        return match &value {
            &Value::String(s) => {

                if s.is_none() {
                    return ExcuterOutput::Error("Value has to be set for this operation".to_string());
                }

                let out = match s.clone().unwrap().to_lowercase().as_str() {
                    "string" => Value::String(None),
                    "number" => Value::Number(None),
                    "bool" => Value::Boolean(None),
                    _ => Value::Undefined
                };


                ExcuterOutput::ValidSome(Arc::new(Mutex::new(out)))
            },

            _ => ExcuterOutput::Error("requires String".to_string())
        }
    }

    ExcuterOutput::ValidNone
}
