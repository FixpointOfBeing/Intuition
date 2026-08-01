// use crate::llvm_ir::debugloc::{DebugLoc, HasDebugLoc};
use crate::llvm_ir::basicblock::BasicBlock;
use crate::llvm_ir::constant::ConstantRef;
use crate::llvm_ir::module::{
    Comdat, DLLStorageClass, Linkage, Visibility,
};
use crate::llvm_ir::name::Name;
use crate::llvm_ir::types::{TypeRef, Typed, Types};

#[derive(PartialEq, Clone, Debug, Hash)]
pub struct Function {
    pub name: String,
    pub parameters: Vec<Parameter>,
    pub is_var_arg: bool,
    pub return_type: TypeRef,
    pub basic_blocks: Vec<BasicBlock>,
    pub function_attributes: Vec<FunctionAttribute>,
    pub return_attributes: Vec<ParameterAttribute>,
    pub linkage: Linkage,
    pub visibility: Visibility,
    pub dll_storage_class: DLLStorageClass,
    pub calling_convention: CallingConvention,
    pub section: Option<String>,
    pub comdat: Option<Comdat>,
    pub alignment: u32,
    pub garbage_collector_name: Option<String>,
    pub personality_function: Option<ConstantRef>,
    // pub debugloc: Option<DebugLoc>,
}

impl Typed for Function {
    fn get_type(&self, types: &Types) -> TypeRef {
        types.func_type(
            self.return_type.clone(),
            self.parameters
                .iter()
                .map(|p| types.type_of(p))
                .collect(),
            self.is_var_arg,
        )
    }
}

#[derive(PartialEq, Clone, Debug, Hash)]
pub struct FunctionDeclaration {
    pub name: String,
    pub parameters: Vec<Parameter>,
    pub is_var_arg: bool,
    pub return_type: TypeRef,
    pub return_attributes: Vec<ParameterAttribute>,
    pub linkage: Linkage,
    pub visibility: Visibility,
    pub dll_storage_class: DLLStorageClass,
    pub calling_convention: CallingConvention,
    pub alignment: u32,
    pub garbage_collector_name: Option<String>,
    // pub debugloc: Option<DebugLoc>,
}

#[derive(PartialEq, Clone, Debug, Hash)]
pub struct Parameter {
    pub name: Name,
    pub ty: TypeRef,
    pub attributes: Vec<ParameterAttribute>,
}

impl Typed for Parameter {
    fn get_type(&self, _types: &Types) -> TypeRef {
        self.ty.clone()
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Debug, Hash)]
#[allow(non_camel_case_types)]
pub enum CallingConvention {
    C,
    Fast,
    Cold,
    GHC,
    HiPE,
    WebKit_JS,
    AnyReg,
    PreserveMost,
    PreserveAll,
    Swift,
    CXX_FastTLS,
    X86_StdCall,
    X86_FastCall,
    X86_RegCall,
    X86_ThisCall,
    X86_VectorCall,
    X86_Intr,
    X86_64_SysV,
    ARM_APCS,
    ARM_AAPCS,
    ARM_AAPCS_VFP,
    MSP430_INTR,
    MSP430_Builtin,
    PTX_Kernel,
    PTX_Device,
    SPIR_FUNC,
    SPIR_KERNEL,
    Intel_OCL_BI,
    Win64,
    HHVM,
    HHVM_C,
    AVR_Intr,
    AVR_Signal,
    AVR_Builtin,
    AMDGPU_CS,
    AMDGPU_ES,
    AMDGPU_GS,
    AMDGPU_HS,
    AMDGPU_LS,
    AMDGPU_PS,
    AMDGPU_VS,
    AMDGPU_Kernel,
    Numbered(u32),
}

#[derive(PartialEq, Eq, Clone, Debug, Hash)]
pub enum MemoryEffect {
    None,
    Read,
    Write,
    ReadWrite,
}

impl MemoryEffect {
    pub(crate) fn from_llvm_bits(val: u64) -> Self {
        match val {
            0b00 => Self::None,
            0b01 => Self::Read,
            0b10 => Self::Write,
            0b11 => Self::ReadWrite,
            _ => {
                panic!("Memory effect given unexpected bits {}", val)
            },
        }
    }
}

#[derive(PartialEq, Eq, Clone, Debug, Hash)]
pub enum FunctionAttribute {
    AlignStack(u64),
    AllocSize {
        elt_size: u32,
        num_elts: Option<u32>,
    },
    AlwaysInline,
    Builtin,
    Cold,
    Convergent,
    InaccessibleMemOnly,
    InaccessibleMemOrArgMemOnly,
    InlineHint,
    JumpTable,
    MinimizeSize,
    Naked,
    NoBuiltin,
    NoCFCheck,
    NoDuplicate,
    NoFree,
    NoImplicitFloat,
    NoInline,
    NoMerge,
    NonLazyBind,
    NoRedZone,
    NoReturn,
    NoRecurse,
    WillReturn,
    ReturnsTwice,
    NoSync,
    NoUnwind,
    NullPointerIsValid,
    OptForFuzzing,
    OptNone,
    OptSize,
    ReadNone,
    ReadOnly,
    WriteOnly,
    ArgMemOnly,
    SafeStack,
    SanitizeAddress,
    SanitizeMemory,
    SanitizeThread,
    SanitizeHWAddress,
    SanitizeMemTag,
    ShadowCallStack,
    SpeculativeLoadHardening,
    Speculatable,
    StackProtect,
    StackProtectReq,
    StackProtectStrong,
    StrictFP,
    UWTable,
    Memory {
        default: MemoryEffect,
        argmem: MemoryEffect,
        inaccessible_mem: MemoryEffect,
    },
    StringAttribute {
        kind: String,
        value: String, // for no value, use ""
    },
    UnknownAttribute, // this is used if we get a value not in the above list
}

#[derive(PartialEq, Eq, Clone, Debug, Hash)]
pub enum ParameterAttribute {
    ZeroExt,
    SignExt,
    InReg,
    ByVal(TypeRef),
    Preallocated(TypeRef),
    InAlloca(TypeRef),
    SRet(TypeRef),
    Alignment(u64),
    NoAlias,
    NoCapture,
    NoFree,
    Nest,
    Returned,
    NonNull,
    Dereferenceable(u64),
    DereferenceableOrNull(u64),
    SwiftSelf,
    SwiftError,
    ImmArg,
    NoUndef,
    StringAttribute {
        kind: String,
        value: String, // for no value, use ""
    },
    UnknownAttribute, // this is used if we get an EnumAttribute not in the above list; or, for LLVM 11 or lower, also for some TypeAttributes (due to C API limitations)
    UnknownTypeAttribute(TypeRef), // this is used if we get a TypeAttribute not in the above list
}

pub type GroupID = usize;
