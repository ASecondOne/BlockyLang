// Path: i_core::stdout::*

use crate::{executers::ExcuterOutput, symbol_resolver::Expression};

pub fn println(mut args: Vec<Box<dyn Expression>>) -> ExcuterOutput {
    let arg = args.remove(0);

    if let Some(text) = arg.display() {
        println!("{text}");
    }

    ExcuterOutput::ValidNone
}