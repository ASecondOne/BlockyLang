use std::mem::take;

use crate::{
    combi::{
        library::FUNCTIONS,
        variable::{VariableMap, parse_reset_variable, parse_variable_expression},
    },
    parser::{
        dot_notation_parser::parse_dot_notation,
        value_parser::{Value, parse_value},
    },
};

#[derive(Debug, Clone)]
pub enum Expression {
    Value(Value),
    Variable(String),

    VariableDefinition((String, Box<Expression>)),

    ExecutionExpression((String, Box<Expression>)),
    ChainingExpression((String, Box<Expression>)),

    ResetVariableValue((String, Box<Expression>)),

    None,
}

fn prepare_strings(lines: Vec<&str>) -> Vec<String> {
    let mut out = Vec::new();

    let mut unfinished_line = String::new();

    let mut inside_string = false;

    for line in lines {
        for char in line.trim_start().chars() {
            if char == '"' {
                inside_string = !inside_string;
            }
            if char == ';' && !inside_string {
                out.push(take(&mut unfinished_line));
            } else {
                unfinished_line.push(char);
            }
        }
    }

    out
}

pub fn parse_lines(lines: Vec<&str>, vars: &mut VariableMap) -> Vec<Expression> {
    let mut out = Vec::new();

    let lines = prepare_strings(lines);

    'lines: for line in lines {
        if let Some(expression) = parse_reset_variable(line.clone(), vars) {
            out.push(expression);
            continue 'lines;
        }

        let mut unfinished_keyword = String::new();

        for ch in line.chars() {
            unfinished_keyword.push(ch);

            if FUNCTIONS.contains_key(unfinished_keyword.as_str()) {
                if let Some(expression) = line.strip_prefix(&unfinished_keyword) {
                    if expression
                        .chars()
                        .next()
                        .is_some_and(|character| !character.is_whitespace())
                    {
                        continue;
                    }

                    if let Some(value) = parse_value(expression.trim().to_string()) {
                        // Value Parser
                        out.push(Expression::ExecutionExpression((
                            unfinished_keyword,
                            Box::new(Expression::Value(value)),
                        )));
                    } else if let Some(exp) =
                        parse_variable_expression(expression.trim().to_string())
                    {
                        // Variable set parser
                        out.push(exp);
                    } else if let Some(exp) =
                        parse_dot_notation(expression.trim().to_string(), Expression::None, vars)
                    // Dot Notation Parser
                    {
                        out.push(Expression::ExecutionExpression((
                            unfinished_keyword,
                            Box::new(exp),
                        )));
                    } else if let Some(exp) = vars.get_var(expression.trim()) {
                        // Variable parser
                        out.push(Expression::ExecutionExpression((
                            unfinished_keyword,
                            Box::new(exp),
                        )));
                    }

                    continue 'lines;
                }
            }
        }
    }

    out
}

#[cfg(test)]
mod tests {
    use super::{Expression, VariableMap, parse_lines};

    #[test]
    fn distinguishes_print_from_println() {
        for function_name in ["print", "println"] {
            let expressions = parse_lines(
                vec![&format!("{function_name} \"hello\";")],
                &mut VariableMap::new(),
            );

            assert!(matches!(
                expressions.as_slice(),
                [Expression::ExecutionExpression((parsed_name, _))] if parsed_name == function_name
            ));
        }
    }
}
