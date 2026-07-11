pub enum Expression {
    Start,
    Error(String),
    Number(f64),
    Add(Box<Expression>, Box<Expression>),
    Subtract(Box<Expression>, Box<Expression>),
    Multiply(Box<Expression>, Box<Expression>),
    Divide(Box<Expression>, Box<Expression>),
}

use crate::var_handler::VarMap;

pub fn attempt_calculator_parse(to_parse: String, vars: &VarMap) -> Expression {
    let mut chars: Vec<char> = to_parse.chars().collect();
    chars.push('\0');

    let mut final_expression = Expression::Start;

    let mut possible_number = String::new();

    let mut last_expresion = Expression::Start;

    for char in chars {
        if char.is_numeric() || char == '.' {
            possible_number.push(char);
        } else if !possible_number.is_empty() {
            println!("{possible_number}");

            match possible_number.parse::<f64>() {
                Ok(number) => last_expresion = Expression::Number(number),
                Err(_) => return Expression::Error(format!("Error while parsing {possible_number}").to_string())
            }

            possible_number.clear();
        }

        match char {
            '+' => {
                
            },
            _ => {}
        }
    }

    Expression::Error(format!("Could not calculator parse: {}", to_parse).to_string())
}