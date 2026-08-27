use crate::{combi::{library::{Output::Success, extract_value}, variable::VariableMap}, executer::dirty_executer::{evaluate, execute}, parser::{Expression, value_parser::Value}};

#[derive(Debug, Clone)]
pub enum Condition {
    Expression(Box<Expression>),

    IsEqual(Box<Condition>, Box<Condition>),
    Bigger(Box<Condition>, Box<Condition>),
    BiggerEquals(Box<Condition>, Box<Condition>),
    Smaller(Box<Condition>, Box<Condition>),
    SmallerEquals(Box<Condition>, Box<Condition>),
    NotEquals(Box<Condition>, Box<Condition>),
    Invert(Box<Condition>),
}

enum Token {
    IsEqual,
    Bigger,
    BiggerEquals,
    Smaller,
    SmallerEquals,
    NotEquals,
}

pub fn parse_condition(s: String, vars: &mut VariableMap) -> Option<Condition> {
    let mut input = s.trim();

    if input.is_empty() {
        return None;
    }
    
    loop {
        if !input.starts_with('(') || !input.ends_with(')') {
            break;
        }

        let mut depth = 0usize;
        let mut inside_string = false;
        let mut closes_at_end = false;

        for (index, character) in input.char_indices() {
            match character {
                '"' => inside_string = !inside_string,
                '(' if !inside_string => depth += 1,
                ')' if !inside_string => {
                    depth = depth.checked_sub(1)?;
                    if depth == 0 {
                        closes_at_end = index + character.len_utf8() == input.len();
                        break;
                    }
                }
                _ => {}
            }
        }

        if !closes_at_end {
            break;
        }

        input = input[1..input.len() - 1].trim();
        if input.is_empty() {
            return None;
        }
    }

    if let Some(inverted) = input.strip_prefix('!')
        && !input.starts_with("!=")
    {
        return Some(Condition::Invert(Box::new(parse_condition(
            inverted.trim().to_string(),
            vars,
        )?)));
    }

    let mut inside_string = false;
    let mut depth = 0usize;
    let mut found_token: Option<(usize, usize, Token)> = None;

    for (index, character) in input.char_indices() {
        match character {
            '"' => inside_string = !inside_string,
            '(' if !inside_string => depth += 1,
            ')' if !inside_string => depth = depth.checked_sub(1)?,
            _ if inside_string || depth > 0 => continue,
            _ => {
                let rest = &input[index..];
                let (length, token) = if rest.starts_with("==") {
                    (2, Token::IsEqual)
                } else if rest.starts_with("!=") {
                    (2, Token::NotEquals)
                } else if rest.starts_with(">=") {
                    (2, Token::BiggerEquals)
                } else if rest.starts_with("<=") {
                    (2, Token::SmallerEquals)
                } else if rest.starts_with('>') {
                    (1, Token::Bigger)
                } else if rest.starts_with('<') {
                    (1, Token::Smaller)
                } else {
                    continue;
                };

                found_token = Some((index, length, token));
                break;
            }
        }
    }

    if inside_string || depth != 0 {
        return None;
    }

    if let Some((index, token_length, token)) = found_token {
        let left = parse_condition(input[..index].trim().to_string(), vars)?;
        let right = parse_condition(input[index + token_length..].trim().to_string(), vars)?;
        let left = Box::new(left);
        let right = Box::new(right);

        return Some(match token {
            Token::IsEqual => Condition::IsEqual(left, right),
            Token::Bigger => Condition::Bigger(left, right),
            Token::BiggerEquals => Condition::BiggerEquals(left, right),
            Token::Smaller => Condition::Smaller(left, right),
            Token::SmallerEquals => Condition::SmallerEquals(left, right),
            Token::NotEquals => Condition::NotEquals(left, right),
        });
    }

    if let Some(value) = crate::parser::value_parser::parse_value(input.to_string()) {
        return Some(Condition::Expression(Box::new(Expression::Value(value))));
    }

    if let Some(expression) = crate::parser::dot_notation_parser::parse_dot_notation(
        input.to_string(),
        Expression::None,
        vars,
    ) {
        return Some(Condition::Expression(Box::new(expression)));
    }

    None
}

pub fn evaluate_condition(input: Condition, vars: &mut VariableMap) -> Option<Value> {
    return match input {
        Condition::Expression(exp) => {
            if let Ok(v) = extract_value(*exp.clone(), vars) {
                return Some(v);
            }

            match evaluate(*exp, vars) {
                Ok(out) => match out {
                    Success => {
                        return None;
                    },
                    crate::combi::library::Output::Expression(expp) => {
                        if let Ok(v) = extract_value(expp, vars) {
                            return Some(v);
                        }
                    }
                },
                Err(_) => {
                    return None;
                }
            }

            None
        },

        Condition::IsEqual(left, right) => {

        },

        _ => None,
    }
}