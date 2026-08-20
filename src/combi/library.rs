use std::{collections::HashMap, format, sync::LazyLock};

use crate::{
    combi::variable::{VariableMap, VariableType}, parser::{
        Expression,
        value_parser::{Value, parse_value},
    },
};

pub enum Output {
    Success,
    Expression(Expression),
}

type Function = fn(Expression, &mut VariableMap) -> Result<Output, ()>;

pub static FUNCTIONS: LazyLock<HashMap<&'static str, Function>> = LazyLock::new(|| {
    HashMap::from([
        ("print", print as Function),
        ("println", println as Function),
        ("len()", len as Function),
        ("inc_one()", inc_one as Function),
        ("let", m_let as Function),
        ("type()", m_type as Function),
        ("if", m_if as Function),
    ])
});

fn extract_value(args: Expression, vars: &mut VariableMap) -> Result<Value, ()> {
    match args {
        Expression::Value(value) => Ok(value),
        Expression::Variable(variable_name) => {
            let variable = vars.get_pure_var(&variable_name).ok_or(())?;

            match variable.get_value() {
                Expression::Value(value) => Ok(value),
                _ => Err(()),
            }
        }
        _ => Err(()),
    }
}

fn println(args: Expression, vars: &mut VariableMap) -> Result<Output, ()> {
    println!("{}", extract_value(args, vars)?);
    Ok(Output::Success)
} 

fn print(args: Expression, vars: &mut VariableMap) -> Result<Output, ()> {
    print!("{}", extract_value(args, vars)?);
    Ok(Output::Success)
}

fn len(args: Expression, vars: &mut VariableMap) -> Result<Output, ()> {
    match extract_value(args, vars)? {
        Value::String(s) => {
            let n = s.len();
            Ok(Output::Expression(Expression::Value(
                parse_value(format!("{n}")).unwrap(),
            )))
        }
        _ => Err(()),
    }
}

fn inc_one(args: Expression, vars: &mut VariableMap) -> Result<Output, ()> {
    match extract_value(args, vars)? {
        Value::Number(mut n) => {
            n += 1.0;
            Ok(Output::Expression(Expression::Value(
                parse_value(format!("{n}")).unwrap(),
            )))
        }
        _ => Err(()),
    }
}

fn m_let(_: Expression, _: &mut VariableMap) -> Result<Output, ()> {
    Err(())
}

fn m_type(args: Expression, vars: &mut VariableMap) -> Result<Output, ()> {
    match args {
        Expression::Value(v) => match v {
            Value::Boolean(_) => Ok(Output::Expression(Expression::Value(Value::String(
                "bool".to_string(),
            )))),
            Value::String(_) => Ok(Output::Expression(Expression::Value(Value::String(
                "String".to_string(),
            )))),
            Value::Number(_) => Ok(Output::Expression(Expression::Value(Value::String(
                "number".to_string(),
            )))),
            Value::Undefined => Ok(Output::Expression(Expression::Value(Value::String(
                "undefined".to_string(),
            )))),
        }

        Expression::Variable(v) => {
            if let Some(var) = vars.get_pure_var(&v) {
                match var.var_type {
                    VariableType::Boolean => Ok(Output::Expression(Expression::Value(Value::String(
                        "bool".to_string(),
                    )))),
                    VariableType::String => Ok(Output::Expression(Expression::Value(Value::String(
                        "String".to_string(),
                    )))),
                    VariableType::Number => Ok(Output::Expression(Expression::Value(Value::String(
                        "number".to_string(),
                    )))),
                    VariableType::Undefined => Ok(Output::Expression(Expression::Value(Value::String(
                        "undefined".to_string(),
                    )))),
                }
            } else {
                return Err(());
            }
        }

        _ => Err(())
    }

}

fn m_if(exp: Expression, _vars: &mut VariableMap) -> Result<Output, ()> {
    Err(())
}