use crate::llvm_ir::name::Name;
use crate::llvm_ir::types::{FPType, LLVMType, TypeRef, Typed, Types};
use std::convert::TryFrom;
use std::fmt::{self, Display};
use std::ops::Deref;
use std::sync::Arc;

#[derive(PartialEq, Clone, Debug, Hash)]
pub enum Constant {
    Int {
        bits: u32,
        value: u64,
    },
    Float(Float),
    Null(TypeRef),
    AggregateZero(TypeRef),
    Struct {
        name: Option<String>, // llvm-hs-pure has Option<Name> here, but I don't think struct types can be numbered
        values: Vec<ConstantRef>,
        is_packed: bool,
    },
    Array {
        element_type: TypeRef,
        elements: Vec<ConstantRef>,
    },
    Vector(Vec<ConstantRef>),
    Undef(TypeRef),
    Poison(TypeRef),
    BlockAddress, // --TODO ideally we want BlockAddress { function: Name, block: Name },
    GlobalReference {
        name: Name,
        ty: TypeRef,
    },
    TokenNone,
    PtrAuth {
        ptr : ConstantRef,
        key : ConstantRef,
        disc : ConstantRef,
        addr_disc : ConstantRef
    },


    Add(Add),
    Sub(Sub),
    Mul(Mul),

    Xor(Xor),


    ExtractElement(ExtractElement),
    InsertElement(InsertElement),
    ShuffleVector(ShuffleVector),


    GetElementPtr(GetElementPtr),

    Trunc(Trunc),
    PtrToInt(PtrToInt),
    IntToPtr(IntToPtr),
    BitCast(BitCast),
    AddrSpaceCast(AddrSpaceCast),

}

#[derive(PartialEq, Clone, Debug)]
#[allow(non_camel_case_types)]
pub enum Float {
    Half, // TODO perhaps Half(u16)
    BFloat, // TODO perhaps BFloat(u16)
    Single(f32),
    Double(f64),
    Quadruple, // TODO perhaps Quadruple(u128)
    X86_FP80,  // TODO perhaps X86_FP80((u16, u64)) with the most-significant bits on the left
    PPC_FP128, // TODO perhaps PPC_FP128((u64, u64)) with the most-significant bits on the left
}

impl std::hash::Hash for Float {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        core::mem::discriminant(self).hash(state);
        match self {
            Float::Single(f) => ordered_float::OrderedFloat(*f).hash(state),
            Float::Double(f) => ordered_float::OrderedFloat(*f).hash(state),
            _ => {},
        }
    }
}

impl Typed for Float {
    fn get_type(&self, types: &Types) -> TypeRef {
        types.fp(match self {
            Float::Half => FPType::Half,
            Float::BFloat => FPType::BFloat,
            Float::Single(_) => FPType::Single,
            Float::Double(_) => FPType::Double,
            Float::Quadruple => FPType::FP128,
            Float::X86_FP80 => FPType::X86_FP80,
            Float::PPC_FP128 => FPType::PPC_FP128,
        })
    }
}

// impl Display for Float {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         match self {
//             Float::Half => write!(f, "half"),
//             Float::BFloat => write!(f, "bfloat"),
//             Float::Single(s) => write!(f, "float {}", s),
//             Float::Double(d) => write!(f, "double {}", d),
//             Float::Quadruple => write!(f, "quadruple"),
//             Float::X86_FP80 => write!(f, "x86_fp80"),
//             Float::PPC_FP128 => write!(f, "ppc_fp128"),
//         }
//     }
// }

