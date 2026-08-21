use crate::intu_ir::constant::{Constant, ConstantRef};
use crate::intu_ir::name::Name;
use crate::intu_ir::types::{TypeRef, Typed, Types};

#[derive(PartialEq, Clone, Debug, Hash)]
pub enum Operand {
    LocalOperand { name: Name, ty: TypeRef },
    ConstantOperand(ConstantRef),
}

impl Typed for Operand {
    fn get_type(&self, types: &Types) -> TypeRef {
        match self {
            Operand::LocalOperand { ty, .. } => ty.clone(),
            Operand::ConstantOperand(c) => types.type_of(c),
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
