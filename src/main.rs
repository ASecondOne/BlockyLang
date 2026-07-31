use std::fs::read_to_string;

use blocky_lang::parser::parse_blocks::parse_blocks;

pub static PRINT_DEBUG: bool = true;

macro_rules! debug_print {
    ($($value:expr),*) => {
        if PRINT_DEBUG {
            $(
                println!("{}", $value);
            )*
        }
    };
}

fn main() {
    let contents = read_to_string("./blocky_src/HelloWorld.block").unwrap();

    debug_print!("CONTENTS", contents);

    parse_blocks(contents);
}