impl Typed for Constant {
    #[rustfmt::skip] // to keep all the branches more consistent with each other
    fn get_type(&self, types: &Types) -> TypeRef {
        match self {
            Constant::Int { bits, .. } => types.int(*bits),
            Constant::Float(f) => types.type_of(f),
            Constant::Null(t) => t.clone(),
            Constant::AggregateZero(t) => t.clone(),
            Constant::Struct { values, is_packed, .. } => types.struct_of(
                values.iter().map(|v| types.type_of(v)).collect(),
                *is_packed,
            ),
            Constant::Array { element_type, elements } => types.array_of(
                element_type.clone(),
                elements.len(),
            ),
            Constant::Vector(v) => types.vector_of(
                types.type_of(&v[0]),
                v.len(),
                false, // I don't think it's possible (at least as of LLVM 11) to have a constant of scalable vector type?
            ),
            Constant::Undef(t) => t.clone(),
            Constant::Poison(t) => t.clone(),
            Constant::BlockAddress { .. } => types.label_type(),
            Constant::GlobalReference { .. } => types.pointer(),
            Constant::TokenNone => types.token_type(),
            Constant::Add(a) => types.type_of(a),
            Constant::Sub(s) => types.type_of(s),
            Constant::Mul(m) => types.type_of(m),
            Constant::Xor(x) => types.type_of(x),
            Constant::ExtractElement(e) => types.type_of(e),
            Constant::InsertElement(i) => types.type_of(i),
            Constant::ShuffleVector(s) => types.type_of(s),
            Constant::GetElementPtr(g) => types.type_of(g),
            Constant::Trunc(t) => types.type_of(t),
            Constant::PtrToInt(p) => types.type_of(p),
            Constant::IntToPtr(i) => types.type_of(i),
            Constant::BitCast(b) => types.type_of(b),
            Constant::AddrSpaceCast(a) => types.type_of(a),
            Constant::PtrAuth { .. } => types.pointer(),
        }
    }
}

// impl Display for Constant {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         match self {
//             Constant::Int { bits, value } => {
//                 if *bits == 1 {
//                     if *value == 0 {
//                         write!(f, "i1 false")
//                     } else {
//                         write!(f, "i1 true")
//                     }
//                 } else {
//                     match *bits {
//                         16 => {
//                             let signed_val = (*value & 0xFFFF) as i16;
//                             if signed_val > -1000 {
//                                 write!(f, "i{} {}", bits, signed_val)
//                             } else {
//                                 write!(f, "i{} {}", bits, *value)
//                             }
//                         },
//                         32 => {
//                             let signed_val = (*value & 0xFFFF_FFFF) as i32;
//                             if signed_val > -1000 {
//                                 write!(f, "i{} {}", bits, signed_val)
//                             } else {
//                                 write!(f, "i{} {}", bits, *value)
//                             }
//                         },
//                         64 => {
//                             let signed_val = *value as i64;
//                             if signed_val > -1000 {
//                                 write!(f, "i{} {}", bits, signed_val)
//                             } else {
//                                 write!(f, "i{} {}", bits, *value)
//                             }
//                         },
//                         _ => write!(f, "i{} {}", bits, value),
//                     }
//                 }
//             },
//             Constant::Float(float) => write!(f, "{}", float),
//             Constant::Null(ty) => write!(f, "{} null", ty),
//             Constant::AggregateZero(ty) => write!(f, "{} zeroinitializer", ty),
//             Constant::Struct {
//                 values, is_packed, ..
//             } => {
//                 if *is_packed {
//                     write!(f, "<")?;
//                 }
//                 write!(f, "{{ ")?;
//                 for (i, val) in values.iter().enumerate() {
//                     if i == values.len() - 1 {
//                         write!(f, "{}", val)?;
//                     } else {
//                         write!(f, "{}, ", val)?;
//                     }
//                 }
//                 write!(f, " }}")?;
//                 if *is_packed {
//                     write!(f, ">")?;
//                 }
//                 Ok(())
//             },
//             Constant::Array { elements, .. } => {
//                 write!(f, "[ ")?;
//                 for (i, elt) in elements.iter().enumerate() {
//                     if i == elements.len() - 1 {
//                         write!(f, "{}", elt)?;
//                     } else {
//                         write!(f, "{}, ", elt)?;
//                     }
//                 }
//                 write!(f, " ]")?;
//                 Ok(())
//             },
//             Constant::Vector(v) => {
//                 write!(f, "< ")?;
//                 for (i, elt) in v.iter().enumerate() {
//                     if i == v.len() - 1 {
//                         write!(f, "{}", elt)?;
//                     } else {
//                         write!(f, "{}, ", elt)?;
//                     }
//                 }
//                 write!(f, " >")?;
//                 Ok(())
//             },
//             Constant::Undef(ty) => write!(f, "{} undef", ty),
//             Constant::Poison(ty) => write!(f, "{} poison", ty),
//             Constant::BlockAddress => write!(f, "blockaddr"),
//             Constant::GlobalReference { name, ty } => {
//                 let name = match name {
//                     Name::Name(n) => String::clone(n),
//                     Name::Number(n) => n.to_string(),
//                 };
//                 match ty.as_ref() {
//                     LLVMType::FuncType { .. } => {
//                         write!(f, "@{}", name)
//                     },
//                     _ => {
//                         write!(f, "ptr @{}", name)
//                     },
//                 }
//             },
//             Constant::TokenNone => write!(f, "none"),
//             Constant::Add(a) => write!(f, "{}", a),
//             Constant::Sub(s) => write!(f, "{}", s),
//             Constant::Mul(m) => write!(f, "{}", m),
//             Constant::Xor(x) => write!(f, "{}", x),
//             Constant::ExtractElement(e) => write!(f, "{}", e),
//             Constant::InsertElement(i) => write!(f, "{}", i),
//             Constant::ShuffleVector(s) => write!(f, "{}", s),
//             Constant::GetElementPtr(g) => write!(f, "{}", g),
//             Constant::Trunc(t) => write!(f, "{}", t),
//             Constant::PtrToInt(p) => write!(f, "{}", p),
//             Constant::IntToPtr(i) => write!(f, "{}", i),
//             Constant::BitCast(b) => write!(f, "{}", b),
//             Constant::AddrSpaceCast(a) => write!(f, "{}", a),
//             Constant::PtrAuth { ptr, key, disc, addr_disc } => {
//                 write!(f, "ptrauth({}, {}, {}, {})", ptr, key, disc, addr_disc)
//             }
//         }
//     }
// }

