use crate::{alu::Expression::{Add, Divide, Multiply, Number, Subtract}, var_handler::VarMap};

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
        if first && pending_operator.is_none() {
            if char.is_numeric() || char == '.' {
                possible_number.push(char);
            } else if !possible_number.is_empty() {
                match possible_number.parse::<f64>() {
                    Ok(number) => current_expression = Some(Expression::Number(number)),
                    Err(_) => return Expression::Error(format!("Error while parsing {possible_number}").to_string())
                }

                possible_number.clear();
            }
        }

        if pending_operator.is_some() {
            first = false;
            if char.is_numeric() || char == '.' {
                possible_number.push(char);
            } else if !possible_number.is_empty() {
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
                                Subtract(left, right) => {
                                    current_expression = Some(Expression::Subtract(
                                        left,
                                        Box::new(Expression::Multiply(
                                            right,
                                            Box::new(second_num),
                                        )),
                                    ));
                                },
                                last_expression @ Multiply(_, _) | last_expression @ Divide(_, _) => {
                                    current_expression = Some(Expression::Multiply(
                                        Box::new(last_expression),
                                        Box::new(second_num),
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
                                Subtract(left, right) => {
                                    current_expression = Some(Expression::Subtract(
                                        left,
                                        Box::new(Expression::Divide(
                                            right,
                                            Box::new(second_num),
                                        )),
                                    ));
                                }
                                last_expression @ Multiply(_, _) | last_expression @ Divide(_, _) => {
                                    current_expression = Some(Expression::Divide(
                                        Box::new(last_expression),
                                        Box::new(second_num),
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

        if vec!['+', '-', '*', '/'].contains(&char) {
            pending_operator = Some(char);
        }
    }

    if current_expression.is_some() {
        return current_expression.unwrap();
    }

    Expression::Error(format!("Could not calculator parse: {}", to_parse).to_string())
}

pub fn attempt_calculator_run(exp: &Expression) -> Result<f64, String> {
    match exp {
        Expression::Error(error) => Err(error.clone()),

        Expression::Number(number) => Ok(*number),

        Expression::Add(left, right) => {
            Ok(attempt_calculator_run(left)? + attempt_calculator_run(right)?)
        }

        Expression::Subtract(left, right) => {
            Ok(attempt_calculator_run(left)? - attempt_calculator_run(right)?)
        }

        Expression::Multiply(left, right) => {
            Ok(attempt_calculator_run(left)? * attempt_calculator_run(right)?)
        }

        Expression::Divide(left, right) => {
            let left = attempt_calculator_run(left)?;
            let right = attempt_calculator_run(right)?;

            if right == 0.0 {
                return Err("Cannot divide by zero".to_string());
            }

            Ok(left / right)
        }
    }
}

#[test]
fn test_simple_division() {
    let vars = VarMap::new();

    let result = attempt_calculator_parse("8 / 2".to_string(), &vars);

    assert_eq!(
        result,
        Expression::Divide(
            Box::new(Expression::Number(8.0)),
            Box::new(Expression::Number(2.0)),
        )
    );
}

#[test]
fn test_multiple_multiplications() {
    let vars = VarMap::new();

    let result = attempt_calculator_parse("2 * 3 * 4".to_string(), &vars);

    assert_eq!(
        result,
        Expression::Multiply(
            Box::new(Expression::Multiply(
                Box::new(Expression::Number(2.0)),
                Box::new(Expression::Number(3.0)),
            )),
            Box::new(Expression::Number(4.0)),
        )
    );
}

#[test]
fn test_multiple_divisions() {
    let vars = VarMap::new();

    let result = attempt_calculator_parse("16 / 4 / 2".to_string(), &vars);

    assert_eq!(
        result,
        Expression::Divide(
            Box::new(Expression::Divide(
                Box::new(Expression::Number(16.0)),
                Box::new(Expression::Number(4.0)),
            )),
            Box::new(Expression::Number(2.0)),
        )
    );
}

#[test]
fn test_multiplication_then_division() {
    let vars = VarMap::new();

    let result = attempt_calculator_parse("2 * 3 / 4".to_string(), &vars);

    assert_eq!(
        result,
        Expression::Divide(
            Box::new(Expression::Multiply(
                Box::new(Expression::Number(2.0)),
                Box::new(Expression::Number(3.0)),
            )),
            Box::new(Expression::Number(4.0)),
        )
    );
}

#[test]
fn test_division_then_multiplication() {
    let vars = VarMap::new();

    let result = attempt_calculator_parse("8 / 4 * 2".to_string(), &vars);

    assert_eq!(
        result,
        Expression::Multiply(
            Box::new(Expression::Divide(
                Box::new(Expression::Number(8.0)),
                Box::new(Expression::Number(4.0)),
            )),
            Box::new(Expression::Number(2.0)),
        )
    );
}

#[test]
fn test_division_precedence() {
    let vars = VarMap::new();

    let result = attempt_calculator_parse("1 + 8 / 4".to_string(), &vars);

    assert_eq!(
        result,
        Expression::Add(
            Box::new(Expression::Number(1.0)),
            Box::new(Expression::Divide(
                Box::new(Expression::Number(8.0)),
                Box::new(Expression::Number(4.0)),
            )),
        )
    );
}

#[test]
fn test_multiple_high_precedence_expressions() {
    let vars = VarMap::new();

    let result =
        attempt_calculator_parse("2 * 3 + 8 / 4".to_string(), &vars);

    assert_eq!(
        result,
        Expression::Add(
            Box::new(Expression::Multiply(
                Box::new(Expression::Number(2.0)),
                Box::new(Expression::Number(3.0)),
            )),
            Box::new(Expression::Divide(
                Box::new(Expression::Number(8.0)),
                Box::new(Expression::Number(4.0)),
            )),
        )
    );
}

#[test]
fn test_long_multiplication_and_division_chain() {
    let vars = VarMap::new();

    let result =
        attempt_calculator_parse("2 * 3 / 4 * 5 / 6".to_string(), &vars);

    assert_eq!(
        result,
        Expression::Divide(
            Box::new(Expression::Multiply(
                Box::new(Expression::Divide(
                    Box::new(Expression::Multiply(
                        Box::new(Expression::Number(2.0)),
                        Box::new(Expression::Number(3.0)),
                    )),
                    Box::new(Expression::Number(4.0)),
                )),
                Box::new(Expression::Number(5.0)),
            )),
            Box::new(Expression::Number(6.0)),
        )
    );
}

#[test]
fn test_all_operators_and_precedence() {
    let vars = VarMap::new();

    let result = attempt_calculator_parse(
        "1 + 2 * 3 - 8 / 4 + 5 * 6 / 3".to_string(),
        &vars,
    );

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
                Box::new(Expression::Divide(
                    Box::new(Expression::Number(8.0)),
                    Box::new(Expression::Number(4.0)),
                )),
            )),
            Box::new(Expression::Divide(
                Box::new(Expression::Multiply(
                    Box::new(Expression::Number(5.0)),
                    Box::new(Expression::Number(6.0)),
                )),
                Box::new(Expression::Number(3.0)),
            )),
        )
    );
}

#[test]
fn test_expression_without_spaces() {
    let vars = VarMap::new();

    let result = attempt_calculator_parse("1+2*3/4-5".to_string(), &vars);

    assert_eq!(
        result,
        Expression::Subtract(
            Box::new(Expression::Add(
                Box::new(Expression::Number(1.0)),
                Box::new(Expression::Divide(
                    Box::new(Expression::Multiply(
                        Box::new(Expression::Number(2.0)),
                        Box::new(Expression::Number(3.0)),
                    )),
                    Box::new(Expression::Number(4.0)),
                )),
            )),
            Box::new(Expression::Number(5.0)),
        )
    );
}
