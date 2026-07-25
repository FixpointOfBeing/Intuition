use crate::llvm_ir::constant::ConstantRef;
// use crate::llvm_ir::debugloc::{DebugLoc, HasDebugLoc};
use crate::llvm_ir::function::{CallingConvention, FunctionAttribute, ParameterAttribute};
use crate::llvm_ir::name::Name;
use crate::llvm_ir::operand::Operand;
use crate::llvm_ir::{predicates::*, Constant};
use crate::llvm_ir::types::{Type, TypeRef, Typed, Types};
use either::Either;
use std::convert::TryFrom;
use std::fmt::{self, Debug, Display};

#[derive(PartialEq, Clone, Debug, Hash)]
pub enum Instruction {
    Add(Add),
    Sub(Sub),
    Mul(Mul),
    UDiv(UDiv),
    SDiv(SDiv),
    URem(URem),
    SRem(SRem),

    And(And),
    Or(Or),
    Xor(Xor),
    Shl(Shl),
    LShr(LShr),
    AShr(AShr),

    FAdd(FAdd),
    FSub(FSub),
    FMul(FMul),
    FDiv(FDiv),
    FRem(FRem),
    FNeg(FNeg),

    ExtractElement(ExtractElement),
    InsertElement(InsertElement),
    ShuffleVector(ShuffleVector),

    ExtractValue(ExtractValue),
    InsertValue(InsertValue),

    Alloca(Alloca),
    Load(Load),
    Store(Store),
    Fence(Fence),
    CmpXchg(CmpXchg),
    AtomicRMW(AtomicRMW),
    GetElementPtr(GetElementPtr),

    Trunc(Trunc),
    ZExt(ZExt),
    SExt(SExt),
    FPTrunc(FPTrunc),
    FPExt(FPExt),
    FPToUI(FPToUI),
    FPToSI(FPToSI),
    UIToFP(UIToFP),
    SIToFP(SIToFP),
    PtrToInt(PtrToInt),
    IntToPtr(IntToPtr),
    BitCast(BitCast),
    AddrSpaceCast(AddrSpaceCast),

    ICmp(ICmp),
    FCmp(FCmp),
    Phi(Phi),
    Select(Select),
    Freeze(Freeze),
    Call(Call),
    VAArg(VAArg),
    LandingPad(LandingPad),
    CatchPad(CatchPad),
    CleanupPad(CleanupPad),
}

impl Typed for Instruction {
    fn get_type(&self, types: &Types) -> TypeRef {
        match self {
            Instruction::Add(i) => types.type_of(i),
            Instruction::Sub(i) => types.type_of(i),
            Instruction::Mul(i) => types.type_of(i),
            Instruction::UDiv(i) => types.type_of(i),
            Instruction::SDiv(i) => types.type_of(i),
            Instruction::URem(i) => types.type_of(i),
            Instruction::SRem(i) => types.type_of(i),
            Instruction::And(i) => types.type_of(i),
            Instruction::Or(i) => types.type_of(i),
            Instruction::Xor(i) => types.type_of(i),
            Instruction::Shl(i) => types.type_of(i),
            Instruction::LShr(i) => types.type_of(i),
            Instruction::AShr(i) => types.type_of(i),
            Instruction::FAdd(i) => types.type_of(i),
            Instruction::FSub(i) => types.type_of(i),
            Instruction::FMul(i) => types.type_of(i),
            Instruction::FDiv(i) => types.type_of(i),
            Instruction::FRem(i) => types.type_of(i),
            Instruction::FNeg(i) => types.type_of(i),
            Instruction::ExtractElement(i) => types.type_of(i),
            Instruction::InsertElement(i) => types.type_of(i),
            Instruction::ShuffleVector(i) => types.type_of(i),
            Instruction::ExtractValue(i) => types.type_of(i),
            Instruction::InsertValue(i) => types.type_of(i),
            Instruction::Alloca(i) => types.type_of(i),
            Instruction::Load(i) => types.type_of(i),
            Instruction::Store(i) => types.type_of(i),
            Instruction::Fence(i) => types.type_of(i),
            Instruction::CmpXchg(i) => types.type_of(i),
            Instruction::AtomicRMW(i) => types.type_of(i),
            Instruction::GetElementPtr(i) => types.type_of(i),
            Instruction::Trunc(i) => types.type_of(i),
            Instruction::ZExt(i) => types.type_of(i),
            Instruction::SExt(i) => types.type_of(i),
            Instruction::FPTrunc(i) => types.type_of(i),
            Instruction::FPExt(i) => types.type_of(i),
            Instruction::FPToUI(i) => types.type_of(i),
            Instruction::FPToSI(i) => types.type_of(i),
            Instruction::UIToFP(i) => types.type_of(i),
            Instruction::SIToFP(i) => types.type_of(i),
            Instruction::PtrToInt(i) => types.type_of(i),
            Instruction::IntToPtr(i) => types.type_of(i),
            Instruction::BitCast(i) => types.type_of(i),
            Instruction::AddrSpaceCast(i) => types.type_of(i),
            Instruction::ICmp(i) => types.type_of(i),
            Instruction::FCmp(i) => types.type_of(i),
            Instruction::Phi(i) => types.type_of(i),
            Instruction::Select(i) => types.type_of(i),
            Instruction::Freeze(i) => types.type_of(i),
            Instruction::Call(i) => types.type_of(i),
            Instruction::VAArg(i) => types.type_of(i),
            Instruction::LandingPad(i) => types.type_of(i),
            Instruction::CatchPad(i) => types.type_of(i),
            Instruction::CleanupPad(i) => types.type_of(i),
        }
    }
}

/* impl HasDebugLoc for Instruction {
    fn get_debug_loc(&self) -> &Option<DebugLoc> {
        match self {
            Instruction::Add(i) => i.get_debug_loc(),
            Instruction::Sub(i) => i.get_debug_loc(),
            Instruction::Mul(i) => i.get_debug_loc(),
            Instruction::UDiv(i) => i.get_debug_loc(),
            Instruction::SDiv(i) => i.get_debug_loc(),
            Instruction::URem(i) => i.get_debug_loc(),
            Instruction::SRem(i) => i.get_debug_loc(),
            Instruction::And(i) => i.get_debug_loc(),
            Instruction::Or(i) => i.get_debug_loc(),
            Instruction::Xor(i) => i.get_debug_loc(),
            Instruction::Shl(i) => i.get_debug_loc(),
            Instruction::LShr(i) => i.get_debug_loc(),
            Instruction::AShr(i) => i.get_debug_loc(),
            Instruction::FAdd(i) => i.get_debug_loc(),
            Instruction::FSub(i) => i.get_debug_loc(),
            Instruction::FMul(i) => i.get_debug_loc(),
            Instruction::FDiv(i) => i.get_debug_loc(),
            Instruction::FRem(i) => i.get_debug_loc(),
            Instruction::FNeg(i) => i.get_debug_loc(),
            Instruction::ExtractElement(i) => i.get_debug_loc(),
            Instruction::InsertElement(i) => i.get_debug_loc(),
            Instruction::ShuffleVector(i) => i.get_debug_loc(),
            Instruction::ExtractValue(i) => i.get_debug_loc(),
            Instruction::InsertValue(i) => i.get_debug_loc(),
            Instruction::Alloca(i) => i.get_debug_loc(),
            Instruction::Load(i) => i.get_debug_loc(),
            Instruction::Store(i) => i.get_debug_loc(),
            Instruction::Fence(i) => i.get_debug_loc(),
            Instruction::CmpXchg(i) => i.get_debug_loc(),
            Instruction::AtomicRMW(i) => i.get_debug_loc(),
            Instruction::GetElementPtr(i) => i.get_debug_loc(),
            Instruction::Trunc(i) => i.get_debug_loc(),
            Instruction::ZExt(i) => i.get_debug_loc(),
            Instruction::SExt(i) => i.get_debug_loc(),
            Instruction::FPTrunc(i) => i.get_debug_loc(),
            Instruction::FPExt(i) => i.get_debug_loc(),
            Instruction::FPToUI(i) => i.get_debug_loc(),
            Instruction::FPToSI(i) => i.get_debug_loc(),
            Instruction::UIToFP(i) => i.get_debug_loc(),
            Instruction::SIToFP(i) => i.get_debug_loc(),
            Instruction::PtrToInt(i) => i.get_debug_loc(),
            Instruction::IntToPtr(i) => i.get_debug_loc(),
            Instruction::BitCast(i) => i.get_debug_loc(),
            Instruction::AddrSpaceCast(i) => i.get_debug_loc(),
            Instruction::ICmp(i) => i.get_debug_loc(),
            Instruction::FCmp(i) => i.get_debug_loc(),
            Instruction::Phi(i) => i.get_debug_loc(),
            Instruction::Select(i) => i.get_debug_loc(),
            Instruction::Freeze(i) => i.get_debug_loc(),
            Instruction::Call(i) => i.get_debug_loc(),
            Instruction::VAArg(i) => i.get_debug_loc(),
            Instruction::LandingPad(i) => i.get_debug_loc(),
            Instruction::CatchPad(i) => i.get_debug_loc(),
            Instruction::CleanupPad(i) => i.get_debug_loc(),
        }
    }
} */

