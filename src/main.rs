use std::{fs::read_to_string, print};

use blocky_lang::parser::parse_blocks::{Block, parse_blocks};

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

    match parse_blocks(contents) {
        Ok(a) => {
            for b in &a {
                match b {
                    Block::Execute(text) => println!("{text}"),
                }
            }
        },
        Err(er) => {
        }

    }
}