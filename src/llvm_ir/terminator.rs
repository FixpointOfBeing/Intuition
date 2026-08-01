// use crate::llvm_ir::debugloc::{DebugLoc, HasDebugLoc};
use crate::llvm_ir::constant::ConstantRef;
use crate::llvm_ir::function::{
    CallingConvention, FunctionAttribute, ParameterAttribute,
};
use crate::llvm_ir::instruction::{HasResult, InlineAssembly};
use crate::llvm_ir::name::Name;
use crate::llvm_ir::operand::Operand;
use crate::llvm_ir::types::{LLVMType, TypeRef, Typed, Types};
use either::Either;
use std::convert::TryFrom;

#[derive(PartialEq, Clone, Debug, Hash)]
pub enum Terminator {
    Ret(Ret),
    Br(Br),
    CondBr(CondBr),
    Switch(Switch),
    IndirectBr(IndirectBr),
    Invoke(Invoke),
    Resume(Resume),
    Unreachable(Unreachable),
    CleanupRet(CleanupRet),
    CatchRet(CatchRet),
    CatchSwitch(CatchSwitch),
    CallBr(CallBr),
}

impl Typed for Terminator {
    fn get_type(&self, types: &Types) -> TypeRef {
        match self {
            Terminator::Ret(t) => types.type_of(t),
            Terminator::Br(t) => types.type_of(t),
            Terminator::CondBr(t) => types.type_of(t),
            Terminator::Switch(t) => types.type_of(t),
            Terminator::IndirectBr(t) => types.type_of(t),
            Terminator::Invoke(t) => types.type_of(t),
            Terminator::Resume(t) => types.type_of(t),
            Terminator::Unreachable(t) => types.type_of(t),
            Terminator::CleanupRet(t) => types.type_of(t),
            Terminator::CatchRet(t) => types.type_of(t),
            Terminator::CatchSwitch(t) => types.type_of(t),
            Terminator::CallBr(t) => types.type_of(t),
        }
    }
}

// impl HasDebugLoc for Terminator {
//     fn get_debug_loc(&self) -> &Option<DebugLoc> {
//         match self {
//             Terminator::Ret(t) => t.get_debug_loc(),
//             Terminator::Br(t) => t.get_debug_loc(),
//             Terminator::CondBr(t) => t.get_debug_loc(),
//             Terminator::Switch(t) => t.get_debug_loc(),
//             Terminator::IndirectBr(t) => t.get_debug_loc(),
//             Terminator::Invoke(t) => t.get_debug_loc(),
//             Terminator::Resume(t) => t.get_debug_loc(),
//             Terminator::Unreachable(t) => t.get_debug_loc(),
//             Terminator::CleanupRet(t) => t.get_debug_loc(),
//             Terminator::CatchRet(t) => t.get_debug_loc(),
//             Terminator::CatchSwitch(t) => t.get_debug_loc(),
//             Terminator::CallBr(t) => t.get_debug_loc(),
//         }
//     }
// }

/* --TODO not yet implemented: metadata
impl Terminator {
    pub fn get_metadata(&self) -> &InstructionMetadata {
        match self {
            Terminator::Ret(t) => &t.metadata,
            Terminator::Br(t) => &t.metadata,
            Terminator::CondBr(t) => &t.metadata,
            Terminator::Switch(t) => &t.metadata,
            Terminator::IndirectBr(t) => &t.metadata,
            Terminator::Invoke(t) => &t.metadata,
            Terminator::Resume(t) => &t.metadata,
            Terminator::Unreachable(t) => &t.metadata,
            Terminator::CleanupRet(t) => &t.metadata,
            Terminator::CatchRet(t) => &t.metadata,
            Terminator::CatchSwitch(t) => &t.metadata,
            Terminator::CallBr(t) => &t.metadata,
        }
    }
}
*/