impl Instruction {
    pub fn try_get_result(&self) -> Option<&Name> {
        match self {
            Instruction::Add(i) => Some(&i.dest),
            Instruction::Sub(i) => Some(&i.dest),
            Instruction::Mul(i) => Some(&i.dest),
            Instruction::UDiv(i) => Some(&i.dest),
            Instruction::SDiv(i) => Some(&i.dest),
            Instruction::URem(i) => Some(&i.dest),
            Instruction::SRem(i) => Some(&i.dest),
            Instruction::And(i) => Some(&i.dest),
            Instruction::Or(i) => Some(&i.dest),
            Instruction::Xor(i) => Some(&i.dest),
            Instruction::Shl(i) => Some(&i.dest),
            Instruction::LShr(i) => Some(&i.dest),
            Instruction::AShr(i) => Some(&i.dest),
            Instruction::FAdd(i) => Some(&i.dest),
            Instruction::FSub(i) => Some(&i.dest),
            Instruction::FMul(i) => Some(&i.dest),
            Instruction::FDiv(i) => Some(&i.dest),
            Instruction::FRem(i) => Some(&i.dest),
            Instruction::FNeg(i) => Some(&i.dest),
            Instruction::ExtractElement(i) => Some(&i.dest),
            Instruction::InsertElement(i) => Some(&i.dest),
            Instruction::ShuffleVector(i) => Some(&i.dest),
            Instruction::ExtractValue(i) => Some(&i.dest),
            Instruction::InsertValue(i) => Some(&i.dest),
            Instruction::Alloca(i) => Some(&i.dest),
            Instruction::Load(i) => Some(&i.dest),
            Instruction::Store(_) => None,
            Instruction::Fence(_) => None,
            Instruction::CmpXchg(i) => Some(&i.dest),
            Instruction::AtomicRMW(i) => Some(&i.dest),
            Instruction::GetElementPtr(i) => Some(&i.dest),
            Instruction::Trunc(i) => Some(&i.dest),
            Instruction::ZExt(i) => Some(&i.dest),
            Instruction::SExt(i) => Some(&i.dest),
            Instruction::FPTrunc(i) => Some(&i.dest),
            Instruction::FPExt(i) => Some(&i.dest),
            Instruction::FPToUI(i) => Some(&i.dest),
            Instruction::FPToSI(i) => Some(&i.dest),
            Instruction::UIToFP(i) => Some(&i.dest),
            Instruction::SIToFP(i) => Some(&i.dest),
            Instruction::PtrToInt(i) => Some(&i.dest),
            Instruction::IntToPtr(i) => Some(&i.dest),
            Instruction::BitCast(i) => Some(&i.dest),
            Instruction::AddrSpaceCast(i) => Some(&i.dest),
            Instruction::ICmp(i) => Some(&i.dest),
            Instruction::FCmp(i) => Some(&i.dest),
            Instruction::Phi(i) => Some(&i.dest),
            Instruction::Select(i) => Some(&i.dest),
            Instruction::Freeze(i) => Some(&i.dest),
            Instruction::Call(i) => i.dest.as_ref(),
            Instruction::VAArg(i) => Some(&i.dest),
            Instruction::LandingPad(i) => Some(&i.dest),
            Instruction::CatchPad(i) => Some(&i.dest),
            Instruction::CleanupPad(i) => Some(&i.dest),
        }
    }

    pub fn is_atomic(&self) -> bool {
        match self {
            Instruction::Add(_) => false,
            Instruction::Sub(_) => false,
            Instruction::Mul(_) => false,
            Instruction::UDiv(_) => false,
            Instruction::SDiv(_) => false,
            Instruction::URem(_) => false,
            Instruction::SRem(_) => false,
            Instruction::And(_) => false,
            Instruction::Or(_) => false,
            Instruction::Xor(_) => false,
            Instruction::Shl(_) => false,
            Instruction::LShr(_) => false,
            Instruction::AShr(_) => false,
            Instruction::FAdd(_) => false,
            Instruction::FSub(_) => false,
            Instruction::FMul(_) => false,
            Instruction::FDiv(_) => false,
            Instruction::FRem(_) => false,
            Instruction::FNeg(_) => false,
            Instruction::ExtractElement(_) => false,
            Instruction::InsertElement(_) => false,
            Instruction::ShuffleVector(_) => false,
            Instruction::ExtractValue(_) => false,
            Instruction::InsertValue(_) => false,
            Instruction::Alloca(_) => false,
            Instruction::Load(i) => i.atomicity.is_some(),
            Instruction::Store(i) => i.atomicity.is_some(),
            Instruction::Fence(_) => true,
            Instruction::CmpXchg(_) => true,
            Instruction::AtomicRMW(_) => true,
            Instruction::GetElementPtr(_) => false,
            Instruction::Trunc(_) => false,
            Instruction::ZExt(_) => false,
            Instruction::SExt(_) => false,
            Instruction::FPTrunc(_) => false,
            Instruction::FPExt(_) => false,
            Instruction::FPToUI(_) => false,
            Instruction::FPToSI(_) => false,
            Instruction::UIToFP(_) => false,
            Instruction::SIToFP(_) => false,
            Instruction::PtrToInt(_) => false,
            Instruction::IntToPtr(_) => false,
            Instruction::BitCast(_) => false,
            Instruction::AddrSpaceCast(_) => false,
            Instruction::ICmp(_) => false,
            Instruction::FCmp(_) => false,
            Instruction::Phi(_) => false,
            Instruction::Select(_) => false,
            Instruction::Freeze(_) => false,
            Instruction::Call(_) => false,
            Instruction::VAArg(_) => false,
            Instruction::LandingPad(_) => false,
            Instruction::CatchPad(_) => false,
            Instruction::CleanupPad(_) => false,
        }
    }
}

/* --TODO not yet implemented: metadata
pub trait HasMetadata {
    fn get_metadata(&self) -> &InstructionMetadata;
}

impl HasMetadata for Instruction {
    fn get_metadata(&self) -> &InstructionMetadata {
        match self {
            Instruction::Add(i) => &i.metadata,
            Instruction::Sub(i) => &i.metadata,
            Instruction::Mul(i) => &i.metadata,
            Instruction::UDiv(i) => &i.metadata,
            Instruction::SDiv(i) => &i.metadata,
            Instruction::URem(i) => &i.metadata,
            Instruction::SRem(i) => &i.metadata,
            Instruction::And(i) => &i.metadata,
            Instruction::Or(i) => &i.metadata,
            Instruction::Xor(i) => &i.metadata,
            Instruction::Shl(i) => &i.metadata,
            Instruction::LShr(i) => &i.metadata,
            Instruction::AShr(i) => &i.metadata,
            Instruction::FAdd(i) => &i.metadata,
            Instruction::FSub(i) => &i.metadata,
            Instruction::FMul(i) => &i.metadata,
            Instruction::FDiv(i) => &i.metadata,
            Instruction::FRem(i) => &i.metadata,
            Instruction::FNeg(i) => &i.metadata,
            Instruction::ExtractElement(i) => &i.metadata,
            Instruction::InsertElement(i) => &i.metadata,
            Instruction::ShuffleVector(i) => &i.metadata,
            Instruction::ExtractValue(i) => &i.metadata,
            Instruction::InsertValue(i) => &i.metadata,
            Instruction::Alloca(i) => &i.metadata,
            Instruction::Load(i) => &i.metadata,
            Instruction::Store(i) => &i.metadata,
            Instruction::Fence(i) => &i.metadata,
            Instruction::CmpXchg(i) => &i.metadata,
            Instruction::AtomicRMW(i) => &i.metadata,
            Instruction::GetElementPtr(i) => &i.metadata,
            Instruction::Trunc(i) => &i.metadata,
            Instruction::ZExt(i) => &i.metadata,
            Instruction::SExt(i) => &i.metadata,
            Instruction::FPTrunc(i) => &i.metadata,
            Instruction::FPExt(i) => &i.metadata,
            Instruction::FPToUI(i) => &i.metadata,
            Instruction::FPToSI(i) => &i.metadata,
            Instruction::UIToFP(i) => &i.metadata,
            Instruction::SIToFP(i) => &i.metadata,
            Instruction::PtrToInt(i) => &i.metadata,
            Instruction::IntToPtr(i) => &i.metadata,
            Instruction::BitCast(i) => &i.metadata,
            Instruction::AddrSpaceCast(i) => &i.metadata,
            Instruction::ICmp(i) => &i.metadata,
            Instruction::FCmp(i) => &i.metadata,
            Instruction::Phi(i) => &i.metadata,
            Instruction::Select(i) => &i.metadata,
            Instruction::Freeze(i) => &i.metadata,
            Instruction::Call(i) => &i.metadata,
            Instruction::VAArg(i) => &i.metadata,
            Instruction::LandingPad(i) => &i.metadata,
            Instruction::CatchPad(i) => &i.metadata,
            Instruction::CleanupPad(i) => &i.metadata,
        }
    }
}
*/

pub trait HasResult: Debug + Typed {
    fn get_result(&self) -> &Name;
}

pub trait IsUnaryOp: HasResult {
    fn get_operand(&self) -> &Operand;
}

pub trait IsBinaryOp: HasResult {
    fn get_operand0(&self) -> &Operand;
    fn get_operand1(&self) -> &Operand;
}

impl Instruction {
    pub fn is_binary_op(&self) -> bool {
        match self {
            Instruction::Add(_) => true,
            Instruction::Sub(_) => true,
            Instruction::Mul(_) => true,
            Instruction::UDiv(_) => true,
            Instruction::SDiv(_) => true,
            Instruction::URem(_) => true,
            Instruction::SRem(_) => true,
            Instruction::And(_) => true,
            Instruction::Or(_) => true,
            Instruction::Xor(_) => true,
            Instruction::Shl(_) => true,
            Instruction::LShr(_) => true,
            Instruction::AShr(_) => true,
            Instruction::FAdd(_) => true,
            Instruction::FSub(_) => true,
            Instruction::FMul(_) => true,
            Instruction::FDiv(_) => true,
            Instruction::FRem(_) => true,
            _ => false,
        }
    }

    pub fn is_unary_op(&self) -> bool {
        match self {
            Instruction::AddrSpaceCast(_) => true,
            Instruction::BitCast(_) => true,
            Instruction::FNeg(_) => true,
            Instruction::FPExt(_) => true,
            Instruction::FPToSI(_) => true,
            Instruction::FPToUI(_) => true,
            Instruction::FPTrunc(_) => true,
            Instruction::Freeze(_) => true,
            Instruction::IntToPtr(_) => true,
            Instruction::PtrToInt(_) => true,
            Instruction::SExt(_) => true,
            Instruction::SIToFP(_) => true,
            Instruction::Trunc(_) => true,
            Instruction::UIToFP(_) => true,
            Instruction::ZExt(_) => true,
            _ => false,
        }
    }
}

