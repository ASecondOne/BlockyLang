use crate::parts::Block;

struct CodeBlock {
    pub start_quota: String,
    inside: String,
    end_quota: String,
}

pub fn attempt_parse(raw: String) {
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

            if let Some(end_i) = lines[i + 1..]
                .iter()
                .position(|l| l.trim() == end_quota)
            {
                let end_i = i + 1 + end_i;

                let inside = lines[i + 1..end_i].join("\n");

                code_blocks.push(CodeBlock {
                    start_quota,
                    inside,
                    end_quota,
                });
            } else {
                println!("No end quota found");
            }
        }
    }

    for codeblock in code_blocks {
        println!("Block Found: {}", codeblock.start_quota)
    }
}