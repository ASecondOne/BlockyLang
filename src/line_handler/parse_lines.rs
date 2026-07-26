use std::{format, println};

use colored::Colorize;

use crate::{alu::Expression, blocks_handler::define_blocks::BlockType, line_handler::define_lines::{Keyword, ParseResult}, utils::{execution_policy::ExecutionPolicy, runtime_error::RuntimeError}, var_handler::VarMap};

pub struct CommandLine {
    keyword: Keyword,
    params: (Vec<String>, Option<Expression>),
}

impl CommandLine {
    pub fn new(keyword: Keyword, params: (Vec<String>, Option<Expression>)) -> Self {
        CommandLine { keyword, params }
    }

    pub fn execute(&mut self, vars: &mut VarMap, policy: &ExecutionPolicy) -> Result<(), RuntimeError> {
        (self.keyword.runner)((&self.params.0, &self.params.1), vars, policy)
    }

    pub fn attempt_parse(line: String, block_type: BlockType, vars: &mut VarMap) -> Result<CommandLine, String> {
        let keywords = Keyword::init();
        let parts: Vec<&str> = line.split_ascii_whitespace().collect();

        println!("{}", line.as_str().cyan());

        if let Some(first) = parts.first() {

            if vars.var_exists(&first.to_string()) {
                if let Some(keyword) = keywords.iter().find(|k| k.definition == "Set Value") {
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
                    }
                }
            }
            
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
            } else {
                return Err(format!("Keyword {} dosnt exist", first).to_string());
            }
        }

        Err("I dont know but smt broke".to_string())
    }
}