impl Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Instruction::Add(i) => write!(f, "{}", i),
            Instruction::Sub(i) => write!(f, "{}", i),
            Instruction::Mul(i) => write!(f, "{}", i),
            Instruction::UDiv(i) => write!(f, "{}", i),
            Instruction::SDiv(i) => write!(f, "{}", i),
            Instruction::URem(i) => write!(f, "{}", i),
            Instruction::SRem(i) => write!(f, "{}", i),
            Instruction::And(i) => write!(f, "{}", i),
            Instruction::Or(i) => write!(f, "{}", i),
            Instruction::Xor(i) => write!(f, "{}", i),
            Instruction::Shl(i) => write!(f, "{}", i),
            Instruction::LShr(i) => write!(f, "{}", i),
            Instruction::AShr(i) => write!(f, "{}", i),
            Instruction::FAdd(i) => write!(f, "{}", i),
            Instruction::FSub(i) => write!(f, "{}", i),
            Instruction::FMul(i) => write!(f, "{}", i),
            Instruction::FDiv(i) => write!(f, "{}", i),
            Instruction::FRem(i) => write!(f, "{}", i),
            Instruction::FNeg(i) => write!(f, "{}", i),
            Instruction::ExtractElement(i) => write!(f, "{}", i),
            Instruction::InsertElement(i) => write!(f, "{}", i),
            Instruction::ShuffleVector(i) => write!(f, "{}", i),
            Instruction::ExtractValue(i) => write!(f, "{}", i),
            Instruction::InsertValue(i) => write!(f, "{}", i),
            Instruction::Alloca(i) => write!(f, "{}", i),
            Instruction::Load(i) => write!(f, "{}", i),
            Instruction::Store(i) => write!(f, "{}", i),
            Instruction::Fence(i) => write!(f, "{}", i),
            Instruction::CmpXchg(i) => write!(f, "{}", i),
            Instruction::AtomicRMW(i) => write!(f, "{}", i),
            Instruction::GetElementPtr(i) => write!(f, "{}", i),
            Instruction::Trunc(i) => write!(f, "{}", i),
            Instruction::ZExt(i) => write!(f, "{}", i),
            Instruction::SExt(i) => write!(f, "{}", i),
            Instruction::FPTrunc(i) => write!(f, "{}", i),
            Instruction::FPExt(i) => write!(f, "{}", i),
            Instruction::FPToUI(i) => write!(f, "{}", i),
            Instruction::FPToSI(i) => write!(f, "{}", i),
            Instruction::UIToFP(i) => write!(f, "{}", i),
            Instruction::SIToFP(i) => write!(f, "{}", i),
            Instruction::PtrToInt(i) => write!(f, "{}", i),
            Instruction::IntToPtr(i) => write!(f, "{}", i),
            Instruction::BitCast(i) => write!(f, "{}", i),
            Instruction::AddrSpaceCast(i) => write!(f, "{}", i),
            Instruction::ICmp(i) => write!(f, "{}", i),
            Instruction::FCmp(i) => write!(f, "{}", i),
            Instruction::Phi(i) => write!(f, "{}", i),
            Instruction::Select(i) => write!(f, "{}", i),
            Instruction::Freeze(i) => write!(f, "{}", i),
            Instruction::Call(i) => write!(f, "{}", i),
            Instruction::VAArg(i) => write!(f, "{}", i),
            Instruction::LandingPad(i) => write!(f, "{}", i),
            Instruction::CatchPad(i) => write!(f, "{}", i),
            Instruction::CleanupPad(i) => write!(f, "{}", i),
        }
    }
}

macro_rules! impl_inst {
    ($inst:ty, $id:ident) => {
        impl From<$inst> for Instruction {
            fn from(inst: $inst) -> Instruction {
                Instruction::$id(inst)
            }
        }

        impl TryFrom<Instruction> for $inst {
            type Error = &'static str;
            fn try_from(inst: Instruction) -> Result<Self, Self::Error> {
                match inst {
                    Instruction::$id(inst) => Ok(inst),
                    _ => Err("Instruction is not of requested type"),
                }
            }
        }

       /*  impl HasDebugLoc for $inst {
            fn get_debug_loc(&self) -> &Option<DebugLoc> {
                &self.debugloc
            }
        } */

        /* --TODO not yet implemented: metadata
        impl HasMetadata for $inst {
            fn get_metadata(&self) -> &InstructionMetadata {
                &self.metadata
            }
        }
        */
    };
}

macro_rules! impl_hasresult {
    ($inst:ty) => {
        impl HasResult for $inst {
            fn get_result(&self) -> &Name {
                &self.dest
            }
        }
    };
}

macro_rules! impl_unop {
    ($inst:ty) => {
        impl_hasresult!($inst);

        impl IsUnaryOp for $inst {
            fn get_operand(&self) -> &Operand {
                &self.operand
            }
        }
    };
}

macro_rules! impl_binop {
    ($inst:ty, $id:ident) => {
        impl_hasresult!($inst);

        impl IsBinaryOp for $inst {
            fn get_operand0(&self) -> &Operand {
                &self.operand0
            }
            fn get_operand1(&self) -> &Operand {
                &self.operand1
            }
        }

        impl From<$inst> for BinaryOp {
            fn from(inst: $inst) -> Self {
                BinaryOp::$id(inst)
            }
        }

        impl TryFrom<BinaryOp> for $inst {
            type Error = &'static str;
            fn try_from(bo: BinaryOp) -> Result<Self, Self::Error> {
                match bo {
                    BinaryOp::$id(i) => Ok(i),
                    _ => Err("BinaryOp is not of requested type"),
                }
            }
        }
    };
}

macro_rules! binop_display {
    ($inst:ty, $dispname:expr) => {
        impl Display for $inst {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(
                    f,
                    "{} = {} {}, {}",
                    &self.dest, $dispname, &self.operand0, &self.operand1,
                )?;
                // if self.debugloc.is_some() {
                //     write!(f, " (with debugloc)")?;
                // }
                Ok(())
            }
        }
    };
}

macro_rules! binop_display_with_flags {
    ($inst:ty, $dispname:expr, ($($flag_display:expr ; $flag_field:ident ; $required_feature:expr),*)) => {
        impl Display for $inst {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(
                    f,
                    "{} = {}",
                    &self.dest, $dispname
                )?;

                $( #[cfg(feature = $required_feature)] if self.$flag_field { write!(f, " {}", $flag_display)?; })*

                write!(
                    f,
                    " {}, {}",
                    &self.operand0, &self.operand1,
                )?;

                // if self.debugloc.is_some() {
                //     write!(f, " (with debugloc)")?;
                // }
                Ok(())
            }
        }
    };
}

macro_rules! unop_same_type {
    ($inst:ty, $dispname:expr) => {
        impl_unop!($inst);

        impl Typed for $inst {
            fn get_type(&self, types: &Types) -> TypeRef {
                types.type_of(self.get_operand())
            }
        }

        impl Display for $inst {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "{} = {} {}", &self.dest, $dispname, &self.operand)?;
                // if self.debugloc.is_some() {
                //     write!(f, " (with debugloc)")?;
                // }
                Ok(())
            }
        }
    };
}

macro_rules! unop_typed_display_with_flags {
    ($inst:ty, $dispname:expr, ($($flag_display:expr ; $flag_field:ident ; $required_feature:expr),*)) => {
        impl Display for $inst {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(
                    f,
                    "{} = {}",
                    &self.dest, $dispname
                )?;

                $( #[cfg(feature = $required_feature)] if self.$flag_field { write!(f, " {}", $flag_display)?; })*

                write!(
                    f,
                    " {} to {}",
                    &self.operand, &self.to_type
                )?;

                // if self.debugloc.is_some() {
                //     write!(f, " (with debugloc)")?;
                // }
                Ok(())
            }
        }
    };
}

macro_rules! unop_explicitly_typed {
    ($inst:ty, $dispname:expr) => {
        impl_unop!($inst);
        explicitly_typed!($inst);

        impl Display for $inst {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(
                    f,
                    "{} = {} {} to {}",
                    &self.dest, $dispname, &self.operand, &self.to_type,
                )?;
                // if self.debugloc.is_some() {
                //     write!(f, " (with debugloc)")?;
                // }
                Ok(())
            }
        }
    };
}

macro_rules! binop_same_type {
    ($inst:ty) => {
        impl Typed for $inst {
            fn get_type(&self, types: &Types) -> TypeRef {
                let ty = types.type_of(self.get_operand0());
                debug_assert_eq!(ty, types.type_of(self.get_operand1()));
                ty
            }
        }
    };
}

macro_rules! binop_left_type {
    ($inst:ty) => {
        impl Typed for $inst {
            fn get_type(&self, types: &Types) -> TypeRef {
                types.type_of(self.get_operand0())
            }
        }
    };
}

macro_rules! explicitly_typed {
    ($inst:ty) => {
        impl Typed for $inst {
            fn get_type(&self, _types: &Types) -> TypeRef {
                self.to_type.clone()
            }
        }
    };
}

macro_rules! void_typed {
    ($inst:ty) => {
        impl Typed for $inst {
            fn get_type(&self, types: &Types) -> TypeRef {
                types.void()
            }
        }
    };
}

#[derive(PartialEq, Clone, Debug, Hash)]
pub struct Add {
    pub operand0: Operand,
    pub operand1: Operand,
    pub dest: Name,
    pub nuw: bool, // prior to LLVM 17, no getter for this was exposed in the LLVM C API, only in the C++ one
    pub nsw: bool, // prior to LLVM 17, no getter for this was exposed in the LLVM C API, only in the C++ one
    // pub debugloc: Option<DebugLoc>,
}

impl_inst!(Add, Add);
impl_binop!(Add, Add);
binop_same_type!(Add);
binop_display_with_flags!(Add, "add", ("nuw" ; nuw ; "llvm-17-or-greater", "nsw" ; nsw ; "llvm-17-or-greater"));

#[derive(PartialEq, Clone, Debug, Hash)]
pub struct Sub {
    pub operand0: Operand,
    pub operand1: Operand,
    pub dest: Name,
    pub nuw: bool, // prior to LLVM 17, no getter for this was exposed in the LLVM C API, only in the C++ one
    pub nsw: bool, // prior to LLVM 17, no getter for this was exposed in the LLVM C API, only in the C++ one
    // pub debugloc: Option<DebugLoc>,
}

impl_inst!(Sub, Sub);
impl_binop!(Sub, Sub);
binop_same_type!(Sub);
binop_display_with_flags!(Sub, "sub", ("nuw" ; nuw ; "llvm-17-or-greater", "nsw" ; nsw ; "llvm-17-or-greater"));

#[derive(PartialEq, Clone, Debug, Hash)]
pub struct Mul {
    pub operand0: Operand,
    pub operand1: Operand,
    pub dest: Name,
    pub nuw: bool, // prior to LLVM 17, no getter for this was exposed in the LLVM C API, only in the C++ one
    pub nsw: bool, // prior to LLVM 17, no getter for this was exposed in the LLVM C API, only in the C++ one
    // pub debugloc: Option<DebugLoc>,
}

impl_inst!(Mul, Mul);
impl_binop!(Mul, Mul);
binop_same_type!(Mul);
binop_display_with_flags!(Mul, "mul", ("nuw" ; nuw ; "llvm-17-or-greater", "nsw" ; nsw ; "llvm-17-or-greater"));

#[derive(PartialEq, Clone, Debug, Hash)]
pub struct UDiv {
    pub operand0: Operand,
    pub operand1: Operand,
    pub dest: Name,
    pub exact: bool, // prior to LLVM 17, no getter for this was exposed in the LLVM C API, only in the C++ one
    // pub debugloc: Option<DebugLoc>,
}

impl_inst!(UDiv, UDiv);
impl_binop!(UDiv, UDiv);
binop_same_type!(UDiv);
binop_display_with_flags!(UDiv, "udiv", ("exact" ; exact ; "llvm-17-or-greater"));

#[derive(PartialEq, Clone, Debug, Hash)]
pub struct SDiv {
    pub operand0: Operand,
    pub operand1: Operand,
    pub dest: Name,
    pub exact: bool, // prior to LLVM 17, no getter for this was exposed in the LLVM C API, only in the C++ one
    // pub debugloc: Option<DebugLoc>,
}

