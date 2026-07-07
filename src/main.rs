use std::fs::read_to_string;

use BlockyLang::parser::attempt_parse;

fn main() {
    let lines = read_to_string("./blocky_src/main.block").unwrap();
    attempt_parse(lines);
}
