use crate::llvm_ir::function::{
    CallingConvention, Function, FunctionAttribute, FunctionDeclaration,
};
use crate::llvm_ir::instruction::{
    AShr, Add, AddrSpaceCast, Alloca, And, AtomicRMW, BitCast, Call, CatchPad, CleanupPad, CmpXchg,
    ExtractElement, ExtractValue, FAdd, FCmp, FDiv, FMul, FNeg, FPExt, FPToSI, FPToUI, FPTrunc,
    FRem, FSub, Fence, Freeze, GetElementPtr, ICmp, InsertElement, InsertValue, IntToPtr, LShr,
    LandingPad, Load, Mul, Or, Phi, PtrToInt, SDiv, SExt, SIToFP, SRem, Select, Shl, ShuffleVector,
    Store, Sub, Trunc, UDiv, UIToFP, URem, VAArg, Xor, ZExt,
};
use crate::llvm_ir::module::{
    DLLStorageClass, GlobalAlias, GlobalIFunc, GlobalVariable, Linkage, Module, ThreadLocalMode,
    Visibility,
};
use crate::llvm_ir::constant;
use crate::llvm_ir::terminator::{
    Br, CallBr, CatchRet, CatchSwitch, CleanupRet, CondBr, IndirectBr, Invoke, Resume, Ret, Switch,
    Terminator, Unreachable,
};
use crate::llvm_ir::types::{NamedStructDef, Types};
use crate::llvm_ir::{Constant, ConstantRef, Instruction, Name, Operand};
use std::fmt::Write;
pub trait Show {
    fn show(&self, types: &Types) -> String;
}

impl Show for Module {
    fn show(&self, types: &Types) -> String {
        let mut parts: Vec<String> = Vec::new();

        let mut header = format!("source_filename = \"{}\"", self.source_file_name);
        if !self.data_layout.layout_str.is_empty() {
            header.push_str(&format!("\ntarget datalayout = \"{}\"", self.data_layout));
        }
        if let Some(ref triple) = self.target_triple {
            header.push_str(&format!("\ntarget triple = \"{}\"", triple));
        }
        parts.push(header);

        let struct_names: Vec<String> = self.types.all_struct_names().cloned().collect();
        for name in &struct_names {
            if let Some(def) = self.types.named_struct_def(name) {
                parts.push(format!("%{} = {}", name, def.show(types)));
            }
        }

        if !self.inline_assembly.is_empty() {
            parts.push(format!("module asm \"{}\"", self.inline_assembly));
        }

        for gv in &self.global_vars {
            parts.push(gv.show(types));
        }
        for ga in &self.global_aliases {
            parts.push(ga.show(types));
        }
        for gi in &self.global_ifuncs {
            parts.push(gi.show(types));
        }
        for fd in &self.func_declarations {
            parts.push(fd.show(types));
        }
        for func in &self.functions {
            parts.push(func.show(types));
        }

        parts.join("\n\n") + "\n"
    }
}

impl Show for GlobalVariable {
    fn show(&self, _types: &Types) -> String {
        let mut s = String::new();
        write!(s, "@{} = ", self.name).unwrap();
        if self.linkage != Linkage::External {
            write!(s, "{} ", self.linkage).unwrap();
        }
        if self.visibility != Visibility::Default {
            write!(s, "{} ", self.visibility).unwrap();
        }
        if self.dll_storage_class != DLLStorageClass::Default {
            write!(s, "{} ", self.dll_storage_class).unwrap();
        }
        if self.thread_local_mode != ThreadLocalMode::NotThreadLocal {
            write!(s, "{} ", self.thread_local_mode).unwrap();
        }
        if let Some(ref unnamed_addr) = self.unnamed_addr {
            write!(s, "{} ", unnamed_addr).unwrap();
        }
        if self.addr_space != 0 {
            write!(s, "addrspace({}) ", self.addr_space).unwrap();
        }
        write!(
            s,
            "{} ",
            if self.is_constant {
                "constant"
            } else {
                "global"
            }
        )
        .unwrap();
        write!(s, "{}", self.ty).unwrap();
        if let Some(ref init) = self.initializer {
            write!(s, " {}", init).unwrap();
        }
        if let Some(ref section) = self.section {
            write!(s, ", section \"{}\"", section).unwrap();
        }
        if let Some(ref comdat) = self.comdat {
            write!(s, ", {}", comdat).unwrap();
        }
        if self.alignment > 0 {
            write!(s, ", align {}", self.alignment).unwrap();
        }
        s
    }
}

impl Show for GlobalAlias {
    fn show(&self, _types: &Types) -> String {
        let mut s = String::new();
        write!(s, "@{} = ", self.name).unwrap();
        if self.linkage != Linkage::External {
            write!(s, "{} ", self.linkage).unwrap();
        }
        if self.visibility != Visibility::Default {
            write!(s, "{} ", self.visibility).unwrap();
        }
        if self.dll_storage_class != DLLStorageClass::Default {
            write!(s, "{} ", self.dll_storage_class).unwrap();
        }
        if self.thread_local_mode != ThreadLocalMode::NotThreadLocal {
            write!(s, "{} ", self.thread_local_mode).unwrap();
        }
        if let Some(ref unnamed_addr) = self.unnamed_addr {
            write!(s, "{} ", unnamed_addr).unwrap();
        }
        write!(s, "alias {}", self.ty).unwrap();
        if self.addr_space != 0 {
            write!(s, ", addrspace({})", self.addr_space).unwrap();
        }
        write!(s, ", {}", self.aliasee).unwrap();
        s
    }
}

