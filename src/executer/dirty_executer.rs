use crate::parser::{
    Expression, library::{FUNCTIONS, Output}, variable_parser::{VariableMap},
};

pub fn execute(ex: Expression, vars: &mut VariableMap) {
    let _ = evaluate(ex, vars);
}

fn evaluate(ex: Expression, vars: &mut VariableMap) -> Result<Output, ()> {
    match ex {
        Expression::Value(_) => Ok(Output::Expression(ex)),
        Expression::Variable(_) => Ok(Output::Expression(ex)),

        Expression::None => Err(()),

        Expression::ExecutionExpression((func_name, exp))
        | Expression::ChainingExpression((func_name, exp)) => {
            let argument = match evaluate(*exp, vars)? {
                Output::Expression(expression) => expression,
                Output::Success => return Err(()),
            };
            let func = FUNCTIONS.get(func_name.as_str()).ok_or(())?;

            func(argument)
        },

        Expression::VariableDefinition((variable_name, variable_value)) => {
            match vars.add_new_variable(variable_name, *variable_value) {
                Ok(o) => Ok(o),
                Err(_) => Err(())
            }
        }
    }
}