#[derive(PartialEq, Clone, Debug, Hash)]
pub struct ConstantRef(Arc<Constant>);

impl AsRef<Constant> for ConstantRef {
    fn as_ref(&self) -> &Constant {
        self.0.as_ref()
    }
}

impl Deref for ConstantRef {
    type Target = Constant;

    fn deref(&self) -> &Constant {
        self.0.deref()
    }
}

impl Typed for ConstantRef {
    fn get_type(&self, types: &Types) -> TypeRef {
        self.as_ref().get_type(types)
    }
}

// impl Display for ConstantRef {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "{}", &self.0)
//     }
// }

impl ConstantRef {
    pub fn new(c: Constant) -> Self {
        Self(Arc::new(c))
    }
}

pub trait ConstUnaryOp {
    fn get_operand(&self) -> ConstantRef;
}

pub trait ConstBinaryOp {
    fn get_operand0(&self) -> ConstantRef;
    fn get_operand1(&self) -> ConstantRef;
}


#[derive(PartialEq, Clone, Debug, Hash)]
pub struct Add {
    pub operand0: ConstantRef,
    pub operand1: ConstantRef,
}

impl From<Add> for Constant {
    fn from(expr: Add) -> Constant {
        Constant::Add(expr)
    }
}

impl TryFrom<Constant> for Add {
    type Error = &'static str;
    fn try_from(constant: Constant) -> Result<Self, Self::Error> {
        match constant {
            Constant::Add(expr) => Ok(expr),
            _ => Err("Constant is not of requested kind"),
        }
    }
}
impl ConstBinaryOp for Add {
    fn get_operand0(&self) -> ConstantRef {
        self.operand0.clone()
    }
    fn get_operand1(&self) -> ConstantRef {
        self.operand1.clone()
    }
}

impl Typed for Add {
    fn get_type(&self, types: &Types) -> TypeRef {
        let t = types.type_of(&self.get_operand0());
        debug_assert_eq!(t, types.type_of(&self.get_operand1()));
        t
    }
}

// impl Display for Add {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "add ({}, {})", &self.operand0, &self.operand1)
//     }
// }

#[derive(PartialEq, Clone, Debug, Hash)]
pub struct Sub {
    pub operand0: ConstantRef,
    pub operand1: ConstantRef,
}

impl From<Sub> for Constant {
    fn from(expr: Sub) -> Constant {
        Constant::Sub(expr)
    }
}

impl TryFrom<Constant> for Sub {
    type Error = &'static str;
    fn try_from(constant: Constant) -> Result<Self, Self::Error> {
        match constant {
            Constant::Sub(expr) => Ok(expr),
            _ => Err("Constant is not of requested kind"),
        }
    }
}
impl ConstBinaryOp for Sub {
    fn get_operand0(&self) -> ConstantRef {
        self.operand0.clone()
    }
    fn get_operand1(&self) -> ConstantRef {
        self.operand1.clone()
    }
}

