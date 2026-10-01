use crate::{executers::ExcuterOutput::{self, ValidNone, ValidSome}, symbolresolver::{localstate::LocalState, symbol_resolver::{ ResolvedBlock, ResolvedExpression, SharedExpression}}};

pub fn dirty_executer(input: Vec<ResolvedBlock>) {
    for block in input {
        for content in block.resolved_lines {
            execute(content, &mut LocalState::default());
        }
    }
}

fn execute(input: ResolvedExpression, local_state: &mut LocalState) -> ExcuterOutput {
    match input {
        ResolvedExpression::Expression(e) => match local_state.resolve(e) {
            Ok(expression) => ValidSome(expression),
            Err(_) => ValidNone,
        },

        ResolvedExpression::KeywordCall(k) => {
            let args: Vec<ExcuterOutput> = k.args
                .into_iter()
                .map(|arg| execute(arg, local_state))
                .collect();

            let args: Vec<SharedExpression> = args
                .into_iter()
                .filter_map(|a| match a {
                    ValidSome(e) => Some(e),
                    ValidNone => {println!("Missing Argument"); None},
                    ExcuterOutput::Error(_) => None,
                })
                .collect();

            (k.keyword.execute)(args, local_state)
        },

        ResolvedExpression::Redirect(into, from) => {
            let out_into = execute(*into, local_state);
            let out_from = execute(*from, local_state);

            if let (ValidSome(into), ValidSome(from)) = (out_into, out_from) {
                if let Ok(mut into) = into.lock() {
                    let _ = into.redirect(from);
                }
            }

            ValidNone
        }

        ResolvedExpression::Closure(mut closure) => {
            closure.local_state = local_state.fork().unwrap_or_default();

            for expression in closure.contents {
                execute(expression, &mut closure.local_state);
            }

            ValidNone
        }
    }
}
