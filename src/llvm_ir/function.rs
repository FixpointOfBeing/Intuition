// use crate::llvm_ir::debugloc::{DebugLoc, HasDebugLoc};
use crate::llvm_ir::module::{Comdat, DLLStorageClass, Linkage, Visibility};
use crate::llvm_ir::types::{TypeRef, Typed, Types};
use crate::llvm_ir::name::Name;
use crate::llvm_ir::basicblock::BasicBlock;
use crate::llvm_ir::constant::ConstantRef;

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
            self.parameters.iter().map(|p| types.type_of(p)).collect(),
            self.is_var_arg,
        )
    }
}

// impl Display for Function {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         for attr in &self.function_attributes {
//             match attr {
//                 FunctionAttribute::UnknownAttribute => {}
//                 _ => write!(f, "{} ", attr)?,
//             }
//         }
//         write!(f, "define ")?;
//         if self.linkage != Linkage::External {
//             write!(f, "{} ", self.linkage)?;
//         }
//         if self.visibility != Visibility::Default {
//             write!(f, "{} ", self.visibility)?;
//         }
//         if self.dll_storage_class != DLLStorageClass::Default {
//             write!(f, "{} ", self.dll_storage_class)?;
//         }
//         if self.calling_convention != CallingConvention::C {
//             write!(f, "{} ", self.calling_convention)?;
//         }
//         write!(f, "{} @{}(", self.return_type, self.name)?;
//         for (i, param) in self.parameters.iter().enumerate() {
//             if i > 0 {
//                 write!(f, ", ")?;
//             }
//             write!(f, "{}", param)?;
//         }
//         if self.is_var_arg {
//             if !self.parameters.is_empty() {
//                 write!(f, ", ")?;
//             }
//             write!(f, "...")?;
//         }
//         write!(f, ")")?;
//         if let Some(ref gc) = self.garbage_collector_name {
//             write!(f, " gc \"{}\"", gc)?;
//         }
//         if let Some(ref pers) = self.personality_function {
//             write!(f, " personality {}", pers)?;
//         }
//         writeln!(f, " {{")?;
//         for bb in &self.basic_blocks {
//             match &bb.name {
//                 Name::Name(s) => write!(f, "{}:\n", s)?,
//                 Name::Number(n) => write!(f, "{}:\n", n)?,
//             }
//             for instr in &bb.instrs {
//                 writeln!(f, "  {}", instr)?;
//             }
//             writeln!(f, "  {}", bb.term)?;
//         }
//         write!(f, "}}")
//     }
// }

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

// impl Display for FunctionDeclaration {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "declare ")?;
//         if self.return_attributes.is_empty() {
//             write!(f, "{}", self.return_type)?;
//         } else {
//             write!(f, "{}", self.return_type)?;
//             for attr in &self.return_attributes {
//                 write!(f, " {}", attr)?;
//             }
//         }
//         write!(f, " @{}(", self.name)?;
//         for (i, param) in self.parameters.iter().enumerate() {
//             if i > 0 {
//                 write!(f, ", ")?;
//             }
//             write!(f, "{}", param)?;
//         }
//         if self.is_var_arg {
//             if !self.parameters.is_empty() {
//                 write!(f, ", ")?;
//             }
//             write!(f, "...")?;
//         }
//         writeln!(f, ")")
//     }
// }

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

// impl Display for Parameter {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "{} {}", self.ty, self.name)?;
//         for attr in &self.attributes {
//             write!(f, " {}", attr)?;
//         }
//         Ok(())
//     }
// }

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

