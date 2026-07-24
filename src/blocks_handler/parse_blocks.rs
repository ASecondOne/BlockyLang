use crate::{
    blocks_handler::define_blocks::{Block, BlockType, CodeBlock},
    utils::{
        execution_policy::ExecutionPolicy,
        runtime_error::{ErrorType, RuntimeError},
    },
};

macro_rules! runtime_error {
    ($error_type:expr, $($arg:tt)*) => {
        RuntimeError::new(format!($($arg)*), $error_type)
    };
}

#[allow(clippy::collapsible_else_if)]
pub fn attempt_parse(
    raw: String,
    policy: &mut ExecutionPolicy,
) -> Result<Vec<CodeBlock>, RuntimeError> {
    let blocks = Block::init();
    let lines: Vec<&str> = raw.lines().collect();

    let mut code_blocks = Vec::new();

    for (i, line) in lines.iter().enumerate() {
        if let Some(block) = blocks
            .iter()
            .find(|block| block.match_block(line.trim().to_string()))
        {
            let start_quota = line.to_string();
            let end_quota = block.get_end_quota();

            if let Some(end_i) = lines[i + 1..]
                .iter()
                .position(|line| line.trim() == end_quota)
            {
                let end_i = i + 1 + end_i;
                let inside = lines[i + 1..end_i].join("\n");

                if let Some(handler) = block.ep_special_handler {
                    handler(policy, inside).map_err(|error| {
                        runtime_error!(
                            ErrorType::AlwaysError,
                            "Failed to parse {start_quota} block starting at line {}: {error}",
                            i + 1
                        )
                    })?;

                    continue;
                }

                let code_block_type = BlockType::parse(&start_quota);

                if code_block_type == BlockType::Unknown {
                    return Err(runtime_error!(
                        ErrorType::AlwaysError,
                        "Unknown BlockType: {start_quota}"
                    ));
                }

                code_blocks.push(CodeBlock::new(inside, code_block_type));
            } else {
                policy.handle_error(runtime_error!(
                    ErrorType::OnCodeBlockParseError,
                    "Missing end quota at line {} for {}: {}",
                    i + 1,
                    start_quota,
                    end_quota
                ))?;
            }
        }
    }

    Ok(code_blocks)
}
