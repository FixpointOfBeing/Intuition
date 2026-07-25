use crate::llvm_ir::instruction::Instruction;
use crate::llvm_ir::name::Name;
use crate::llvm_ir::terminator::Terminator;

#[derive(PartialEq, Clone, Debug, Hash)]
pub struct BasicBlock {
    pub name: Name,
    pub instrs: Vec<Instruction>,
    pub term: Terminator,
}

impl BasicBlock {
    pub fn new(name: Name) -> Self {
        use crate::llvm_ir::terminator::Unreachable;
        Self {
            name,
            instrs: vec![],
            term: Terminator::Unreachable(Unreachable {
                // debugloc: None,
            }),
        }
    }
}
