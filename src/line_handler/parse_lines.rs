use std::{format, println};

use colored::Colorize;

use crate::{blocks_handler::define_blocks::BlockType, line_handler::define_lines::{Keyword, ParseResult}, utils::{execution_policy::ExecutionPolicy, runtime_error::RuntimeError}, var_handler::VarMap};

pub struct CommandLine {
    keyword: Keyword,
    params: Vec<ParseResult>,
}

impl CommandLine {
    pub fn new(keyword: Keyword, params: Vec<ParseResult>) -> Self {
        CommandLine { keyword, params }
    }

    pub fn execute(self, vars: &mut VarMap, policy: &ExecutionPolicy) -> Result<(), RuntimeError> {
        (self.keyword.runner)(self.params, vars, policy)
    }

    pub fn attempt_parse(line: String, block_type: BlockType, vars: &mut VarMap) -> Result<CommandLine, String> {
        let keywords = Keyword::init();
        let parts: Vec<&str> = line.split_ascii_whitespace().collect();

        println!("{}", line.as_str().cyan());

        if let Some(first) = parts.first() {

            if vars.var_exists(&first.to_string()) {
                if let Some(keyword) = keywords.iter().find(|k| k.definition == "Set Value") {
                    if keyword.allowed_in.contains(&block_type) { 
                        let mut params = Vec::new();

                        match (keyword.parser)(line, vars) {
                            ParseResult::StandardOut(v) => {
                                params.extend(v);
                            },
                            ParseResult::ParseError(e) => return Err(e),
                            _ => {},
                        }

                        return Ok(CommandLine::new((*keyword).clone(), params));
                    }
                }
            }
            
            if let Some(keyword) = keywords.iter().find(|k| k.definition == *first) {
                if keyword.allowed_in.contains(&block_type) {
                    let mut params = Vec::new();

                    match (keyword.parser)(line, vars) {
                        ParseResult::StandardOut(v) => {
                            params.extend(v);
                        },
                        ParseResult::ParseError(e) => return Err(e),
                        _ => {},
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
