use crate::llvm_ir::constant::ConstantRef;
// use crate::llvm_ir::debugloc::*;
use crate::llvm_ir::function::{Function, FunctionAttribute, FunctionDeclaration, GroupID};
use crate::llvm_ir::name::Name;
use crate::llvm_ir::types::{FPType, Type, TypeRef, Typed, Types};
use std::collections::{BTreeMap, HashMap, HashSet};

#[derive(Clone)]
pub struct Module {
    pub name: String,
    pub source_file_name: String,
    pub data_layout: DataLayout,
    pub target_triple: Option<String>,
    pub functions: Vec<Function>,
    pub func_declarations: Vec<FunctionDeclaration>,
    pub global_vars: Vec<GlobalVariable>,
    pub global_aliases: Vec<GlobalAlias>,
    pub global_ifuncs: Vec<GlobalIFunc>,
    pub inline_assembly: String,
    pub types: Types,
}

impl Module {
    pub fn type_of<T: Typed + ?Sized>(&self, t: &T) -> TypeRef {
        self.types.type_of(t)
    }

    pub fn get_func_by_name(&self, name: &str) -> Option<&Function> {
        self.functions.iter().find(|func| func.name == name)
    }

    pub fn get_func_decl_by_name(&self, name: &str) -> Option<&FunctionDeclaration> {
        self.func_declarations.iter().find(|decl| decl.name == name)
    }

    pub fn get_global_var_by_name(&self, name: &Name) -> Option<&GlobalVariable> {
        self.global_vars.iter().find(|global| global.name == *name)
    }

    pub fn get_global_alias_by_name(&self, name: &Name) -> Option<&GlobalAlias> {
        self.global_aliases
            .iter()
            .find(|global| global.name == *name)
    }

    pub fn get_global_ifunc_by_name(&self, name: &Name) -> Option<&GlobalIFunc> {
        self.global_ifuncs
            .iter()
            .find(|global| global.name == *name)
    }
}

#[derive(PartialEq, Clone, Debug, Hash)]
pub struct GlobalVariable {
    pub name: Name,
    pub linkage: Linkage,
    pub visibility: Visibility,
    pub is_constant: bool,
    pub ty: TypeRef,
    pub addr_space: AddrSpace,
    pub dll_storage_class: DLLStorageClass,
    pub thread_local_mode: ThreadLocalMode,
    pub unnamed_addr: Option<UnnamedAddr>,
    pub initializer: Option<ConstantRef>,
    pub section: Option<String>,
    pub comdat: Option<Comdat>, // llvm-hs-pure has Option<String> for some reason
    pub alignment: u32,
    // pub debugloc: Option<DebugLoc>,
    pub value_type: TypeRef, // --TODO not yet implemented-- pub metadata: Vec<(String, MetadataRef<MetadataNode>)>,
}

impl Typed for GlobalVariable {
    fn get_type(&self, _types: &Types) -> TypeRef {
        self.ty.clone()
    }
}

// impl HasDebugLoc for GlobalVariable {
//     fn get_debug_loc(&self) -> &Option<DebugLoc> {
//         &self.debugloc
//     }
// }

#[derive(PartialEq, Clone, Debug, Hash)]
pub struct GlobalAlias {
    pub name: Name,
    pub aliasee: ConstantRef,
    pub linkage: Linkage,
    pub visibility: Visibility,
    pub ty: TypeRef,
    pub addr_space: AddrSpace,
    pub dll_storage_class: DLLStorageClass,
    pub thread_local_mode: ThreadLocalMode,
    pub unnamed_addr: Option<UnnamedAddr>,
}

impl Typed for GlobalAlias {
    fn get_type(&self, _types: &Types) -> TypeRef {
        self.ty.clone()
    }
}

#[derive(PartialEq, Clone, Debug, Hash)]
pub struct GlobalIFunc {
    pub name: Name,
    pub linkage: Linkage,
    pub visibility: Visibility,
    pub ty: TypeRef,
    pub resolver_fn: ConstantRef,
}

impl Typed for GlobalIFunc {
    fn get_type(&self, _types: &Types) -> TypeRef {
        self.ty.clone()
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Debug, Hash)]
pub enum UnnamedAddr {
    Local,
    Global,
}

#[derive(PartialEq, Eq, Clone, Copy, Debug, Hash)]
pub enum Linkage {
    Private,
    Internal,
    External,
    ExternalWeak,
    AvailableExternally,
    LinkOnceAny,
    LinkOnceODR,
    LinkOnceODRAutoHide,
    WeakAny,
    WeakODR,
    Common,
    Appending,
    DLLImport,
    DLLExport,
    Ghost,
    LinkerPrivate,
    LinkerPrivateWeak,
}

#[derive(PartialEq, Eq, Clone, Copy, Debug, Hash)]
pub enum Visibility {
    Default,
    Hidden,
    Protected,
}

#[derive(PartialEq, Eq, Clone, Copy, Debug, Hash)]
pub enum DLLStorageClass {
    Default,
    Import,
    Export,
}

#[derive(PartialEq, Eq, Clone, Copy, Debug, Hash)]
pub enum ThreadLocalMode {
    NotThreadLocal,
    GeneralDynamic,
    LocalDynamic,
    InitialExec,
    LocalExec,
}

pub type AddrSpace = u32;

#[derive(PartialEq, Eq, Clone, Debug, Hash)]
pub struct FunctionAttributeGroup {
    pub group_id: GroupID,
    pub attrs: Vec<FunctionAttribute>,
}

