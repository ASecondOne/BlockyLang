use std::fs::read_to_string;

use blocky_lang::{
    combi::variable::VariableMap,
    executer::dirty_executer::execute,
    parser::parse_blocks::{Block, BlockKind, parse_blocks},
};

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

    let mut vars = VariableMap::new();

    match parse_blocks(&contents, &mut vars, BlockKind::Define) {
        Ok(a) => {
            for b in a {
                match b {
                    Block::Define(expressions) => {
                        for expression in expressions {
                            execute(expression.clone(), &mut vars);
                        }
                    }
                    _ => {}
                }
            }
        }
        Err(_er) => {}
    }

    match parse_blocks(&contents, &mut vars, BlockKind::Execute) {
        Ok(a) => {
            for b in a {
                match b {
                    Block::Execute(expressions) => {
                        for expression in expressions {
                            execute(expression.clone(), &mut vars);
                        }
                    }
                    _ => {}
                }
            }
        }
        Err(_er) => {}
    }
}
