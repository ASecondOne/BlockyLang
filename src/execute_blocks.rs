use crate::{parse_blocks::CodeBlock, parse_lines::Keyword};

pub fn parse_execute_block(block: CodeBlock) {
    let insides = block.get_inside();

    let keywords = Keyword::init();

    for line in insides.lines().map(str::trim).filter(|l| !l.is_empty()) {
        let out = Keyword::attempt_parse(line.to_string(), &keywords);

        if let Some(mut o) = out {
            o.execute();
        }
    }
}