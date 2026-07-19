use crate::{alu::Expression, blocks_handler::define_blocks::BlockType, line_handler::define_lines::{Keyword, ParseResult}, utils::runtime_error::RuntimeError, var_handler::VarMap};

pub struct CommandLine {
    keyword: Keyword,
    params: (Vec<String>, Option<Expression>),
}

impl CommandLine {
    pub fn new(keyword: Keyword, params: (Vec<String>, Option<Expression>)) -> Self {
        CommandLine { keyword, params }
    }

    pub fn execute(&mut self, vars: &mut VarMap) -> Result<(), RuntimeError> {
        (self.keyword.runner)((&self.params.0, &self.params.1), vars)
    }

    pub fn attempt_parse(mut line: String, block_type: BlockType, vars: &mut VarMap) -> Result<CommandLine, String> {
        let keywords = Keyword::init();

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

