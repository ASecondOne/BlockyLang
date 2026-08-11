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
        ("let",  m_let as Function),
    ])
});

fn println(args: Expression) -> Result<Output, ()> {
    match args {
        Expression::Value(v) => {
            println!("{v}");
            Ok(Output::Success)
        },
        Expression::Variable(mut v) => {
            if let Some(v) = v.get_value() {
                println!("{v}");
                return Ok(Output::Success);
            }

            Err(())
        }
        _ => Err(()),
    }
}

fn print(args: Expression) -> Result<Output, ()> {
    match args {
        Expression::Value(v) => {
            print!("{v}");
            Ok(Output::Success)
        },
        Expression::Variable(mut v) => {
            if let Some(v) = v.get_value() {
                println!("{v}");
                return Ok(Output::Success);
            }

            Err(())
        }
        _ => Err(()),
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
        _ => Err(()),
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
        _ => Err(()),
    }
}

fn m_let(_: Expression) -> Result<Output, ()> {Err(())}