impl Terminator {
    pub fn try_get_result(&self) -> Option<&Name> {
        match self {
            Terminator::Ret(_) => None,
            Terminator::Br(_) => None,
            Terminator::CondBr(_) => None,
            Terminator::Switch(_) => None,
            Terminator::IndirectBr(_) => None,
            Terminator::Invoke(t) => Some(&t.result),
            Terminator::Resume(_) => None,
            Terminator::Unreachable(_) => None,
            Terminator::CleanupRet(_) => None,
            Terminator::CatchRet(_) => None,
            Terminator::CatchSwitch(t) => Some(&t.result),
            Terminator::CallBr(t) => Some(&t.result),
        }
    }
}

#[derive(PartialEq, Clone, Debug, Hash)]
pub struct Ret {
    pub return_operand: Option<Operand>,
    // pub debugloc: Option<DebugLoc>,
}

impl From<Ret> for Terminator {
    fn from(term: Ret) -> Terminator {
        Terminator::Ret(term)
    }
}

impl TryFrom<Terminator> for Ret {
    type Error = &'static str;
    fn try_from(term: Terminator) -> Result<Self, Self::Error> {
        match term {
            Terminator::Ret(term) => Ok(term),
            _ => Err("Terminator is not of requested type"),
        }
    }
}
impl Typed for Ret {
    fn get_type(&self, types: &Types) -> TypeRef {
        types.void()
    }
} // technically the instruction has void type, even though the function may not

#[derive(PartialEq, Clone, Debug, Hash)]
pub struct Br {
    pub dest: Name,
    // pub debugloc: Option<DebugLoc>,
}

impl From<Br> for Terminator {
    fn from(term: Br) -> Terminator {
        Terminator::Br(term)
    }
}

impl TryFrom<Terminator> for Br {
    type Error = &'static str;
    fn try_from(term: Terminator) -> Result<Self, Self::Error> {
        match term {
            Terminator::Br(term) => Ok(term),
            _ => Err("Terminator is not of requested type"),
        }
    }
}
impl Typed for Br {
    fn get_type(&self, types: &Types) -> TypeRef {
        types.void()
    }
}

#[derive(PartialEq, Clone, Debug, Hash)]
pub struct CondBr {
    pub condition: Operand,
    pub true_dest: Name,
    pub false_dest: Name,
    // pub debugloc: Option<DebugLoc>,
}

impl From<CondBr> for Terminator {
    fn from(term: CondBr) -> Terminator {
        Terminator::CondBr(term)
    }
}

impl TryFrom<Terminator> for CondBr {
    type Error = &'static str;
    fn try_from(term: Terminator) -> Result<Self, Self::Error> {
        match term {
            Terminator::CondBr(term) => Ok(term),
            _ => Err("Terminator is not of requested type"),
        }
    }
}
impl Typed for CondBr {
    fn get_type(&self, types: &Types) -> TypeRef {
        types.void()
    }
}

#[derive(PartialEq, Clone, Debug, Hash)]
pub struct Switch {
    pub operand: Operand,
    pub dests: Vec<(ConstantRef, Name)>,
    pub default_dest: Name,
    // pub debugloc: Option<DebugLoc>,
}

impl From<Switch> for Terminator {
    fn from(term: Switch) -> Terminator {
        Terminator::Switch(term)
    }
}

impl TryFrom<Terminator> for Switch {
    type Error = &'static str;
    fn try_from(term: Terminator) -> Result<Self, Self::Error> {
        match term {
            Terminator::Switch(term) => Ok(term),
            _ => Err("Terminator is not of requested type"),
        }
    }
}
impl Typed for Switch {
    fn get_type(&self, types: &Types) -> TypeRef {
        types.void()
    }
}

#[derive(PartialEq, Clone, Debug, Hash)]
pub struct IndirectBr {
    pub operand: Operand,
    pub possible_dests: Vec<Name>,
    // pub debugloc: Option<DebugLoc>,
}

impl From<IndirectBr> for Terminator {
    fn from(term: IndirectBr) -> Terminator {
        Terminator::IndirectBr(term)
    }
}

