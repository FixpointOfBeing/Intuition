// use crate::llvm_ir::debugloc::{DebugLoc, HasDebugLoc};
use crate::llvm_ir::function::{CallingConvention, FunctionAttribute, ParameterAttribute};
use crate::llvm_ir::instruction::{HasResult, InlineAssembly};
use crate::llvm_ir::types::{Typed, Types, TypeRef, LLVMType};
use crate::llvm_ir::constant::{ConstantRef};
use crate::llvm_ir::name::Name;
use crate::llvm_ir::operand::Operand;
use either::Either;
use std::convert::TryFrom;
use std::fmt::{self, Display};

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

/* impl Display for Terminator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Terminator::Ret(t) => write!(f, "{}", t),
            Terminator::Br(t) => write!(f, "{}", t),
            Terminator::CondBr(t) => write!(f, "{}", t),
            Terminator::Switch(t) => write!(f, "{}", t),
            Terminator::IndirectBr(t) => write!(f, "{}", t),
            Terminator::Invoke(t) => write!(f, "{}", t),
            Terminator::Resume(t) => write!(f, "{}", t),
            Terminator::Unreachable(t) => write!(f, "{}", t),
            Terminator::CleanupRet(t) => write!(f, "{}", t),
            Terminator::CatchRet(t) => write!(f, "{}", t),
            Terminator::CatchSwitch(t) => write!(f, "{}", t),
            Terminator::CallBr(t) => write!(f, "{}", t),
        }
    }
} */

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

// impl Display for Ret {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(
//             f,
//             "ret {}",
//             match &self.return_operand {
//                 None => "void".into(),
//                 Some(op) => format!("{}", op),
//             },
//         )?;
//         /* if self.debugloc.is_some() {
//             write!(f, " (with debugloc)")?;
//         } */
//         Ok(())
//     }
// }

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

// impl Display for Br {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "br label {}", &self.dest)?;
//        /*  if self.debugloc.is_some() {
//             write!(f, " (with debugloc)")?;
//         } */
//         Ok(())
//     }
// }

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

// impl Display for CondBr {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(
//             f,
//             "br {}, label {}, label {}",
//             &self.condition, &self.true_dest, &self.false_dest,
//         )?;
//        /*  if self.debugloc.is_some() {
//             write!(f, " (with debugloc)")?;
//         } */
//         Ok(())
//     }
// }

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

// impl Display for Switch {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(
//             f,
//             "switch {}, label {} [ ",
//             &self.operand, &self.default_dest,
//         )?;
//         for (val, label) in &self.dests {
//             write!(f, "{}, label {}; ", val, label)?;
//         }
//         write!(f, "]")?;
//         // if self.debugloc.is_some() {
//         //     write!(f, " (with debugloc)")?;
//         // }
//         Ok(())
//     }
// }

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

// impl Display for IndirectBr {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(
//             f,
//             "indirectbr {}, [ label {}",
//             &self.operand,
//             &self
//                 .possible_dests
//                 .get(0)
//                 .expect("IndirectBr with no possible dests"),
//         )?;
//         for dest in &self.possible_dests[1..] {
//             write!(f, ", label {}", dest)?;
//         }
//         write!(f, " ]")?;
//         // if self.debugloc.is_some() {
//         //     write!(f, " (with debugloc)")?;
//         // }
//         Ok(())
//     }
// }

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
            LLVMType::FuncType { result_type, .. } => result_type.clone(),
            ty => panic!("Expected Invoke.function_ty to be a FuncType, got {:?}", ty),
        }
    }
}

// impl Display for Invoke {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(
//             f,
//             "{} = invoke {}(",
//             &self.result,
//             match &self.function {
//                 Either::Left(_) => "<inline assembly>".into(),
//                 Either::Right(op) => format!("{}", op),
//             }
//         )?;
//         for (i, (arg, _)) in self.arguments.iter().enumerate() {
//             if i == self.arguments.len() - 1 {
//                 write!(f, "{}", arg)?;
//             } else {
//                 write!(f, "{}, ", arg)?;
//             }
//         }
//         write!(
//             f,
//             ") to label {} unwind label {}",
//             &self.return_label, &self.exception_label,
//         )?;
//         // if self.debugloc.is_some() {
//         //     write!(f, " (with debugloc)")?;
//         // }
//         Ok(())
//     }
// }

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

// impl Display for Resume {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "resume {}", &self.operand)?;
//         // if self.debugloc.is_some() {
//         //     write!(f, " (with debugloc)")?;
//         // }
//         Ok(())
//     }
// }

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

// impl Display for Unreachable {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "unreachable")?;
//         // if self.debugloc.is_some() {
//         //     write!(f, " (with debugloc)")?;
//         // }
//         Ok(())
//     }
// }

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

// impl Display for CleanupRet {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(
//             f,
//             "cleanupret from {} unwind {}",
//             &self.cleanup_pad,
//             match &self.unwind_dest {
//                 None => "to caller".into(),
//                 Some(dest) => format!("label {}", dest),
//             },
//         )?;
//         // if self.debugloc.is_some() {
//         //     write!(f, " (with debugloc)")?;
//         // }
//         Ok(())
//     }
// }

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

// impl Display for CatchRet {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(
//             f,
//             "catchret from {} to label {}",
//             &self.catch_pad, &self.successor,
//         )?;
//         // if self.debugloc.is_some() {
//         //     write!(f, " (with debugloc)")?;
//         // }
//         Ok(())
//     }
// }

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

// impl Display for CatchSwitch {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(
//             f,
//             "{} = catchswitch within {} [ label {}",
//             &self.result,
//             &self.parent_pad,
//             &self
//                 .catch_handlers
//                 .get(0)
//                 .expect("CatchSwitch with no handlers"),
//         )?;
//         for handler in &self.catch_handlers[1..] {
//             write!(f, ", label {}", handler)?;
//         }
//         write!(
//             f,
//             " ] unwind {}",
//             match &self.default_unwind_dest {
//                 None => "to caller".into(),
//                 Some(dest) => format!("label {}", dest),
//             },
//         )?;
//         // if self.debugloc.is_some() {
//         //     write!(f, " (with debugloc)")?;
//         // }
//         Ok(())
//     }
// }

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
            LLVMType::FuncType { result_type, .. } => result_type.clone(),
            ty => panic!(
                "Expected the function argument of a CallBr to have type FuncType; got {:?}",
                ty
            ),
        }
    }
}

// impl Display for CallBr {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(
//             f,
//             "{} = callbr {}(",
//             &self.result,
//             match &self.function {
//                 Either::Left(_) => "<inline assembly>".into(),
//                 Either::Right(op) => format!("{}", op),
//             }
//         )?;
//         for (i, (arg, _)) in self.arguments.iter().enumerate() {
//             if i == self.arguments.len() - 1 {
//                 write!(f, "{}", arg)?;
//             } else {
//                 write!(f, "{}, ", arg)?;
//             }
//         }
//         write!(f, ") to label {}", &self.return_label)?;
//         // if self.debugloc.is_some() {
//         //     write!(f, " (with debugloc)")?;
//         // }
//         Ok(())
//     }
// }
