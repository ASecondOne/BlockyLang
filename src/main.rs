use std::fs::read_to_string;

use blocky_lang::{execute_blocks, parse_blocks::attempt_parse};

fn main() {
    let lines = read_to_string("./blocky_src/main.block").unwrap();
    let cmdls = attempt_parse(lines);

    for cmdl in cmdls {
        execute_blocks::parse_execute_block(cmdl);
    }
}