impl Show for GlobalIFunc {
    fn show(&self, _types: &Types) -> String {
        let mut s = String::new();
        write!(s, "@{} = ", self.name).unwrap();
        if self.linkage != Linkage::External {
            write!(s, "{} ", self.linkage).unwrap();
        }
        if self.visibility != Visibility::Default {
            write!(s, "{} ", self.visibility).unwrap();
        }
        write!(s, "ifunc {}, {}", self.ty, self.resolver_fn).unwrap();

        s
    }
}

impl Show for Function {
    fn show(&self, types: &Types) -> String {
        let mut s = String::new();
        for attr in &self.function_attributes {
            match attr {
                FunctionAttribute::UnknownAttribute => {}
                _ => write!(s, "{} ", attr).unwrap(),
            }
        }
        write!(s, "define ").unwrap();
        if self.linkage != Linkage::External {
            write!(s, "{} ", self.linkage).unwrap();
        }
        if self.visibility != Visibility::Default {
            write!(s, "{} ", self.visibility).unwrap();
        }
        if self.dll_storage_class != DLLStorageClass::Default {
            write!(s, "{} ", self.dll_storage_class).unwrap();
        }
        if self.calling_convention != CallingConvention::C {
            write!(s, "{} ", self.calling_convention).unwrap();
        }
        write!(s, "{} @{}(", self.return_type, self.name).unwrap();
        for (i, param) in self.parameters.iter().enumerate() {
            if i > 0 {
                write!(s, ", ").unwrap();
            }
            write!(s, "{}", param).unwrap();
        }
        if self.is_var_arg {
            if !self.parameters.is_empty() {
                write!(s, ", ").unwrap();
            }
            write!(s, "...").unwrap();
        }
        write!(s, ")").unwrap();
        if let Some(ref gc) = self.garbage_collector_name {
            write!(s, " gc \"{}\"", gc).unwrap();
        }
        if let Some(ref pers) = self.personality_function {
            write!(s, " personality {}", pers).unwrap();
        }
        writeln!(s, " {{").unwrap();
        for bb in &self.basic_blocks {
            match &bb.name {
                Name::Name(name) => write!(s, "{}:\n", name).unwrap(),
                Name::Number(num) => write!(s, "{}:\n", num).unwrap(),
            }
            for instr in &bb.instrs {
                writeln!(s, "  {}", instr.show(types)).unwrap();
            }
            // writeln!(s, "  {}", bb.term.show(types)).unwrap();
            writeln!(s, "  {}", bb.term).unwrap(); // todo
        }
        write!(s, "}}").unwrap();

        s
    }
}

