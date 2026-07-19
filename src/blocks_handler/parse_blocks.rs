use crate::{blocks_handler::define_blocks::{Block, BlockType, CodeBlock}, execution_policy::ExecutionPolicy};

pub fn attempt_parse(raw: String, policy: &mut ExecutionPolicy) -> Result<Vec<CodeBlock>, String> {
    let blocks = Block::init();
    let lines: Vec<&str> = raw.lines().collect();

    let mut code_blocks: Vec<CodeBlock> = Vec::new();

    for (i, line) in lines.iter().enumerate() {
        if let Some(block) = blocks
            .iter()
            .find(|block| block.match_block(line.trim().to_string()))
        {
            let start_quota = line.to_string();
            let end_quota = block.get_end_quota();

            if let Some(end_i) = lines[i + 1..].iter().position(|l| l.trim() == end_quota) {
                let end_i = i + 1 + end_i;

                let inside = lines[i + 1..end_i].join("\n");

                if let Some(sh) = block.ep_special_handler {
                    sh(policy, inside).map_err(|error| {
                        format!(
                            "Failed to parse {start_quota} block starting at line {}: {error}",
                            i + 1
                        )
                    })?;
                    continue;
                }

                let code_block_type = BlockType::parse(&start_quota);

                if code_block_type == BlockType::Unknown {
                    return Err(format!("Unknown BlockType: {}", start_quota.clone()));
                }
                
                code_blocks.push(CodeBlock::new(inside, code_block_type));
            } else {
                if policy.should_halt_on_code_block_parse_error() {
                    return Err(format!(
                        "Missing end quota at line {} for {}: {}",
                        i + 1,
                        start_quota,
                        end_quota
                    ));
                }
            }
        }
    }

    Ok(code_blocks)
}
