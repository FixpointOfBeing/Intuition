use crate::llvm_ir::module::AddrSpace;
use either::Either;
use std::borrow::Borrow;
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::fmt::{self, Display};
use std::hash::Hash;
use std::ops::Deref;
use std::sync::Arc;

#[derive(PartialEq, Eq, Clone, Debug, Hash)]
#[allow(non_camel_case_types)]
pub enum LLVMType {
    VoidType,
    IntegerType {
        bits: u32,
    },
    PointerType {
        addr_space: AddrSpace,
    },
    FPType(FPType),
    FuncType {
        result_type: TypeRef,
        param_types: Vec<TypeRef>,
        is_var_arg: bool,
    },
    VectorType {
        element_type: TypeRef,
        num_elements: usize,
        scalable: bool,
    },
    ArrayType {
        element_type: TypeRef,
        num_elements: usize,
    },
    StructType {
        element_types: Vec<TypeRef>,
        is_packed: bool,
    },
    NamedStructType {
        name: String, // llvm-hs-pure has Name rather than String
    },
    X86_MMXType,
    X86_AMXType,
    MetadataType,
    LabelType,
    TokenType,
    TargetExtType, // TODO ideally we want something like TargetExtType { name: String, contained_types: Vec<TypeRef>, contained_ints: Vec<u32> }
}

impl Display for LLVMType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LLVMType::VoidType => write!(f, "void"),
            LLVMType::IntegerType { bits } => write!(f, "i{}", bits),
            LLVMType::PointerType { .. } => write!(f, "ptr"),
            LLVMType::FPType(fpt) => write!(f, "{}", fpt),
            LLVMType::FuncType {
                result_type,
                param_types,
                is_var_arg,
            } => {
                write!(f, "{} (", result_type)?;
                for (i, param_ty) in param_types.iter().enumerate() {
                    if i == param_types.len() - 1 {
                        write!(f, "{}", param_ty)?;
                    } else {
                        write!(f, "{}, ", param_ty)?;
                    }
                }
                if *is_var_arg {
                    write!(f, ", ...")?;
                }
                write!(f, ")")?;
                Ok(())
            },
            LLVMType::VectorType {
                element_type,
                num_elements,
                scalable,
            } => {
                if *scalable {
                    write!(f, "<vscale x {} x {}>", num_elements, element_type)
                } else {
                    write!(f, "<{} x {}>", num_elements, element_type)
                }
            },
            LLVMType::ArrayType {
                element_type,
                num_elements,
            } => write!(f, "[{} x {}]", num_elements, element_type),
            LLVMType::StructType {
                element_types,
                is_packed,
            } => {
                if *is_packed {
                    write!(f, "<")?;
                }
                write!(f, "{{ ")?;
                for (i, element_ty) in element_types.iter().enumerate() {
                    if i == element_types.len() - 1 {
                        write!(f, "{}", element_ty)?;
                    } else {
                        write!(f, "{}, ", element_ty)?;
                    }
                }
                write!(f, " }}")?;
                if *is_packed {
                    write!(f, ">")?;
                }
                Ok(())
            },
            LLVMType::NamedStructType { name } => write!(f, "%{}", name),
            LLVMType::X86_MMXType => write!(f, "x86_mmx"),
            LLVMType::X86_AMXType => write!(f, "x86_amx"),
            LLVMType::MetadataType => write!(f, "metadata"),
            LLVMType::LabelType => write!(f, "label"),
            LLVMType::TokenType => write!(f, "token"),
            LLVMType::TargetExtType => write!(f, "target()"),
            /*
            let members = [name]
                .iter()
                .map(|name| format!("\"{name}\""))
                .chain(contained_types.iter().map(ToString::to_string))
                .chain(contained_ints.iter().map(ToString::to_string))
                .collect::<Vec<_>>()
                .join(", ");
            write!(f, "target({members})")?;

            Ok(())
            */
        }
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Debug, Hash)]
#[allow(non_camel_case_types)]
pub enum FPType {
    Half,
    BFloat,
    Single,
    Double,
    FP128,
    X86_FP80,
    PPC_FP128,
}

impl From<FPType> for LLVMType {
    fn from(fpt: FPType) -> LLVMType {
        LLVMType::FPType(fpt)
    }
}

impl Display for FPType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            FPType::Half => write!(f, "half"),
            FPType::BFloat => write!(f, "bfloat"),
            FPType::Single => write!(f, "float"),
            FPType::Double => write!(f, "double"),
            FPType::FP128 => write!(f, "fp128"),
            FPType::X86_FP80 => write!(f, "x86_fp80"),
            FPType::PPC_FP128 => write!(f, "ppc_fp128"),
        }
    }
}

#[derive(PartialEq, Eq, Clone, Debug, Hash)]
pub struct TypeRef(Arc<LLVMType>);

impl AsRef<LLVMType> for TypeRef {
    fn as_ref(&self) -> &LLVMType {
        self.0.as_ref()
    }
}

impl Deref for TypeRef {
    type Target = LLVMType;

