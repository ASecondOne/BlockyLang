use std::collections::HashMap;

#[derive(Clone)]
pub enum Value {
    String(String),
    Number(f64),
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
    fn as_string(&self) -> String {
        match &self.value {
            Value::Number(n) => n.to_string(),
            Value::String(s) => s.to_string(),
        }
    }
}

impl VarMap {
    pub fn new() -> Self {
        VarMap { 
            vars: HashMap::new() 
        }
    }

    pub fn add_new(&mut self, name: String, value: String) -> Result<(), String> {
        match parse_type(&value) {
            Ok(v) => {
                self.vars.insert(name, Var { value: v });
                Ok(())
            }
            Err(msg) => Err(msg)
        }
    }

    pub fn get_var(&self, name: String) -> Option<String> {
        if let Some(found) = self.vars.get(&name) {
            return Some(found.as_string().clone());
        }

        None
    }

    pub fn var_exists(&self, name: &String) -> bool {
        self.vars.contains_key(name)
    }
}

pub fn parse_type(value: &str) -> Result<Value, String> {
    let value = value.trim();

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