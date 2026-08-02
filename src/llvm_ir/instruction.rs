use crate::llvm_ir::constant::ConstantRef;
use crate::llvm_ir::function::{
    CallingConvention, FunctionAttribute, ParameterAttribute,
};
use crate::llvm_ir::name::Name;
use crate::llvm_ir::operand::Operand;
use crate::llvm_ir::types::{LLVMType, TypeRef, Typed, Types};
use either::Either;
use std::fmt::Debug;

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

#[derive(PartialEq, Clone, Debug, Hash)]
pub struct Add {
    pub operand0: Operand,
    pub operand1: Operand,
    pub dest: Name,
    pub nuw: bool,
    pub nsw: bool,
}

impl Typed for Add {
    fn get_type(&self, types: &Types) -> TypeRef {
        let ty = types.type_of(&self.operand0);
        debug_assert_eq!(ty, types.type_of(&self.operand1));
        ty
    }
}

#[derive(PartialEq, Clone, Debug, Hash)]
pub struct Sub {
    pub operand0: Operand,
    pub operand1: Operand,
    pub dest: Name,
    pub nuw: bool,
    pub nsw: bool,
}

impl Typed for Sub {
    fn get_type(&self, types: &Types) -> TypeRef {
        let ty = types.type_of(&self.operand0);
        debug_assert_eq!(ty, types.type_of(&self.operand1));
        ty
    }
}

#[derive(PartialEq, Clone, Debug, Hash)]
pub struct Mul {
    pub operand0: Operand,
    pub operand1: Operand,
    pub dest: Name,
    pub nuw: bool,
    pub nsw: bool,
}

impl Typed for Mul {
    fn get_type(&self, types: &Types) -> TypeRef {
        let ty = types.type_of(&self.operand0);
        debug_assert_eq!(ty, types.type_of(&self.operand1));
        ty
    }
}

#[derive(PartialEq, Clone, Debug, Hash)]
pub struct UDiv {
    pub operand0: Operand,
    pub operand1: Operand,
    pub dest: Name,
    pub exact: bool,
}

impl Typed for UDiv {
    fn get_type(&self, types: &Types) -> TypeRef {
        let ty = types.type_of(&self.operand0);
        debug_assert_eq!(ty, types.type_of(&self.operand1));
        ty
    }
}

#[derive(PartialEq, Clone, Debug, Hash)]
pub struct SDiv {
    pub operand0: Operand,
    pub operand1: Operand,
    pub dest: Name,
    pub exact: bool,
}

impl Typed for SDiv {
    fn get_type(&self, types: &Types) -> TypeRef {
        let ty = types.type_of(&self.operand0);
        debug_assert_eq!(ty, types.type_of(&self.operand1));
        ty
    }
}

#[derive(PartialEq, Clone, Debug, Hash)]
pub struct URem {
    pub operand0: Operand,
    pub operand1: Operand,
    pub dest: Name,
}


impl Typed for URem {
    fn get_type(&self, types: &Types) -> TypeRef {
        let ty = types.type_of(&self.operand0);
        debug_assert_eq!(ty, types.type_of(&self.operand1));
        ty
    }
}

#[derive(PartialEq, Clone, Debug, Hash)]
pub struct SRem {
    pub operand0: Operand,
    pub operand1: Operand,
    pub dest: Name,
}

impl From<SRem> for Instruction {
    fn from(inst: SRem) -> Instruction {
        Instruction::SRem(inst)
    }
}

impl Typed for SRem {
    fn get_type(&self, types: &Types) -> TypeRef {
        let ty = types.type_of(&self.operand0);
        debug_assert_eq!(ty, types.type_of(&self.operand1));
        ty
    }
}

#[derive(PartialEq, Clone, Debug, Hash)]
pub struct And {
    pub operand0: Operand,
    pub operand1: Operand,
    pub dest: Name,
}

impl Typed for And {
    fn get_type(&self, types: &Types) -> TypeRef {
        let ty = types.type_of(&self.operand0);
        debug_assert_eq!(ty, types.type_of(&self.operand1));
        ty
    }
}

#[derive(PartialEq, Clone, Debug, Hash)]
pub struct Or {
    pub operand0: Operand,
    pub operand1: Operand,
    pub dest: Name,
    pub disjoint: bool,
}

impl Typed for Or {
    fn get_type(&self, types: &Types) -> TypeRef {
        let ty = types.type_of(&self.operand0);
        debug_assert_eq!(ty, types.type_of(&self.operand1));
        ty
    }
}

#[derive(PartialEq, Clone, Debug, Hash)]
pub struct Xor {
    pub operand0: Operand,
    pub operand1: Operand,
    pub dest: Name,
}

