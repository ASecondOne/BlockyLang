use std::sync::Arc;

use crate::{execution_policy::ExecutionPolicy, parse_blocks::CommandLine, var_handler::{VarMap, VarType, parse_type}};

enum ParseResult {
    One(String),
    Many(Vec<String>),
    ParseError(String),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlockType {
    Execute, ExecutionPolicy, Define, Unknown
}

impl BlockType {
    pub fn parse(s: &String) -> Self {
        match s[1..s.len() - 1].to_lowercase().as_str() {
            "execute" => return Self::Execute,
            "executionolicy" => return Self::ExecutionPolicy,
            "define" => return Self::Define,
            _ => return Self::Unknown
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
        let mut out = Vec::new();

        out.push(Block {
            definition: "<execute>".to_string(),
            ep_special_handler: None,
        });
        out.push(Block {
            definition: "<execution_policy>".to_string(),
            ep_special_handler: Some(ExecutionPolicy::change_policy),
        });
        out.push(Block { 
            definition: "<define>".to_string(), 
            ep_special_handler: None 
        });

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
    pub runner: Arc<dyn Fn(&[String], &mut VarMap) -> i32>,
    parser: Arc<dyn Fn(String, &mut VarMap) -> ParseResult>,
    pub allowed_in: Vec<BlockType>,
}

impl Keyword {
    pub fn init() -> Vec<Keyword> {
        let mut out = Vec::new();

        out.push(Keyword {
            definition: "print".to_string(),
            runner: Arc::new(|a: &[String], _vars: &mut VarMap| {
                if let Some(first) = a.first() {
                    println!("{first}");
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

                ParseResult::ParseError(format!("Could not parse print value: {a}"))
            }),
            allowed_in: vec![BlockType::Execute]
        });

        out.push(Keyword { 
            definition: "let".to_string(), 
            runner: Arc::new(|a: &[String], vars: &mut VarMap| {
                
                if let [name, value, ..] = a {
                    vars.add_new(name.to_string(), value.to_string());
                }

                1
            }),
            parser: Arc::new(|a: String, _vars: &mut VarMap| {

                let (name, value) = a
                    .strip_prefix("let ")
                    .and_then(|s| s.split_once('='))
                    .map(|(n, v)| (n.trim(), v.trim()))
                    .unwrap();

                match parse_type(value) {
                    VarType::Unknown => return ParseResult::ParseError("Unknown Data Type".to_string()),
                    _ => {}
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
                    let mut params = Vec::new();

                    match (keyword.parser)(line, vars) {
                        ParseResult::One(s) => params.push(s),
                        ParseResult::Many(v) => params.extend(v),
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
