use std::{collections::HashMap, format, sync::LazyLock};

use crate::parser::{
    Expression,
    value_parser::{Value, parse_value},
};

pub enum Output {
    Success,
    Expression(Expression),
}

type Function = fn(Expression) -> Result<Output, ()>;

pub static FUNCTIONS: LazyLock<HashMap<&'static str, Function>> = LazyLock::new(|| {
    HashMap::from([
        ("print", print as Function),
        ("println", println as Function),
        ("len()", len as Function),
        ("inc_one()", inc_one as Function),
    ])
});

fn println(args: Expression) -> Result<Output, ()> {
    match args {
        Expression::Value(v) => {
            println!("{v}");
            Ok(Output::Success)
        }
        Expression::ChainingExpression(_) => {
            return Err(());
        }
        Expression::ExecutionExpression(_) => Err(()),
        Expression::None => Err(()),
    }
}

fn print(args: Expression) -> Result<Output, ()> {
    match args {
        Expression::Value(v) => {
            print!("{v}");
            Ok(Output::Success)
        }
        Expression::ChainingExpression(_) => {
            return Err(());
        }
        Expression::ExecutionExpression(_) => Err(()),
        Expression::None => Err(()),
    }
}

fn len(args: Expression) -> Result<Output, ()> {
    match args {
        Expression::Value(v) => match v {
            Value::String(s) => {
                let n = s.len();
                Ok(Output::Expression(Expression::Value(
                    parse_value(format!("{n}")).unwrap(),
                )))
            }
            Value::Number(_) => Err(()),
            Value::Boolean(_) => Err(()),
        },
        Expression::ChainingExpression(_) => Err(()),
        Expression::ExecutionExpression(_) => Err(()),
        Expression::None => Err(()),
    }
}

fn inc_one(args: Expression) -> Result<Output, ()> {
    match args {
        Expression::Value(v) => match v {
            Value::String(_) => {Err(())},
            Value::Number(mut n) => {
                n += 1.0;
                Ok(Output::Expression(Expression::Value(
                    parse_value(format!("{n}")).unwrap(),
                )))
            },
            Value::Boolean(_) => Err(()),
        },
        Expression::ChainingExpression(_) => Err(()),
        Expression::ExecutionExpression(_) => Err(()),
        Expression::None => Err(()),
    }
}