impl Show for Operand {
    fn show(&self, types: &Types) -> String {
        let mut s = String::new();
        match self {
            Operand::LocalOperand { name, ty: _ } => write!(s, "{}", name).unwrap(),
            Operand::ConstantOperand(cref) => write!(s, "{}", cref.show(types)).unwrap(),
            Operand::MetadataOperand => write!(s, "<metadata>").unwrap(),
        }
        s
    }
}
impl Show for Add {
    fn show(&self, types: &Types) -> String {
        let mut s = String::new();
        let ty = types.type_of(&self.operand0);
        write!(s, "{} = add", &self.dest).unwrap();
        if self.nuw {
            write!(s, " nuw").unwrap();
        }
        if self.nsw {
            write!(s, " nsw").unwrap();
        }
        write!(s, " {} {}, {}", ty, &self.operand0.show(types), &self.operand1.show(types)).unwrap();
        s
    }
}
impl Show for Sub {
    fn show(&self, types: &Types) -> String {
        let mut s = String::new();
        let ty = types.type_of(&self.operand0);
        write!(s, "{} = sub", &self.dest).unwrap();
        if self.nuw {
            write!(s, " nuw").unwrap();
        }
        if self.nsw {
            write!(s, " nsw").unwrap();
        }
        write!(s, " {} {}, {}", ty, &self.operand0.show(types), &self.operand1.show(types)).unwrap();
        s
    }
}
impl Show for Mul {
    fn show(&self, types: &Types) -> String {
        let mut s = String::new();
        let ty = types.type_of(&self.operand0);
        write!(s, "{} = mul", &self.dest).unwrap();
        if self.nuw {
            write!(s, " nuw").unwrap();
        }
        if self.nsw {
            write!(s, " nsw").unwrap();
        }
        write!(s, " {} {}, {}", ty, &self.operand0.show(types), &self.operand1.show(types)).unwrap();
        s
    }
}
impl Show for UDiv {
    fn show(&self, types: &Types) -> String {
        let mut s = String::new();
        let ty = types.type_of(&self.operand0);
        write!(s, "{} = udiv", &self.dest).unwrap();
        if self.exact {
            write!(s, " exact").unwrap();
        }
        write!(s, " {} {}, {}", ty, &self.operand0.show(types), &self.operand1.show(types)).unwrap();
        s
    }
}
impl Show for SDiv {
    fn show(&self, types: &Types) -> String {
        let mut s = String::new();
        let ty = types.type_of(&self.operand0);
        write!(s, "{} = sdiv", &self.dest).unwrap();
        if self.exact {
            write!(s, " exact").unwrap();
        }
        write!(s, " {} {}, {}", ty, &self.operand0.show(types), &self.operand1.show(types)).unwrap();
        s
    }
}
impl Show for URem {
    fn show(&self, types: &Types) -> String {
        let mut s = String::new();
        let ty = types.type_of(&self.operand0);
        write!(
            s,
            "{} = urem {} {}, {}",
            &self.dest, ty, &self.operand0.show(types), &self.operand1.show(types)
        )
        .unwrap();
        s
    }
}
impl Show for SRem {
    fn show(&self, types: &Types) -> String {
        let mut s = String::new();
        let ty = types.type_of(&self.operand0);
        write!(
            s,
            "{} = srem {} {}, {}",
            &self.dest, ty, &self.operand0.show(types), &self.operand1.show(types)
        )
        .unwrap();
        s
    }
}
impl Show for And {
    fn show(&self, types: &Types) -> String {
        let mut s = String::new();
        let ty = types.type_of(&self.operand0);
        write!(
            s,
            "{} = and {} {}, {}",
            &self.dest, ty, &self.operand0.show(types), &self.operand1.show(types)
        )
        .unwrap();
        s
    }
}
impl Show for Or {
    fn show(&self, types: &Types) -> String {
        let mut s = String::new();
        let ty = types.type_of(&self.operand0);
        write!(s, "{} = or", &self.dest).unwrap();
        if self.disjoint {
            write!(s, " disjoint").unwrap();
        }
        write!(s, " {} {}, {}", ty, &self.operand0.show(types), &self.operand1.show(types)).unwrap();
        s
    }
}
impl Show for Xor {
    fn show(&self, types: &Types) -> String {
        let mut s = String::new();
        let ty = types.type_of(&self.operand0);
        write!(
            s,
            "{} = xor {} {}, {}",
            &self.dest, ty, &self.operand0.show(types), &self.operand1.show(types)
        )
        .unwrap();
        s
    }
}
impl Show for Shl {
    fn show(&self, types: &Types) -> String {
        let mut s = String::new();
        let ty = types.type_of(&self.operand0);
        write!(s, "{} = shl", &self.dest).unwrap();
        if self.nuw {
            write!(s, " nuw").unwrap();
        }
        if self.nsw {
            write!(s, " nsw").unwrap();
        }
        write!(s, " {} {}, {}", ty, &self.operand0.show(types), &self.operand1.show(types)).unwrap();
        s
    }
}
impl Show for LShr {
    fn show(&self, types: &Types) -> String {
        let mut s = String::new();
        let ty = types.type_of(&self.operand0);
        write!(s, "{} = lshr", &self.dest).unwrap();
        if self.exact {
            write!(s, " exact").unwrap();
        }
        write!(s, " {} {}, {}", ty, &self.operand0.show(types), &self.operand1.show(types)).unwrap();
        s
    }
}
impl Show for AShr {
    fn show(&self, types: &Types) -> String {
        let mut s = String::new();
        let ty = types.type_of(&self.operand0);
        write!(s, "{} = ashr", &self.dest).unwrap();
        if self.exact {
            write!(s, " exact").unwrap();
        }
        write!(s, " {} {}, {}", ty, &self.operand0.show(types), &self.operand1.show(types)).unwrap();
        s
    }
}
impl Show for FAdd {
    fn show(&self, types: &Types) -> String {
        let mut s = String::new();
        let ty = types.type_of(&self.operand0);
        write!(
            s,
            "{} = fadd {} {}, {}",
            &self.dest, ty, &self.operand0.show(types), &self.operand1.show(types)
        )
        .unwrap();
        s
    }
}
impl Show for FSub {
    fn show(&self, types: &Types) -> String {
        let mut s = String::new();
        let ty = types.type_of(&self.operand0);
        write!(
            s,
            "{} = fsub {} {}, {}",
            &self.dest, ty, &self.operand0.show(types), &self.operand1.show(types)
        )
        .unwrap();
        s
    }
}
impl Show for FMul {
    fn show(&self, types: &Types) -> String {
        let mut s = String::new();
        let ty = types.type_of(&self.operand0);
        write!(
            s,
            "{} = fmul {} {}, {}",
            &self.dest, ty, &self.operand0.show(types), &self.operand1.show(types)
        )
        .unwrap();
        s
    }
}
impl Show for FDiv {
    fn show(&self, types: &Types) -> String {
        let mut s = String::new();
        let ty = types.type_of(&self.operand0);
        write!(
            s,
            "{} = fdiv {} {}, {}",
            &self.dest, ty, &self.operand0.show(types), &self.operand1.show(types)
        )
        .unwrap();
        s
    }
}
impl Show for FRem {
    fn show(&self, types: &Types) -> String {
        let mut s = String::new();
        let ty = types.type_of(&self.operand0);
        write!(
            s,
            "{} = frem {} {}, {}",
            &self.dest, ty, &self.operand0.show(types), &self.operand1.show(types)
        )
        .unwrap();
        s
    }
}
impl Show for FNeg {
    fn show(&self, types: &Types) -> String {
        let mut s = String::new();
        let ty = types.type_of(&self.operand);
        write!(s, "{} = fneg {} {}", &self.dest, ty, &self.operand.show(types)).unwrap();
        s
    }
}
impl Show for ExtractElement {
    fn show(&self, types: &Types) -> String {
        let mut s = String::new();
        let vec_ty = types.type_of(&self.vector);
        write!(s, "{} = extractelement {} {}, {}", &self.dest, vec_ty, &self.vector.show(types), &self.index.show(types)).unwrap();
        s
    }
}
impl Show for InsertElement {
    fn show(&self, types: &Types) -> String {
        let mut s = String::new();
        let vec_ty = types.type_of(&self.vector);
        write!(s, "{} = insertelement {} {}, {}, {}", &self.dest, vec_ty, &self.vector.show(types), &self.element.show(types), &self.index.show(types)).unwrap();
        s
    }
}
impl Show for ShuffleVector {
    fn show(&self, types: &Types) -> String {
        let mut s = String::new();
        let vec_ty = types.type_of(&self.operand0);
        write!(s, "{} = shufflevector {} {}, {}, {}", &self.dest, vec_ty, &self.operand0.show(types), &self.operand1.show(types), &self.mask.show(types)).unwrap();
        s
    }
}
impl Show for ExtractValue {
    fn show(&self, types: &Types) -> String {
        let mut s = String::new();
        let agg_ty = types.type_of(&self.aggregate);
        write!(s, "{} = extractvalue {} {}, {}", &self.dest, agg_ty, &self.aggregate.show(types), self.indices.first().expect("ExtractValue with no indices")).unwrap();
        for idx in &self.indices[1..] { write!(s, ", {idx}").unwrap(); }
        s
    }
}
impl Show for InsertValue {
    fn show(&self, types: &Types) -> String {
        let mut s = String::new();
        let agg_ty = types.type_of(&self.aggregate);
        write!(s, "{} = insertvalue {} {}, {}, {}", &self.dest, agg_ty, &self.aggregate.show(types), &self.element.show(types), self.indices.first().expect("InsertValue with no indices")).unwrap();
        for idx in &self.indices[1..] { write!(s, ", {idx}").unwrap(); }
        s
    }
}
impl Show for Alloca {
    fn show(&self, _types: &Types) -> String {
        let mut s = String::new();
        write!(s, "{} = alloca {}", &self.dest, &self.allocated_type).unwrap();
        if let Some(Constant::Int { value: 1, .. }) = self.num_elements.as_constant() {
        } else {
            write!(s, ", {}", &self.num_elements).unwrap();
        }
        write!(s, ", align {}", &self.alignment).unwrap();
        s
    }
}
impl Show for Load {
    fn show(&self, _types: &Types) -> String {
        let mut s = String::new();
        write!(s, "{} = load ", &self.dest).unwrap();
        if self.atomicity.is_some() {
            write!(s, "atomic ").unwrap();
        }
        if self.volatile {
            write!(s, "volatile ").unwrap();
        }
        write!(s, "{}, {}", &self.loaded_ty, &self.address).unwrap();
        if let Some(a) = &self.atomicity {
            write!(s, " {}", a).unwrap();
        }
        write!(s, ", align {}", &self.alignment).unwrap();
        s
    }
}
impl Show for Store {
    fn show(&self, _types: &Types) -> String {
        let mut s = String::new();
        write!(s, "store ").unwrap();
        if self.atomicity.is_some() {
            write!(s, "atomic ").unwrap();
        }
        if self.volatile {
            write!(s, "volatile ").unwrap();
        }
        write!(s, "{}, {}", &self.value, &self.address).unwrap();
        if let Some(a) = &self.atomicity {
            write!(s, " {}", a).unwrap();
        }
        write!(s, ", align {}", &self.alignment).unwrap();
        s
    }
}
impl Show for Fence {
    fn show(&self, _types: &Types) -> String {
        let mut s = String::new();
        write!(s, "fence {}", &self.atomicity).unwrap();
        s
    }
}
impl Show for CmpXchg {
    fn show(&self, _types: &Types) -> String {
        let mut s = String::new();
        write!(s, "{} = cmpxchg ", &self.dest).unwrap();
        if self.weak {
            write!(s, "weak ").unwrap();
        }
        if self.volatile {
            write!(s, "volatile ").unwrap();
        }
        write!(
            s,
            "{}, {}, {} {} {}",
            &self.address,
            &self.expected,
            &self.replacement,
            &self.atomicity,
            &self.failure_memory_ordering
        )
        .unwrap();
        s
    }
}
impl Show for AtomicRMW {
    fn show(&self, _types: &Types) -> String {
        let mut s = String::new();
        write!(s, "{} = atomicrmw ", &self.dest).unwrap();
        if self.volatile {
            write!(s, "volatile ").unwrap();
        }
        write!(
            s,
            "{} {}, {} {}",
            &self.operation, &self.address, &self.value, &self.atomicity
        )
        .unwrap();
        s
    }
}
impl Show for GetElementPtr {
    fn show(&self, _types: &Types) -> String {
        let mut s = String::new();
        write!(s, "{} = getelementptr ", &self.dest).unwrap();
        if self.in_bounds {
            write!(s, "inbounds ").unwrap();
        }
        write!(s, "{}, {}", &self.source_element_type, &self.address).unwrap();
        for idx in &self.indices {
            write!(s, ", {}", idx).unwrap();
        }
        s
    }
}
impl Show for Trunc {
    fn show(&self, types: &Types) -> String {
        let mut s = String::new();
        let from_ty = types.type_of(&self.operand);
        write!(
            s,
            "{} = trunc {} {} to {}",
            &self.dest, from_ty, &self.operand.show(types), &self.to_type
        )
        .unwrap();
        s
    }
}
impl Show for ZExt {
    fn show(&self, types: &Types) -> String {
        let mut s = String::new();
        let from_ty = types.type_of(&self.operand);
        write!(s, "{} = zext", &self.dest).unwrap();
        if self.nneg {
            write!(s, " nneg").unwrap();
        }
        write!(s, " {} {} to {}", from_ty, &self.operand.show(types), &self.to_type).unwrap();
        s
    }
}
impl Show for SExt {
    fn show(&self, types: &Types) -> String {
        let mut s = String::new();
        let from_ty = types.type_of(&self.operand);
        write!(
            s,
            "{} = sext {} {} to {}",
            &self.dest, from_ty, &self.operand.show(types), &self.to_type
        )
        .unwrap();
        s
    }
}
impl Show for FPTrunc {
    fn show(&self, types: &Types) -> String {
        let mut s = String::new();
        let from_ty = types.type_of(&self.operand);
        write!(
            s,
            "{} = fptrunc {} {} to {}",
            &self.dest, from_ty, &self.operand.show(types), &self.to_type
        )
        .unwrap();
        s
    }
}
impl Show for FPExt {
    fn show(&self, types: &Types) -> String {
        let mut s = String::new();
        let from_ty = types.type_of(&self.operand);
        write!(
            s,
            "{} = fpext {} {} to {}",
            &self.dest, from_ty, &self.operand.show(types), &self.to_type
        )
        .unwrap();
        s
    }
}
impl Show for FPToUI {
    fn show(&self, types: &Types) -> String {
        let mut s = String::new();
        let from_ty = types.type_of(&self.operand);
        write!(
            s,
            "{} = fptoui {} {} to {}",
            &self.dest, from_ty, &self.operand.show(types), &self.to_type
        )
        .unwrap();
        s
    }
}
impl Show for FPToSI {
    fn show(&self, types: &Types) -> String {
        let mut s = String::new();
        let from_ty = types.type_of(&self.operand);
        write!(
            s,
            "{} = fptosi {} {} to {}",
            &self.dest, from_ty, &self.operand.show(types), &self.to_type
        )
        .unwrap();
        s
    }
}
impl Show for UIToFP {
    fn show(&self, types: &Types) -> String {
        let mut s = String::new();
        let from_ty = types.type_of(&self.operand);
        write!(
            s,
            "{} = uitofp {} {} to {}",
            &self.dest, from_ty, &self.operand.show(types), &self.to_type
        )
        .unwrap();
        s
    }
}
impl Show for SIToFP {
    fn show(&self, types: &Types) -> String {
        let mut s = String::new();
        let from_ty = types.type_of(&self.operand);
        write!(
            s,
            "{} = sitofp {} {} to {}",
            &self.dest, from_ty, &self.operand.show(types), &self.to_type
        )
        .unwrap();
        s
    }
}
impl Show for PtrToInt {
    fn show(&self, types: &Types) -> String {
        let mut s = String::new();
        let from_ty = types.type_of(&self.operand);
        write!(
            s,
            "{} = ptrtoint {} {} to {}",
            &self.dest, from_ty, &self.operand.show(types), &self.to_type
        )
        .unwrap();
        s
    }
}
impl Show for IntToPtr {
    fn show(&self, types: &Types) -> String {
        let mut s = String::new();
        let from_ty = types.type_of(&self.operand);
        write!(
            s,
            "{} = inttoptr {} {} to {}",
            &self.dest, from_ty, &self.operand.show(types), &self.to_type
        )
        .unwrap();
        s
    }
}
impl Show for BitCast {
    fn show(&self, types: &Types) -> String {
        let mut s = String::new();
        let from_ty = types.type_of(&self.operand);
        write!(
            s,
            "{} = bitcast {} {} to {}",
            &self.dest, from_ty, &self.operand.show(types), &self.to_type
        )
        .unwrap();
        s
    }
}
impl Show for AddrSpaceCast {
    fn show(&self, types: &Types) -> String {
        let mut s = String::new();
        let from_ty = types.type_of(&self.operand);
        write!(
            s,
            "{} = addrspacecast {} {} to {}",
            &self.dest, from_ty, &self.operand.show(types), &self.to_type
        )
        .unwrap();
        s
    }
}
impl Show for ICmp {
    fn show(&self, types: &Types) -> String {
        let mut s = String::new();
        let ty = types.type_of(&self.operand0);
        write!(
            s,
            "{} = icmp {} {} {}, {}",
            &self.dest, &self.predicate, ty, &self.operand0.show(types), &self.operand1.show(types)
        )
        .unwrap();
        s
    }
}
impl Show for FCmp {
    fn show(&self, types: &Types) -> String {
        let mut s = String::new();
        let ty = types.type_of(&self.operand0);
        write!(
            s,
            "{} = fcmp {} {} {}, {}",
            &self.dest, &self.predicate, ty, &self.operand0.show(types), &self.operand1.show(types)
        )
        .unwrap();
        s
    }
}
impl Show for Phi {
    fn show(&self, _types: &Types) -> String {
        let mut s = String::new();
        let (first_val, first_label) = &self
            .incoming_values
            .get(0)
            .expect("Phi with no incoming values");
        write!(
            s,
            "{} = phi {} [ {}, {} ]",
            &self.dest, &self.to_type, first_val, first_label
        )
        .unwrap();
        for (val, label) in &self.incoming_values[1..] {
            write!(s, ", [ {}, {} ]", val, label).unwrap();
        }
        s
    }
}
impl Show for Select {
    fn show(&self, types: &Types) -> String {
        let mut s = String::new();
        let ty = types.type_of(&self.true_value);
        write!(s, "{} = select {} {}, {} {}, {} {}", &self.dest, ty, &self.condition.show(types), ty, &self.true_value.show(types), ty, &self.false_value.show(types)).unwrap();
        s
    }
}
impl Show for Freeze {
    fn show(&self, types: &Types) -> String {
        let mut s = String::new();
        let ty = types.type_of(&self.operand);
        write!(s, "{} = freeze {} {}", &self.dest, ty, &self.operand.show(types)).unwrap();
        s
    }
}
impl Show for Call {
    fn show(&self, types: &Types) -> String {
        let mut s = String::new();
        if let Some(dest) = &self.dest {
            write!(s, "{} = ", dest).unwrap();
        }
        if self.is_tail_call {
            write!(s, "tail ").unwrap();
        }
        write!(
            s,
            "call {}(",
            match &self.function {
                either::Either::Left(_) => "<inline assembly>".into(),
                either::Either::Right(op) => format!("{} {}", types.type_of(op), op),
            }
        )
        .unwrap();
        for (i, (arg, _)) in self.arguments.iter().enumerate() {
            if i == self.arguments.len() - 1 {
                write!(s, "{}", arg).unwrap();
            } else {
                write!(s, "{}, ", arg).unwrap();
            }
        }
        write!(s, ")").unwrap();
        s
    }
}
impl Show for VAArg {
    fn show(&self, _types: &Types) -> String {
        let mut s = String::new();
        write!(
            s,
            "{} = va_arg {}, {}",
            &self.dest, &self.arg_list, &self.cur_type
        )
        .unwrap();
        s
    }
}
impl Show for LandingPad {
    fn show(&self, _types: &Types) -> String {
        let mut s = String::new();
        write!(s, "{} = landingpad {}", &self.dest, &self.result_type).unwrap();
        if self.cleanup {
            write!(s, " cleanup").unwrap();
        }
        s
    }
}
impl Show for CatchPad {
    fn show(&self, _types: &Types) -> String {
        let mut s = String::new();
        write!(
            s,
            "{} = catchpad within {} [",
            &self.dest, &self.catch_switch
        )
        .unwrap();
        for (i, arg) in self.args.iter().enumerate() {
            if i == self.args.len() - 1 {
                write!(s, "{}", arg).unwrap();
            } else {
                write!(s, "{}, ", arg).unwrap();
            }
        }
        write!(s, "]").unwrap();
        s
    }
}
impl Show for CleanupPad {
    fn show(&self, _types: &Types) -> String {
        let mut s = String::new();
        write!(
            s,
            "{} = cleanuppad within {} [",
            &self.dest, &self.parent_pad
        )
        .unwrap();
        for (i, arg) in self.args.iter().enumerate() {
            if i == self.args.len() - 1 {
                write!(s, "{}", arg).unwrap();
            } else {
                write!(s, "{}, ", arg).unwrap();
            }
        }
        write!(s, "]").unwrap();
        s
    }
}
impl Show for Instruction {
    fn show(&self, types: &Types) -> String {
        match self {
            Instruction::Add(i) => i.show(types),
            Instruction::Sub(i) => i.show(types),
            Instruction::Mul(i) => i.show(types),
            Instruction::UDiv(i) => i.show(types),
            Instruction::SDiv(i) => i.show(types),
            Instruction::URem(i) => i.show(types),
            Instruction::SRem(i) => i.show(types),
            Instruction::And(i) => i.show(types),
            Instruction::Or(i) => i.show(types),
            Instruction::Xor(i) => i.show(types),
            Instruction::Shl(i) => i.show(types),
            Instruction::LShr(i) => i.show(types),
            Instruction::AShr(i) => i.show(types),
            Instruction::FAdd(i) => i.show(types),
            Instruction::FSub(i) => i.show(types),
            Instruction::FMul(i) => i.show(types),
            Instruction::FDiv(i) => i.show(types),
            Instruction::FRem(i) => i.show(types),
            Instruction::FNeg(i) => i.show(types),
            Instruction::ExtractElement(i) => i.show(types),
            Instruction::InsertElement(i) => i.show(types),
            Instruction::ShuffleVector(i) => i.show(types),
            Instruction::ExtractValue(i) => i.show(types),
            Instruction::InsertValue(i) => i.show(types),
            Instruction::Alloca(i) => i.show(types),
            Instruction::Load(i) => i.show(types),
            Instruction::Store(i) => i.show(types),
            Instruction::Fence(i) => i.show(types),
            Instruction::CmpXchg(i) => i.show(types),
            Instruction::AtomicRMW(i) => i.show(types),
            Instruction::GetElementPtr(i) => i.show(types),
            Instruction::Trunc(i) => i.show(types),
            Instruction::ZExt(i) => i.show(types),
            Instruction::SExt(i) => i.show(types),
            Instruction::FPTrunc(i) => i.show(types),
            Instruction::FPExt(i) => i.show(types),
            Instruction::FPToUI(i) => i.show(types),
            Instruction::FPToSI(i) => i.show(types),
            Instruction::UIToFP(i) => i.show(types),
            Instruction::SIToFP(i) => i.show(types),
            Instruction::PtrToInt(i) => i.show(types),
            Instruction::IntToPtr(i) => i.show(types),
            Instruction::BitCast(i) => i.show(types),
            Instruction::AddrSpaceCast(i) => i.show(types),
            Instruction::ICmp(i) => i.show(types),
            Instruction::FCmp(i) => i.show(types),
            Instruction::Phi(i) => i.show(types),
            Instruction::Select(i) => i.show(types),
            Instruction::Freeze(i) => i.show(types),
            Instruction::Call(i) => i.show(types),
            Instruction::VAArg(i) => i.show(types),
            Instruction::LandingPad(i) => i.show(types),
            Instruction::CatchPad(i) => i.show(types),
            Instruction::CleanupPad(i) => i.show(types),
        }
    }
}
impl Show for FunctionDeclaration {
    fn show(&self, _types: &Types) -> String {
        let mut s = String::new();
        write!(s, "declare ").unwrap();
        if self.return_attributes.is_empty() {
            write!(s, "{}", self.return_type).unwrap();
        } else {
            write!(s, "{}", self.return_type).unwrap();
            for attr in &self.return_attributes {
                write!(s, " {}", attr).unwrap();
            }
        }
        write!(s, " @{}(", self.name).unwrap();
        for (i, param) in self.parameters.iter().enumerate() {
            if i > 0 {
                write!(s, ", ").unwrap();
            }
            write!(s, "{}", param).unwrap();
        }
        if self.is_var_arg {
            if !self.parameters.is_empty() {
                write!(s, ", ").unwrap();
            }
            write!(s, "...").unwrap();
        }
        writeln!(s, ")").unwrap();
        s
    }
}

