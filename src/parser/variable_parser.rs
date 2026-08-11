use crate::parser::{Expression, library::Output, value_parser::{Value, parse_value}};

#[derive(Debug, Clone)]
pub struct Variable {
    value: Value,
    name: String,
}

impl Variable {
    pub fn get_value(&mut self) -> Option<Value> {
        Some(self.value.clone())
    }
}

pub struct VariableMap {
    variables: Vec<Variable>
}

impl VariableMap {
    pub fn new() -> VariableMap {
        VariableMap { variables: Vec::new() }
    }

    pub fn add_new(&mut self, new_var: Variable) {
        self.variables.push(new_var);
    }

    pub fn add_new_variable(&mut self, variable_name: String, expression: Expression) -> Result<Output, ()> {
        if let Some((i, _)) = self.variables.iter().enumerate().find(|(_, v)| v.name == variable_name) {
            self.variables.remove(i-1);
        }

        match expression {
            Expression::Value(v) => {
                self.variables.push(
                    Variable { value: v, name: variable_name }
                );
                return Ok(Output::Success);
            },
            _ => {
                return Err(());
            }
        }
    }

    pub fn get_var(&self, variable_name: String) -> Option<Expression> {
        if let Some(v) = self.variables.iter().find(|v| v.name == variable_name) {
            return Some(Expression::Variable(v.clone()));
        }

        None
    }

}

pub fn parse_variable_expression(expression: String) -> Option<Expression> {

    let mut potential_name = String::new();
    let mut potential_value = String::new();

    let mut past= false;

    for char in expression.chars() {
        if char == '=' {
            past = true;
            continue;
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