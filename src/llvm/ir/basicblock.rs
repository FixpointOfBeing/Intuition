use crate::llvm::ir::Instruction;
use crate::llvm::ir::Name;
use crate::llvm::ir::Terminator;

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
