use crate::{blocks_handler::define_blocks::CodeBlock, line_handler::parse_lines::CommandLine, utils::{execution_policy::ExecutionPolicy, runtime_error::{ErrorType, RuntimeError}}, var_handler::VarMap};

pub fn parse_execute_block(block: CodeBlock, vars: &mut VarMap, policy: &ExecutionPolicy) -> Result<(), RuntimeError> {
    let insides = block.get_inside();
    let mut line_to_expand = String::new();

    for line in insides.lines().map(str::trim).filter(|l| !l.is_empty()) {
        let content = line.trim_end_matches(';').trim();

        if !content.is_empty() {
            if !line_to_expand.is_empty() {
                line_to_expand.push(' ');
            }

            line_to_expand.push_str(content);
        }

        if !line.ends_with(';') {
            continue;
        }

        let complete_line = std::mem::take(&mut line_to_expand);

        let mut command = CommandLine::attempt_parse(
            complete_line,
            block.get_block_type(),
            vars,
        )
        .map_err(|msg| RuntimeError::new(msg, ErrorType::AlwaysError))?;

        command.execute(vars, policy)?;
    }

    Ok(())
}