impl TryFrom<Terminator> for IndirectBr {
    type Error = &'static str;
    fn try_from(term: Terminator) -> Result<Self, Self::Error> {
        match term {
            Terminator::IndirectBr(term) => Ok(term),
            _ => Err("Terminator is not of requested type"),
        }
    }
}
impl Typed for IndirectBr {
    fn get_type(&self, types: &Types) -> TypeRef {
        types.void()
    }
}

#[derive(PartialEq, Clone, Debug, Hash)]
pub struct Invoke {
    pub function: Either<InlineAssembly, Operand>,
    pub function_ty: TypeRef,
    pub arguments: Vec<(Operand, Vec<ParameterAttribute>)>,
    pub return_attributes: Vec<ParameterAttribute>,
    pub result: Name, // The name of the variable that will get the result of the call (if the callee returns with 'ret')
    pub return_label: Name, // Should be the name of a basic block. If the callee returns normally (i.e., with 'ret'), control flow resumes here.
    pub exception_label: Name, // Should be the name of a basic block. If the callee returns with 'resume' or another exception-handling mechanism, control flow resumes here.
    pub function_attributes: Vec<FunctionAttribute>, // llvm-hs has the equivalent of Vec<Either<GroupID, FunctionAttribute>>, but I'm not sure how the GroupID option comes up
    pub calling_convention: CallingConvention,
    // pub debugloc: Option<DebugLoc>,
}

impl From<Invoke> for Terminator {
    fn from(term: Invoke) -> Terminator {
        Terminator::Invoke(term)
    }
}

impl TryFrom<Terminator> for Invoke {
    type Error = &'static str;
    fn try_from(term: Terminator) -> Result<Self, Self::Error> {
        match term {
            Terminator::Invoke(term) => Ok(term),
            _ => Err("Terminator is not of requested type"),
        }
    }
}
impl HasResult for Invoke {
    fn get_result(&self) -> &Name {
        &self.result
    }
}

impl Typed for Invoke {
    fn get_type(&self, _types: &Types) -> TypeRef {
        match self.function_ty.as_ref() {
            LLVMType::FuncType { result_type, .. } => {
                result_type.clone()
            },
            ty => panic!(
                "Expected Invoke.function_ty to be a FuncType, got {:?}",
                ty
            ),
        }
    }
}

#[derive(PartialEq, Clone, Debug, Hash)]
pub struct Resume {
    pub operand: Operand,
    // pub debugloc: Option<DebugLoc>,
}

impl From<Resume> for Terminator {
    fn from(term: Resume) -> Terminator {
        Terminator::Resume(term)
    }
}

impl TryFrom<Terminator> for Resume {
    type Error = &'static str;
    fn try_from(term: Terminator) -> Result<Self, Self::Error> {
        match term {
            Terminator::Resume(term) => Ok(term),
            _ => Err("Terminator is not of requested type"),
        }
    }
}
impl Typed for Resume {
    fn get_type(&self, types: &Types) -> TypeRef {
        types.void()
    }
}

#[derive(PartialEq, Clone, Debug, Hash)]
pub struct Unreachable {
    // pub debugloc: Option<DebugLoc>,
}

impl From<Unreachable> for Terminator {
    fn from(term: Unreachable) -> Terminator {
        Terminator::Unreachable(term)
    }
}

impl TryFrom<Terminator> for Unreachable {
    type Error = &'static str;
    fn try_from(term: Terminator) -> Result<Self, Self::Error> {
        match term {
            Terminator::Unreachable(term) => Ok(term),
            _ => Err("Terminator is not of requested type"),
        }
    }
}
impl Typed for Unreachable {
    fn get_type(&self, types: &Types) -> TypeRef {
        types.void()
    }
}

#[derive(PartialEq, Clone, Debug, Hash)]
pub struct CleanupRet {
    pub cleanup_pad: Operand,
    pub unwind_dest: Option<Name>,
    // pub debugloc: Option<DebugLoc>,
}

impl From<CleanupRet> for Terminator {
    fn from(term: CleanupRet) -> Terminator {
        Terminator::CleanupRet(term)
    }
}

