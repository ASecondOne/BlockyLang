use std::{io::{self, Write}, sync::Arc};

use colored::Colorize;

use crate::{alu::{Expression, attempt_calculator_parse, attempt_calculator_run}, execution_policy::ExecutionPolicy, output_state, parse_blocks::CommandLine, var_handler::{VarMap, VarType, parse_type}};

enum ParseResult {
    One(String),
    Many(Vec<String>),
    ParseError(String),
    OneAlu(Expression),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlockType {
    Execute, ExecutionPolicy, Define, Unknown
}

impl BlockType {
    pub fn parse(s: &str) -> Self {
        match s[1..s.len() - 1].to_lowercase().as_str() {
            "execute" =>  Self::Execute,
            "executionolicy" => Self::ExecutionPolicy,
            "define" => Self::Define,
            _ => Self::Unknown
        }
    }
}

// Standard escape quota </...>
pub struct Block {
    definition: String,
    pub ep_special_handler: Option<fn(&mut ExecutionPolicy, String) -> Result<(), String>>,
}

impl Block {
    pub fn init() -> Vec<Self> {
        let out = vec![
            Block {
                definition: "<execute>".to_string(),
                ep_special_handler: None,
            },
            Block {
                definition: "<execution_policy>".to_string(),
                ep_special_handler: Some(ExecutionPolicy::change_policy),
            },
            Block { 
                definition: "<define>".to_string(), 
                ep_special_handler: None 
            },
        ];

        out
    }

    pub fn match_block(&self, quota: String) -> bool {
        if self.definition == quota {
            return true;
        }

        false
    }

    pub fn get_end_quota(&self) -> String {
        let mut b = self.definition.clone();
        b.insert(1, '/');
        b
    }
}

// escape Quota: ;
#[derive(Clone)]
pub struct Keyword {
    pub definition: String,
    pub runner: Arc<dyn Fn((&[String], &Option<Expression>), &mut VarMap) -> i32>,
    parser: Arc<dyn Fn(String, &mut VarMap) -> ParseResult>,
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
                            return 0;
                        },
                        Err(e) => {
                            print!("{}", e.as_str().red());
                            output_state::used_print();
                            io::stdout().flush().unwrap();
                            return 1;
                        }
                    }
                }
                if let Some(first) = a.first() {
                    print!("{first}");
                    io::stdout().flush().unwrap();
                    return 0;
                }

                1
            }),
            parser: Arc::new(|a: String, vars: &mut VarMap| {
                let a = a.strip_prefix("print").unwrap().trim();

                if let Some(inside) = a.strip_prefix('"').and_then(|s| s.strip_suffix('"')) {
                    return ParseResult::One(inside.to_string());
                }

                if let Some(value) = vars.get_var(a.to_string()) {
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
            definition: "println".to_string(),
            runner: Arc::new(|(a, b): (&[String], &Option<Expression>), vars: &mut VarMap| {
                if let Some(exp) = b {
                    match attempt_calculator_run(exp, vars) {
                        Ok(v) => {
                            println!("{v}");
                            output_state::used_println();
                            return 0;
                        },
                        Err(e) => {
                            println!("{}", e.as_str().red());
                            output_state::used_println();
                            io::stdout().flush().unwrap();
                            return 1;
                        }
                    }
                }

                if let Some(first) = a.first() {
                    println!("{first}");
                    io::stdout().flush().unwrap();
                    return 0;
                }

                1
            }),
            parser: Arc::new(|a: String, vars: &mut VarMap| {
                let a = a.strip_prefix("println").unwrap().trim();

                if let Some(inside) = a.strip_prefix('"').and_then(|s| s.strip_suffix('"')) {
                    return ParseResult::One(inside.to_string());
                }

                if let Some(value) = vars.get_var(a.to_string()) {
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

                
                
                if let [name, value, ..] = a {
                    vars.add_new(name.to_string(), value.to_string());
                    return 0;
                }

                1
            }),
            parser: Arc::new(|a: String, _vars: &mut VarMap| {

                let Some(rest) = a.strip_prefix("let ") else {
                    return ParseResult::ParseError("expected `let`".to_string());
                };

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

                if let VarType::Unknown = parse_type(value) { 
                    return ParseResult::ParseError("Unknown Data Type".to_string()) 
                }

                ParseResult::Many(vec![name.to_string(), value.to_string()])
            }),
            allowed_in: vec![BlockType::Define] 
        });

        out
    }

    pub fn attempt_parse(mut line: String, keywords: &[Keyword], block_type: BlockType, vars: &mut VarMap) -> Result<CommandLine, String> {
        line = line.trim_end_matches(';').to_string();
        let parts: Vec<&str> = line.split_ascii_whitespace().collect();

        if let Some(first) = parts.first() {
            if let Some(keyword) = keywords.iter().find(|k| k.definition == *first) {
                if keyword.allowed_in.contains(&block_type) {
                    let mut params: (Vec<String>, Option<Expression>) = (Vec::new(), None);

                    match (keyword.parser)(line, vars) {
                        ParseResult::One(s) => {
                            params.0.push(s);
                        }
                        ParseResult::Many(v) => {
                            params.0.extend(v);
                        }
                        ParseResult::OneAlu(exp) => {
                            params.1 = Some(exp);
                        }
                        ParseResult::ParseError(e) => return Err(e),
                    }

                    return Ok(CommandLine::new((*keyword).clone(), params));
                } else {
                    return Err(format!("Keyword {} not allowed inside {:?} block", keyword.definition, block_type))
                }
            }
        }

        Err("I dont know but smt broke".to_string())
    }
}
