use std::collections::HashMap;

#[derive(Clone)]
pub enum VarType {
    String, Number, Unknown
}

#[derive(Clone)]
struct Var {
    var_type: VarType,
    value: (Option<String>, Option<f32>),
}

#[derive(Clone)]
pub struct VarMap {
    vars: HashMap<String, Var>
}

impl Var {
    fn as_string(&self) -> String {
        if let Some(s) = &self.value.0 {
            s.clone().strip_prefix('"').unwrap().strip_suffix('"').unwrap().to_string()
        } else if let Some(n) = self.value.1 {
            n.to_string()
        } else {
            String::new()
        }
    }
}

impl VarMap {
    pub fn new() -> Self {
        VarMap { 
            vars: HashMap::new() 
        }
    }

    pub fn add_new(&mut self, name: String, value: String) {
        let var_type = parse_type(&value);

        let value = match var_type {
            VarType::Number => (None, Some(value.parse::<f32>().unwrap())),
            _ => (Some(value), None),
        };

        self.vars.insert(name, Var {
            var_type,
            value,
        });
    }

    pub fn get_var(&self, name: String) -> Option<String> {
        if let Some(found) = self.vars.get(&name) {
            return Some(found.as_string().clone());
        }

        None
    }
}

pub fn parse_type(value: &str) -> VarType {
    let value = value.trim();

    if value.starts_with('"') && value.ends_with('"') {
        VarType::String
    } else if value.parse::<f64>().is_ok() {
        VarType::Number
    } else {
        VarType::Unknown
    }
}