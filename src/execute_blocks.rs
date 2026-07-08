use std::process::exit;

use colored::Colorize;

use crate::{parse_blocks::CodeBlock, parse_lines::Keyword, var_handler::VarMap};

pub fn parse_execute_block(block: CodeBlock, vars: &mut VarMap) {
    let insides = block.get_inside();

    let keywords = Keyword::init();

    for line in insides.lines().map(str::trim).filter(|l| !l.is_empty()) {
        let out = Keyword::attempt_parse(line.to_string(), &keywords, block.get_block_type(), vars);

        match out {
            Ok(mut o) => {
                o.execute(vars);
            },
            Err(msg) => {
                eprintln!("{}", msg.as_str().red());
                exit(1);
            }
        }
    }
}
