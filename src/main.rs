use std::fs::read_to_string;

use blocky_lang::{executer::dirty_executer::execute, parser::parse_blocks::{Block, parse_blocks}};

pub static PRINT_DEBUG: bool = false;

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
    let contents = read_to_string("./blocky_src/Variable.block").unwrap();

    debug_print!("CONTENTS", contents);

    match parse_blocks(contents) {
        Ok(a) => {
            for b in &a {
                match b {
                    Block::Execute(expressions) => {
                        for expression in expressions {
                            execute(expression.clone());
                        }
                    },
                    Block::Define(expressions) => {
                        
                    },
                    Block::None => {
                        
                    }
                }
            }
        },
        Err(_er) => {
        }

    }
}