impl_inst!(SDiv, SDiv);
impl_binop!(SDiv, SDiv);
binop_same_type!(SDiv);
binop_display_with_flags!(SDiv, "sdiv", ("exact" ; exact ; "llvm-17-or-greater"));

#[derive(PartialEq, Clone, Debug, Hash)]
pub struct URem {
    pub operand0: Operand,
    pub operand1: Operand,
    pub dest: Name,
    // pub debugloc: Option<DebugLoc>,
}

impl_inst!(URem, URem);
impl_binop!(URem, URem);
binop_same_type!(URem);
binop_display!(URem, "urem");

#[derive(PartialEq, Clone, Debug, Hash)]
pub struct SRem {
    pub operand0: Operand,
    pub operand1: Operand,
    pub dest: Name,
    // pub debugloc: Option<DebugLoc>,
}

impl_inst!(SRem, SRem);
impl_binop!(SRem, SRem);
binop_same_type!(SRem);
binop_display!(SRem, "srem");

#[derive(PartialEq, Clone, Debug, Hash)]
pub struct And {
    pub operand0: Operand,
    pub operand1: Operand,
    pub dest: Name,
    // pub debugloc: Option<DebugLoc>,
}

impl_inst!(And, And);
impl_binop!(And, And);
binop_same_type!(And);
binop_display!(And, "and");

#[derive(PartialEq, Clone, Debug, Hash)]
pub struct Or {
    pub operand0: Operand,
    pub operand1: Operand,
    pub dest: Name,
    pub disjoint: bool,
    // pub debugloc: Option<DebugLoc>,
}

impl_inst!(Or, Or);
impl_binop!(Or, Or);
binop_same_type!(Or);
binop_display_with_flags!(Or, "or", ("disjoint" ; disjoint ; "llvm-18-or-greater"));

#[derive(PartialEq, Clone, Debug, Hash)]
pub struct Xor {
    pub operand0: Operand,
    pub operand1: Operand,
    pub dest: Name,
    // pub debugloc: Option<DebugLoc>,
}

impl_inst!(Xor, Xor);
impl_binop!(Xor, Xor);
binop_same_type!(Xor);
binop_display!(Xor, "xor");

#[derive(PartialEq, Clone, Debug, Hash)]
pub struct Shl {
    pub operand0: Operand,
    pub operand1: Operand,
    pub dest: Name,
    pub nuw: bool, // prior to LLVM 17, no getter for this was exposed in the LLVM C API, only in the C++ one
    pub nsw: bool, // prior to LLVM 17, no getter for this was exposed in the LLVM C API, only in the C++ one
    // pub debugloc: Option<DebugLoc>,
}

impl_inst!(Shl, Shl);
impl_binop!(Shl, Shl);
binop_left_type!(Shl);
binop_display_with_flags!(Shl, "shl", ("nuw" ; nuw ; "llvm-17-or-greater", "nsw" ; nsw ; "llvm-17-or-greater"));

#[derive(PartialEq, Clone, Debug, Hash)]
pub struct LShr {
    pub operand0: Operand,
    pub operand1: Operand,
    pub dest: Name,
    pub exact: bool, // prior to LLVM 17, no getter for this was exposed in the LLVM C API, only in the C++ one
    // pub debugloc: Option<DebugLoc>,
}

impl_inst!(LShr, LShr);
impl_binop!(LShr, LShr);
binop_left_type!(LShr);
binop_display_with_flags!(LShr, "lshr", ("exact" ; exact ; "llvm-17-or-greater"));

#[derive(PartialEq, Clone, Debug, Hash)]
pub struct AShr {
    pub operand0: Operand,
    pub operand1: Operand,
    pub dest: Name,
    pub exact: bool, // prior to LLVM 17, no getter for this was exposed in the LLVM C API, only in the C++ one
    // pub debugloc: Option<DebugLoc>,
}

impl_inst!(AShr, AShr);
impl_binop!(AShr, AShr);
binop_left_type!(AShr);
binop_display_with_flags!(AShr, "ashr", ("exact" ; exact ; "llvm-17-or-greater"));

#[derive(PartialEq, Clone, Debug, Hash)]
pub struct FAdd {
    pub operand0: Operand,
    pub operand1: Operand,
    pub dest: Name,
    // pub debugloc: Option<DebugLoc>,
}

impl_inst!(FAdd, FAdd);
impl_binop!(FAdd, FAdd);
binop_same_type!(FAdd);
binop_display!(FAdd, "fadd");

#[derive(PartialEq, Clone, Debug, Hash)]
pub struct FSub {
    pub operand0: Operand,
    pub operand1: Operand,
    pub dest: Name,
    // pub debugloc: Option<DebugLoc>,
}

impl_inst!(FSub, FSub);
impl_binop!(FSub, FSub);
binop_same_type!(FSub);
binop_display!(FSub, "fsub");

#[derive(PartialEq, Clone, Debug, Hash)]
pub struct FMul {
    pub operand0: Operand,
    pub operand1: Operand,
    pub dest: Name,
    // pub debugloc: Option<DebugLoc>,
}

impl_inst!(FMul, FMul);
impl_binop!(FMul, FMul);
binop_same_type!(FMul);
binop_display!(FMul, "fmul");

#[derive(PartialEq, Clone, Debug, Hash)]
pub struct FDiv {
    pub operand0: Operand,
    pub operand1: Operand,
    pub dest: Name,
    // pub debugloc: Option<DebugLoc>,
}

impl_inst!(FDiv, FDiv);
impl_binop!(FDiv, FDiv);
binop_same_type!(FDiv);
binop_display!(FDiv, "fdiv");

#[derive(PartialEq, Clone, Debug, Hash)]
pub struct FRem {
    pub operand0: Operand,
    pub operand1: Operand,
    pub dest: Name,
    // pub debugloc: Option<DebugLoc>,
}

impl_inst!(FRem, FRem);
impl_binop!(FRem, FRem);
binop_same_type!(FRem);
binop_display!(FRem, "frem");

#[derive(PartialEq, Clone, Debug, Hash)]
pub struct FNeg {
    pub operand: Operand,
    pub dest: Name,
    // pub debugloc: Option<DebugLoc>,
}

impl_inst!(FNeg, FNeg);
unop_same_type!(FNeg, "fneg");

#[derive(PartialEq, Clone, Debug, Hash)]
pub struct ExtractElement {
    pub vector: Operand,
    pub index: Operand,
    pub dest: Name,
    // pub debugloc: Option<DebugLoc>,
}

impl_inst!(ExtractElement, ExtractElement);
impl_hasresult!(ExtractElement);

impl Typed for ExtractElement {
    fn get_type(&self, types: &Types) -> TypeRef {
        match types.type_of(&self.vector).as_ref() {
            Type::VectorType { element_type, .. } => element_type.clone(),
            ty => panic!(
                "Expected an ExtractElement vector to be VectorType, got {:?}",
                ty
            ),
        }
    }
}

impl Display for ExtractElement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{} = extractelement {}, {}",
            &self.dest, &self.vector, &self.index,
        )?;
        // if self.debugloc.is_some() {
        //     write!(f, " (with debugloc)")?;
        // }
        Ok(())
    }
}

#[derive(PartialEq, Clone, Debug, Hash)]
pub struct InsertElement {
    pub vector: Operand,
    pub element: Operand,
    pub index: Operand,
    pub dest: Name,
    // pub debugloc: Option<DebugLoc>,
}

impl_inst!(InsertElement, InsertElement);
impl_hasresult!(InsertElement);

impl Typed for InsertElement {
    fn get_type(&self, types: &Types) -> TypeRef {
        types.type_of(&self.vector)
    }
}

impl Display for InsertElement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{} = insertelement {}, {}, {}",
            &self.dest, &self.vector, &self.element, &self.index,
        )?;
        // if self.debugloc.is_some() {
        //     write!(f, " (with debugloc)")?;
        // }
        Ok(())
    }
}

#[derive(PartialEq, Clone, Debug, Hash)]
pub struct ShuffleVector {
    pub operand0: Operand,
    pub operand1: Operand,
    pub dest: Name,
    pub mask: ConstantRef,
    // pub debugloc: Option<DebugLoc>,
}

impl_inst!(ShuffleVector, ShuffleVector);
impl_hasresult!(ShuffleVector);

impl Typed for ShuffleVector {
    fn get_type(&self, types: &Types) -> TypeRef {
        let ty = types.type_of(&self.operand0);
        debug_assert_eq!(ty, types.type_of(&self.operand1));
        match ty.as_ref() {
            Type::VectorType { element_type, .. } => match types.type_of(&self.mask).as_ref() {
                Type::VectorType {
                    num_elements,
                    scalable,
                    ..
                } => types.vector_of(element_type.clone(), *num_elements, *scalable),
                ty => panic!(
                    "Expected a ShuffleVector mask to be VectorType, got {:?}",
                    ty
                ),
            },
            _ => panic!(
                "Expected a ShuffleVector operand to be VectorType, got {:?}",
                ty
            ),
        }
    }
}

impl Display for ShuffleVector {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{} = shufflevector {}, {}, {}",
            &self.dest, &self.operand0, &self.operand1, &self.mask,
        )?;
        // if self.debugloc.is_some() {
        //     write!(f, " (with debugloc)")?;
        // }
        Ok(())
    }
}

#[derive(PartialEq, Clone, Debug, Hash)]
pub struct ExtractValue {
    pub aggregate: Operand,
    pub indices: Vec<u32>,
    pub dest: Name,
    // pub debugloc: Option<DebugLoc>,
}

impl_inst!(ExtractValue, ExtractValue);
impl_hasresult!(ExtractValue);

impl Typed for ExtractValue {
    fn get_type(&self, types: &Types) -> TypeRef {
        ev_type(types.type_of(&self.aggregate), self.indices.iter().copied())
    }
}

fn ev_type(cur_type: TypeRef, mut indices: impl Iterator<Item = u32>) -> TypeRef {
    match indices.next() {
        None => cur_type,
        Some(index) => match cur_type.as_ref() {
            Type::ArrayType { element_type, .. } => ev_type(element_type.clone(), indices),
            Type::StructType { element_types, .. } => ev_type(
                element_types
                    .get(index as usize)
                    .expect("ExtractValue index out of range")
                    .clone(),
                indices,
            ),
            _ => panic!(
                "ExtractValue from something that's not ArrayType or StructType; its type is {:?}",
                cur_type
            ),
        },
    }
}