// impl Display for CallingConvention {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         match self {
//             CallingConvention::C => write!(f, "ccc"),
//             CallingConvention::Fast => write!(f, "fastcc"),
//             CallingConvention::Cold => write!(f, "coldcc"),
//             CallingConvention::GHC => write!(f, "ghccc"),
//             CallingConvention::HiPE => write!(f, "cc 11"),
//             CallingConvention::WebKit_JS => write!(f, "webkit_jscc"),
//             CallingConvention::AnyReg => write!(f, "anyregcc"),
//             CallingConvention::PreserveMost => write!(f, "preserve_mostcc"),
//             CallingConvention::PreserveAll => write!(f, "preserve_allcc"),
//             CallingConvention::Swift => write!(f, "swiftcc"),
//             CallingConvention::CXX_FastTLS => write!(f, "cxx_fast_tlscc"),
//             CallingConvention::X86_StdCall => write!(f, "x86_stdcallcc"),
//             CallingConvention::X86_FastCall => write!(f, "x86_fastcallcc"),
//             CallingConvention::X86_RegCall => write!(f, "x86_regcallcc"),
//             CallingConvention::X86_ThisCall => write!(f, "x86_thiscallcc"),
//             CallingConvention::X86_VectorCall => write!(f, "x86_vectorcallcc"),
//             CallingConvention::X86_Intr => write!(f, "x86_intrcc"),
//             CallingConvention::X86_64_SysV => write!(f, "x86_64_sysvcc"),
//             CallingConvention::ARM_APCS => write!(f, "arm_apcscc"),
//             CallingConvention::ARM_AAPCS => write!(f, "arm_aapcscc"),
//             CallingConvention::ARM_AAPCS_VFP => write!(f, "arm_aapcs_vfpcc"),
//             CallingConvention::MSP430_INTR => write!(f, "msp430_intrcc"),
//             CallingConvention::MSP430_Builtin => write!(f, "msp430_builtincc"),
//             CallingConvention::PTX_Kernel => write!(f, "ptx_kernel"),
//             CallingConvention::PTX_Device => write!(f, "ptx_device"),
//             CallingConvention::SPIR_FUNC => write!(f, "spir_func"),
//             CallingConvention::SPIR_KERNEL => write!(f, "spir_kernel"),
//             CallingConvention::Intel_OCL_BI => write!(f, "intel_ocl_bicc"),
//             CallingConvention::Win64 => write!(f, "win64cc"),
//             CallingConvention::HHVM => write!(f, "hhvmcc"),
//             CallingConvention::HHVM_C => write!(f, "hhvm_c"),
//             CallingConvention::AVR_Intr => write!(f, "avr_intrcc"),
//             CallingConvention::AVR_Signal => write!(f, "avr_signcc"),
//             CallingConvention::AVR_Builtin => write!(f, "avr_builtincc"),
//             CallingConvention::AMDGPU_CS => write!(f, "amdgpu_cs"),
//             CallingConvention::AMDGPU_ES => write!(f, "amdgpu_es"),
//             CallingConvention::AMDGPU_GS => write!(f, "amdgpu_gs"),
//             CallingConvention::AMDGPU_HS => write!(f, "amdgpu_hs"),
//             CallingConvention::AMDGPU_LS => write!(f, "amdgpu_ls"),
//             CallingConvention::AMDGPU_PS => write!(f, "amdgpu_ps"),
//             CallingConvention::AMDGPU_VS => write!(f, "amdgpu_vs"),
//             CallingConvention::AMDGPU_Kernel => write!(f, "amdgpu_kernel"),
//             CallingConvention::Numbered(n) => write!(f, "cc {}", n),
//         }
//     }
// }

#[derive(PartialEq, Eq, Clone, Debug, Hash)]
pub enum MemoryEffect {
    None,
    Read,
    Write,
    ReadWrite
}

