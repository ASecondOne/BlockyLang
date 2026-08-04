use std::{mem::take};

use crate::parser::{library::FUNCTIONS, value_parser::{Value, parse_value}};

#[derive(Debug, Clone)]
pub enum Expression {
    Value(Value),

    ExecutionExpression((String, Box<Expression>)),

    None
}

fn prepare_strings(lines: Vec<&str>) -> Vec<String> {
    let mut out = Vec::new();

    let mut unfinished_line = String::new();

    for line in lines {   
        for char in line.trim_start().chars() {
            if char == ';' {
                out.push(take(&mut unfinished_line));
            } else {
                unfinished_line.push(char);
            }
        }
    }

    out
}

pub fn parse_lines(lines: Vec<&str>) -> Vec<Expression> {

    let mut out = Vec::new();

    let lines = prepare_strings(lines);

    'lines: for line in lines {
        let mut unfinished_keyword = String::new();

        for ch in line.chars() {
            unfinished_keyword.push(ch);

            if FUNCTIONS.contains_key(unfinished_keyword.as_str()) {
                if let Some(expression) = line.strip_prefix(&unfinished_keyword) {                    
                    if let Some(value) = parse_value(expression.trim().to_string()) {
                        out.push(
                            Expression::ExecutionExpression(
                                (unfinished_keyword, Box::new(Expression::Value(value)))
                            )
                        );
                    }

                    continue 'lines;
                }
            }
        }
    }

    out
}