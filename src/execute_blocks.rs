use std::process::exit;

use colored::Colorize;

use crate::{parse_blocks::CodeBlock, parse_lines::Keyword};

pub fn parse_execute_block(block: CodeBlock) {
    let insides = block.get_inside();

    let keywords = Keyword::init();

    for line in insides.lines().map(str::trim).filter(|l| !l.is_empty()) {
        let out = Keyword::attempt_parse(line.to_string(), &keywords, block.get_block_type());

        match out {
            Ok(mut o) => o.execute(),
            Err(msg) => {
                eprintln!("{}", msg.as_str().red());
                exit(1);
            }
        }
    }
}