use crate::parser::{
    Expression,
    library::{FUNCTIONS, Output},
};

pub fn execute(ex: Expression) {
    let _ = evaluate(ex);
}

fn evaluate(ex: Expression) -> Result<Output, ()> {
    match ex {
        Expression::Value(_) => Ok(Output::Expression(ex)),
        Expression::None => Err(()),
        Expression::ExecutionExpression((func_name, exp))
        | Expression::ChainingExpression((func_name, exp)) => {
            let argument = match evaluate(*exp)? {
                Output::Expression(expression) => expression,
                Output::Success => return Err(()),
            };
            let func = FUNCTIONS.get(func_name.as_str()).ok_or(())?;

            func(argument)
        }
    }
}