impl Typed for Sub {
    fn get_type(&self, types: &Types) -> TypeRef {
        let t = types.type_of(&self.get_operand0());
        debug_assert_eq!(t, types.type_of(&self.get_operand1()));
        t
    }
}

// impl Display for Sub {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "sub ({}, {})", &self.operand0, &self.operand1)
//     }
// }

#[derive(PartialEq, Clone, Debug, Hash)]
pub struct Mul {
    pub operand0: ConstantRef,
    pub operand1: ConstantRef,
}

impl From<Mul> for Constant {
    fn from(expr: Mul) -> Constant {
        Constant::Mul(expr)
    }
}

impl TryFrom<Constant> for Mul {
    type Error = &'static str;
    fn try_from(constant: Constant) -> Result<Self, Self::Error> {
        match constant {
            Constant::Mul(expr) => Ok(expr),
            _ => Err("Constant is not of requested kind"),
        }
    }
}
impl ConstBinaryOp for Mul {
    fn get_operand0(&self) -> ConstantRef {
        self.operand0.clone()
    }
    fn get_operand1(&self) -> ConstantRef {
        self.operand1.clone()
    }
}

impl Typed for Mul {
    fn get_type(&self, types: &Types) -> TypeRef {
        let t = types.type_of(&self.get_operand0());
        debug_assert_eq!(t, types.type_of(&self.get_operand1()));
        t
    }
}

// impl Display for Mul {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "mul ({}, {})", &self.operand0, &self.operand1)
//     }
// }


#[derive(PartialEq, Clone, Debug, Hash)]
pub struct Xor {
    pub operand0: ConstantRef,
    pub operand1: ConstantRef,
}

impl From<Xor> for Constant {
    fn from(expr: Xor) -> Constant {
        Constant::Xor(expr)
    }
}

impl TryFrom<Constant> for Xor {
    type Error = &'static str;
    fn try_from(constant: Constant) -> Result<Self, Self::Error> {
        match constant {
            Constant::Xor(expr) => Ok(expr),
            _ => Err("Constant is not of requested kind"),
        }
    }
}
impl ConstBinaryOp for Xor {
    fn get_operand0(&self) -> ConstantRef {
        self.operand0.clone()
    }
    fn get_operand1(&self) -> ConstantRef {
        self.operand1.clone()
    }
}

impl Typed for Xor {
    fn get_type(&self, types: &Types) -> TypeRef {
        let t = types.type_of(&self.get_operand0());
        debug_assert_eq!(t, types.type_of(&self.get_operand1()));
        t
    }
}

// impl Display for Xor {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "xor ({}, {})", &self.operand0, &self.operand1)
//     }
// }


pub struct FRem {
    pub operand0: ConstantRef,
    pub operand1: ConstantRef,
}


#[derive(PartialEq, Clone, Debug, Hash)]
pub struct ExtractElement {
    pub vector: ConstantRef,
    pub index: ConstantRef,
}

impl From<ExtractElement> for Constant {
    fn from(expr: ExtractElement) -> Constant {
        Constant::ExtractElement(expr)
    }
}

impl TryFrom<Constant> for ExtractElement {
    type Error = &'static str;
    fn try_from(constant: Constant) -> Result<Self, Self::Error> {
        match constant {
            Constant::ExtractElement(expr) => Ok(expr),
            _ => Err("Constant is not of requested kind"),
        }
    }
}

impl Typed for ExtractElement {
    fn get_type(&self, types: &Types) -> TypeRef {
        match types.type_of(&self.vector).as_ref() {
            LLVMType::VectorType { element_type, .. } => element_type.clone(),
            ty => panic!(
                "Expected an ExtractElement vector to be VectorType, got {:?}",
                ty
            ),
        }
    }
}

// impl Display for ExtractElement {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "extractelement ({}, {})", &self.vector, &self.index)
//     }
// }

#[derive(PartialEq, Clone, Debug, Hash)]
pub struct InsertElement {
    pub vector: ConstantRef,
    pub element: ConstantRef,
    pub index: ConstantRef,
}

impl From<InsertElement> for Constant {
    fn from(expr: InsertElement) -> Constant {
        Constant::InsertElement(expr)
    }
}

