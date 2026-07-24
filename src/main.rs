use std::fs::read_to_string;
use std::process::exit;

use blocky_lang::{blocks_handler::{execute_blocks, parse_blocks::attempt_parse}, utils::{execution_policy::ExecutionPolicy, output_state::take_newline_needed}, var_handler::VarMap};

fn main() {
    let mut policy = ExecutionPolicy::new();
    let mut vars = VarMap::new();

    let lines = read_to_string("./blocky_src/main.block").unwrap();

    let cmdls = match attempt_parse(lines, &mut policy) {
        Ok(blocks) => blocks,
        Err(error) => {
            error.report();
            exit(1);
        }
    };

    for cmdl in cmdls {
        match execute_blocks::parse_execute_block(cmdl, &mut vars, &policy) {
            Ok(()) => {}
            Err(re) => {
                re.report();
                exit(1)
            }
        };
    }

    if take_newline_needed() {
        println!();
    }
}
