use crate::intu_ir::instruction::Instruction;
use crate::intu_ir::name::Name;
use crate::intu_ir::terminator::Terminator;

#[derive(PartialEq, Clone, Debug, Hash)]
pub struct BasicBlock {
    pub name: Name,
    pub instrs: Vec<Instruction>,
    pub term: Terminator,
}

impl BasicBlock {
    pub fn new(name: Name) -> Self {
        Self { name, instrs: vec![], term: Terminator::Unreachable }
    }
}
