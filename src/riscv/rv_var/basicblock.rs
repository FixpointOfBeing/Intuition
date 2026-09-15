use crate::riscv::rv_var::RvVarInstr;
use crate::riscv::rv64imfd::Label;
use std::fmt;

#[derive(Clone, PartialEq)]
pub struct RvVarBasicBlock {
    pub name: Label,
    pub instrs: Vec<RvVarInstr>,
}

impl RvVarBasicBlock {
    pub fn new(name: Label) -> Self {
        RvVarBasicBlock { name, instrs: vec![] }
    }
}

impl fmt::Display for RvVarBasicBlock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{}:", self.name)?;
        for instr in &self.instrs {
            writeln!(f, "    {instr}")?;
        }
        Ok(())
    }
}
