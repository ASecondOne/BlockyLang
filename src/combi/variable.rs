use std::vec;

use crate::{
    combi::library::Output, parser::{
        Expression, conditional_parser::condition_parse, dot_notation_parser::{self, parse_dot_notation}, parse_lines::parse_lines, value_parser::{Value, parse_value},
    },
};

#[derive(Debug, Clone)]
pub struct Variable {
    value: Expression,
    pub var_type: VariableType,
    name: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VariableType {
    String,
    Boolean,
    Number,
    Undefined,
}

impl Variable {
    pub fn get_value(&mut self) -> Expression {
        self.value.clone()
    }

    pub fn set_value(&mut self, value: Expression) -> Result<(), ()> {
        let value_type = expression_type(&value).ok_or(())?;

        match (&self.var_type, &value_type) {
            (VariableType::Undefined, VariableType::Undefined) => {}
            (VariableType::Undefined, _) => self.var_type = value_type,
            (_, VariableType::Undefined) => {}
            (variable_type, value_type) if variable_type == value_type => {}
            _ => return Err(()),
        }

        self.value = value;
        Ok(())
    }
}

fn expression_type(expression: &Expression) -> Option<VariableType> {
    match expression {
        Expression::Value(Value::Boolean(_)) => Some(VariableType::Boolean),
        Expression::Value(Value::String(_)) => Some(VariableType::String),
        Expression::Value(Value::Number(_)) => Some(VariableType::Number),
        Expression::Value(Value::Undefined) => Some(VariableType::Undefined),
        _ => None,
    }
}

pub struct VariableMap {
    variables: Vec<Variable>,
}

impl Default for VariableMap {
    fn default() -> Self {
        Self::new()
    }
}

impl VariableMap {
    pub fn new() -> VariableMap {
        VariableMap {
            variables: Vec::new(),
        }
    }

    pub fn add_new(&mut self, new_var: Variable) {
        self.variables.push(new_var);
    }

    pub fn add_new_variable(
        &mut self,
        variable_name: String,
        expression: Expression,
        var_type: VariableType,
    ) -> Result<Output, ()> {
        let mut variable = Variable {
            value: Expression::Value(Value::Undefined),
            var_type,
            name: variable_name.clone(),
        };
        variable.set_value(expression)?;

        if let Some((i, _)) = self
            .variables
            .iter()
            .enumerate()
            .find(|(_, v)| v.name == variable_name)
        {
            self.variables.remove(i);
        }

        self.variables.push(variable);
        Ok(Output::Success)
    }

    pub fn get_var(&self, variable_name: &str) -> Option<Expression> {
        self.variables
            .iter()
            .find(|variable| variable.name == variable_name)?;

        Some(Expression::Variable(variable_name.to_string()))
    }

    pub fn get_pure_var(&mut self, variable_name: &str) -> Option<&mut Variable> {
        if let Some(v) = self.variables.iter_mut().find(|v| v.name == variable_name) {
            return Some(v);
        }

        None
    }
}

pub fn parse_variable_expression(expression: String) -> Option<Expression> {
    let mut past = false;

    let parts = expression.split_whitespace();

    let mut var_type: Option<VariableType> = None;
    let mut var_name: Option<String> = None;
    let mut var_value: Option<Value> = None;

    for (i, part) in parts.enumerate().map(|(i, f)| (i, f.trim())) {
        if let Some(v) = parse_string_to_var_type(part)
            && i == 0
        {
            var_type = Some(v);
            continue;
        }

        if (i == 1 || i == 0) && var_name.is_none() {
            var_name = Some(part.to_string());
            continue;
        }

        if (i == 2 || i == 1) && part == "=" {
            past = true;
            continue;
        }

        if let Some(value) = parse_value(part.to_string())
            && (i == 3 || i == 2)
            && past == true
        {
            var_value = Some(value);
            continue;
        }
    }

    if var_type.is_none() {
        var_type = Some(if let Some(value) = &var_value {
            match value {
                Value::Boolean(_) => VariableType::Boolean,
                Value::String(_) => VariableType::String,
                Value::Number(_) => VariableType::Number,
                Value::Undefined => VariableType::Undefined,
            }
        } else {
            VariableType::Undefined
        })
    }

    if var_value.is_none() {
        var_value = Some(Value::Undefined)
    }

    if var_name.is_some() {
        return Some(Expression::VariableDefinition((
            var_name.unwrap(),
            var_type.unwrap(),
            Box::new(Expression::Value(var_value.unwrap())),
        )));
    }

    None
}

fn parse_string_to_var_type(s: &str) -> Option<VariableType> {
    match s {
        "bool" | "boolean" => Some(VariableType::Boolean),
        "String" => Some(VariableType::String),
        "number" => Some(VariableType::Number),
        "undefined" => Some(VariableType::Undefined),
        _ => None,
    }
}

pub fn parse_reset_variable(expression: String, vars: &mut VariableMap) -> Option<Expression> {
    let (variable_name, value) = expression.split_once('=')?;
    let variable_name = variable_name.trim();

    vars.get_pure_var(variable_name)?;

    let mut r_value = Expression::None;

    if let Some(exp) = parse_value(value.trim().to_string()) {
        r_value = Expression::Value(exp)
    } else if let Some(exp) = vars.get_pure_var(value.trim()) {
        r_value = exp.get_value();
    } else if let Some(exp) = parse_dot_notation(value.trim().to_string(), Expression::None, vars) {
        r_value = exp;
    }


    Some(Expression::ResetVariableValue((
        variable_name.to_string(),
        Box::new(r_value),
    )))
}

#[cfg(test)]
mod tests {
    use super::{Variable, VariableType};
    use crate::parser::{Expression, value_parser::Value};

    #[test]
    fn undefined_variable_adopts_the_assigned_value_type() {
        let mut variable = Variable {
            value: Expression::Value(Value::Undefined),
            var_type: VariableType::Undefined,
            name: "value".to_string(),
        };

        assert!(
            variable
                .set_value(Expression::Value(Value::Number(5.0)))
                .is_ok()
        );
        assert_eq!(variable.var_type, VariableType::Number);
    }

    #[test]
    fn mismatched_assignment_is_rejected_without_changing_the_value() {
        let mut variable = Variable {
            value: Expression::Value(Value::Number(5.0)),
            var_type: VariableType::Number,
            name: "value".to_string(),
        };

        assert!(
            variable
                .set_value(Expression::Value(Value::String("wrong".to_string())))
                .is_err()
        );
        assert!(matches!(
            variable.get_value(),
            Expression::Value(Value::Number(value)) if value == 5.0
        ));
    }

    #[test]
    fn undefined_value_does_not_replace_the_declared_type() {
        let mut variable = Variable {
            value: Expression::Value(Value::String("hello".to_string())),
            var_type: VariableType::String,
            name: "value".to_string(),
        };

        assert!(
            variable
                .set_value(Expression::Value(Value::Undefined))
                .is_ok()
        );
        assert_eq!(variable.var_type, VariableType::String);
    }
}