impl TryFrom<Constant> for InsertElement {
    type Error = &'static str;
    fn try_from(constant: Constant) -> Result<Self, Self::Error> {
        match constant {
            Constant::InsertElement(expr) => Ok(expr),
            _ => Err("Constant is not of requested kind"),
        }
    }
}

impl Typed for InsertElement {
    fn get_type(&self, types: &Types) -> TypeRef {
        types.type_of(&self.vector)
    }
}

// impl Display for InsertElement {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(
//             f,
//             "insertelement ({}, {}, {})",
//             &self.vector, &self.element, &self.index,
//         )
//     }
// }

#[derive(PartialEq, Clone, Debug, Hash)]
pub struct ShuffleVector {
    pub operand0: ConstantRef,
    pub operand1: ConstantRef,
    pub mask: ConstantRef,
}

impl From<ShuffleVector> for Constant {
    fn from(expr: ShuffleVector) -> Constant {
        Constant::ShuffleVector(expr)
    }
}

impl TryFrom<Constant> for ShuffleVector {
    type Error = &'static str;
    fn try_from(constant: Constant) -> Result<Self, Self::Error> {
        match constant {
            Constant::ShuffleVector(expr) => Ok(expr),
            _ => Err("Constant is not of requested kind"),
        }
    }
}
impl ConstBinaryOp for ShuffleVector {
    fn get_operand0(&self) -> ConstantRef {
        self.operand0.clone()
    }
    fn get_operand1(&self) -> ConstantRef {
        self.operand1.clone()
    }
}

