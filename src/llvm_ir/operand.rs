use crate::llvm_ir::constant::{Constant, ConstantRef};
use crate::llvm_ir::name::Name;
use crate::llvm_ir::types::{TypeRef, Typed, Types};

#[derive(PartialEq, Clone, Debug, Hash)]
pub enum Operand {
    LocalOperand { name: Name, ty: TypeRef },
    ConstantOperand(ConstantRef),
    MetadataOperand, // --TODO not yet implemented-- MetadataOperand(Box<Metadata>),
}

impl Typed for Operand {
    fn get_type(&self, types: &Types) -> TypeRef {
        match self {
            Operand::LocalOperand { ty, .. } => ty.clone(),
            Operand::ConstantOperand(c) => types.type_of(c),
            Operand::MetadataOperand => types.metadata_type(),
        }
    }
}

impl Operand {
    pub fn as_constant(&self) -> Option<&Constant> {
        match self {
            Operand::ConstantOperand(cref) => Some(cref),
            _ => None,
        }
    }
}
