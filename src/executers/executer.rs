use std::sync::{Arc, Mutex};

use colored::Colorize;

use crate::{
    executers::ExcuterOutput::{self, Error, ValidNone, ValidSome}, symbolresolver::{localstate::LocalState, symbol_resolver::{Closure, ResolvedBlock, ResolvedExpression, SharedExpression}},
};

pub fn executer(mut input: Vec<ResolvedBlock>) {

    input.retain(|block| block.block_type.execution_order != 0);
    input.sort_by_key(|block| block.block_type.execution_order);

    for block in input {
        for content in block.resolved_lines {
            if let Error(message) = execute(content) {
                eprintln!("{}", format!("Execution error: {message}").red());
                std::process::exit(1);
            }
        }
    }
}

fn execute(input: ResolvedExpression) -> ExcuterOutput {
    execute_in_state(input, &mut LocalState::default())
}

fn execute_in_state(input: ResolvedExpression, local_state: &mut LocalState) -> ExcuterOutput {
    match input {
        ResolvedExpression::Expression(expression) => match local_state.resolve(expression) {
            Ok(expression) => ValidSome(expression),
            Err(message) => Error(message),
        },

        ResolvedExpression::KeywordCall(call) => {
            let mut args: Vec<SharedExpression> = Vec::new();

            for arg in call.args {
                //? Closures are passed as they are, the keyword decides when (and if) they run
                if let ResolvedExpression::Closure(closure) = arg {
                    args.push(Arc::new(Mutex::new(closure)));
                    continue;
                }

                match execute_in_state(arg, local_state) {
                    ValidSome(expression) => args.push(expression),
                    ValidNone => {
                        return Error(format!(
                            "Missing argument for `{}`",
                            call.keyword.origin
                        ));
                    }
                    Error(message) => return Error(message),
                }
            }

            (call.keyword.execute)(args, local_state)
        }

        ResolvedExpression::Redirect(into, from) => {
            let into = match execute_in_state(*into, local_state) {
                ValidSome(expression) => expression,
                ValidNone => return Error("Redirect target produced no value".to_string()),
                Error(message) => return Error(message),
            };

            let from = match execute_in_state(*from, local_state) {
                ValidSome(expression) => expression,
                ValidNone => return Error("Redirect source produced no value".to_string()),
                Error(message) => return Error(message),
            };

            let mut into = match into.lock() {
                Ok(expression) => expression,
                Err(_) => return Error("Could not lock redirect target".to_string()),
            };

            into.redirect(from)
        }

        ResolvedExpression::Closure(mut closure) => execute_closure(&mut closure, local_state),
    }
}

pub fn execute_closure(closure: &mut Closure, local_state: &mut LocalState) -> ExcuterOutput {
    closure.local_state = match local_state.fork() {
        Ok(state) => state,
        Err(message) => return Error(message),
    };

    // //! std::mem::take empties closure.contents, so a closure can only run once (needs changing for loops)
    for expression in std::mem::take(&mut closure.contents) {
        match execute_in_state(expression, &mut closure.local_state) {
            Error(message) => return Error(message),
            ValidNone | ValidSome(_) => {}
        }
    }

    ValidNone
}
