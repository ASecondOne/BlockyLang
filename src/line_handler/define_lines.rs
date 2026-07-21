use std::{io::{self, Write}, sync::Arc};

use crate::{alu::{Expression, attempt_calculator_parse, attempt_calculator_run}, blocks_handler::define_blocks::BlockType, utils::{output_state, runtime_error::RuntimeError}, var_handler::{VarMap, parse_type}};

pub enum ParseResult {
    One(String),
    Many(Vec<String>),
    ParseError(String),
    OneAlu(Expression),
}

type RunnerType = Arc<dyn Fn((&[String], &Option<Expression>), &mut VarMap) -> Result<(), RuntimeError>>;
type ParserType = Arc<dyn Fn(String, &mut VarMap) -> ParseResult>;

#[derive(Clone)]
pub struct Keyword {
    pub definition: String,
    pub runner: RunnerType,
    pub parser: ParserType,
    pub allowed_in: Vec<BlockType>,
}

impl Keyword {
    pub fn init() -> Vec<Keyword> {
        let mut out = Vec::new();

        out.push(Keyword {
            definition: "print".to_string(),
            runner: Arc::new(|(a, b): (&[String], &Option<Expression>), vars: &mut VarMap| {
                if let Some(exp) = b {
                    match attempt_calculator_run(exp, vars) {
                        Ok(v) => {
                            print!("{v}");
                            io::stdout().flush().unwrap();
                            output_state::used_print();
                            return Ok(());
                        },
                        Err(e) => {
                            return Err(RuntimeError::new(e));
                        }
                    }
                }
                if let Some(first) = a.first() {
                    if let Some(_) = a.get(1).map(|v| v.parse::<bool>()) {
                        return Err(RuntimeError::new("Cannot print an undefined value".to_string()));
                    }
                    print!("{first}");
                    output_state::used_print();
                    io::stdout().flush().unwrap();
                    return Ok(());
                }

                Err(RuntimeError::new("An Unknown error accord ups".to_string()))
            }),
            parser: Arc::new(|a: String, vars: &mut VarMap| {
                let a = a.strip_prefix("print").unwrap().trim();

                if let Some(inside) = a.strip_prefix('"').and_then(|s| s.strip_suffix('"')) {
                    return ParseResult::One(inside.to_string());
                }

                if let Some((value, undefined)) = vars.get_var(a.to_string()) {
                    if undefined {
                        return ParseResult::Many(vec![value, undefined.to_string()]);
                    }
                    return ParseResult::One(value.to_string());
                }


                match attempt_calculator_parse(a.to_string(), vars) {
                    Expression::Error(error) => ParseResult::ParseError(error),
                    expression => ParseResult::OneAlu(expression),
                }
            }),
            allowed_in: vec![BlockType::Execute]
        });

        out.push(Keyword {
            definition: "println".to_string(),
            runner: Arc::new(|(a, b): (&[String], &Option<Expression>), vars: &mut VarMap| {
                if let Some(exp) = b {
                    match attempt_calculator_run(exp, vars) {
                        Ok(v) => {
                            println!("{v}");
                            output_state::used_println();
                            return Ok(());
                        },
                        Err(e) => {
                            return Err(RuntimeError::new(e));
                        }
                    }
                }

                if let Some(first) = a.first() {
                    if let Some(_) = a.get(1).map(|v| v.parse::<bool>()) {
                        return Err(RuntimeError::new("Cannot print an undefined value".to_string()));
                    }
                    output_state::used_println();
                    println!("{first}");
                    io::stdout().flush().unwrap();
                    return Ok(());
                }

                Err(RuntimeError::new("An Unknown error accord".to_string()))
            }),
            parser: Arc::new(|a: String, vars: &mut VarMap| {
                let a = a.strip_prefix("println").unwrap().trim();

                if let Some(inside) = a.strip_prefix('"').and_then(|s| s.strip_suffix('"')) {
                    return ParseResult::One(inside.to_string());
                }

                if let Some((value, undefined)) = vars.get_var(a.to_string()) {
                    if undefined {
                        return ParseResult::Many(vec![value, undefined.to_string()]);
                    }
                    return ParseResult::One(value.to_string());
                }

                match attempt_calculator_parse(a.to_string(), vars) {
                    Expression::Error(error) => ParseResult::ParseError(error),
                    expression => ParseResult::OneAlu(expression),
                }

                // ParseResult::ParseError(format!("Could not parse print value: {a}"))
            }),
            allowed_in: vec![BlockType::Execute]
        });

        out.push(Keyword { 
            definition: "let".to_string(), 
            runner: Arc::new(|(a, _b): (&[String], &Option<Expression>), vars: &mut VarMap| {
                if let [name, value, undefined, ..] = a {
                    match vars.add_new(name.to_string(), value.to_string(), 
                    match undefined.as_str() {
                        "" => true,
                        _ => false,
                    }
                ) {
                        Ok(()) => return Ok(()),
                        Err(e) => return Err(RuntimeError::new(e)),
                    }
                }

                Err(RuntimeError::new("An Unknown Error accord".to_string()))
            }),
            parser: Arc::new(|a: String, _vars: &mut VarMap| {

                let Some(rest) = a.strip_prefix("let ") else {
                    return ParseResult::ParseError("expected `let`".to_string());
                };

                if let Some(rest) = rest.strip_suffix(';').unwrap_or(rest).strip_prefix("undefined ") {
                    let name = rest.trim();
                    return ParseResult::Many(vec![name.to_string(), "N/A".to_string(), "".to_string()]);
                }

                let Some((name, value)) = rest.strip_suffix(';').unwrap_or(rest).split_once('=') else {
                    return ParseResult::ParseError("missing `=`".to_string());
                };

                let name = name.trim();
                let value = value.trim();

                if name.is_empty() {
                    return ParseResult::ParseError("missing variable name".to_string());
                }

                if value.is_empty() {
                    return ParseResult::ParseError("missing variable value".to_string());
                }

                match parse_type(value, false) {
                    Ok(_) => {},
                    Err(msg) => return ParseResult::ParseError(msg)
                }

                ParseResult::Many(vec![name.to_string(), value.to_string(), "DEFINED".to_string()])
            }),
            allowed_in: vec![BlockType::Define] 
        });

        out
    }
}