// path: i_core::value::*

use crate::symbol_resolver::Expression;

enum Value {
    String(String),
}

impl Expression for Value {
    fn evaluate(self: Box<Self>) -> Option<Box<dyn Expression>> {
        None
    }

    fn display(self: Box<Self>) -> Option<String> {
        match *self {
            Value::String(s) => {
                return Some(s);
            }
        }
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
    }

    None
}