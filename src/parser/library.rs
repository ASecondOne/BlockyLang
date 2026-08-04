use std::{
    collections::HashMap,
    sync::LazyLock,
};

use crate::parser::{Expression, value_parser::Value};

type Function = fn(Expression) -> Result<(), ()>;

pub static FUNCTIONS: LazyLock<HashMap<&'static str, Function>> = LazyLock::new(|| {
    HashMap::from([
        ("println", print as Function),
    ])
});

fn print(args: Expression) -> Result<(), ()> {
    match args {
        Expression::Value(v) => {
            println!("{v}");
            Ok(())
        },
        Expression::ExecutionExpression(_) => {Err(())},
        Expression::None => {Err(())},

    }
}