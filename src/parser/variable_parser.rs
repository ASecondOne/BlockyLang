use crate::parser::{Expression, value_parser::{Value, parse_value}};

#[derive(Debug, Clone)]
pub struct Variable {
    value: Value,
    name: String,
}

pub struct VariableMap {
    variables: Vec<Variable>
}

impl Variable {
    pub fn new(value: Value, name: String) -> Variable {
        Variable { value, name }
    }
}

impl VariableMap {
    pub fn new() -> VariableMap {
        VariableMap { variables: Vec::new() }
    }

    pub fn add_new(&mut self, new_var: Variable) {
        self.variables.push(new_var);
    }
}

pub fn parse_variable(expression: String) -> Option<Expression> {

    let mut potential_name = String::new();
    let mut potential_value = String::new();

    let mut past= false;

    for char in expression.chars() {
        if char == '=' {
            past = true;
        } 
        
        if past {
            potential_value.push(char);
            continue;
        }

        potential_name.push(char);
    }

    if let Some(value) = parse_value(potential_value.trim().to_string()) {
        return Some(Expression::VariableDefinition((
            potential_name.trim().to_string(), 
            Box::new(
                Expression::Value(value)
            )
        )));
    }

    None
}