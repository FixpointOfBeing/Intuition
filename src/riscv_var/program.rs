use crate::riscv_var::basicblock::RvVarBasicBlock;
use std::fmt;
pub struct RvVarProgram {
    pub blocks: Vec<RvVarBasicBlock>,
}

impl RvVarProgram {
    pub fn new() -> Self {
        let blocks = vec![];
        RvVarProgram { blocks }
    }

    pub fn append_basic_block(&mut self, block: RvVarBasicBlock) {
        self.blocks.push(block);
    }
}
impl fmt::Display for RvVarProgram {
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
    }
}