impl Display for ExtractValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{} = extractvalue {}, {}",
            &self.dest,
            &self.aggregate,
            &self.indices.first().expect("ExtractValue with no indices")
        )?;
        for idx in &self.indices[1 ..] {
            write!(f, ", {idx}")?;
        }
        // if self.debugloc.is_some() {
        //     write!(f, " (with debugloc)")?;
        // }
        Ok(())
    }
}

#[derive(PartialEq, Clone, Debug, Hash)]
pub struct InsertValue {
    pub aggregate: Operand,
    pub element: Operand,
    pub indices: Vec<u32>,
    pub dest: Name,
    // pub debugloc: Option<DebugLoc>,
}

impl_inst!(InsertValue, InsertValue);
impl_hasresult!(InsertValue);

impl Typed for InsertValue {
    fn get_type(&self, types: &Types) -> TypeRef {
        types.type_of(&self.aggregate)
    }
}

impl Display for InsertValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{} = insertvalue {}, {}, {}",
            &self.dest,
            &self.aggregate,
            &self.element,
            &self.indices.first().expect("InsertValue with no indices"),
        )?;
        for idx in &self.indices[1 ..] {
            write!(f, ", {idx}")?;
        }
        // if self.debugloc.is_some() {
        //     write!(f, " (with debugloc)")?;
        // }
        Ok(())
    }
}

#[derive(PartialEq, Clone, Debug, Hash)]
pub struct Alloca {
    pub allocated_type: TypeRef,
    pub num_elements: Operand, // llvm-hs-pure has Option<Operand>
    pub dest: Name,
    pub alignment: u32,
    // pub debugloc: Option<DebugLoc>,
}

impl_inst!(Alloca, Alloca);
impl_hasresult!(Alloca);

impl Typed for Alloca {
    fn get_type(&self, types: &Types) -> TypeRef {
        types.pointer()
    }
}

impl Display for Alloca {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} = alloca {}", &self.dest, &self.allocated_type)?;
        if let Some(Constant::Int { value: 1, .. }) = self.num_elements.as_constant() {
        } else {
            write!(f, ", {}", &self.num_elements)?;
        }
        write!(f, ", align {}", &self.alignment)?;
        // if self.debugloc.is_some() {
        //     write!(f, " (with debugloc)")?;
        // }
        Ok(())
    }
}

#[derive(PartialEq, Clone, Debug, Hash)]
pub struct Load {
    pub address: Operand,
    pub dest: Name,
    pub loaded_ty: TypeRef,
    pub volatile: bool,
    pub atomicity: Option<Atomicity>,
    pub alignment: u32,
    // pub debugloc: Option<DebugLoc>,
}

impl_inst!(Load, Load);
impl_hasresult!(Load);

impl Typed for Load {
    fn get_type(&self, _types: &Types) -> TypeRef {
        self.loaded_ty.clone()
    }
}

impl Display for Load {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} = load ", &self.dest)?;
        if self.atomicity.is_some() {
            write!(f, "atomic ")?;
        }
        if self.volatile {
            write!(f, "volatile ")?;
        }
        {
            write!(f, "{}, ", &self.loaded_ty)?;
        }
        write!(f, "{}", &self.address)?;
        if let Some(a) = &self.atomicity {
            write!(f, " {}", a)?;
        }
        write!(f, ", align {}", &self.alignment)?;
        // if self.debugloc.is_some() {
        //     write!(f, " (with debugloc)")?;
        // }
        Ok(())
    }
}

#[derive(PartialEq, Clone, Debug, Hash)]
pub struct Store {
    pub address: Operand,
    pub value: Operand,
    pub volatile: bool,
    pub atomicity: Option<Atomicity>,
    pub alignment: u32,
    // pub debugloc: Option<DebugLoc>,
}

impl_inst!(Store, Store);
void_typed!(Store);

impl Display for Store {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "store ")?;
        if self.atomicity.is_some() {
            write!(f, "atomic ")?;
        }
        if self.volatile {
            write!(f, "volatile ")?;
        }
        write!(f, "{}, {}", &self.value, &self.address)?;
        if let Some(a) = &self.atomicity {
            write!(f, " {}", a)?;
        }
        write!(f, ", align {}", &self.alignment)?;
        // if self.debugloc.is_some() {
        //     write!(f, " (with debugloc)")?;
        // }
        Ok(())
    }
}

#[derive(PartialEq, Clone, Debug, Hash)]
pub struct Fence {
    pub atomicity: Atomicity,
    // pub debugloc: Option<DebugLoc>,
}

impl_inst!(Fence, Fence);
void_typed!(Fence);

impl Display for Fence {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "fence {}", &self.atomicity)?;
        // if self.debugloc.is_some() {
        //     write!(f, " (with debugloc)")?;
        // }
        Ok(())
    }
}

#[derive(PartialEq, Clone, Debug, Hash)]
pub struct CmpXchg {
    pub address: Operand,
    pub expected: Operand,
    pub replacement: Operand,
    pub dest: Name,
    pub volatile: bool,
    pub atomicity: Atomicity,
    pub failure_memory_ordering: MemoryOrdering,
    pub weak: bool,
    // pub debugloc: Option<DebugLoc>,
}

impl_inst!(CmpXchg, CmpXchg);
impl_hasresult!(CmpXchg);

impl Typed for CmpXchg {
    fn get_type(&self, types: &Types) -> TypeRef {
        let ty = types.type_of(&self.expected);
        debug_assert_eq!(ty, types.type_of(&self.replacement));
        types.struct_of(vec![ty, types.bool()], false)
    }
}

impl Display for CmpXchg {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} = cmpxchg ", &self.dest)?;
        if self.weak {
            write!(f, "weak ")?;
        }
        if self.volatile {
            write!(f, "volatile ")?;
        }
        write!(
            f,
            "{}, {}, {} {} {}",
            &self.address,
            &self.expected,
            &self.replacement,
            &self.atomicity,
            &self.failure_memory_ordering,
        )?;
        // if self.debugloc.is_some() {
        //     write!(f, " (with debugloc)")?;
        // }
        Ok(())
    }
}

#[derive(PartialEq, Clone, Debug, Hash)]
pub struct AtomicRMW {
    pub operation: RMWBinOp,
    pub address: Operand,
    pub value: Operand,
    pub dest: Name,
    pub volatile: bool,
    pub atomicity: Atomicity,
    // pub debugloc: Option<DebugLoc>,
}

impl_inst!(AtomicRMW, AtomicRMW);
impl_hasresult!(AtomicRMW);

impl Typed for AtomicRMW {
    fn get_type(&self, types: &Types) -> TypeRef {
        self.value.get_type(types)
    }
}

impl Display for AtomicRMW {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} = atomicrmw ", &self.dest)?;
        if self.volatile {
            write!(f, "volatile ")?;
        }
        write!(f, "{} ", &self.operation)?;
        write!(f, "{}, {} {}", &self.address, &self.value, &self.atomicity)?;
        // if self.debugloc.is_some() {
        //     write!(f, " (with debugloc)")?;
        // }
        Ok(())
    }
}

#[derive(PartialEq, Clone, Debug, Hash)]
pub struct GetElementPtr {
    pub address: Operand,
    pub indices: Vec<Operand>,
    pub dest: Name,
    pub in_bounds: bool,
    // pub debugloc: Option<DebugLoc>,
    pub source_element_type: TypeRef, // --TODO not yet implemented-- pub metadata: InstructionMetadata,
}

impl_inst!(GetElementPtr, GetElementPtr);
impl_hasresult!(GetElementPtr);

impl Typed for GetElementPtr {
    fn get_type(&self, types: &Types) -> TypeRef {
        types.pointer()
    }
}


impl Display for GetElementPtr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} = getelementptr ", &self.dest)?;
        if self.in_bounds {
            write!(f, "inbounds ")?;
        }
        write!(f, "{}", &self.address)?;
        for idx in &self.indices {
            write!(f, ", {}", idx)?;
        }
        // if self.debugloc.is_some() {
        //     write!(f, " (with debugloc)")?;
        // }
        Ok(())
    }
}

#[derive(PartialEq, Clone, Debug, Hash)]
pub struct Trunc {
    pub operand: Operand,
    pub to_type: TypeRef,
    pub dest: Name,
    // pub debugloc: Option<DebugLoc>,
}

impl_inst!(Trunc, Trunc);
unop_explicitly_typed!(Trunc, "trunc");

#[derive(PartialEq, Clone, Debug, Hash)]
pub struct ZExt {
    pub operand: Operand,
    pub to_type: TypeRef,
    pub dest: Name,
    pub nneg: bool,
    // pub debugloc: Option<DebugLoc>,
}

impl_inst!(ZExt, ZExt);
impl_unop!(ZExt);
explicitly_typed!(ZExt);
unop_typed_display_with_flags!(ZExt, "zext", ("nneg" ; nneg ; "llvm-18-or-greater"));

#[derive(PartialEq, Clone, Debug, Hash)]
pub struct SExt {
    pub operand: Operand,
    pub to_type: TypeRef,
    pub dest: Name,
    // pub debugloc: Option<DebugLoc>,
}

impl_inst!(SExt, SExt);
unop_explicitly_typed!(SExt, "sext");

#[derive(PartialEq, Clone, Debug, Hash)]
pub struct FPTrunc {
    pub operand: Operand,
    pub to_type: TypeRef,
    pub dest: Name,
    // pub debugloc: Option<DebugLoc>,
}

impl_inst!(FPTrunc, FPTrunc);
unop_explicitly_typed!(FPTrunc, "fptrunc");

#[derive(PartialEq, Clone, Debug, Hash)]
pub struct FPExt {
    pub operand: Operand,
    pub to_type: TypeRef,
    pub dest: Name,
    // pub debugloc: Option<DebugLoc>,
}

impl_inst!(FPExt, FPExt);
unop_explicitly_typed!(FPExt, "fpext");

#[derive(PartialEq, Clone, Debug, Hash)]
pub struct FPToUI {
    pub operand: Operand,
    pub to_type: TypeRef,
    pub dest: Name,
    // pub debugloc: Option<DebugLoc>,
}

impl_inst!(FPToUI, FPToUI);
unop_explicitly_typed!(FPToUI, "fptoui");

#[derive(PartialEq, Clone, Debug, Hash)]
pub struct FPToSI {
    pub operand: Operand,
    pub to_type: TypeRef,
    pub dest: Name,
    // pub debugloc: Option<DebugLoc>,
}

impl_inst!(FPToSI, FPToSI);
unop_explicitly_typed!(FPToSI, "fptosi");

#[derive(PartialEq, Clone, Debug, Hash)]
pub struct UIToFP {
    pub operand: Operand,
    pub to_type: TypeRef,
    pub dest: Name,
    // pub debugloc: Option<DebugLoc>,
}

impl_inst!(UIToFP, UIToFP);
unop_explicitly_typed!(UIToFP, "uitofp");

