use crate::{executers::ExcuterOutput::{self, Error, ValidNone, ValidSome}, symbol_resolver::{ResolvedBlock, ResolvedExpression, SharedExpression}};

pub fn dirty_executer(input: Vec<ResolvedBlock>) {
    for block in input {
        for content in block.resolved_lines {
            execute(content);
        }
    }
}

fn execute(input: ResolvedExpression) -> ExcuterOutput {
    match input {
        ResolvedExpression::Expression(e) => ValidSome(e),

        ResolvedExpression::KeywordCall(k) => {
            let args: Vec<ExcuterOutput> = k.args
                .into_iter()
                .map(execute)
                .collect();

            let args: Vec<SharedExpression> = args
                .into_iter()
                .filter_map(|a| match a {
                    ValidSome(e) => Some(e),
                    ValidNone => {println!("Missing Argument"); None},
                    ExcuterOutput::Error(_) => None,
                })
                .collect();

            (k.keyword.execute)(args)
        },

        ResolvedExpression::Redirect(into, from) => {
            let out_into = execute(*into);
            let out_from = execute(*from);

            match out_into {
                ValidNone => {}
                ValidSome(e) => {}
                Error(_) => {}
            }

            match out_from {
                ValidNone => {}
                ValidSome(e) => {}
                Error(_) => {}
            }

            ValidNone
        }
    }
}
