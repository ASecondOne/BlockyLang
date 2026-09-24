use crate::symbol_resolver::{Expression, ResolvedBlock, ResolvedExpression};

pub fn dirty_executer(input: Vec<ResolvedBlock>) {
    for block in input {
        for content in block.resolved_lines {
            execute(content);
        }
    }
}

fn execute(input: ResolvedExpression) -> Option<Box<dyn Expression>> {
    match input {
        ResolvedExpression::Expression(e) => Some(e),

        ResolvedExpression::KeywordCall(k) => {
            let args = k.args
                .into_iter()
                .map(|arg| execute(arg).unwrap())
                .collect();

            (k.keyword.execute)(args)
        }
    }
}