impl From<Xor> for Instruction {
    fn from(inst: Xor) -> Instruction {
        Instruction::Xor(inst)
    }
}

impl Typed for Xor {
    fn get_type(&self, types: &Types) -> TypeRef {
        let ty = types.type_of(&self.operand0);
        debug_assert_eq!(ty, types.type_of(&self.operand1));
        ty
    }
}

#[derive(PartialEq, Clone, Debug, Hash)]
pub struct Shl {
    pub operand0: Operand,
    pub operand1: Operand,
    pub dest: Name,
    pub nuw: bool,
    pub nsw: bool,
}


impl Typed for Shl {
    fn get_type(&self, types: &Types) -> TypeRef {
        types.type_of(&self.operand0)
    }
}

#[derive(PartialEq, Clone, Debug, Hash)]
pub struct LShr {
    pub operand0: Operand,
    pub operand1: Operand,
    pub dest: Name,
    pub exact: bool,
}

impl Typed for LShr {
    fn get_type(&self, types: &Types) -> TypeRef {
        types.type_of(&self.operand0)
    }
}

#[derive(PartialEq, Clone, Debug, Hash)]
pub struct AShr {
    pub operand0: Operand,
    pub operand1: Operand,
    pub dest: Name,
    pub exact: bool,
}


impl Typed for AShr {
    fn get_type(&self, types: &Types) -> TypeRef {
        types.type_of(&self.operand0)
    }
}

#[derive(PartialEq, Clone, Debug, Hash)]
pub struct FAdd {
    pub operand0: Operand,
    pub operand1: Operand,
    pub dest: Name,
}

impl Typed for FAdd {
    fn get_type(&self, types: &Types) -> TypeRef {
        let ty = types.type_of(&self.operand0);
        debug_assert_eq!(ty, types.type_of(&self.operand1));
        ty
    }
}

#[derive(PartialEq, Clone, Debug, Hash)]
pub struct FSub {
    pub operand0: Operand,
    pub operand1: Operand,
    pub dest: Name,
}

impl From<FSub> for Instruction {
    fn from(inst: FSub) -> Instruction {
        Instruction::FSub(inst)
    }
}

impl Typed for FSub {
    fn get_type(&self, types: &Types) -> TypeRef {
        let ty = types.type_of(&self.operand0);
        debug_assert_eq!(ty, types.type_of(&self.operand1));
        ty
    }
}

#[derive(PartialEq, Clone, Debug, Hash)]
pub struct FMul {
    pub operand0: Operand,
    pub operand1: Operand,
    pub dest: Name,
}

impl Typed for FMul {
    fn get_type(&self, types: &Types) -> TypeRef {
        let ty = types.type_of(&self.operand0);
        debug_assert_eq!(ty, types.type_of(&self.operand1));
        ty
    }
}

#[derive(PartialEq, Clone, Debug, Hash)]
pub struct FDiv {
    pub operand0: Operand,
    pub operand1: Operand,
    pub dest: Name,
}

impl Typed for FDiv {
    fn get_type(&self, types: &Types) -> TypeRef {
        let ty = types.type_of(&self.operand0);
        debug_assert_eq!(ty, types.type_of(&self.operand1));
        ty
    }
}

#[derive(PartialEq, Clone, Debug, Hash)]
pub struct FRem {
    pub operand0: Operand,
    pub operand1: Operand,
    pub dest: Name,
}

impl Typed for FRem {
    fn get_type(&self, types: &Types) -> TypeRef {
        let ty = types.type_of(&self.operand0);
        debug_assert_eq!(ty, types.type_of(&self.operand1));
        ty
    }
}

#[derive(PartialEq, Clone, Debug, Hash)]
pub struct FNeg {
    pub operand: Operand,
    pub dest: Name,
}

impl Typed for FNeg {
    fn get_type(&self, types: &Types) -> TypeRef {
        types.type_of(&self.operand)
    }
}

#[derive(PartialEq, Clone, Debug, Hash)]
pub struct ExtractElement {
    pub vector: Operand,
    pub index: Operand,
    pub dest: Name,
}

impl Typed for ExtractElement {
    fn get_type(&self, types: &Types) -> TypeRef {
        match types.type_of(&self.vector).as_ref() {
            LLVMType::VectorType { element_type, .. } => {
                element_type.clone()
            },
            ty => panic!(
                "Expected an ExtractElement vector to be VectorType, got {:?}",
                ty
            ),
        }
    }
}

#[derive(PartialEq, Clone, Debug, Hash)]
pub struct InsertElement {
    pub vector: Operand,
    pub element: Operand,
    pub index: Operand,
    pub dest: Name,
}

