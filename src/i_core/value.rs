// path: i_core::value::*

use crate::symbol_resolver::Expression;

#[derive(Debug)]
pub enum Value {
    String(String),
    Number(i32),
    Boolean(bool),
}

impl Expression for Value {
    fn evaluate(self: Box<Self>) -> Option<Box<dyn Expression>> {
        Some(self)
    }

    fn display(self: Box<Self>) -> Option<String> {
        match *self {
            Value::String(s) => {
                return Some(s);
            },
            Value::Number(s ) => {
                return Some(s.to_string())
            },
            Value::Boolean(s) => {
                return Some(s.to_string());
            }
        }
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

pub fn expression_parser(input: &str) -> Option<Box<dyn Expression>> {
    if input.starts_with('"') && input.ends_with('"') {
        return Some(Box::new(Value::String(
            input.strip_prefix('"')
                .unwrap()
                .strip_suffix('"')
                .unwrap()
                .to_string()
        )));
    } else if let Ok(num) = input.parse::<i32>() {
        return Some(Box::new(Value::Number(num)));
    } else if input == "true" || input == "false" {
        return Some(Box::new(Value::Boolean(
            match input {
                "true" => true,
                "false" => false,

                _ => false
            }
        )));
    }

    None
}