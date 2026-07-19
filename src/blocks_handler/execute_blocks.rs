use std::process::exit;

use colored::Colorize;

use crate::{blocks_handler::define_blocks::CodeBlock, line_handler::parse_lines::CommandLine, var_handler::VarMap};

pub fn parse_execute_block(block: CodeBlock, vars: &mut VarMap) {
    let insides = block.get_inside();

    for line in insides.lines().map(str::trim).filter(|l| !l.is_empty()) {
        let out = CommandLine::attempt_parse(line.to_string(), block.get_block_type(), vars);

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