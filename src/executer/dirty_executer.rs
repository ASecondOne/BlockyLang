use crate::{
    combi::{
        library::{FUNCTIONS, Output},
        variable::VariableMap,
    },
    parser::Expression,
};

pub fn execute(ex: Expression, vars: &mut VariableMap) {
    let _ = evaluate(ex, vars);
}

fn evaluate(ex: Expression, vars: &mut VariableMap) -> Result<Output, ()> {
    match ex {
        Expression::Value(_) => Ok(Output::Expression(ex)),
        Expression::Variable(_) => Ok(Output::Expression(ex)),

        Expression::None => Err(()),

        Expression::ConditionalExpression(_) => Err(()),

        Expression::ExecutionExpression((func_name, exp))
        | Expression::ChainingExpression((func_name, exp)) => {
            let argument = match evaluate(*exp, vars)? {
                Output::Expression(expression) => expression,
                Output::Success => return Err(()),
            };
            let func = FUNCTIONS.get(func_name.as_str()).ok_or(())?;

            func(argument, vars)
        }

        Expression::VariableDefinition((variable_name, variable_type, variable_value)) => {
            match vars.add_new_variable(variable_name, *variable_value, variable_type) {
                Ok(o) => Ok(o),
                Err(_) => Err(()),
            }
        }

        Expression::ResetVariableValue((variable_name, value)) => {
            let value = match evaluate(*value, vars)? {
                Output::Expression(exp) => exp,
                _ => return Err(()),
            };

            let variable = vars.get_pure_var(&variable_name).ok_or(())?;
            variable.set_value(value)?;

            Ok(Output::Success)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{evaluate, execute};
    use crate::{
        combi::{
            library::Output,
            variable::{VariableMap, VariableType},
        },
        parser::{Expression, value_parser::Value},
    };

    #[test]
    fn library_functions_resolve_the_updated_variable_value() {
        let mut vars = VariableMap::new();

        execute(
            Expression::VariableDefinition((
                "a".to_string(),
                VariableType::Number,
                Box::new(Expression::Value(Value::Number(6.0))),
            )),
            &mut vars,
        );

        let variable_reference = Expression::Variable("a".to_string());

        execute(
            Expression::ResetVariableValue((
                "a".to_string(),
                Box::new(Expression::Value(Value::Number(5.0))),
            )),
            &mut vars,
        );

        assert!(matches!(
            evaluate(variable_reference.clone(), &mut vars),
            Ok(Output::Expression(Expression::Variable(name))) if name == "a"
        ));

        assert!(matches!(
            evaluate(
                Expression::ChainingExpression((
                    "inc_one()".to_string(),
                    Box::new(variable_reference),
                )),
                &mut vars,
            ),
            Ok(Output::Expression(Expression::Value(Value::Number(value)))) if value == 6.0
        ));
    }
}