impl Typed for InsertElement {
    fn get_type(&self, types: &Types) -> TypeRef {
        types.type_of(&self.vector)
    }
}

#[derive(PartialEq, Clone, Debug, Hash)]
pub struct ShuffleVector {
    pub operand0: Operand,
    pub operand1: Operand,
    pub dest: Name,
    pub mask: ConstantRef,
}

impl Typed for ShuffleVector {
    fn get_type(&self, types: &Types) -> TypeRef {
        let ty = types.type_of(&self.operand0);
        debug_assert_eq!(ty, types.type_of(&self.operand1));
        match ty.as_ref() {
            LLVMType::VectorType { element_type, .. } => {
                match types.type_of(&self.mask).as_ref() {
                    LLVMType::VectorType {
                        num_elements,
                        scalable,
                        ..
                    } => types.vector_of(
                        element_type.clone(),
                        *num_elements,
                        *scalable,
                    ),
                    ty => panic!(
                        "Expected a ShuffleVector mask to be VectorType, got {:?}",
                        ty
                    ),
                }
            },
            _ => panic!(
                "Expected a ShuffleVector operand to be VectorType, got {:?}",
                ty
            ),
        }
    }
}

#[derive(PartialEq, Clone, Debug, Hash)]
pub struct ExtractValue {
    pub aggregate: Operand,
    pub indices: Vec<u32>,
    pub dest: Name,
}

impl Typed for ExtractValue {
    fn get_type(&self, types: &Types) -> TypeRef {
        ev_type(
            types.type_of(&self.aggregate),
            self.indices.iter().copied(),
        )
    }
}

