use colored::Colorize;

use crate::{
    executers::ExcuterOutput::{self, Error, ValidNone, ValidSome},
    symbol_resolver::{ResolvedBlock, ResolvedExpression, SharedExpression},
};

pub fn executer(input: Vec<ResolvedBlock>) {
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
    match input {
        ResolvedExpression::Expression(expression) => ValidSome(expression),

        ResolvedExpression::KeywordCall(call) => {
            let mut args: Vec<SharedExpression> = Vec::new();

            for arg in call.args {
                match execute(arg) {
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

            (call.keyword.execute)(args)
        }

        ResolvedExpression::Redirect(into, from) => {
            let into = match execute(*into) {
                ValidSome(expression) => expression,
                ValidNone => return Error("Redirect target produced no value".to_string()),
                Error(message) => return Error(message),
            };

            let from = match execute(*from) {
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
    }
}
