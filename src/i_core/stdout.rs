// Path: i_core::stdout::*

use crate::symbol_resolver::Expression;

pub fn println(i: Box<dyn Expression>) -> Option<Box<dyn Expression>> {
    if let Some(value) = i.display() {
        println!("{value}")
    }

    None
}