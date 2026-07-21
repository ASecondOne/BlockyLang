use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    String(String),
    Number(f64),
    Undefined
}

#[derive(Clone)]
struct Var {
    value: Value,
}

#[derive(Clone)]
pub struct VarMap {
    vars: HashMap<String, Var>
}

impl Default for VarMap {
    fn default() -> Self {
        Self::new()
    }
}

impl Var {
    fn as_string(&self) -> Option<String> {
        match &self.value {
            Value::Number(n) => Some(n.to_string()),
            Value::String(s) => Some(s.to_string()),
            Value::Undefined => None,
        }
    }

    fn get_value(&self) -> &Value {
        return &self.value;
    }
}

impl VarMap {
    pub fn new() -> Self {
        VarMap { 
            vars: HashMap::new() 
        }
    }

    pub fn add_new(&mut self, name: String, value: String, undefined: bool) -> Result<(), String> {
        match parse_type(&value, undefined) {
            Ok(v) => {
                self.vars.insert(name, Var { value: v });
                Ok(())
            }
            Err(msg) => Err(msg)
        }
    }

    pub fn get_var(&self, name: String) -> Option<(String, bool)> {
        if let Some(found) = self.vars.get(&name) {
            return match found.get_value() {
                Value::Undefined => Some(("".to_string(), true)),
                _ => Some((found.as_string().unwrap(), false))
            };
        }

        None
    }

    pub fn var_exists(&self, name: &String) -> bool {
        self.vars.contains_key(name)
    }
}

pub fn parse_type(value: &str, undefined: bool) -> Result<Value, String> {
    let value = value.trim();

    if undefined {
        return Ok(Value::Undefined);
    }

    if value.starts_with('"') && value.ends_with('"') {
        if let Some(inner) = value
            .strip_prefix('"')
            .and_then(|inner| inner.strip_suffix('"'))
        {
            return Ok(Value::String(inner.to_string()));
        }

        Err("Something up with ya String".to_string())
    } else if value.parse::<f64>().is_ok() {
        Ok(Value::Number(value.parse::<f64>().unwrap()))
    } else {
        Err("Unknown data Type".to_string())
    }
}