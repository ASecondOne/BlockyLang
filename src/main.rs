use std::fs::read_to_string;

use BlockyLang::parser::attempt_parse;

fn main() {
    println!("Hello, world!");

    let lines = read_to_string("./blocky_src/main.block").unwrap();

    println!("{lines}");

    attempt_parse(lines);
}
