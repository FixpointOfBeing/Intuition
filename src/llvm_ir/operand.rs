use crate::llvm_ir::types::{TypeRef, Typed, Types};
use crate::llvm_ir::{Constant, ConstantRef, Name};
use std::fmt::{self, Display};

#[derive(PartialEq, Clone, Debug, Hash)]
pub enum Operand {
    LocalOperand {
        name: Name,
        ty: TypeRef,
    },
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

impl Display for Operand {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Operand::LocalOperand { name, ty } => write!(f, "{} {}", ty, name),
            Operand::ConstantOperand(cref) => write!(f, "{}", &cref),
            Operand::MetadataOperand => write!(f, "<metadata>"),
        }
    }
}