impl Show for NamedStructDef {
    fn show(&self, _types: &Types) -> String {
        let mut s = String::new();
        match self {
            NamedStructDef::Opaque => write!(s, "type opaque").unwrap(),
            NamedStructDef::Defined(ty) => write!(s, "type {}", ty).unwrap(),
        };
        s
    }
}

impl Show for constant::Add {
    fn show(&self, types: &Types) -> String {
        format!("{} {}", types.type_of(self), self)
    }
}
impl Show for constant::Sub {
    fn show(&self, types: &Types) -> String {
        format!("{} {}", types.type_of(self), self)
    }
}
impl Show for constant::Mul {
    fn show(&self, types: &Types) -> String {
        format!("{} {}", types.type_of(self), self)
    }
}
impl Show for constant::Xor {
    fn show(&self, types: &Types) -> String {
        format!("{} {}", types.type_of(self), self)
    }
}
impl Show for constant::FRem {
    fn show(&self, types: &Types) -> String {
        format!(
            "{} frem ({}, {})",
            types.type_of(&self.operand0),
            &self.operand0,
            &self.operand1
        )
    }
}
impl Show for constant::ExtractElement {
    fn show(&self, types: &Types) -> String {
        format!("{} {}", types.type_of(self), self)
    }
}
impl Show for constant::InsertElement {
    fn show(&self, types: &Types) -> String {
        format!("{} {}", types.type_of(self), self)
    }
}
impl Show for constant::ShuffleVector {
    fn show(&self, types: &Types) -> String {
        format!("{} {}", types.type_of(self), self)
    }
}
impl Show for constant::GetElementPtr {
    fn show(&self, types: &Types) -> String {
        format!("{} {}", types.type_of(self), self)
    }
}
impl Show for constant::Trunc {
    fn show(&self, types: &Types) -> String {
        format!("{} {}", types.type_of(self), self)
    }
}
impl Show for constant::PtrToInt {
    fn show(&self, types: &Types) -> String {
        format!("{} {}", types.type_of(self), self)
    }
}
impl Show for constant::IntToPtr {
    fn show(&self, types: &Types) -> String {
        format!("{} {}", types.type_of(self), self)
    }
}
impl Show for constant::BitCast {
    fn show(&self, types: &Types) -> String {
        format!("{} {}", types.type_of(self), self)
    }
}
impl Show for constant::AddrSpaceCast {
    fn show(&self, types: &Types) -> String {
        format!("{} {}", types.type_of(self), self)
    }
}
impl Show for Constant {
    fn show(&self, _types: &Types) -> String {
        let mut s = String::new();
        match self {
            Constant::Int { bits, value } => {
                if *bits == 1 {
                    if *value == 0 { write!(s, "false").unwrap() }
                    else { write!(s, "true").unwrap() }
                } else {
                    match *bits {
                        16 => write!(s, "{}", (*value & 0xFFFF) as i16).unwrap(),
                        32 => write!(s, "{}", (*value & 0xFFFF_FFFF) as i32).unwrap(),
                        64 => write!(s, "{}", *value as i64).unwrap(),
                        _ => write!(s, "{}", value).unwrap(),
                    }
                }
            }
            Constant::Float(f) => write!(s, "{}", f).unwrap(),
            Constant::Null(_) => write!(s, "null").unwrap(),
            Constant::AggregateZero(_) => write!(s, "zeroinitializer").unwrap(),
            Constant::Undef(_) => write!(s, "undef").unwrap(),
            Constant::Poison(_) => write!(s, "poison").unwrap(),
            Constant::BlockAddress => write!(s, "blockaddr").unwrap(),
            Constant::GlobalReference { name, ty: _ } => {
                match name {
                    Name::Name(n) => write!(s, "@{}", n).unwrap(),
                    Name::Number(n) => write!(s, "@{}", n).unwrap(),
                }
            },
            Constant::TokenNone => write!(s, "none").unwrap(),
            _ => write!(s, "{}", self).unwrap(),
        }
        s
    }
}
impl Show for ConstantRef {
    fn show(&self, types: &Types) -> String {
        self.as_ref().show(types)
    }
}
impl Show for Ret {
    fn show(&self, _types: &Types) -> String {
        let mut s = String::new();
        write!(s, "ret ").unwrap();
        match &self.return_operand {
            None => write!(s, "void").unwrap(),
            Some(op) => write!(s, "{}", op).unwrap(),
        }
        s
    }
}
impl Show for Br {
    fn show(&self, _types: &Types) -> String {
        format!("br label {}", &self.dest)
    }
}
impl Show for CondBr {
    fn show(&self, types: &Types) -> String {
        let mut s = String::new();
        write!(s, "br {}, label {}, label {}", &self.condition.show(types), &self.true_dest, &self.false_dest).unwrap();
        s
    }
}
impl Show for Switch {
    fn show(&self, _types: &Types) -> String {
        let mut s = String::new();
        write!(s, "switch {}, label {} [ ", &self.operand, &self.default_dest).unwrap();
        for (val, label) in &self.dests {
            write!(s, "{}, label {}; ", val, label).unwrap();
        }
        write!(s, "]").unwrap();
        s
    }
}
impl Show for IndirectBr {
    fn show(&self, _types: &Types) -> String {
        let mut s = String::new();
        write!(s, "indirectbr {}, [ label {}", &self.operand, self.possible_dests.get(0).expect("IndirectBr with no possible dests")).unwrap();
        for dest in &self.possible_dests[1..] { write!(s, ", label {}", dest).unwrap(); }
        write!(s, " ]").unwrap();
        s
    }
}
impl Show for Invoke {
    fn show(&self, _types: &Types) -> String {
        let mut s = String::new();
        write!(s, "{} = invoke {}(", &self.result, match &self.function {
            either::Either::Left(_) => "<inline assembly>".into(),
            either::Either::Right(op) => format!("{}", op),
        }).unwrap();
        for (i, (arg, _)) in self.arguments.iter().enumerate() {
            if i == self.arguments.len() - 1 { write!(s, "{}", arg).unwrap(); }
            else { write!(s, "{}, ", arg).unwrap(); }
        }
        write!(s, ") to label {} unwind label {}", &self.return_label, &self.exception_label).unwrap();
        s
    }
}
impl Show for Resume {
    fn show(&self, _types: &Types) -> String {
        format!("resume {}", &self.operand)
    }
}
impl Show for Unreachable {
    fn show(&self, _types: &Types) -> String {
        "unreachable".to_string()
    }
}
impl Show for CleanupRet {
    fn show(&self, _types: &Types) -> String {
        let mut s = String::new();
        write!(s, "cleanupret from {} unwind ", &self.cleanup_pad).unwrap();
        match &self.unwind_dest {
            None => write!(s, "to caller").unwrap(),
            Some(dest) => write!(s, "label {}", dest).unwrap(),
        }
        s
    }
}
impl Show for CatchRet {
    fn show(&self, _types: &Types) -> String {
        format!("catchret from {} to label {}", &self.catch_pad, &self.successor)
    }
}
impl Show for CatchSwitch {
    fn show(&self, _types: &Types) -> String {
        let mut s = String::new();
        write!(s, "{} = catchswitch within {} [ label {}", &self.result, &self.parent_pad, self.catch_handlers.get(0).expect("CatchSwitch with no handlers")).unwrap();
        for handler in &self.catch_handlers[1..] { write!(s, ", label {}", handler).unwrap(); }
        write!(s, " ] unwind {}", match &self.default_unwind_dest {
            None => "to caller".into(),
            Some(dest) => format!("label {}", dest),
        }).unwrap();
        s
    }
}
impl Show for CallBr {
    fn show(&self, _types: &Types) -> String {
        let mut s = String::new();
        write!(s, "{} = callbr {}(", &self.result, match &self.function {
            either::Either::Left(_) => "<inline assembly>".into(),
            either::Either::Right(op) => format!("{}", op),
        }).unwrap();
        for (i, (arg, _)) in self.arguments.iter().enumerate() {
            if i == self.arguments.len() - 1 { write!(s, "{}", arg).unwrap(); }
            else { write!(s, "{}, ", arg).unwrap(); }
        }
        write!(s, ") to label {}", &self.return_label).unwrap();
        s
    }
}
impl Show for Terminator {
    fn show(&self, types: &Types) -> String {
        match self {
            Terminator::Ret(t) => t.show(types),
            Terminator::Br(t) => t.show(types),
            Terminator::CondBr(t) => t.show(types),
            Terminator::Switch(t) => t.show(types),
            Terminator::IndirectBr(t) => t.show(types),
            Terminator::Invoke(t) => t.show(types),
            Terminator::Resume(t) => t.show(types),
            Terminator::Unreachable(t) => t.show(types),
            Terminator::CleanupRet(t) => t.show(types),
            Terminator::CatchRet(t) => t.show(types),
            Terminator::CatchSwitch(t) => t.show(types),
            Terminator::CallBr(t) => t.show(types),
        }
    }
}