impl Typed for ShuffleVector {
    fn get_type(&self, types: &Types) -> TypeRef {
        let ty = types.type_of(&self.operand0);
        debug_assert_eq!(ty, types.type_of(&self.operand1));
        match ty.as_ref() {
            LLVMType::VectorType { element_type, .. } => match types.type_of(&self.mask).as_ref() {
                LLVMType::VectorType {
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

// impl Display for ShuffleVector {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(
//             f,
//             "shufflevector ({}, {}, {})",
//             &self.operand0, &self.operand1, &self.mask,
//         )
//     }
// }


#[derive(PartialEq, Clone, Debug, Hash)]
pub struct GetElementPtr {
    pub address: ConstantRef,
    pub indices: Vec<ConstantRef>,
    pub in_bounds: bool,
}

impl From<GetElementPtr> for Constant {
    fn from(expr: GetElementPtr) -> Constant {
        Constant::GetElementPtr(expr)
    }
}

impl TryFrom<Constant> for GetElementPtr {
    type Error = &'static str;
    fn try_from(constant: Constant) -> Result<Self, Self::Error> {
        match constant {
            Constant::GetElementPtr(expr) => Ok(expr),
            _ => Err("Constant is not of requested kind"),
        }
    }
}

impl Typed for GetElementPtr {
    fn get_type(&self, types: &Types) -> TypeRef {
        types.pointer()
    }
}


// impl Display for GetElementPtr {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(
//             f,
//             "getelementptr{} ({}",
//             if self.in_bounds { " inbounds" } else { "" },
//             &self.address
//         )?;
//         for idx in &self.indices {
//             write!(f, ", {}", idx)?;
//         }
//         write!(f, ")")?;
//         Ok(())
//     }
// }

#[derive(PartialEq, Clone, Debug, Hash)]
pub struct Trunc {
    pub operand: ConstantRef,
    pub to_type: TypeRef,
}

impl From<Trunc> for Constant {
    fn from(expr: Trunc) -> Constant {
        Constant::Trunc(expr)
    }
}

impl TryFrom<Constant> for Trunc {
    type Error = &'static str;
    fn try_from(constant: Constant) -> Result<Self, Self::Error> {
        match constant {
            Constant::Trunc(expr) => Ok(expr),
            _ => Err("Constant is not of requested kind"),
        }
    }
}
impl ConstUnaryOp for Trunc {
    fn get_operand(&self) -> ConstantRef {
        self.operand.clone()
    }
}

impl Typed for Trunc {
    fn get_type(&self, _types: &Types) -> TypeRef {
        self.to_type.clone()
    }
}

// impl Display for Trunc {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(
//             f,
//             "trunc ({} to {})",
//             &self.get_operand(),
//             &self.to_type,
//         )
//     }
// }


#[derive(PartialEq, Clone, Debug, Hash)]
pub struct PtrToInt {
    pub operand: ConstantRef,
    pub to_type: TypeRef,
}

impl From<PtrToInt> for Constant {
    fn from(expr: PtrToInt) -> Constant {
        Constant::PtrToInt(expr)
    }
}

impl TryFrom<Constant> for PtrToInt {
    type Error = &'static str;
    fn try_from(constant: Constant) -> Result<Self, Self::Error> {
        match constant {
            Constant::PtrToInt(expr) => Ok(expr),
            _ => Err("Constant is not of requested kind"),
        }
    }
}
impl ConstUnaryOp for PtrToInt {
    fn get_operand(&self) -> ConstantRef {
        self.operand.clone()
    }
}

impl Typed for PtrToInt {
    fn get_type(&self, _types: &Types) -> TypeRef {
        self.to_type.clone()
    }
}

// impl Display for PtrToInt {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(
//             f,
//             "ptrtoint ({} to {})",
//             &self.get_operand(),
//             &self.to_type,
//         )
//     }
// }

#[derive(PartialEq, Clone, Debug, Hash)]
pub struct IntToPtr {
    pub operand: ConstantRef,
    pub to_type: TypeRef,
}

impl From<IntToPtr> for Constant {
    fn from(expr: IntToPtr) -> Constant {
        Constant::IntToPtr(expr)
    }
}

impl TryFrom<Constant> for IntToPtr {
    type Error = &'static str;
    fn try_from(constant: Constant) -> Result<Self, Self::Error> {
        match constant {
            Constant::IntToPtr(expr) => Ok(expr),
            _ => Err("Constant is not of requested kind"),
        }
    }
}
impl ConstUnaryOp for IntToPtr {
    fn get_operand(&self) -> ConstantRef {
        self.operand.clone()
    }
}

impl Typed for IntToPtr {
    fn get_type(&self, _types: &Types) -> TypeRef {
        self.to_type.clone()
    }
}

// impl Display for IntToPtr {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(
//             f,
//             "inttoptr ({} to {})",
//             &self.get_operand(),
//             &self.to_type,
//         )
//     }
// }

#[derive(PartialEq, Clone, Debug, Hash)]
pub struct BitCast {
    pub operand: ConstantRef,
    pub to_type: TypeRef,
}

impl From<BitCast> for Constant {
    fn from(expr: BitCast) -> Constant {
        Constant::BitCast(expr)
    }
}

impl TryFrom<Constant> for BitCast {
    type Error = &'static str;
    fn try_from(constant: Constant) -> Result<Self, Self::Error> {
        match constant {
            Constant::BitCast(expr) => Ok(expr),
            _ => Err("Constant is not of requested kind"),
        }
    }
}
impl ConstUnaryOp for BitCast {
    fn get_operand(&self) -> ConstantRef {
        self.operand.clone()
    }
}

impl Typed for BitCast {
    fn get_type(&self, _types: &Types) -> TypeRef {
        self.to_type.clone()
    }
}

// impl Display for BitCast {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(
//             f,
//             "bitcast ({} to {})",
//             &self.get_operand(),
//             &self.to_type,
//         )
//     }
// }

#[derive(PartialEq, Clone, Debug, Hash)]
pub struct AddrSpaceCast {
    pub operand: ConstantRef,
    pub to_type: TypeRef,
}

impl From<AddrSpaceCast> for Constant {
    fn from(expr: AddrSpaceCast) -> Constant {
        Constant::AddrSpaceCast(expr)
    }
}

impl TryFrom<Constant> for AddrSpaceCast {
    type Error = &'static str;
    fn try_from(constant: Constant) -> Result<Self, Self::Error> {
        match constant {
            Constant::AddrSpaceCast(expr) => Ok(expr),
            _ => Err("Constant is not of requested kind"),
        }
    }
}
impl ConstUnaryOp for AddrSpaceCast {
    fn get_operand(&self) -> ConstantRef {
        self.operand.clone()
    }
}

impl Typed for AddrSpaceCast {
    fn get_type(&self, _types: &Types) -> TypeRef {
        self.to_type.clone()
    }
}

// impl Display for AddrSpaceCast {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(
//             f,
//             "addrspacecast ({} to {})",
//             &self.get_operand(),
//             &self.to_type,
//         )
//     }
// }