impl MemoryEffect {
    pub(crate) fn from_llvm_bits(val : u64) -> Self {
        match val {
            0b00 => Self::None,
            0b01 => Self::Read,
            0b10 => Self::Write,
            0b11 => Self::ReadWrite,
            _ => panic!("Memory effect given unexpected bits {}", val)
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
        inaccessible_mem: MemoryEffect
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

// impl Display for FunctionAttribute {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         match self {
//             FunctionAttribute::AlignStack(n) => write!(f, "alignstack({})", n),
//             FunctionAttribute::AllocSize { elt_size, num_elts } => {
//                 if let Some(n) = num_elts {
//                     write!(f, "allocsize({}, {})", elt_size, n)
//                 } else {
//                     write!(f, "allocsize({})", elt_size)
//                 }
//             }
//             FunctionAttribute::AlwaysInline => write!(f, "alwaysinline"),
//             FunctionAttribute::Builtin => write!(f, "builtin"),
//             FunctionAttribute::Cold => write!(f, "cold"),
//             FunctionAttribute::Convergent => write!(f, "convergent"),
//             FunctionAttribute::InaccessibleMemOnly => write!(f, "inaccessiblememonly"),
//             FunctionAttribute::InaccessibleMemOrArgMemOnly => write!(f, "inaccessiblemem_or_argmemonly"),
//             FunctionAttribute::InlineHint => write!(f, "inlinehint"),
//             FunctionAttribute::JumpTable => write!(f, "jumptable"),
//             FunctionAttribute::MinimizeSize => write!(f, "minsize"),
//             FunctionAttribute::Naked => write!(f, "naked"),
//             FunctionAttribute::NoBuiltin => write!(f, "nobuiltin"),
//             FunctionAttribute::NoCFCheck => write!(f, "nocf_check"),
//             FunctionAttribute::NoDuplicate => write!(f, "noduplicate"),
//             FunctionAttribute::NoFree => write!(f, "nofree"),
//             FunctionAttribute::NoImplicitFloat => write!(f, "noimplicitfloat"),
//             FunctionAttribute::NoInline => write!(f, "noinline"),
//             FunctionAttribute::NoMerge => write!(f, "nomerge"),
//             FunctionAttribute::NonLazyBind => write!(f, "nonlazybind"),
//             FunctionAttribute::NoRedZone => write!(f, "noredzone"),
//             FunctionAttribute::NoReturn => write!(f, "noreturn"),
//             FunctionAttribute::NoRecurse => write!(f, "norecurse"),
//             FunctionAttribute::WillReturn => write!(f, "willreturn"),
//             FunctionAttribute::ReturnsTwice => write!(f, "returns_twice"),
//             FunctionAttribute::NoSync => write!(f, "nosync"),
//             FunctionAttribute::NoUnwind => write!(f, "nounwind"),
//             FunctionAttribute::NullPointerIsValid => write!(f, "null_pointer_is_valid"),
//             FunctionAttribute::OptForFuzzing => write!(f, "optforfuzzing"),
//             FunctionAttribute::OptNone => write!(f, "optnone"),
//             FunctionAttribute::OptSize => write!(f, "optsize"),
//             FunctionAttribute::ReadNone => write!(f, "readnone"),
//             FunctionAttribute::ReadOnly => write!(f, "readonly"),
//             FunctionAttribute::WriteOnly => write!(f, "writeonly"),
//             FunctionAttribute::ArgMemOnly => write!(f, "argmemonly"),
//             FunctionAttribute::SafeStack => write!(f, "safestack"),
//             FunctionAttribute::SanitizeAddress => write!(f, "sanitize_address"),
//             FunctionAttribute::SanitizeMemory => write!(f, "sanitize_memory"),
//             FunctionAttribute::SanitizeThread => write!(f, "sanitize_thread"),
//             FunctionAttribute::SanitizeHWAddress => write!(f, "sanitize_hwaddress"),
//             FunctionAttribute::SanitizeMemTag => write!(f, "sanitize_memtag"),
//             FunctionAttribute::ShadowCallStack => write!(f, "shadowcallstack"),
//             FunctionAttribute::SpeculativeLoadHardening => write!(f, "speculative_load_hardening"),
//             FunctionAttribute::Speculatable => write!(f, "speculatable"),
//             FunctionAttribute::StackProtect => write!(f, "ssp"),
//             FunctionAttribute::StackProtectReq => write!(f, "sspreq"),
//             FunctionAttribute::StackProtectStrong => write!(f, "sspstrong"),
//             FunctionAttribute::StrictFP => write!(f, "strictfp"),
//             FunctionAttribute::UWTable => write!(f, "uwtable"),
//             FunctionAttribute::Memory { default, argmem, inaccessible_mem } => {
//                 write!(f, "memory(")?;
//                 let mut first = true;
//                 let write_part = |w: &mut fmt::Formatter, first: &mut bool, pre: &str, me: &MemoryEffect| -> fmt::Result {
//                     match me {
//                         MemoryEffect::None => {}
//                         MemoryEffect::Read => {
//                             if !*first { write!(w, ", ")?; }
//                             write!(w, "{}read", pre)?;
//                             *first = false;
//                         }
//                         MemoryEffect::Write => {
//                             if !*first { write!(w, ", ")?; }
//                             write!(w, "{}write", pre)?;
//                             *first = false;
//                         }
//                         MemoryEffect::ReadWrite => {
//                             if !*first { write!(w, ", ")?; }
//                             write!(w, "{}readwrite", pre)?;
//                             *first = false;
//                         }
//                     }
//                     Ok(())
//                 };
//                 write_part(f, &mut first, "", default)?;
//                 write_part(f, &mut first, "argmem: ", argmem)?;
//                 write_part(f, &mut first, "inaccessiblemem: ", inaccessible_mem)?;
//                 write!(f, ")")
//             }
//             FunctionAttribute::StringAttribute { kind, value } => {
//                 if value.is_empty() {
//                     write!(f, "\"{}\"", kind)
//                 } else {
//                     write!(f, "\"{}\"=\"{}\"", kind, value)
//                 }
//             }
//             FunctionAttribute::UnknownAttribute => write!(f, ""),
//         }
//     }
// }

// impl Display for ParameterAttribute {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         match self {
//             ParameterAttribute::ZeroExt => write!(f, "zeroext"),
//             ParameterAttribute::SignExt => write!(f, "signext"),
//             ParameterAttribute::InReg => write!(f, "inreg"),
//             ParameterAttribute::ByVal(ty) => write!(f, "byval({})", ty),
//             ParameterAttribute::Preallocated(ty) => write!(f, "preallocated({})", ty),
//             ParameterAttribute::InAlloca(ty) => write!(f, "inalloca({})", ty),
//             ParameterAttribute::SRet(ty) => write!(f, "sret({})", ty),
//             ParameterAttribute::Alignment(n) => write!(f, "align {}", n),
//             ParameterAttribute::NoAlias => write!(f, "noalias"),
//             ParameterAttribute::NoCapture => write!(f, "nocapture"),
//             ParameterAttribute::NoFree => write!(f, "nofree"),
//             ParameterAttribute::Nest => write!(f, "nest"),
//             ParameterAttribute::Returned => write!(f, "returned"),
//             ParameterAttribute::NonNull => write!(f, "nonnull"),
//             ParameterAttribute::Dereferenceable(n) => write!(f, "dereferenceable({})", n),
//             ParameterAttribute::DereferenceableOrNull(n) => write!(f, "dereferenceable_or_null({})", n),
//             ParameterAttribute::SwiftSelf => write!(f, "swiftself"),
//             ParameterAttribute::SwiftError => write!(f, "swifterror"),
//             ParameterAttribute::ImmArg => write!(f, "immarg"),
//             ParameterAttribute::NoUndef => write!(f, "noundef"),
//             ParameterAttribute::StringAttribute { kind, value } => {
//                 if value.is_empty() {
//                     write!(f, "\"{}\"", kind)
//                 } else {
//                     write!(f, "\"{}\"=\"{}\"", kind, value)
//                 }
//             }
//             ParameterAttribute::UnknownAttribute => write!(f, ""),
//             ParameterAttribute::UnknownTypeAttribute(ty) => write!(f, "{}", ty),
//         }
//     }
// }

pub type GroupID = usize;
