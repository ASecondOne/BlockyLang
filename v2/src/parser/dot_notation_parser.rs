use crate::{
    combi::{library::FUNCTIONS, variable::VariableMap},
    parser::{Expression, value_parser::parse_value},
};

pub fn parse_dot_notation(
    expression: String,
    exp: Expression,
    vars: &mut VariableMap,
) -> Option<Expression> {
    let (current, rest) = split_at_dot(&expression);

    let parsed = match exp {
        Expression::None => {
            if let Some(value) = parse_value(current.trim().to_string()) {
                Expression::Value(value)
            } else {
                vars.get_var(current.trim())?
            }
        }
        previous => {
            let function_name = current.trim();

            if !FUNCTIONS.contains_key(function_name) {
                return None;
            }

            Expression::ChainingExpression((function_name.to_string(), Box::new(previous)))
        }
    };

    match rest {
        Some(rest) if !rest.trim().is_empty() => parse_dot_notation(rest.to_string(), parsed, vars),
        Some(_) => None,
        None if matches!(parsed, Expression::Value(_)) => None,
        None => Some(parsed),
    }
}

fn split_at_dot(expression: &str) -> (&str, Option<&str>) {
    let mut inside_string = false;

    for (index, character) in expression.char_indices() {
        match character {
            '"' => inside_string = !inside_string,
            '.' if !inside_string => {
                return (&expression[..index], Some(&expression[index + 1..]));
            }
            _ => {}
        }
    }

    (expression, None)
}