impl TryFrom<Terminator> for CleanupRet {
    type Error = &'static str;
    fn try_from(term: Terminator) -> Result<Self, Self::Error> {
        match term {
            Terminator::CleanupRet(term) => Ok(term),
            _ => Err("Terminator is not of requested type"),
        }
    }
}
impl Typed for CleanupRet {
    fn get_type(&self, types: &Types) -> TypeRef {
        types.void()
    }
}

#[derive(PartialEq, Clone, Debug, Hash)]
pub struct CatchRet {
    pub catch_pad: Operand,
    pub successor: Name,
    // pub debugloc: Option<DebugLoc>,
}

impl From<CatchRet> for Terminator {
    fn from(term: CatchRet) -> Terminator {
        Terminator::CatchRet(term)
    }
}

impl TryFrom<Terminator> for CatchRet {
    type Error = &'static str;
    fn try_from(term: Terminator) -> Result<Self, Self::Error> {
        match term {
            Terminator::CatchRet(term) => Ok(term),
            _ => Err("Terminator is not of requested type"),
        }
    }
}
impl Typed for CatchRet {
    fn get_type(&self, types: &Types) -> TypeRef {
        types.void()
    }
}

#[derive(PartialEq, Clone, Debug, Hash)]
pub struct CatchSwitch {
    pub parent_pad: Operand,
    pub catch_handlers: Vec<Name>,
    pub default_unwind_dest: Option<Name>,
    pub result: Name,
    // pub debugloc: Option<DebugLoc>,
}

impl From<CatchSwitch> for Terminator {
    fn from(term: CatchSwitch) -> Terminator {
        Terminator::CatchSwitch(term)
    }
}

impl TryFrom<Terminator> for CatchSwitch {
    type Error = &'static str;
    fn try_from(term: Terminator) -> Result<Self, Self::Error> {
        match term {
            Terminator::CatchSwitch(term) => Ok(term),
            _ => Err("Terminator is not of requested type"),
        }
    }
}
impl HasResult for CatchSwitch {
    fn get_result(&self) -> &Name {
        &self.result
    }
}

impl Typed for CatchSwitch {
    fn get_type(&self, _types: &Types) -> TypeRef {
        unimplemented!("Typed for CatchSwitch")
    }
}

#[derive(PartialEq, Clone, Debug, Hash)]
pub struct CallBr {
    pub function: Either<InlineAssembly, Operand>,
    pub arguments: Vec<(Operand, Vec<ParameterAttribute>)>,
    pub return_attributes: Vec<ParameterAttribute>,
    pub result: Name, // The name of the variable that will get the result of the call (if the callee returns with 'ret')
    pub return_label: Name, // Should be the name of a basic block. If the callee returns normally (i.e., with 'ret'), control flow resumes here.
    pub other_labels: (), //Vec<Name>, // Should be names of basic blocks. The callee may use an inline-asm 'goto' to resume control flow at one of these places.
    pub function_attributes: Vec<FunctionAttribute>,
    pub calling_convention: CallingConvention,
    // pub debugloc: Option<DebugLoc>,
}

impl From<CallBr> for Terminator {
    fn from(term: CallBr) -> Terminator {
        Terminator::CallBr(term)
    }
}

impl TryFrom<Terminator> for CallBr {
    type Error = &'static str;
    fn try_from(term: Terminator) -> Result<Self, Self::Error> {
        match term {
            Terminator::CallBr(term) => Ok(term),
            _ => Err("Terminator is not of requested type"),
        }
    }
}
impl HasResult for CallBr {
    fn get_result(&self) -> &Name {
        &self.result
    }
}

impl Typed for CallBr {
    fn get_type(&self, types: &Types) -> TypeRef {
        match types.type_of(&self.function).as_ref() {
            LLVMType::FuncType { result_type, .. } => {
                result_type.clone()
            },
            ty => panic!(
                "Expected the function argument of a CallBr to have type FuncType; got {:?}",
                ty
            ),
        }
    }
}