#[derive(PartialEq, Clone, Debug, Hash)]
pub struct SIToFP {
    pub operand: Operand,
    pub to_type: TypeRef,
    pub dest: Name,
    // pub debugloc: Option<DebugLoc>,
}

impl_inst!(SIToFP, SIToFP);
unop_explicitly_typed!(SIToFP, "sitofp");

#[derive(PartialEq, Clone, Debug, Hash)]
pub struct PtrToInt {
    pub operand: Operand,
    pub to_type: TypeRef,
    pub dest: Name,
    // pub debugloc: Option<DebugLoc>,
}

impl_inst!(PtrToInt, PtrToInt);
unop_explicitly_typed!(PtrToInt, "ptrtoint");

#[derive(PartialEq, Clone, Debug, Hash)]
pub struct IntToPtr {
    pub operand: Operand,
    pub to_type: TypeRef,
    pub dest: Name,
    // pub debugloc: Option<DebugLoc>,
}

impl_inst!(IntToPtr, IntToPtr);
unop_explicitly_typed!(IntToPtr, "inttoptr");

#[derive(PartialEq, Clone, Debug, Hash)]
pub struct BitCast {
    pub operand: Operand,
    pub to_type: TypeRef,
    pub dest: Name,
    // pub debugloc: Option<DebugLoc>,
}

impl_inst!(BitCast, BitCast);
unop_explicitly_typed!(BitCast, "bitcast");

#[derive(PartialEq, Clone, Debug, Hash)]
pub struct AddrSpaceCast {
    pub operand: Operand,
    pub to_type: TypeRef,
    pub dest: Name,
    // pub debugloc: Option<DebugLoc>,
}

impl_inst!(AddrSpaceCast, AddrSpaceCast);
unop_explicitly_typed!(AddrSpaceCast, "addrspacecast");

#[derive(PartialEq, Clone, Debug, Hash)]
pub struct ICmp {
    pub predicate: IntPredicate,
    pub operand0: Operand,
    pub operand1: Operand,
    pub dest: Name,
    // pub debugloc: Option<DebugLoc>,
}

impl_inst!(ICmp, ICmp);
impl_hasresult!(ICmp);

impl Typed for ICmp {
    fn get_type(&self, types: &Types) -> TypeRef {
        let ty = types.type_of(&self.operand0);
        debug_assert_eq!(ty, types.type_of(&self.operand1));
        match ty.as_ref() {
            Type::VectorType {
                num_elements,
                scalable,
                ..
            } => types.vector_of(types.bool(), *num_elements, *scalable),
            _ => types.bool(),
        }
    }
}

impl Display for ICmp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{} = icmp {} {}, {}",
            &self.dest, &self.predicate, &self.operand0, &self.operand1,
        )?;
        // if self.debugloc.is_some() {
        //     write!(f, " (with debugloc)")?;
        // }
        Ok(())
    }
}

#[derive(PartialEq, Clone, Debug, Hash)]
pub struct FCmp {
    pub predicate: FPPredicate,
    pub operand0: Operand,
    pub operand1: Operand,
    pub dest: Name,
    // pub debugloc: Option<DebugLoc>,
}

impl_inst!(FCmp, FCmp);
impl_hasresult!(FCmp);

impl Typed for FCmp {
    fn get_type(&self, types: &Types) -> TypeRef {
        let ty = types.type_of(&self.operand0);
        debug_assert_eq!(ty, types.type_of(&self.operand1));
        match ty.as_ref() {
            Type::VectorType {
                num_elements,
                scalable,
                ..
            } => types.vector_of(types.bool(), *num_elements, *scalable),
            _ => types.bool(),
        }
    }
}

impl Display for FCmp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{} = fcmp {} {}, {}",
            &self.dest, &self.predicate, &self.operand0, &self.operand1,
        )?;
        // if self.debugloc.is_some() {
        //     write!(f, " (with debugloc)")?;
        // }
        Ok(())
    }
}

#[derive(PartialEq, Clone, Debug, Hash)]
pub struct Phi {
    pub incoming_values: Vec<(Operand, Name)>,
    pub dest: Name,
    pub to_type: TypeRef,
    // pub debugloc: Option<DebugLoc>,
}

impl_inst!(Phi, Phi);
impl_hasresult!(Phi);
explicitly_typed!(Phi);

impl Display for Phi {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let (first_val, first_label) = &self
            .incoming_values
            .get(0)
            .expect("Phi with no incoming values");
        write!(
            f,
            "{} = phi {} [ {}, {} ]",
            &self.dest, &self.to_type, first_val, first_label,
        )?;
        for (val, label) in &self.incoming_values[1 ..] {
            write!(f, ", [ {}, {} ]", val, label)?;
        }
        // if self.debugloc.is_some() {
        //     write!(f, " (with debugloc)")?;
        // }
        Ok(())
    }
}

#[derive(PartialEq, Clone, Debug, Hash)]
pub struct Select {
    pub condition: Operand,
    pub true_value: Operand,
    pub false_value: Operand,
    pub dest: Name,
    // pub debugloc: Option<DebugLoc>,
}

impl_inst!(Select, Select);
impl_hasresult!(Select);

impl Typed for Select {
    fn get_type(&self, types: &Types) -> TypeRef {
        let t = types.type_of(&self.true_value);
        debug_assert_eq!(t, types.type_of(&self.false_value));
        t
    }
}

impl Display for Select {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{} = select {}, {}, {}",
            &self.dest, &self.condition, &self.true_value, &self.false_value,
        )?;
        // if self.debugloc.is_some() {
        //     write!(f, " (with debugloc)")?;
        // }
        Ok(())
    }
}

#[derive(PartialEq, Clone, Debug, Hash)]
pub struct Freeze {
    pub operand: Operand,
    pub dest: Name,
    // pub debugloc: Option<DebugLoc>,
}

impl_inst!(Freeze, Freeze);
unop_same_type!(Freeze, "freeze");

#[derive(PartialEq, Clone, Debug, Hash)]
pub struct Call {
    pub function: Either<InlineAssembly, Operand>,
    pub function_ty: TypeRef,
    pub arguments: Vec<(Operand, Vec<ParameterAttribute>)>,
    pub return_attributes: Vec<ParameterAttribute>,
    pub dest: Option<Name>, // will be None if the `function` returns void
    pub function_attributes: Vec<FunctionAttribute>, // llvm-hs has the equivalent of Vec<Either<GroupID, FunctionAttribute>>, but I'm not sure how the GroupID option comes up
    pub is_tail_call: bool, // llvm-hs has the more sophisticated structure Option<TailCallKind>, but the LLVM C API just gives us true/false
    pub calling_convention: CallingConvention,
    // pub debugloc: Option<DebugLoc>,
}

impl_inst!(Call, Call);

impl Typed for Call {
    fn get_type(&self, _types: &Types) -> TypeRef {
        match self.function_ty.as_ref() {
            Type::FuncType { result_type, .. } => result_type.clone(),
            ty => panic!("Expected Call.function_ty to be a FuncType, got {:?}", ty),
        }
    }
}

impl Display for Call {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(dest) = &self.dest {
            write!(f, "{} = ", dest)?;
        }
        if self.is_tail_call {
            write!(f, "tail ")?;
        }
        write!(
            f,
            "call {}(",
            match &self.function {
                Either::Left(_) => "<inline assembly>".into(),
                Either::Right(op) => format!("{}", op),
            }
        )?;
        for (i, (arg, _)) in self.arguments.iter().enumerate() {
            if i == self.arguments.len() - 1 {
                write!(f, "{}", arg)?;
            } else {
                write!(f, "{}, ", arg)?;
            }
        }
        write!(f, ")")?;
        // if self.debugloc.is_some() {
        //     write!(f, " (with debugloc)")?;
        // }
        Ok(())
    }
}

#[derive(PartialEq, Clone, Debug, Hash)]
pub struct VAArg {
    pub arg_list: Operand,
    pub cur_type: TypeRef,
    pub dest: Name,
    // pub debugloc: Option<DebugLoc>,
}

impl_inst!(VAArg, VAArg);
impl_hasresult!(VAArg);

impl Typed for VAArg {
    fn get_type(&self, _types: &Types) -> TypeRef {
        self.cur_type.clone()
    }
}

impl Display for VAArg {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{} = va_arg {}, {}",
            &self.dest, &self.arg_list, &self.cur_type,
        )?;
        // if self.debugloc.is_some() {
        //     write!(f, " (with debugloc)")?;
        // }
        Ok(())
    }
}

#[derive(PartialEq, Clone, Debug, Hash)]
pub struct LandingPad {
    pub result_type: TypeRef,
    pub clauses: Vec<LandingPadClause>,
    pub dest: Name,
    pub cleanup: bool,
    // pub debugloc: Option<DebugLoc>,
}

impl_inst!(LandingPad, LandingPad);
impl_hasresult!(LandingPad);

impl Typed for LandingPad {
    fn get_type(&self, _types: &Types) -> TypeRef {
        self.result_type.clone()
    }
}

impl Display for LandingPad {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} = landingpad {}", &self.dest, &self.result_type)?;
        if self.cleanup {
            write!(f, " cleanup")?;
        }
        // if self.debugloc.is_some() {
        //     write!(f, " (with debugloc)")?;
        // }
        Ok(())
    }
}

#[derive(PartialEq, Clone, Debug, Hash)]
pub struct CatchPad {
    pub catch_switch: Operand,
    pub args: Vec<Operand>,
    pub dest: Name,
    // pub debugloc: Option<DebugLoc>,
}

impl_inst!(CatchPad, CatchPad);
impl_hasresult!(CatchPad);

impl Typed for CatchPad {
    fn get_type(&self, types: &Types) -> TypeRef {
        types.token_type()
    }
}

impl Display for CatchPad {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{} = catchpad within {} [",
            &self.dest, &self.catch_switch,
        )?;
        for (i, arg) in self.args.iter().enumerate() {
            if i == self.args.len() - 1 {
                write!(f, "{}", arg)?;
            } else {
                write!(f, "{}, ", arg)?;
            }
        }
        write!(f, "]")?;
        // if self.debugloc.is_some() {
        //     write!(f, " (with debugloc)")?;
        // }
        Ok(())
    }
}

#[derive(PartialEq, Clone, Debug, Hash)]
pub struct CleanupPad {
    pub parent_pad: Operand,
    pub args: Vec<Operand>,
    pub dest: Name,
    // pub debugloc: Option<DebugLoc>,
}

impl_inst!(CleanupPad, CleanupPad);
impl_hasresult!(CleanupPad);

impl Typed for CleanupPad {
    fn get_type(&self, types: &Types) -> TypeRef {
        types.token_type()
    }
}

