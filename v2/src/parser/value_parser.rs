use core::fmt;
use std::{fmt::{Display, Formatter}, println};

#[derive(Debug, Clone)]
pub enum Value {
    String(String),
    Boolean(bool),
    Number(f64),
    Undefined,
}

impl Display for Value {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Value::String(s) => write!(f, "{s}"),
            Value::Number(n) => write!(f, "{n}"),
            Value::Boolean(b) => write!(f, "{b}"),
            Value::Undefined => write!(f, "undefined"),
        }
    }
}

pub fn parse_value(s: String) -> Option<Value> {

    if s == "undefined" {
        return Some(Value::Undefined);
    } else if s.starts_with('"') && s.ends_with('"') {
        let out = s.strip_prefix('"').unwrap().strip_suffix('"').unwrap();

        return Some(Value::String(out.to_string()));
    } else if let Ok(value) = s.parse::<f64>() {
        return Some(Value::Number(value));
    } else if let Ok(value) = s.parse::<bool>() {
        return Some(Value::Boolean(value));
    }

    None
}
