use crate::{alu::Expression::{Add, Divide, Multiply, Number}, var_handler::VarMap};

#[derive(Debug, PartialEq)]
pub enum Expression {
    Error(String),
    Number(f64),
    Add(Box<Expression>, Box<Expression>),
    Subtract(Box<Expression>, Box<Expression>),
    Multiply(Box<Expression>, Box<Expression>),
    Divide(Box<Expression>, Box<Expression>),
}

pub fn attempt_calculator_parse(to_parse: String, vars: &VarMap) -> Expression {
    let mut chars: Vec<char> = to_parse.chars().collect();
    chars.push('\0');

    let mut possible_number = String::new();

    let mut current_expression: Option<Expression> = None;
    let mut pending_operator: Option<char> = None;

    let mut first = true;

    for char in chars {
        if first {
            if char.is_numeric() || char == '.' {
                possible_number.push(char);
            } else if !possible_number.is_empty() {
                println!("{possible_number}");

                match possible_number.parse::<f64>() {
                    Ok(number) => current_expression = Some(Expression::Number(number)),
                    Err(_) => return Expression::Error(format!("Error while parsing {possible_number}").to_string())
                }

                possible_number.clear();
            }
        }

        if vec!['+', '-', '*', '/'].contains(&char) {
            pending_operator = Some(char);
        }

        if pending_operator.is_some() {
            first = false;
            if char.is_numeric() || char == '.' {
                possible_number.push(char);
            } else if !possible_number.is_empty() {
                println!("{possible_number}");

                match possible_number.parse::<f64>() {
                    Ok(number) => {
                        let last_expression = current_expression.take().unwrap();
                        let second_num = Some(Expression::Number(number)).unwrap();

                        if pending_operator.unwrap() == '+' {
                            current_expression = Some(Expression::Add(Box::new(last_expression), Box::new(second_num)))
                        } else if pending_operator.unwrap() == '-' {
                            current_expression = Some(Expression::Subtract(Box::new(last_expression), Box::new(second_num)))
                        } else if pending_operator.unwrap() == '*' {
                            match last_expression {
                                last_expression @ Number(_) => {
                                    current_expression = Some(Expression::Multiply(Box::new(last_expression), Box::new(second_num)))
                                }
                                Add(left, right) => {
                                    current_expression = Some(
                                        Add(left, Box::new(
                                            Multiply(right, Box::new(
                                                second_num
                                            ))
                                        ))
                                    );
                                },
                                Expression::Subtract(left, right) => {
                                    current_expression = Some(Expression::Subtract(
                                        left,
                                        Box::new(Expression::Multiply(
                                            right,
                                            Box::new(second_num),
                                        )),
                                    ));
                                }
                                _ => {}
                            }
                        } else if pending_operator.unwrap() == '/' {
                            match last_expression {
                                last_expression @ Number(_) => {
                                    current_expression = Some(Expression::Divide(Box::new(last_expression), Box::new(second_num)))
                                }
                                Add(left, right) => {
                                    current_expression = Some(
                                        Add(left, Box::new(
                                            Divide(right, Box::new(
                                                second_num
                                            ))
                                        ))
                                    );
                                },
                                Expression::Subtract(left, right) => {
                                    current_expression = Some(Expression::Subtract(
                                        left,
                                        Box::new(Expression::Divide(
                                            right,
                                            Box::new(second_num),
                                        )),
                                    ));
                                }
                                _ => {}
                            }
                        }
                    },
                    Err(_) => return Expression::Error(format!("Error while parsing {possible_number}").to_string())
                }

                possible_number.clear();
            }
        }

    }

    return current_expression.unwrap();

    Expression::Error(format!("Could not calculator parse: {}", to_parse).to_string())
}

#[test]
fn test_simple_addition() {
    let vars = VarMap::new();

    let result = attempt_calculator_parse("1.01 + 1.01".to_string(), &vars);

    println!("{result:#?}");

    assert_eq!(
        result,
        Expression::Add(
            Box::new(Expression::Number(1.01)),
            Box::new(Expression::Number(1.01)),
        )
    );
}

#[test]
fn test_three_number_addition() {
    let vars = VarMap::new();

    let result = attempt_calculator_parse(
        "1.01 + 1.01 + 1.01".to_string(),
        &vars,
    );

    println!("{result:#?}");

    assert_eq!(
        result,
        Expression::Add(
            Box::new(Expression::Add(
                Box::new(Expression::Number(1.01)),
                Box::new(Expression::Number(1.01)),
            )),
            Box::new(Expression::Number(1.01)),
        )
    );
}

#[test]
fn test_addition_and_subtraction() {
    let vars = VarMap::new();

    let result = attempt_calculator_parse(
        "1 + 2 - 3".to_string(),
        &vars,
    );

    println!("{result:#?}");

    assert_eq!(
        result,
        Expression::Subtract(
            Box::new(Expression::Add(
                Box::new(Expression::Number(1.0)),
                Box::new(Expression::Number(2.0)),
            )),
            Box::new(Expression::Number(3.0)),
        )
    );
}

#[test]
fn test_multiplication_precedence() {
    let vars = VarMap::new();

    let result = attempt_calculator_parse(
        "1 + 2 * 3".to_string(),
        &vars,
    );

    println!("{result:#?}");

    assert_eq!(
        result,
        Expression::Add(
            Box::new(Expression::Number(1.0)),
            Box::new(Expression::Multiply(
                Box::new(Expression::Number(2.0)),
                Box::new(Expression::Number(3.0)),
            )),
        )
    );
}

#[test]
fn test_long_mixed_expression() {
    let vars = VarMap::new();

    let result = attempt_calculator_parse(
        "1 + 2 * 3 - 4 * 5 + 6".to_string(),
        &vars,
    );

    println!("{result:#?}");

    assert_eq!(
        result,
        Expression::Add(
            Box::new(Expression::Subtract(
                Box::new(Expression::Add(
                    Box::new(Expression::Number(1.0)),
                    Box::new(Expression::Multiply(
                        Box::new(Expression::Number(2.0)),
                        Box::new(Expression::Number(3.0)),
                    )),
                )),
                Box::new(Expression::Multiply(
                    Box::new(Expression::Number(4.0)),
                    Box::new(Expression::Number(5.0)),
                )),
            )),
            Box::new(Expression::Number(6.0)),
        )
    );
}