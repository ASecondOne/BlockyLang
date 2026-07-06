
// Standard escape quota </...>
pub struct Block {
    definition: String

}

impl Block {
    pub fn init() -> Vec<Self> {
        let mut out = Vec::new();

        out.push(Block { definition: "<execute>".to_string() });

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