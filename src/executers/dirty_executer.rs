use crate::{executers::ExcuterOutput::{self, ValidNone, ValidSome}, symbol_resolver::{Expression, ResolvedBlock, ResolvedExpression}};

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

            let args: Vec<Box<dyn Expression>> = args
                .into_iter()
                .filter_map(|a| match a {
                    ValidSome(e) => Some(e),
                    ValidNone => {println!("Missing Argument"); None},
                    ExcuterOutput::Error(_) => None,
                })
                .collect();

            (k.keyword.execute)(args)
        }
    }
}