impl Display for CleanupPad {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{} = cleanuppad within {} [",
            &self.dest, &self.parent_pad,
        )?;
        for (i, arg) in self.args.iter().enumerate() {
            if i == self.args.len() - 1 {
                write!(f, "{}", arg)?;
            } else {
                write!(f, "{}, ", arg)?;
            }
        }
        write!(f, "]")?;
        // if self.debugloc.is_some() {
        //     write!(f, " (with debugloc)")?;
        // }
        Ok(())
    }
}

/*
#[derive(PartialEq, Eq, Clone, Copy, Debug, Hash)]
pub enum TailCallKind {
    Tail,
    MustTail,
    NoTail,
}
*/

#[derive(PartialEq, Eq, Clone, Debug, Hash)]
#[allow(non_snake_case)]
pub struct FastMathFlags {
    pub allow_reassoc: bool,
    pub no_NaNs: bool,
    pub no_Infs: bool,
    pub no_signed_zeros: bool,
    pub allow_reciprocal: bool,
    pub allow_contract: bool,
    pub approx_func: bool,
}

#[derive(PartialEq, Eq, Clone, Debug, Hash)]
pub struct Atomicity {
    pub synch_scope: SynchronizationScope,
    pub mem_ordering: MemoryOrdering,
}

impl Display for Atomicity {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.synch_scope {
            SynchronizationScope::SingleThread => write!(f, "syncscope(\"singlethread\") "),
            SynchronizationScope::System => Ok(()),
        }?;
        write!(f, "{}", &self.mem_ordering)?;
        Ok(())
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Debug, Hash)]
pub enum SynchronizationScope {
    SingleThread,
    System,
}

#[derive(PartialEq, Eq, Clone, Copy, Debug, Hash)]
pub enum MemoryOrdering {
    Unordered,
    Monotonic,
    Acquire,
    Release,
    AcquireRelease,
    SequentiallyConsistent,
    NotAtomic, // since we only have a `MemoryOrdering` on atomic instructions, we should never need this. But empirically, some atomic instructions -- e.g. the first 'atomicrmw' instruction in our 'atomic_no_syncscope' test -- have this `MemoryOrdering`
}

impl Display for MemoryOrdering {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MemoryOrdering::Unordered => write!(f, "unordered"),
            MemoryOrdering::Monotonic => write!(f, "monotonic"),
            MemoryOrdering::Acquire => write!(f, "acquire"),
            MemoryOrdering::Release => write!(f, "release"),
            MemoryOrdering::AcquireRelease => write!(f, "acq_rel"),
            MemoryOrdering::SequentiallyConsistent => write!(f, "seq_cst"),
            MemoryOrdering::NotAtomic => write!(f, "not_atomic"),
        }
    }
}

#[derive(PartialEq, Eq, Clone, Debug, Hash)]
pub struct InlineAssembly {
    pub assembly: String,
    pub ty: TypeRef,
    pub constraints: String,
    pub has_side_effects: bool,
    pub align_stack: bool,
    pub dialect: AssemblyDialect,
}

#[derive(PartialEq, Eq, Clone, Copy, Debug, Hash)]
pub enum AssemblyDialect {
    ATT,
    Intel,
}

impl Typed for InlineAssembly {
    fn get_type(&self, _types: &Types) -> TypeRef {
        self.ty.clone()
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Debug, Hash)]
pub enum RMWBinOp {
    Xchg,
    Add,
    Sub,
    And,
    Nand,
    Or,
    Xor,
    Max,
    Min,
    UMax,
    UMin,
    FAdd,
    FSub,
    FMax,
    FMin,
    UIncWrap,
    UDecWrap,
}

impl Display for RMWBinOp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Xchg => write!(f, "xchg"),
            Self::Add => write!(f, "add"),
            Self::Sub => write!(f, "sub"),
            Self::And => write!(f, "and"),
            Self::Nand => write!(f, "nand"),
            Self::Or => write!(f, "or"),
            Self::Xor => write!(f, "xor"),
            Self::Max => write!(f, "max"),
            Self::Min => write!(f, "min"),
            Self::UMax => write!(f, "umax"),
            Self::UMin => write!(f, "umin"),
            Self::FAdd => write!(f, "fadd"),
            Self::FSub => write!(f, "fsub"),
            Self::FMax => write!(f, "fmax"),
            Self::FMin => write!(f, "fmin"),
            Self::UIncWrap => write!(f, "uinc_wrap"),
            Self::UDecWrap => write!(f, "udec_wrap"),
        }
    }
}

/*
#[derive(PartialEq, Clone, Debug, Hash)]
pub enum LandingPadClause {
    Catch(Constant),
    Filter(Constant),
}
*/
#[derive(PartialEq, Eq, Clone, Copy, Debug, Hash)]
pub struct LandingPadClause {}

#[derive(PartialEq, Clone, Debug, Hash)]
pub enum BinaryOp {
    Add(Add),
    Sub(Sub),
    Mul(Mul),
    UDiv(UDiv),
    SDiv(SDiv),
    URem(URem),
    SRem(SRem),

    And(And),
    Or(Or),
    Xor(Xor),
    Shl(Shl),
    LShr(LShr),
    AShr(AShr),

    FAdd(FAdd),
    FSub(FSub),
    FMul(FMul),
    FDiv(FDiv),
    FRem(FRem),
}

#[derive(PartialEq, Clone, Debug, Hash)]
pub enum UnaryOp {
    AddrSpaceCast(AddrSpaceCast),
    BitCast(BitCast),
    FNeg(FNeg),
    FPExt(FPExt),
    FPToSI(FPToSI),
    FPToUI(FPToUI),
    FPTrunc(FPTrunc),
    Freeze(Freeze),
    IntToPtr(IntToPtr),
    PtrToInt(PtrToInt),
    SExt(SExt),
    SIToFP(SIToFP),
    Trunc(Trunc),
    UIToFP(UIToFP),
    ZExt(ZExt),
}

impl From<BinaryOp> for Instruction {
    fn from(bo: BinaryOp) -> Instruction {
        match bo {
            BinaryOp::Add(i) => i.into(),
            BinaryOp::Sub(i) => i.into(),
            BinaryOp::Mul(i) => i.into(),
            BinaryOp::UDiv(i) => i.into(),
            BinaryOp::SDiv(i) => i.into(),
            BinaryOp::URem(i) => i.into(),
            BinaryOp::SRem(i) => i.into(),
            BinaryOp::And(i) => i.into(),
            BinaryOp::Or(i) => i.into(),
            BinaryOp::Xor(i) => i.into(),
            BinaryOp::Shl(i) => i.into(),
            BinaryOp::LShr(i) => i.into(),
            BinaryOp::AShr(i) => i.into(),
            BinaryOp::FAdd(i) => i.into(),
            BinaryOp::FSub(i) => i.into(),
            BinaryOp::FMul(i) => i.into(),
            BinaryOp::FDiv(i) => i.into(),
            BinaryOp::FRem(i) => i.into(),
        }
    }
}

impl From<UnaryOp> for Instruction {
    fn from(uo: UnaryOp) -> Instruction {
        match uo {
            UnaryOp::AddrSpaceCast(i) => i.into(),
            UnaryOp::BitCast(i) => i.into(),
            UnaryOp::FNeg(i) => i.into(),
            UnaryOp::FPExt(i) => i.into(),
            UnaryOp::FPToSI(i) => i.into(),
            UnaryOp::FPToUI(i) => i.into(),
            UnaryOp::FPTrunc(i) => i.into(),
            UnaryOp::Freeze(i) => i.into(),
            UnaryOp::IntToPtr(i) => i.into(),
            UnaryOp::PtrToInt(i) => i.into(),
            UnaryOp::SExt(i) => i.into(),
            UnaryOp::SIToFP(i) => i.into(),
            UnaryOp::Trunc(i) => i.into(),
            UnaryOp::UIToFP(i) => i.into(),
            UnaryOp::ZExt(i) => i.into(),
        }
    }
}

impl TryFrom<Instruction> for BinaryOp {
    type Error = &'static str;
    fn try_from(inst: Instruction) -> Result<Self, Self::Error> {
        match inst {
            Instruction::Add(i) => Ok(BinaryOp::Add(i)),
            Instruction::Sub(i) => Ok(BinaryOp::Sub(i)),
            Instruction::Mul(i) => Ok(BinaryOp::Mul(i)),
            Instruction::UDiv(i) => Ok(BinaryOp::UDiv(i)),
            Instruction::SDiv(i) => Ok(BinaryOp::SDiv(i)),
            Instruction::URem(i) => Ok(BinaryOp::URem(i)),
            Instruction::SRem(i) => Ok(BinaryOp::SRem(i)),
            Instruction::And(i) => Ok(BinaryOp::And(i)),
            Instruction::Or(i) => Ok(BinaryOp::Or(i)),
            Instruction::Xor(i) => Ok(BinaryOp::Xor(i)),
            Instruction::Shl(i) => Ok(BinaryOp::Shl(i)),
            Instruction::LShr(i) => Ok(BinaryOp::LShr(i)),
            Instruction::AShr(i) => Ok(BinaryOp::AShr(i)),
            Instruction::FAdd(i) => Ok(BinaryOp::FAdd(i)),
            Instruction::FSub(i) => Ok(BinaryOp::FSub(i)),
            Instruction::FMul(i) => Ok(BinaryOp::FMul(i)),
            Instruction::FDiv(i) => Ok(BinaryOp::FDiv(i)),
            Instruction::FRem(i) => Ok(BinaryOp::FRem(i)),
            _ => Err("Not a binary op"),
        }
    }
}

impl TryFrom<Instruction> for UnaryOp {
    type Error = &'static str;
    fn try_from(inst: Instruction) -> Result<Self, Self::Error> {
        match inst {
            Instruction::AddrSpaceCast(i) => Ok(UnaryOp::AddrSpaceCast(i)),
            Instruction::BitCast(i) => Ok(UnaryOp::BitCast(i)),
            Instruction::FNeg(i) => Ok(UnaryOp::FNeg(i)),
            Instruction::FPExt(i) => Ok(UnaryOp::FPExt(i)),
            Instruction::FPToSI(i) => Ok(UnaryOp::FPToSI(i)),
            Instruction::FPToUI(i) => Ok(UnaryOp::FPToUI(i)),
            Instruction::FPTrunc(i) => Ok(UnaryOp::FPTrunc(i)),
            Instruction::Freeze(i) => Ok(UnaryOp::Freeze(i)),
            Instruction::IntToPtr(i) => Ok(UnaryOp::IntToPtr(i)),
            Instruction::PtrToInt(i) => Ok(UnaryOp::PtrToInt(i)),
            Instruction::SExt(i) => Ok(UnaryOp::SExt(i)),
            Instruction::SIToFP(i) => Ok(UnaryOp::SIToFP(i)),
            Instruction::Trunc(i) => Ok(UnaryOp::Trunc(i)),
            Instruction::UIToFP(i) => Ok(UnaryOp::UIToFP(i)),
            Instruction::ZExt(i) => Ok(UnaryOp::ZExt(i)),
            _ => Err("Not a unary op"),
        }
    }
}