fn ev_type(
    cur_type: TypeRef,
    mut indices: impl Iterator<Item = u32>,
) -> TypeRef {
    match indices.next() {
        None => cur_type,
        Some(index) => match cur_type.as_ref() {
            LLVMType::ArrayType { element_type, .. } => {
                ev_type(element_type.clone(), indices)
            },
            LLVMType::StructType { element_types, .. } => ev_type(
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

#[derive(PartialEq, Clone, Debug, Hash)]
pub struct InsertValue {
    pub aggregate: Operand,
    pub element: Operand,
    pub indices: Vec<u32>,
    pub dest: Name,
}

impl Typed for InsertValue {
    fn get_type(&self, types: &Types) -> TypeRef {
        types.type_of(&self.aggregate)
    }
}

#[derive(PartialEq, Clone, Debug, Hash)]
pub struct Alloca {
    pub allocated_type: TypeRef,
    pub num_elements: Operand,
    pub dest: Name,
    pub alignment: u32,
}

impl Typed for Alloca {
    fn get_type(&self, types: &Types) -> TypeRef {
        types.pointer()
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
}

impl Typed for Load {
    fn get_type(&self, _types: &Types) -> TypeRef {
        self.loaded_ty.clone()
    }
}

#[derive(PartialEq, Clone, Debug, Hash)]
pub struct Store {
    pub address: Operand,
    pub value: Operand,
    pub volatile: bool,
    pub atomicity: Option<Atomicity>,
    pub alignment: u32,
}

impl Typed for Store {
    fn get_type(&self, types: &Types) -> TypeRef {
        types.void()
    }
}

#[derive(PartialEq, Clone, Debug, Hash)]
pub struct Fence {
    pub atomicity: Atomicity,
}

impl Typed for Fence {
    fn get_type(&self, types: &Types) -> TypeRef {
        types.void()
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
}

impl Typed for CmpXchg {
    fn get_type(&self, types: &Types) -> TypeRef {
        let ty = types.type_of(&self.expected);
        debug_assert_eq!(ty, types.type_of(&self.replacement));
        types.struct_of(vec![ty, types.bool()], false)
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
}

impl Typed for AtomicRMW {
    fn get_type(&self, types: &Types) -> TypeRef {
        self.value.get_type(types)
    }
}

#[derive(PartialEq, Clone, Debug, Hash)]
pub struct GetElementPtr {
    pub address: Operand,
    pub indices: Vec<Operand>,
    pub dest: Name,
    pub in_bounds: bool,
    pub source_element_type: TypeRef,
}

impl Typed for GetElementPtr {
    fn get_type(&self, types: &Types) -> TypeRef {
        types.pointer()
    }
}

#[derive(PartialEq, Clone, Debug, Hash)]
pub struct Trunc {
    pub operand: Operand,
    pub to_type: TypeRef,
    pub dest: Name,
}
impl Typed for Trunc {
    fn get_type(&self, _types: &Types) -> TypeRef {
        self.to_type.clone()
    }
}

#[derive(PartialEq, Clone, Debug, Hash)]
pub struct ZExt {
    pub operand: Operand,
    pub to_type: TypeRef,
    pub dest: Name,
    pub nneg: bool,
}

impl Typed for ZExt {
    fn get_type(&self, _types: &Types) -> TypeRef {
        self.to_type.clone()
    }
}

#[derive(PartialEq, Clone, Debug, Hash)]
pub struct SExt {
    pub operand: Operand,
    pub to_type: TypeRef,
    pub dest: Name,
}
impl Typed for SExt {
    fn get_type(&self, _types: &Types) -> TypeRef {
        self.to_type.clone()
    }
}

#[derive(PartialEq, Clone, Debug, Hash)]
pub struct FPTrunc {
    pub operand: Operand,
    pub to_type: TypeRef,
    pub dest: Name,
}

impl Typed for FPTrunc {
    fn get_type(&self, _types: &Types) -> TypeRef {
        self.to_type.clone()
    }
}

#[derive(PartialEq, Clone, Debug, Hash)]
pub struct FPExt {
    pub operand: Operand,
    pub to_type: TypeRef,
    pub dest: Name,
}

impl Typed for FPExt {
    fn get_type(&self, _types: &Types) -> TypeRef {
        self.to_type.clone()
    }
}

#[derive(PartialEq, Clone, Debug, Hash)]
pub struct FPToUI {
    pub operand: Operand,
    pub to_type: TypeRef,
    pub dest: Name,
}

impl Typed for FPToUI {
    fn get_type(&self, _types: &Types) -> TypeRef {
        self.to_type.clone()
    }
}

#[derive(PartialEq, Clone, Debug, Hash)]
pub struct FPToSI {
    pub operand: Operand,
    pub to_type: TypeRef,
    pub dest: Name,
}

impl Typed for FPToSI {
    fn get_type(&self, _types: &Types) -> TypeRef {
        self.to_type.clone()
    }
}

#[derive(PartialEq, Clone, Debug, Hash)]
pub struct UIToFP {
    pub operand: Operand,
    pub to_type: TypeRef,
    pub dest: Name,
}

impl Typed for UIToFP {
    fn get_type(&self, _types: &Types) -> TypeRef {
        self.to_type.clone()
    }
}

#[derive(PartialEq, Clone, Debug, Hash)]
pub struct SIToFP {
    pub operand: Operand,
    pub to_type: TypeRef,
    pub dest: Name,
}

impl Typed for SIToFP {
    fn get_type(&self, _types: &Types) -> TypeRef {
        self.to_type.clone()
    }
}

#[derive(PartialEq, Clone, Debug, Hash)]
pub struct PtrToInt {
    pub operand: Operand,
    pub to_type: TypeRef,
    pub dest: Name,
}

impl Typed for PtrToInt {
    fn get_type(&self, _types: &Types) -> TypeRef {
        self.to_type.clone()
    }
}

#[derive(PartialEq, Clone, Debug, Hash)]
pub struct IntToPtr {
    pub operand: Operand,
    pub to_type: TypeRef,
    pub dest: Name,
}
impl Typed for IntToPtr {
    fn get_type(&self, _types: &Types) -> TypeRef {
        self.to_type.clone()
    }
}

#[derive(PartialEq, Clone, Debug, Hash)]
pub struct BitCast {
    pub operand: Operand,
    pub to_type: TypeRef,
    pub dest: Name,
}

impl Typed for BitCast {
    fn get_type(&self, _types: &Types) -> TypeRef {
        self.to_type.clone()
    }
}

#[derive(PartialEq, Clone, Debug, Hash)]
pub struct AddrSpaceCast {
    pub operand: Operand,
    pub to_type: TypeRef,
    pub dest: Name,
}

impl Typed for AddrSpaceCast {
    fn get_type(&self, _types: &Types) -> TypeRef {
        self.to_type.clone()
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Debug, Hash)]
pub enum IntPredicate {
    EQ,
    NE,
    UGT,
    UGE,
    ULT,
    ULE,
    SGT,
    SGE,
    SLT,
    SLE,
}

#[derive(PartialEq, Eq, Clone, Copy, Debug, Hash)]
pub enum FPPredicate {
    False,
    OEQ,
    OGT,
    OGE,
    OLT,
    OLE,
    ONE,
    ORD,
    UNO,
    UEQ,
    UGT,
    UGE,
    ULT,
    ULE,
    UNE,
    True,
}

#[derive(PartialEq, Clone, Debug, Hash)]
pub struct ICmp {
    pub predicate: IntPredicate,
    pub operand0: Operand,
    pub operand1: Operand,
    pub dest: Name,
}

impl Typed for ICmp {
    fn get_type(&self, types: &Types) -> TypeRef {
        let ty = types.type_of(&self.operand0);
        debug_assert_eq!(ty, types.type_of(&self.operand1));
        match ty.as_ref() {
            LLVMType::VectorType {
                num_elements, scalable, ..
            } => types.vector_of(
                types.bool(),
                *num_elements,
                *scalable,
            ),
            _ => types.bool(),
        }
    }
}

#[derive(PartialEq, Clone, Debug, Hash)]
pub struct FCmp {
    pub predicate: FPPredicate,
    pub operand0: Operand,
    pub operand1: Operand,
    pub dest: Name,
}

impl Typed for FCmp {
    fn get_type(&self, types: &Types) -> TypeRef {
        let ty = types.type_of(&self.operand0);
        debug_assert_eq!(ty, types.type_of(&self.operand1));
        match ty.as_ref() {
            LLVMType::VectorType {
                num_elements, scalable, ..
            } => types.vector_of(
                types.bool(),
                *num_elements,
                *scalable,
            ),
            _ => types.bool(),
        }
    }
}

#[derive(PartialEq, Clone, Debug, Hash)]
pub struct Phi {
    pub incoming_values: Vec<(Operand, Name)>,
    pub dest: Name,
    pub to_type: TypeRef,
}

impl Typed for Phi {
    fn get_type(&self, _types: &Types) -> TypeRef {
        self.to_type.clone()
    }
}

#[derive(PartialEq, Clone, Debug, Hash)]
pub struct Select {
    pub condition: Operand,
    pub true_value: Operand,
    pub false_value: Operand,
    pub dest: Name,
}

impl Typed for Select {
    fn get_type(&self, types: &Types) -> TypeRef {
        let t = types.type_of(&self.true_value);
        debug_assert_eq!(t, types.type_of(&self.false_value));
        t
    }
}

#[derive(PartialEq, Clone, Debug, Hash)]
pub struct Freeze {
    pub operand: Operand,
    pub dest: Name,
}

impl Typed for Freeze {
    fn get_type(&self, types: &Types) -> TypeRef {
        types.type_of(&self.operand)
    }
}

#[derive(PartialEq, Clone, Debug, Hash)]
pub struct Call {
    pub function: Either<InlineAssembly, Operand>,
    pub function_ty: TypeRef,
    pub arguments: Vec<(Operand, Vec<ParameterAttribute>)>,
    pub return_attributes: Vec<ParameterAttribute>,
    pub dest: Option<Name>, // will be None if the `function` returns void
    pub function_attributes: Vec<FunctionAttribute>,
    pub is_tail_call: bool,
    pub calling_convention: CallingConvention,
}

impl Typed for Call {
    fn get_type(&self, _types: &Types) -> TypeRef {
        match self.function_ty.as_ref() {
            LLVMType::FuncType { result_type, .. } => {
                result_type.clone()
            },
            ty => panic!(
                "Expected Call.function_ty to be a FuncType, got {:?}",
                ty
            ),
        }
    }
}

#[derive(PartialEq, Clone, Debug, Hash)]
pub struct VAArg {
    pub arg_list: Operand,
    pub cur_type: TypeRef,
    pub dest: Name,
}

impl Typed for VAArg {
    fn get_type(&self, _types: &Types) -> TypeRef {
        self.cur_type.clone()
    }
}

#[derive(PartialEq, Clone, Debug, Hash)]
pub struct LandingPad {
    pub result_type: TypeRef,
    pub clauses: Vec<LandingPadClause>,
    pub dest: Name,
    pub cleanup: bool,
}

impl Typed for LandingPad {
    fn get_type(&self, _types: &Types) -> TypeRef {
        self.result_type.clone()
    }
}

#[derive(PartialEq, Clone, Debug, Hash)]
pub struct CatchPad {
    pub catch_switch: Operand,
    pub args: Vec<Operand>,
    pub dest: Name,
}

impl Typed for CatchPad {
    fn get_type(&self, types: &Types) -> TypeRef {
        types.token_type()
    }
}

#[derive(PartialEq, Clone, Debug, Hash)]
pub struct CleanupPad {
    pub parent_pad: Operand,
    pub args: Vec<Operand>,
    pub dest: Name,
}

impl Typed for CleanupPad {
    fn get_type(&self, types: &Types) -> TypeRef {
        types.token_type()
    }
}

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

#[derive(PartialEq, Eq, Clone, Copy, Debug, Hash)]
pub struct LandingPadClause {}

