use crate::riscv::rv_var::{RvVarAsm, RvVarInstr, RvVarInstrPseudo};
use crate::riscv::rv64imfd::Label;
use std::fmt;

#[derive(Clone, PartialEq)]
pub struct RvVarBasicBlock {
    pub name: Label,
    pub instrs: Vec<RvVarAsm>,
}

impl RvVarBasicBlock {
    pub fn new(name: Label) -> Self {
        RvVarBasicBlock { name, instrs: vec![] }
    }

    pub fn append_instr(&mut self, instr: RvVarInstr) -> &mut Self {
        self.instrs.push(RvVarAsm::Real(instr));
        self
    }

    pub fn append_pesudo(&mut self, pseudo: RvVarInstrPseudo) -> &mut Self {
        self.instrs.push(RvVarAsm::Pseudo(pseudo));
        self
    }

    pub fn append_asm(&mut self, asm: RvVarAsm) -> &mut Self {
        self.instrs.push(asm);
        self
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
