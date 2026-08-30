use crate::riscv_var::{instruction::RvVarInstr, label::Label};
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
        todo!()
    }
}
