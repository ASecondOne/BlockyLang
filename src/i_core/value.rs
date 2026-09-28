// path: i_core::value::*

use std::sync::{Arc, Mutex};

use crate::symbol_resolver::{Expression, SharedExpression};

#[derive(Debug, Clone)]
pub enum Value {
    String(String),
    Number(i32),
    Boolean(bool),

    Undefined
}

impl Expression for Value {
    fn evaluate(&self) -> Option<SharedExpression> {
        Some(Arc::new(Mutex::new(self.clone())))
    }

    fn display(&self) -> Option<String> {
        match self {
            Value::String(s) => {
                return Some(s.clone());
            },
            Value::Number(s ) => {
                return Some(s.to_string())
            },
            Value::Boolean(s) => {
                return Some(s.to_string());
            }
            Value::Undefined => {
                return Some("Undefined".to_string());
            }
        }
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn redirect(&mut self, _: SharedExpression) -> crate::executers::ExcuterOutput {
        crate::executers::ExcuterOutput::Error("Cant redirect into Value".to_string())
    }
}

pub fn value_parser(input: &str) -> Option<SharedExpression> {
    if input.starts_with('"') && input.ends_with('"') {
        return Some(Arc::new(Mutex::new(Value::String(
            input.strip_prefix('"')
                .unwrap()
                .strip_suffix('"')
                .unwrap()
                .to_string()
        ))));
    } else if let Ok(num) = input.parse::<i32>() {
        return Some(Arc::new(Mutex::new(Value::Number(num))));
    } else if input == "true" || input == "false" {
        return Some(Arc::new(Mutex::new(Value::Boolean(
            match input {
                "true" => true,
                "false" => false,

                _ => false
            }
        ))));
    }

    None
}
