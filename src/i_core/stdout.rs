// Path: i_core::stdout::*

use crate::symbol_resolver::Expression;

pub fn println(mut args: Vec<Box<dyn Expression>>) -> Option<Box<dyn Expression>> {
    let arg = args.remove(0);

    if let Some(text) = arg.display() {
        println!("{text}");
    }

    None
}