use std::print;

use crate::{combi::variable::VariableMap, parser::Expression};

#[derive(Debug, Clone)]
pub enum Condition {
    Expression(Box<Expression>),

    IsEqual(Box<Condition>, Box<Condition>),
    Bigger(Box<Condition>, Box<Condition>),
    BiggerEquals(Box<Condition>, Box<Condition>),
    Smaller(Box<Condition>, Box<Condition>),
    SmallerEquals(Box<Condition>, Box<Condition>),
    NotEquals(Box<Condition>, Box<Condition>),
}

pub fn condition_parse(s: String, _vars: &mut VariableMap) -> Option<Expression> {

    print!("{s}");

    None
}