enum Expression {
    Number(f64),
    Add(Box<Expression>, Box<Expression>),
    Subtract(Box<Expression>, Box<Expression>),
    Multiply(Box<Expression>, Box<Expression>),
    Divide(Box<Expression>, Box<Expression>),
}

use crate::var_handler::VarMap;

pub fn attempt_calculator_parse(to_parse: String, vars: &VarMap) -> Expression {

    let mut chars: Vec<char> = to_parse.chars().collect();

    for char in chars {
        if char.is_numeric() {
            
        }
    }



    Expression::Number(0.0)
}