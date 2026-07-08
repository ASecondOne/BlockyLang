use std::collections::HashMap;

#[derive(Clone)]
pub enum VarType {
    String, Number, Unknown
}

#[derive(Clone)]
struct Var {
    var_type: VarType
}

#[derive(Clone)]
pub struct VarMap {
    vars: HashMap<String, Var>
}

impl VarMap {
    pub fn new() -> Self {
        VarMap { 
            vars: HashMap::new() 
        }
    }

    pub fn add_new(&mut self, name: String, value: String) -> i32 {

        1
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