impl Typed for BinaryOp {
    fn get_type(&self, types: &Types) -> TypeRef {
        match self {
            BinaryOp::Add(i) => types.type_of(i),
            BinaryOp::Sub(i) => types.type_of(i),
            BinaryOp::Mul(i) => types.type_of(i),
            BinaryOp::UDiv(i) => types.type_of(i),
            BinaryOp::SDiv(i) => types.type_of(i),
            BinaryOp::URem(i) => types.type_of(i),
            BinaryOp::SRem(i) => types.type_of(i),
            BinaryOp::And(i) => types.type_of(i),
            BinaryOp::Or(i) => types.type_of(i),
            BinaryOp::Xor(i) => types.type_of(i),
            BinaryOp::Shl(i) => types.type_of(i),
            BinaryOp::LShr(i) => types.type_of(i),
            BinaryOp::AShr(i) => types.type_of(i),
            BinaryOp::FAdd(i) => types.type_of(i),
            BinaryOp::FSub(i) => types.type_of(i),
            BinaryOp::FMul(i) => types.type_of(i),
            BinaryOp::FDiv(i) => types.type_of(i),
            BinaryOp::FRem(i) => types.type_of(i),
        }
    }
}

impl Typed for UnaryOp {
    fn get_type(&self, types: &Types) -> TypeRef {
        match self {
            UnaryOp::AddrSpaceCast(i) => types.type_of(i),
            UnaryOp::BitCast(i) => types.type_of(i),
            UnaryOp::FNeg(i) => types.type_of(i),
            UnaryOp::FPExt(i) => types.type_of(i),
            UnaryOp::FPToSI(i) => types.type_of(i),
            UnaryOp::FPToUI(i) => types.type_of(i),
            UnaryOp::FPTrunc(i) => types.type_of(i),
            UnaryOp::Freeze(i) => types.type_of(i),
            UnaryOp::IntToPtr(i) => types.type_of(i),
            UnaryOp::PtrToInt(i) => types.type_of(i),
            UnaryOp::SExt(i) => types.type_of(i),
            UnaryOp::SIToFP(i) => types.type_of(i),
            UnaryOp::Trunc(i) => types.type_of(i),
            UnaryOp::UIToFP(i) => types.type_of(i),
            UnaryOp::ZExt(i) => types.type_of(i),
        }
    }
}

/* --TODO not yet implemented: metadata
impl HasMetadata for BinaryOp {
    fn get_metadata(&self) -> &InstructionMetadata {
        match self {
            BinaryOp::Add(i) => i.get_metadata(),
            BinaryOp::Sub(i) => i.get_metadata(),
            BinaryOp::Mul(i) => i.get_metadata(),
            BinaryOp::UDiv(i) => i.get_metadata(),
            BinaryOp::SDiv(i) => i.get_metadata(),
            BinaryOp::URem(i) => i.get_metadata(),
            BinaryOp::SRem(i) => i.get_metadata(),
            BinaryOp::And(i) => i.get_metadata(),
            BinaryOp::Or(i) => i.get_metadata(),
            BinaryOp::Xor(i) => i.get_metadata(),
            BinaryOp::Shl(i) => i.get_metadata(),
            BinaryOp::LShr(i) => i.get_metadata(),
            BinaryOp::AShr(i) => i.get_metadata(),
            BinaryOp::FAdd(i) => i.get_metadata(),
            BinaryOp::FSub(i) => i.get_metadata(),
            BinaryOp::FMul(i) => i.get_metadata(),
            BinaryOp::FDiv(i) => i.get_metadata(),
            BinaryOp::FRem(i) => i.get_metadata(),
        }
    }
}

impl HasMetadata for UnaryOp {
    fn get_metadata(&self) -> &InstructionMetadata {
        match self {
            UnaryOp::AddrSpaceCast(i) => i.get_metadata(),
            UnaryOp::BitCast(i) => i.get_metadata(),
            UnaryOp::FNeg(i) => i.get_metadata(),
            UnaryOp::FPExt(i) => i.get_metadata(),
            UnaryOp::FPToSI(i) => i.get_metadata(),
            UnaryOp::FPToUI(i) => i.get_metadata(),
            UnaryOp::FPTrunc(i) => i.get_metadata(),
            UnaryOp::Freeze(i) => i.get_metadata(),
            UnaryOp::IntToPtr(i) => i.get_metadata(),
            UnaryOp::PtrToInt(i) => i.get_metadata(),
            UnaryOp::SExt(i) => i.get_metadata(),
            UnaryOp::SIToFP(i) => i.get_metadata(),
            UnaryOp::Trunc(i) => i.get_metadata(),
            UnaryOp::UIToFP(i) => i.get_metadata(),
            UnaryOp::ZExt(i) => i.get_metadata(),
        }
    }
}
*/

impl HasResult for BinaryOp {
    fn get_result(&self) -> &Name {
        match self {
            BinaryOp::Add(i) => i.get_result(),
            BinaryOp::Sub(i) => i.get_result(),
            BinaryOp::Mul(i) => i.get_result(),
            BinaryOp::UDiv(i) => i.get_result(),
            BinaryOp::SDiv(i) => i.get_result(),
            BinaryOp::URem(i) => i.get_result(),
            BinaryOp::SRem(i) => i.get_result(),
            BinaryOp::And(i) => i.get_result(),
            BinaryOp::Or(i) => i.get_result(),
            BinaryOp::Xor(i) => i.get_result(),
            BinaryOp::Shl(i) => i.get_result(),
            BinaryOp::LShr(i) => i.get_result(),
            BinaryOp::AShr(i) => i.get_result(),
            BinaryOp::FAdd(i) => i.get_result(),
            BinaryOp::FSub(i) => i.get_result(),
            BinaryOp::FMul(i) => i.get_result(),
            BinaryOp::FDiv(i) => i.get_result(),
            BinaryOp::FRem(i) => i.get_result(),
        }
    }
}

impl HasResult for UnaryOp {
    fn get_result(&self) -> &Name {
        match self {
            UnaryOp::AddrSpaceCast(i) => i.get_result(),
            UnaryOp::BitCast(i) => i.get_result(),
            UnaryOp::FNeg(i) => i.get_result(),
            UnaryOp::FPExt(i) => i.get_result(),
            UnaryOp::FPToSI(i) => i.get_result(),
            UnaryOp::FPToUI(i) => i.get_result(),
            UnaryOp::FPTrunc(i) => i.get_result(),
            UnaryOp::Freeze(i) => i.get_result(),
            UnaryOp::IntToPtr(i) => i.get_result(),
            UnaryOp::PtrToInt(i) => i.get_result(),
            UnaryOp::SExt(i) => i.get_result(),
            UnaryOp::SIToFP(i) => i.get_result(),
            UnaryOp::Trunc(i) => i.get_result(),
            UnaryOp::UIToFP(i) => i.get_result(),
            UnaryOp::ZExt(i) => i.get_result(),
        }
    }
}

impl IsBinaryOp for BinaryOp {
    fn get_operand0(&self) -> &Operand {
        match self {
            BinaryOp::Add(i) => i.get_operand0(),
            BinaryOp::Sub(i) => i.get_operand0(),
            BinaryOp::Mul(i) => i.get_operand0(),
            BinaryOp::UDiv(i) => i.get_operand0(),
            BinaryOp::SDiv(i) => i.get_operand0(),
            BinaryOp::URem(i) => i.get_operand0(),
            BinaryOp::SRem(i) => i.get_operand0(),
            BinaryOp::And(i) => i.get_operand0(),
            BinaryOp::Or(i) => i.get_operand0(),
            BinaryOp::Xor(i) => i.get_operand0(),
            BinaryOp::Shl(i) => i.get_operand0(),
            BinaryOp::LShr(i) => i.get_operand0(),
            BinaryOp::AShr(i) => i.get_operand0(),
            BinaryOp::FAdd(i) => i.get_operand0(),
            BinaryOp::FSub(i) => i.get_operand0(),
            BinaryOp::FMul(i) => i.get_operand0(),
            BinaryOp::FDiv(i) => i.get_operand0(),
            BinaryOp::FRem(i) => i.get_operand0(),
        }
    }

    fn get_operand1(&self) -> &Operand {
        match self {
            BinaryOp::Add(i) => i.get_operand1(),
            BinaryOp::Sub(i) => i.get_operand1(),
            BinaryOp::Mul(i) => i.get_operand1(),
            BinaryOp::UDiv(i) => i.get_operand1(),
            BinaryOp::SDiv(i) => i.get_operand1(),
            BinaryOp::URem(i) => i.get_operand1(),
            BinaryOp::SRem(i) => i.get_operand1(),
            BinaryOp::And(i) => i.get_operand1(),
            BinaryOp::Or(i) => i.get_operand1(),
            BinaryOp::Xor(i) => i.get_operand1(),
            BinaryOp::Shl(i) => i.get_operand1(),
            BinaryOp::LShr(i) => i.get_operand1(),
            BinaryOp::AShr(i) => i.get_operand1(),
            BinaryOp::FAdd(i) => i.get_operand1(),
            BinaryOp::FSub(i) => i.get_operand1(),
            BinaryOp::FMul(i) => i.get_operand1(),
            BinaryOp::FDiv(i) => i.get_operand1(),
            BinaryOp::FRem(i) => i.get_operand1(),
        }
    }
}

impl IsUnaryOp for UnaryOp {
    fn get_operand(&self) -> &Operand {
        match self {
            UnaryOp::AddrSpaceCast(i) => i.get_operand(),
            UnaryOp::BitCast(i) => i.get_operand(),
            UnaryOp::FNeg(i) => i.get_operand(),
            UnaryOp::FPExt(i) => i.get_operand(),
            UnaryOp::FPToSI(i) => i.get_operand(),
            UnaryOp::FPToUI(i) => i.get_operand(),
            UnaryOp::FPTrunc(i) => i.get_operand(),
            UnaryOp::Freeze(i) => i.get_operand(),
            UnaryOp::IntToPtr(i) => i.get_operand(),
            UnaryOp::PtrToInt(i) => i.get_operand(),
            UnaryOp::SExt(i) => i.get_operand(),
            UnaryOp::SIToFP(i) => i.get_operand(),
            UnaryOp::Trunc(i) => i.get_operand(),
            UnaryOp::UIToFP(i) => i.get_operand(),
            UnaryOp::ZExt(i) => i.get_operand(),
        }
    }
}