    fn deref(&self) -> &LLVMType {
        self.0.deref()
    }
}

impl Display for TypeRef {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", &self.0)
    }
}

impl TypeRef {
    fn new(ty: LLVMType) -> Self {
        Self(Arc::new(ty))
    }
}

pub trait Typed {
    fn get_type(&self, types: &Types) -> TypeRef;
}

impl Typed for TypeRef {
    fn get_type(&self, _types: &Types) -> TypeRef {
        self.clone()
    }
}

impl Typed for LLVMType {
    fn get_type(&self, types: &Types) -> TypeRef {
        types.get_for_type(self)
    }
}

impl Typed for FPType {
    fn get_type(&self, types: &Types) -> TypeRef {
        types.fp(*self)
    }
}

impl<A, B> Typed for Either<A, B>
where
    A: Typed,
    B: Typed,
{
    fn get_type(&self, types: &Types) -> TypeRef {
        match self {
            Either::Left(x) => types.type_of(x),
            Either::Right(y) => types.type_of(y),
        }
    }
}

#[derive(Clone, Debug, Hash)]
pub enum NamedStructDef {
    Opaque,
    Defined(TypeRef),
}

impl Display for NamedStructDef {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            NamedStructDef::Opaque => write!(f, "type opaque"),
            NamedStructDef::Defined(ty) => write!(f, "type {}", ty),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Types {
    void_type: TypeRef,
    int_types: TypeCache<u32>,
    pointer_types: TypeCache<AddrSpace>,
    fp_types: TypeCache<FPType>,
    func_types: TypeCache<(TypeRef, Vec<TypeRef>, bool)>,
    vec_types: TypeCache<(TypeRef, usize, bool)>,
    arr_types: TypeCache<(TypeRef, usize)>,
    struct_types: TypeCache<(Vec<TypeRef>, bool)>,
    named_struct_types: TypeCache<String>,
    named_struct_defs: HashMap<String, NamedStructDef>,
    x86_mmx_type: TypeRef,
    x86_amx_type: TypeRef,
    metadata_type: TypeRef,
    label_type: TypeRef,
    token_type: TypeRef,
    target_ext_type: TypeRef,
}

impl Types {
    pub fn new() -> Self {
        Self {
            void_type: TypeRef::new(LLVMType::VoidType),
            int_types: TypeCache::new(),
            pointer_types: TypeCache::new(),
            fp_types: TypeCache::new(),
            func_types: TypeCache::new(),
            vec_types: TypeCache::new(),
            arr_types: TypeCache::new(),
            struct_types: TypeCache::new(),
            named_struct_types: TypeCache::new(),
            named_struct_defs: HashMap::new(),
            x86_mmx_type: TypeRef::new(LLVMType::X86_MMXType),
            x86_amx_type: TypeRef::new(LLVMType::X86_AMXType),
            metadata_type: TypeRef::new(LLVMType::MetadataType),
            label_type: TypeRef::new(LLVMType::LabelType),
            token_type: TypeRef::new(LLVMType::TokenType),
            target_ext_type: TypeRef::new(LLVMType::TargetExtType),
        }
    }
    pub fn type_of<T: Typed + ?Sized>(&self, t: &T) -> TypeRef {
        t.get_type(self)
    }

    pub fn void(&self) -> TypeRef {
        self.void_type.clone()
    }

    pub fn int(&self, bits: u32) -> TypeRef {
        self.int_types
            .lookup(&bits)
            .unwrap_or_else(|| TypeRef::new(LLVMType::IntegerType { bits }))
    }

    pub fn bool(&self) -> TypeRef {
        self.int(1)
    }

    pub fn i8(&self) -> TypeRef {
        self.int(8)
    }

    pub fn i16(&self) -> TypeRef {
        self.int(16)
    }

    pub fn i32(&self) -> TypeRef {
        self.int(32)
    }

    pub fn i64(&self) -> TypeRef {
        self.int(64)
    }

    pub fn pointer(&self) -> TypeRef {
        self.pointer_in_addr_space(0)
    }

    pub fn pointer_in_addr_space(&self, addr_space: AddrSpace) -> TypeRef {
        self.pointer_types
            .lookup(&addr_space)
            .unwrap_or_else(|| TypeRef::new(LLVMType::PointerType { addr_space }))
    }

    pub fn fp(&self, fpt: FPType) -> TypeRef {
        self.fp_types
            .lookup(&fpt)
            .unwrap_or_else(|| TypeRef::new(LLVMType::FPType(fpt)))
    }

    pub fn single(&self) -> TypeRef {
        self.fp(FPType::Single)
    }

    pub fn double(&self) -> TypeRef {
        self.fp(FPType::Double)
    }

    pub fn func_type(
        &self,
        result_type: TypeRef,
        param_types: Vec<TypeRef>,
        is_var_arg: bool,
    ) -> TypeRef {
        self.func_types
            .lookup(&(result_type.clone(), param_types.clone(), is_var_arg))
            .unwrap_or_else(|| {
                TypeRef::new(LLVMType::FuncType {
                    result_type,
                    param_types,
                    is_var_arg,
                })
            })
    }