/* --TODO not yet implemented: metadata
#[derive(PartialEq, Eq, Clone, Debug, Hash)]
pub struct NamedMetadata {
    pub name: String,
    pub node_ids: Vec<MetadataNodeID>,
}
*/

#[derive(PartialEq, Eq, Clone, Debug, Hash)]
pub struct Comdat {
    pub name: String,
    pub selection_kind: SelectionKind,
}

#[derive(PartialEq, Eq, Clone, Copy, Debug, Hash)]
pub enum SelectionKind {
    Any,
    ExactMatch,
    Largest,
    NoDuplicates,
    SameSize,
}

#[derive(Clone, Debug)]
pub struct DataLayout {
    pub layout_str: String,
    pub endianness: Endianness,
    pub stack_alignment: Option<u32>,
    pub program_address_space: AddrSpace,
    pub alloca_address_space: AddrSpace,
    pub alignments: Alignments,
    pub mangling: Option<Mangling>,
    pub native_int_widths: Option<HashSet<u32>>,
    pub non_integral_ptr_types: HashSet<AddrSpace>,
}

impl PartialEq for DataLayout {
    fn eq(&self, other: &Self) -> bool {
        self.layout_str == other.layout_str
    }
}

impl Eq for DataLayout {}

#[derive(PartialEq, Eq, Clone, Copy, Debug, Hash)]
pub enum Endianness {
    LittleEndian,
    BigEndian,
}

#[derive(Clone, PartialEq, Eq, Debug, Hash)]
pub struct Alignment {
    pub abi: u32,
    pub pref: u32,
}

#[derive(Clone, PartialEq, Eq, Debug, Hash)]
pub struct FunctionPtrAlignment {
    pub independent: bool,
    pub abi: u32,
}

#[derive(Clone, PartialEq, Eq, Debug, Hash)]
pub struct PointerLayout {
    pub size: u32,
    pub alignment: Alignment,
    pub index_size: u32,
}

#[derive(Clone, Debug)]
pub struct Alignments {
    int_alignments: BTreeMap<u32, Alignment>,
    vec_alignments: BTreeMap<u32, Alignment>,
    fp_alignments: HashMap<u32, Alignment>,
    agg_alignment: Alignment,
    fptr_alignment: FunctionPtrAlignment,
    fptr_alignment_as_alignment: Alignment,
    pointer_layouts: HashMap<AddrSpace, PointerLayout>,
}

impl Alignments {
    pub fn type_alignment(&self, ty: &Type) -> &Alignment {
        match ty {
            Type::IntegerType { bits } => self.int_alignment(*bits),
            Type::VectorType {
                element_type,
                num_elements,
                ..
            } => {
                let element_size_bits = match element_type.as_ref() {
                    Type::IntegerType { bits } => *bits,
                    Type::FPType(fpt) => Self::fpt_size(*fpt),
                    ty => panic!("Didn't expect a vector with element type {:?}", ty),
                };
                self.vec_alignment(element_size_bits * (*num_elements as u32))
            }
            Type::FPType(fpt) => self.fp_alignment(*fpt),
            Type::StructType { .. } | Type::NamedStructType { .. } | Type::ArrayType { .. } => {
                self.agg_alignment()
            }
            Type::PointerType { addr_space } => &self.ptr_alignment(*addr_space).alignment,
            _ => panic!("Don't know how to get the alignment of {:?}", ty),
        }
    }

    pub fn int_alignment(&self, size: u32) -> &Alignment {
        if let Some(alignment) = self.int_alignments.get(&size) {
            return alignment;
        }
        let next_largest_entry = self.int_alignments.iter().find(|(k, _)| **k > size);
        match next_largest_entry {
            Some((_, alignment)) => alignment,
            None => {
                self.int_alignments
                    .values()
                    .rev()
                    .next()
                    .expect("Should have at least one explicit entry")
            }
        }
    }

    pub fn vec_alignment(&self, size: u32) -> &Alignment {
        if let Some(alignment) = self.vec_alignments.get(&size) {
            return alignment;
        }
        let next_smaller_entry = self.vec_alignments.iter().find(|(k, _)| **k < size);
        match next_smaller_entry {
            Some((_, alignment)) => alignment,
            None => {
                self.vec_alignments
                    .values()
                    .next()
                    .expect("Should have at least one explicit entry")
            }
        }
    }

    pub fn fp_alignment(&self, fpt: FPType) -> &Alignment {
        self.fp_alignments
            .get(&Self::fpt_size(fpt))
            .unwrap_or_else(|| {
                panic!(
                    "No alignment information for {:?} - does the target support that type?",
                    fpt
                )
            })
    }

    pub fn agg_alignment(&self) -> &Alignment {
        &self.agg_alignment
    }

    pub fn fptr_alignment(&self) -> &FunctionPtrAlignment {
        &self.fptr_alignment
    }

    pub fn ptr_alignment(&self, addr_space: AddrSpace) -> &PointerLayout {
        match self.pointer_layouts.get(&addr_space) {
            Some(layout) => layout,
            None => self
                .pointer_layouts
                .get(&0)
                .expect("Should have a pointer layout for address space 0"),
        }
    }

    fn fpt_size(fpt: FPType) -> u32 {
        match fpt {
            FPType::Half => 16,
            FPType::BFloat => 16,
            FPType::Single => 32,
            FPType::Double => 64,
            FPType::FP128 => 128,
            FPType::X86_FP80 => 80,
            FPType::PPC_FP128 => 128,
        }
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Debug, Hash)]
pub enum Mangling {
    ELF,
    MIPS,
    MachO,
    WindowsX86COFF,
    WindowsCOFF,
    XCOFF,
}
