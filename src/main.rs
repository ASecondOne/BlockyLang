use std::fs::read_to_string;
use std::process::exit;

use colored::Colorize;

use blocky_lang::{blocks_handler::{execute_blocks, parse_blocks::attempt_parse}, execution_policy::ExecutionPolicy, utils::output_state::take_newline_needed, var_handler::VarMap};

fn main() {
    let mut policy = ExecutionPolicy::new();
    let mut vars = VarMap::new();

    let lines = read_to_string("./blocky_src/main.block").unwrap();

    let cmdls = match attempt_parse(lines, &mut policy) {
        Ok(blocks) => blocks,
        Err(error) => {
            eprintln!("{}", error.red());
            exit(1);
        }
    };

    for cmdl in cmdls {
        execute_blocks::parse_execute_block(cmdl, &mut vars);
    }

    if take_newline_needed() {
        println!();
    }
}
