// Path: i_core::stdout::*

use crate::{executers::ExcuterOutput, symbol_resolver::SharedExpression};

pub fn println(mut args: Vec<SharedExpression>) -> ExcuterOutput {
    let arg = args.remove(0);

    if let Some(text) = arg.lock().unwrap().display() {
        println!("{text}");
    }

    ExcuterOutput::ValidNone
}
