use crate::execution_policy::ExecutionPolicy;

pub struct CodeBlock {
    block_type: BlockType,
    inside: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlockType {
    Execute, ExecutionPolicy, Define, Unknown
}

pub struct Block {
    definition: String,
    pub ep_special_handler: Option<fn(&mut ExecutionPolicy, String) -> Result<(), String>>,
}

impl CodeBlock {
    pub fn new(inside: String, block_type: BlockType) -> Self {
        CodeBlock { 
            inside,
            block_type
        }
    }

    pub fn get_inside(&self) -> String {
        self.inside.clone()
    }

    pub fn get_block_type(&self) -> BlockType {
        self.block_type
    }
}

impl BlockType {
    pub fn parse(s: &str) -> Self {
        match s[1..s.len() - 1].to_lowercase().as_str() {
            "execute" =>  Self::Execute,
            "executionolicy" => Self::ExecutionPolicy,
            "define" => Self::Define,
            _ => Self::Unknown
        }
    }
}

impl Block {
    pub fn init() -> Vec<Self> {
        let out = vec![
            Block {
                definition: "<execute>".to_string(),
                ep_special_handler: None,
            },
            Block {
                definition: "<execution_policy>".to_string(),
                ep_special_handler: Some(ExecutionPolicy::change_policy),
            },
            Block { 
                definition: "<define>".to_string(), 
                ep_special_handler: None 
            },
        ];

        out
    }

    pub fn match_block(&self, quota: String) -> bool {
        if self.definition == quota {
            return true;
        }

        false
    }

    pub fn get_end_quota(&self) -> String {
        let mut b = self.definition.clone();
        b.insert(1, '/');
        b
    }
}