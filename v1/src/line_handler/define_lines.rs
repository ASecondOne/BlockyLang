use std::{io::{self, Write}, mem::discriminant, println, sync::Arc, vec};

use colored::Colorize;

use crate::{blocks_handler::define_blocks::BlockType, logic::{LogicExpression, attempt_logic_parse}, math::{AluExpression, attempt_calculator_parse, attempt_calculator_run}, utils::{execution_policy::ExecutionPolicy, output_state, runtime_error::{ErrorType::{self, AlwaysError}, RuntimeError}}, var_handler::{Value, Var, VarMap, parse_type}};

pub enum ParseResult {
    Text(String),                   // Standard for Strings, stuff that dose not need special attention
    Alu(AluExpression),             // For Alu things like math, logic, stuff like that
    Var(Var),                     // Var yea variables    

    StandardOut(Vec<ParseResult>),  // Normal out can contain bot Alu and Text Results

    ParseError(String),             // Standard error output
}

type RunnerType = Arc<dyn Fn(Vec<ParseResult>, &mut VarMap, &ExecutionPolicy) -> Result<(), RuntimeError>>;
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
            runner: Arc::new(|a, vars: &mut VarMap, policy: &ExecutionPolicy| {
                match a.get(0).unwrap() {
                    ParseResult::Alu(exp) => {
                        match attempt_calculator_run(exp, vars, policy) {
                            Ok(v) => {
                                print!("{v}");
                                io::stdout().flush().unwrap();
                                output_state::used_print();
                                return Ok(());
                            },
                            Err(e) => {
                                return Err(e);
                            }
                        }
                    },

                    ParseResult::Var(v) => {
                        if discriminant(v.get_value()) == discriminant(&Value::Undefined) {
                            let error = RuntimeError::new(
                                "Cannot print an undefined value".to_string(),
                                ErrorType::OnUndefinedValue
                            );

                            if let Some(value) = policy.handle_error(error)? {
                                match value {
                                    Value::String(value) => print!("{value}"),
                                    Value::Number(value) => print!("{value}"),
                                    Value::Bool(value) => print!("{value}"),
                                    Value::Undefined => {}
                                }
                                output_state::used_print();
                                io::stdout().flush().unwrap();
                                return Ok(());
                            }
                        }
                    },

                    ParseResult::Text(s) => {
                        print!("{s}");
                        output_state::used_print();
                        io::stdout().flush().unwrap();
                        return Ok(());
                    }

                    _ => {}
                }

                Err(RuntimeError::new(
                    "An Unknown error accord ups".to_string(),
                    ErrorType::AlwaysError
                ))
            }),
            parser: Arc::new(|a: String, vars: &mut VarMap| {
                let a = a.strip_prefix("print").unwrap().trim();

                if let Some(inside) = a.strip_prefix('"').and_then(|s| s.strip_suffix('"')) {
                    return ParseResult::StandardOut(
                        vec![
                            ParseResult::Text(inside.to_string())
                        ]
                    );
                }

                if let Some(var) = vars.get_pure_var(a.to_string()) {
                    return ParseResult::StandardOut(
                        vec![
                            ParseResult::Var(var)
                        ]
                    );
                }

                match attempt_calculator_parse(a.to_string(), vars) {
                    AluExpression::Error(error) => ParseResult::ParseError(error),
                    alu_expression => ParseResult::StandardOut(
                        vec![
                            ParseResult::Alu(alu_expression)
                        ]
                    ),
                }
            }),
            allowed_in: vec![BlockType::Execute]
        });

        out.push(Keyword {
            definition: "println".to_string(),
            runner: Arc::new(|a, vars: &mut VarMap, policy: &ExecutionPolicy| {
                match a.get(0).unwrap() {
                    ParseResult::Alu(exp) => {
                        match attempt_calculator_run(exp, vars, policy) {
                            Ok(v) => {
                                println!("{v}");
                                io::stdout().flush().unwrap();
                                output_state::used_println();
                                return Ok(());
                            },
                            Err(e) => {
                                return Err(e);
                            }
                        }
                    },

                    ParseResult::Var(v) => {
                        if discriminant(v.get_value()) == discriminant(&Value::Undefined) {
                            let error = RuntimeError::new(
                                "Cannot print an undefined value".to_string(),
                                ErrorType::OnUndefinedValue
                            );

                            if let Some(value) = policy.handle_error(error)? {
                                match value {
                                    Value::String(value) => println!("{value}"),
                                    Value::Number(value) => println!("{value}"),
                                    Value::Bool(value) => println!("{value}"),
                                    Value::Undefined => {}
                                }
                                output_state::used_println();
                                io::stdout().flush().unwrap();
                                return Ok(());
                            }
                        }
                    },

                    ParseResult::Text(s) => {
                        println!("{s}");
                        output_state::used_println();
                        io::stdout().flush().unwrap();
                        return Ok(());
                    }

                    _ => {}
                }

                Err(RuntimeError::new(
                    "An Unknown error accord ups".to_string(),
                    ErrorType::AlwaysError
                ))
            }),
            parser: Arc::new(|a: String, vars: &mut VarMap| {
                let a = a.strip_prefix("println").unwrap().trim();

                if let Some(inside) = a.strip_prefix('"').and_then(|s| s.strip_suffix('"')) {
                    return ParseResult::StandardOut(
                        vec![
                            ParseResult::Text(inside.to_string())
                        ]
                    );
                }

                if let Some((value, undefined)) = vars.get_var(a.to_string()) {
                    if undefined {
                        return ParseResult::StandardOut(
                            vec![
                                ParseResult::Text(value.to_string()),
                                ParseResult::Text(undefined.to_string()),
                            ]
                        );
                    }
                    return ParseResult::StandardOut(
                        vec![
                            ParseResult::Text(value.to_string())
                        ]
                    );
                }

                match attempt_calculator_parse(a.to_string(), vars) {
                    AluExpression::Error(error) => ParseResult::ParseError(error),
                    alu_expression => ParseResult::StandardOut(
                        vec![
                            ParseResult::Alu(alu_expression)
                        ]
                    ),
                }

                // ParseResult::ParseError(format!("Could not parse print value: {a}"))
            }),
            allowed_in: vec![BlockType::Execute]
        });

        out.push(Keyword { 
            definition: "let".to_string(), 
            runner: Arc::new(|a: Vec<ParseResult>, vars: &mut VarMap, _policy: &ExecutionPolicy| {
                    let [
                        ParseResult::Text(name),
                        ParseResult::Text(value),
                        ParseResult::Text(undefined),
                    ] = a.as_slice()
                    else {
                        return Err(RuntimeError::new(
                            "Invalid arguments for let".to_string(),
                            ErrorType::AlwaysError,
                        ));
                    };

                    vars.add_new(
                        name.clone(),
                        value.clone(),
                        undefined.is_empty(),
                    )
                    .map_err(|e| RuntimeError::new(e, ErrorType::AlwaysError))
                },
            ),
            parser: Arc::new(|a: String, _vars: &mut VarMap| {

                let Some(rest) = a.strip_prefix("let ") else {
                    return ParseResult::ParseError("expected `let`".to_string());
                };

                if let Some(rest) = rest.strip_suffix(';').unwrap_or(rest).strip_prefix("undefined ") {
                    let name = rest.trim();

                    return ParseResult::StandardOut(
                        vec![
                            ParseResult::Text(name.to_string()),
                            ParseResult::Text("N/A".to_string()),
                            ParseResult::Text("".to_string()),
                        ]
                    )
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

                return ParseResult::StandardOut(
                    vec![
                        ParseResult::Text(name.to_string()),
                        ParseResult::Text(value.to_string()),
                        ParseResult::Text("DEFINED".to_string()),
                    ]
                )
            }),
            allowed_in: vec![BlockType::Define] 
        });

        out.push(Keyword { 
            definition: "Set Value".to_string(), 
            runner: Arc::new(|a, vars: &mut VarMap, policy: &ExecutionPolicy| {
                
                match a.as_slice() {
                    [ParseResult::Text(var), ParseResult::Text(new_value)] => {
                        match parse_type(new_value, false) {
                            Ok(new_var) => {
                                let old_var = vars.get_pure_value(var.to_string()).unwrap();

                                if discriminant(&new_var) == discriminant(&old_var) {
                                    vars.replace_value(var.to_string(), new_var);
                                    return Ok(());
                                } else {
                                    return Err(RuntimeError::new("Cannot set an different value to an origin var".to_string(), ErrorType::AlwaysError));
                                }

                            },
                            Err(e) => return Err(RuntimeError::new(e, ErrorType::AlwaysError))
                        }
                    }

                    [ParseResult::Text(var), ParseResult::Alu(exp)] => {
                        match attempt_calculator_run(exp, vars, policy) {
                            Ok(v) => {
                                let old_var = vars.get_pure_value(var.to_string()).unwrap();

                                match parse_type(&v.to_string(), false) {
                                    Ok(v) => {
                                        if discriminant(&v) == discriminant(&old_var) {
                                            vars.replace_value(var.to_string(), v);
                                            return Ok(());
                                        } else if discriminant(&old_var) == discriminant(&Value::Undefined) {
                                            vars.replace_value(var.to_string(), v);
                                            return Ok(());
                                        } else {
                                            return Err(RuntimeError::new("Cannot set an different value to an origin var".to_string(), ErrorType::AlwaysError));
                                        }
                                    },
                                    Err(e) => {
                                        return Err(RuntimeError::new(e, AlwaysError));
                                    }
                                }
                            },
                            Err(e) => {
                                return Err(e);
                            }
                        }
                    }

                    _ => {
                        return Err(RuntimeError::new(
                            "Invalid arguments for let".to_string(),
                            ErrorType::AlwaysError,
                        ));
                    }
                }
            }), 
            parser: Arc::new(|a: String, vars: &mut VarMap| {
                let parts = a.split_once('=');

                if parts.is_some() {
                    let (var, value) = parts.unwrap();

                    println!("{}", value.cyan());

                    if parse_type(value, false).is_ok() {
                        return ParseResult::StandardOut(
                            vec![
                                ParseResult::Text(var.trim().to_string()),
                                ParseResult::Text(value.trim().to_string()),
                            ]
                        )
                    }

                    match attempt_logic_parse(value.to_string(), vars) {
                        LogicExpression::Error(error) => return ParseResult::ParseError(error),
                        exp => {
                            
                        }
                        
                    }

                    match attempt_calculator_parse(value.to_string(), vars) {
                        AluExpression::Error(error) => return ParseResult::ParseError(error),
                        alu_expression => {
                            return ParseResult::StandardOut(
                                vec![
                                    ParseResult::Text(var.trim().to_string()),
                                    ParseResult::Alu(alu_expression)
                                ]
                            );
                        }
                    }
                }

                return ParseResult::StandardOut(
                    vec![
                        ParseResult::Text("Problem".to_string()),
                    ]
                )
            }), 
            allowed_in: vec![BlockType::Execute]
        });

        out
    }
}
