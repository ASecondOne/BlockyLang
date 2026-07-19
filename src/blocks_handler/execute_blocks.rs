use crate::{blocks_handler::define_blocks::CodeBlock, line_handler::parse_lines::CommandLine, utils::runtime_error::RuntimeError, var_handler::VarMap};

pub fn parse_execute_block(block: CodeBlock, vars: &mut VarMap) -> Result<(), RuntimeError> {
    let insides = block.get_inside();

    for line in insides.lines().map(str::trim).filter(|l| !l.is_empty()) {
        let out = CommandLine::attempt_parse(line.to_string(), block.get_block_type(), vars);

        match out {
            Ok(mut o) => {
                match o.execute(vars) {
                    Ok(()) => {},
                    Err(re) => return Err(re)
                }
            },
            Err(msg) => {
                return Err(RuntimeError::new(msg));
            }
        }
    }

    Ok(())
}