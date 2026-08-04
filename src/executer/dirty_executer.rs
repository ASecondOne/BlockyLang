use crate::parser::{Expression, library::FUNCTIONS};

pub fn execute(ex: Expression) {
    match ex {
        Expression::Value(_) => {},
        Expression::None => {},
        Expression::ExecutionExpression((func_name, exp)) => {
            let func = FUNCTIONS.get(func_name.as_str()).unwrap();

            let _out = func(*exp);
        },

    }
}