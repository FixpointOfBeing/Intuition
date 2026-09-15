use crate::riscv::rv_var::basicblock::RvVarBasicBlock;
use std::fmt;

// todo: Function { name, params, blocks, entry }，Program = Vec<Function>。

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
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (i, block) in self.blocks.iter().enumerate() {
            if i > 0 {
                writeln!(f)?;
            }
            write!(f, "{block}")?;
        }
        Ok(())
    }
}