    pub fn vector_of(&self, element_type: TypeRef, num_elements: usize, scalable: bool) -> TypeRef {
        self.vec_types
            .lookup(&(element_type.clone(), num_elements, scalable))
            .unwrap_or_else(|| {
                TypeRef::new(LLVMType::VectorType {
                    element_type,
                    num_elements,
                    scalable,
                })
            })
    }

    pub fn array_of(&self, element_type: TypeRef, num_elements: usize) -> TypeRef {
        self.arr_types
            .lookup(&(element_type.clone(), num_elements))
            .unwrap_or_else(|| {
                TypeRef::new(LLVMType::ArrayType {
                    element_type,
                    num_elements,
                })
            })
    }

    pub fn struct_of(&self, element_types: Vec<TypeRef>, is_packed: bool) -> TypeRef {
        self.struct_types
            .lookup(&(element_types.clone(), is_packed))
            .unwrap_or_else(|| {
                TypeRef::new(LLVMType::StructType {
                    element_types,
                    is_packed,
                })
            })
    }

    pub fn named_struct(&self, name: &str) -> TypeRef {
        self.named_struct_types
            .lookup(name)
            .unwrap_or_else(|| TypeRef::new(LLVMType::NamedStructType { name: name.into() }))
    }

    pub fn named_struct_def(&self, name: &str) -> Option<&NamedStructDef> {
        self.named_struct_defs.get(name)
    }

    pub fn all_struct_names(&self) -> impl Iterator<Item = &String> {
        self.named_struct_defs.keys()
    }

    pub fn add_named_struct_def(&mut self, name: String, def: NamedStructDef) {
        match self.named_struct_defs.entry(name) {
            Entry::Occupied(_) => {
                panic!("Trying to redefine named struct");
            },
            Entry::Vacant(ventry) => {
                ventry.insert(def);
            },
        }
    }

    pub fn remove_named_struct_def(&mut self, name: &str) -> bool {
        self.named_struct_defs.remove(name).is_some()
    }

    pub fn x86_mmx(&self) -> TypeRef {
        self.x86_mmx_type.clone()
    }

    pub fn x86_amx(&self) -> TypeRef {
        self.x86_amx_type.clone()
    }

    pub fn metadata_type(&self) -> TypeRef {
        self.metadata_type.clone()
    }

    pub fn label_type(&self) -> TypeRef {
        self.label_type.clone()
    }

    pub fn token_type(&self) -> TypeRef {
        self.token_type.clone()
    }

    pub fn target_ext_type(
        &self,
    ) -> TypeRef {
        self.target_ext_type.clone()
    }

    #[rustfmt::skip] // so we can keep each of the match arms more consistent with each other
    pub fn get_for_type(&self, ty: &LLVMType) -> TypeRef {
        match ty {
            LLVMType::VoidType => self.void(),
            LLVMType::IntegerType{ bits } => self.int(*bits),
            LLVMType::PointerType { addr_space } => {
                self.pointer_in_addr_space(*addr_space)
            },
            LLVMType::FPType(fpt) => self.fp(*fpt),
            LLVMType::FuncType { result_type, param_types, is_var_arg } => {
                self.func_type(result_type.clone(), param_types.clone(), *is_var_arg)
            },
            LLVMType::VectorType { element_type, num_elements, scalable } => {
                self.vector_of(element_type.clone(), *num_elements, *scalable)
            },
            LLVMType::ArrayType { element_type, num_elements } => {
                self.array_of(element_type.clone(), *num_elements)
            },
            LLVMType::StructType { element_types, is_packed } => {
                self.struct_of(element_types.clone(), *is_packed)
            },
            LLVMType::NamedStructType { name  } => self.named_struct(name),
            LLVMType::X86_MMXType => self.x86_mmx(),
            LLVMType::X86_AMXType => self.x86_amx(),
            LLVMType::MetadataType => self.metadata_type(),
            LLVMType::LabelType => self.label_type(),
            LLVMType::TokenType => self.token_type(),
            LLVMType::TargetExtType => self.target_ext_type(),
        }
    }
}

#[derive(Clone, Debug)]
struct TypeCache<K: Eq + Hash + Clone> {
    map: HashMap<K, TypeRef>,
}

#[allow(dead_code)]
impl<K: Eq + Hash + Clone> TypeCache<K> {
    fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }

    fn lookup<Q: ?Sized>(&self, key: &Q) -> Option<TypeRef>
    where
        K: Borrow<Q>,
        Q: Hash + Eq,
    {
        self.map.get(key).cloned()
    }

    fn lookup_or_insert(&mut self, key: K, if_missing: impl FnOnce() -> LLVMType) -> TypeRef {
        self.map
            .entry(key)
            .or_insert_with(|| TypeRef::new(if_missing()))
            .clone()
    }

    fn contains_key(&self, key: &K) -> bool {
        self.map.contains_key(key)
    }
}
