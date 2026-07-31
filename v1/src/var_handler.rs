use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    String(String),
    Number(f64),
    Bool(bool),
    Undefined
}

#[derive(Clone)]
pub struct Var {
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
            Value::Bool(b) => {
                if *b { return Some("true".to_string()) }
                Some("false".to_string())
            }
            Value::Undefined => None,
        }
    }

    pub fn get_value(&self) -> &Value {
        return &self.value;
    }
}

impl VarMap {
    pub fn new() -> Self {
        VarMap { 
            vars: HashMap::new() 
        }
    }

    pub fn replace_value(&mut self, old_value: String, new_value: Value) {
        self.vars.insert(old_value, Var { value: new_value });
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

    pub fn get_pure_value(&self, name: String) -> Option<Value> {
        if let Some(found) = self.vars.get(&name) {
           return Some(found.value.clone());
        }

        None
    }

    pub fn get_pure_var(&self, name: String) -> Option<Var> {
        if let Some(found) = self.vars.get(&name) {
           return Some(found.clone());
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

    if value == "true" {
        return Ok(Value::Bool(true))
    }

    if value == "false" {
        return Ok(Value::Bool(false))
    }

    if value.starts_with('"') && value.ends_with('"') {
        if let Some(inner) = value
            .strip_prefix('"')
            .and_then(|inner| inner.strip_suffix('"'))
        {
            return Ok(Value::String(inner.to_string()));
        }

        return Err("Something up with ya String".to_string())
    }
    
    if value.parse::<f64>().is_ok() {
        return Ok(Value::Number(value.parse::<f64>().unwrap()))
    } 
    
    Err("Unknown data Type".to_string())
}