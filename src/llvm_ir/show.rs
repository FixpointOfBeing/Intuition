use crate::llvm_ir::types::Types;

pub trait Show {
    fn show(&self, types: &Types) -> String;
}

mod module_show {
    use crate::llvm_ir::module;
    use crate::llvm_ir::module::{DLLStorageClass, Linkage, ThreadLocalMode, Visibility};
    use crate::llvm_ir::types::Types;
    use std::fmt::Write;

    use super::Show;
    impl Show for module::Module {
        fn show(&self, types: &Types) -> String {
            let mut parts: Vec<String> = Vec::new();

            let mut header = format!("source_filename = \"{}\"", self.source_file_name);
            if !self.data_layout.layout_str.is_empty() {
                header.push_str(&format!(
                    "\ntarget datalayout = \"{}\"",
                    self.data_layout.show(types)
                ));
            }
            if !self.target_triple.is_empty() {
                header.push_str(&format!("\ntarget triple = \"{}\"", self.target_triple));
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
    impl Show for module::GlobalVariable {
        fn show(&self, types: &Types) -> String {
            let mut s = String::new();
            write!(s, "@{} = ", self.name.show(types)).unwrap();
            if self.linkage != Linkage::External {
                write!(s, "{} ", self.linkage.show(types)).unwrap();
            }
            if self.visibility != Visibility::Default {
                write!(s, "{} ", self.visibility.show(types)).unwrap();
            }
            if self.dll_storage_class != DLLStorageClass::Default {
                write!(s, "{} ", self.dll_storage_class.show(types)).unwrap();
            }
            if self.thread_local_mode != ThreadLocalMode::NotThreadLocal {
                write!(s, "{} ", self.thread_local_mode.show(types)).unwrap();
            }
            if let Some(ref unnamed_addr) = self.unnamed_addr {
                write!(s, "{} ", unnamed_addr.show(types)).unwrap();
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
            write!(s, "{}", self.ty.show(types)).unwrap();
            if let Some(ref init) = self.initializer {
                write!(s, " {}", init.show(types)).unwrap();
            }
            if let Some(ref section) = self.section {
                write!(s, ", section \"{}\"", section).unwrap();
            }
            if let Some(ref comdat) = self.comdat {
                write!(s, ", {}", comdat.show(types)).unwrap();
            }
            if self.alignment > 0 {
                write!(s, ", align {}", self.alignment).unwrap();
            }
            s
        }
    }

    impl Show for module::GlobalAlias {
        fn show(&self, types: &Types) -> String {
            let mut s = String::new();
            write!(s, "@{} = ", self.name.show(types)).unwrap();
            if self.linkage != Linkage::External {
                write!(s, "{} ", self.linkage.show(types)).unwrap();
            }
            if self.visibility != Visibility::Default {
                write!(s, "{} ", self.visibility.show(types)).unwrap();
            }
            if self.dll_storage_class != DLLStorageClass::Default {
                write!(s, "{} ", self.dll_storage_class.show(types)).unwrap();
            }
            if self.thread_local_mode != ThreadLocalMode::NotThreadLocal {
                write!(s, "{} ", self.thread_local_mode.show(types)).unwrap();
            }
            if let Some(ref unnamed_addr) = self.unnamed_addr {
                write!(s, "{} ", unnamed_addr.show(types)).unwrap();
            }
            write!(s, "alias {}", self.ty.show(types)).unwrap();
            if self.addr_space != 0 {
                write!(s, ", addrspace({})", self.addr_space).unwrap();
            }
            write!(s, ", {}", self.aliasee.show(types)).unwrap();
            s
        }
    }

    impl Show for module::GlobalIFunc {
        fn show(&self, types: &Types) -> String {
            let mut s = String::new();
            write!(s, "@{} = ", self.name.show(types)).unwrap();
            if self.linkage != Linkage::External {
                write!(s, "{} ", self.linkage.show(types)).unwrap();
            }
            if self.visibility != Visibility::Default {
                write!(s, "{} ", self.visibility.show(types)).unwrap();
            }
            write!(s, "ifunc {}, {}", self.ty.show(types), self.resolver_fn.show(types)).unwrap();

            s
        }
    }
    impl Show for module::UnnamedAddr {
        fn show(&self, types: &Types) -> String {
            todo!();
        }
    }
    impl Show for module::Linkage {
        fn show(&self, types: &Types) -> String {
            todo!();
        }
    }
    impl Show for module::Visibility {
        fn show(&self, types: &Types) -> String {
            todo!();
        }
    }
    impl Show for module::DLLStorageClass {
        fn show(&self, types: &Types) -> String {
            todo!();
        }
    }
    impl Show for module::ThreadLocalMode {
        fn show(&self, types: &Types) -> String {
            todo!();
        }
    }
    impl Show for module::Comdat {
        fn show(&self, types: &Types) -> String {
            todo!();
        }
    }
    impl Show for module::SelectionKind {
        fn show(&self, types: &Types) -> String {
            todo!();
        }
    }
    impl Show for module::DataLayout {
        fn show(&self, types: &Types) -> String {
            todo!();
        }
    }
    impl Show for module::Endianness {
        fn show(&self, types: &Types) -> String {
            todo!();
        }
    }
    impl Show for module::Mangling {
        fn show(&self, types: &Types) -> String {
            todo!();
        }
    }
}

mod function_show {
    use crate::llvm_ir::function;
    use crate::llvm_ir::function::{CallingConvention, FunctionAttribute};
    use crate::llvm_ir::module::{DLLStorageClass, Linkage, Visibility};
    use crate::llvm_ir::name::Name;
    use crate::llvm_ir::types::Types;
    use std::fmt::Write;

    use super::Show;

    impl Show for function::Function {
        fn show(&self, types: &Types) -> String {
            let mut s = String::new();
            for attr in &self.function_attributes {
                match attr {
                    FunctionAttribute::UnknownAttribute => {}
                    _ => write!(s, "{} ", attr.show(types)).unwrap(),
                }
            }
            write!(s, "define ").unwrap();
            if self.linkage != Linkage::External {
                write!(s, "{} ", self.linkage.show(types)).unwrap();
            }
            if self.visibility != Visibility::Default {
                write!(s, "{} ", self.visibility.show(types)).unwrap();
            }
            if self.dll_storage_class != DLLStorageClass::Default {
                write!(s, "{} ", self.dll_storage_class.show(types)).unwrap();
            }
            if self.calling_convention != CallingConvention::C {
                write!(s, "{} ", self.calling_convention.show(types)).unwrap();
            }
            write!(s, "{} @{}(", self.return_type.show(types), self.name).unwrap();
            for (i, param) in self.parameters.iter().enumerate() {
                if i > 0 {
                    write!(s, ", ").unwrap();
                }
                write!(s, "{}", param.show(types)).unwrap();
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
                write!(s, " personality {}", pers.show(types)).unwrap();
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
                writeln!(s, "  {}", bb.term.show(types)).unwrap();
            }
            write!(s, "}}").unwrap();

            s
        }
    }
    impl Show for function::FunctionDeclaration {
        fn show(&self, types: &Types) -> String {
            let mut s = String::new();
            write!(s, "declare ").unwrap();
            if self.return_attributes.is_empty() {
                write!(s, "{}", self.return_type.show(types)).unwrap();
            } else {
                write!(s, "{}", self.return_type.show(types)).unwrap();
                for attr in &self.return_attributes {
                    write!(s, " {}", attr.show(types)).unwrap();
                }
            }
            write!(s, " @{}(", self.name).unwrap();
            for (i, param) in self.parameters.iter().enumerate() {
                if i > 0 {
                    write!(s, ", ").unwrap();
                }
                write!(s, "{}", param.show(types)).unwrap();
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

    impl Show for function::ParameterAttribute {
        fn show(&self, types: &Types) -> String {
            todo!()
        }
    }
    impl Show for function::Parameter {
        fn show(&self, types: &Types) -> String {
            todo!()
        }
    }
    impl Show for function::FunctionAttribute {
        fn show(&self, types: &Types) -> String {
            todo!()
        }
    }

    impl Show for function::CallingConvention {
        fn show(&self, types: &Types) -> String {
            todo!()
        }
    }
}

mod operand_show {
    use crate::llvm_ir::operand;
    use crate::llvm_ir::operand::Operand;
    use crate::llvm_ir::types::Types;
    use std::fmt::Write;

    use super::Show;

    impl Show for operand::Operand {
        fn show(&self, types: &Types) -> String {
            let mut s = String::new();
            match self {
                Operand::LocalOperand { name, ty: _ } => write!(s, "{}", name.show(types)).unwrap(),
                Operand::ConstantOperand(cref) => write!(s, "{}", cref.show(types)).unwrap(),
                Operand::MetadataOperand => write!(s, "<metadata>").unwrap(),
            }
            s
        }
    }
}

mod instruction_show {
    use crate::llvm_ir::constant::Constant;
    use crate::llvm_ir::instruction;
    use crate::llvm_ir::types::Types;
    use std::fmt::Write;

    use super::Show;

    impl Show for instruction::Add {
        fn show(&self, types: &Types) -> String {
            let mut s = String::new();
            let ty = types.type_of(&self.operand0);
            write!(s, "{} = add", &self.dest.show(types)).unwrap();
            if self.nuw {
                write!(s, " nuw").unwrap();
            }
            if self.nsw {
                write!(s, " nsw").unwrap();
            }
            write!(
                s,
                " {} {}, {}",
                ty.show(types),
                &self.operand0.show(types),
                &self.operand1.show(types)
            )
            .unwrap();
            s
        }
    }
    impl Show for instruction::Sub {
        fn show(&self, types: &Types) -> String {
            let mut s = String::new();
            let ty = types.type_of(&self.operand0);
            write!(s, "{} = sub", &self.dest.show(types)).unwrap();
            if self.nuw {
                write!(s, " nuw").unwrap();
            }
            if self.nsw {
                write!(s, " nsw").unwrap();
            }
            write!(
                s,
                " {} {}, {}",
                ty.show(types),
                &self.operand0.show(types),
                &self.operand1.show(types)
            )
            .unwrap();
            s
        }
    }
    impl Show for instruction::Mul {
        fn show(&self, types: &Types) -> String {
            let mut s = String::new();
            let ty = types.type_of(&self.operand0);
            write!(s, "{} = mul", &self.dest.show(types)).unwrap();
            if self.nuw {
                write!(s, " nuw").unwrap();
            }
            if self.nsw {
                write!(s, " nsw").unwrap();
            }
            write!(
                s,
                " {} {}, {}",
                ty.show(types),
                &self.operand0.show(types),
                &self.operand1.show(types)
            )
            .unwrap();
            s
        }
    }
    impl Show for instruction::UDiv {
        fn show(&self, types: &Types) -> String {
            let mut s = String::new();
            let ty = types.type_of(&self.operand0);
            write!(s, "{} = udiv", &self.dest.show(types)).unwrap();
            if self.exact {
                write!(s, " exact").unwrap();
            }
            write!(
                s,
                " {} {}, {}",
                ty.show(types),
                &self.operand0.show(types),
                &self.operand1.show(types)
            )
            .unwrap();
            s
        }
    }
    impl Show for instruction::SDiv {
        fn show(&self, types: &Types) -> String {
            let mut s = String::new();
            let ty = types.type_of(&self.operand0);
            write!(s, "{} = sdiv", &self.dest.show(types)).unwrap();
            if self.exact {
                write!(s, " exact").unwrap();
            }
            write!(
                s,
                " {} {}, {}",
                ty.show(types),
                &self.operand0.show(types),
                &self.operand1.show(types)
            )
            .unwrap();
            s
        }
    }
    impl Show for instruction::URem {
        fn show(&self, types: &Types) -> String {
            let mut s = String::new();
            let ty = types.type_of(&self.operand0);
            write!(
                s,
                "{} = urem {} {}, {}",
                &self.dest.show(types),
                ty.show(types),
                &self.operand0.show(types),
                &self.operand1.show(types)
            )
            .unwrap();
            s
        }
    }
    impl Show for instruction::SRem {
        fn show(&self, types: &Types) -> String {
            let mut s = String::new();
            let ty = types.type_of(&self.operand0);
            write!(
                s,
                "{} = srem {} {}, {}",
                &self.dest.show(types),
                ty.show(types),
                &self.operand0.show(types),
                &self.operand1.show(types)
            )
            .unwrap();
            s
        }
    }
    impl Show for instruction::And {
        fn show(&self, types: &Types) -> String {
            let mut s = String::new();
            let ty = types.type_of(&self.operand0);
            write!(
                s,
                "{} = and {} {}, {}",
                &self.dest.show(types),
                ty.show(types),
                &self.operand0.show(types),
                &self.operand1.show(types)
            )
            .unwrap();
            s
        }
    }
    impl Show for instruction::Or {
        fn show(&self, types: &Types) -> String {
            let mut s = String::new();
            let ty = types.type_of(&self.operand0);
            write!(s, "{} = or", &self.dest.show(types)).unwrap();
            if self.disjoint {
                write!(s, " disjoint").unwrap();
            }
            write!(
                s,
                " {} {}, {}",
                ty.show(types),
                &self.operand0.show(types),
                &self.operand1.show(types)
            )
            .unwrap();
            s
        }
    }
    impl Show for instruction::Xor {
        fn show(&self, types: &Types) -> String {
            let mut s = String::new();
            let ty = types.type_of(&self.operand0);
            write!(
                s,
                "{} = xor {} {}, {}",
                &self.dest.show(types),
                ty.show(types),
                &self.operand0.show(types),
                &self.operand1.show(types)
            )
            .unwrap();
            s
        }
    }
    impl Show for instruction::Shl {
        fn show(&self, types: &Types) -> String {
            let mut s = String::new();
            let ty = types.type_of(&self.operand0);
            write!(s, "{} = shl", &self.dest.show(types)).unwrap();
            if self.nuw {
                write!(s, " nuw").unwrap();
            }
            if self.nsw {
                write!(s, " nsw").unwrap();
            }
            write!(
                s,
                " {} {}, {}",
                ty.show(types),
                &self.operand0.show(types),
                &self.operand1.show(types)
            )
            .unwrap();
            s
        }
    }
    impl Show for instruction::LShr {
        fn show(&self, types: &Types) -> String {
            let mut s = String::new();
            let ty = types.type_of(&self.operand0);
            write!(s, "{} = lshr", &self.dest.show(types)).unwrap();
            if self.exact {
                write!(s, " exact").unwrap();
            }
            write!(
                s,
                " {} {}, {}",
                ty.show(types),
                &self.operand0.show(types),
                &self.operand1.show(types)
            )
            .unwrap();
            s
        }
    }
    impl Show for instruction::AShr {
        fn show(&self, types: &Types) -> String {
            let mut s = String::new();
            let ty = types.type_of(&self.operand0);
            write!(s, "{} = ashr", &self.dest.show(types)).unwrap();
            if self.exact {
                write!(s, " exact").unwrap();
            }
            write!(
                s,
                " {} {}, {}",
                ty.show(types),
                &self.operand0.show(types),
                &self.operand1.show(types)
            )
            .unwrap();
            s
        }
    }
    impl Show for instruction::FAdd {
        fn show(&self, types: &Types) -> String {
            let mut s = String::new();
            let ty = types.type_of(&self.operand0);
            write!(
                s,
                "{} = fadd {} {}, {}",
                &self.dest.show(types),
                ty.show(types),
                &self.operand0.show(types),
                &self.operand1.show(types)
            )
            .unwrap();
            s
        }
    }
    impl Show for instruction::FSub {
        fn show(&self, types: &Types) -> String {
            let mut s = String::new();
            let ty = types.type_of(&self.operand0);
            write!(
                s,
                "{} = fsub {} {}, {}",
                &self.dest.show(types),
                ty.show(types),
                &self.operand0.show(types),
                &self.operand1.show(types)
            )
            .unwrap();
            s
        }
    }
    impl Show for instruction::FMul {
        fn show(&self, types: &Types) -> String {
            let mut s = String::new();
            let ty = types.type_of(&self.operand0);
            write!(
                s,
                "{} = fmul {} {}, {}",
                &self.dest.show(types),
                ty.show(types),
                &self.operand0.show(types),
                &self.operand1.show(types)
            )
            .unwrap();
            s
        }
    }
    impl Show for instruction::FDiv {
        fn show(&self, types: &Types) -> String {
            let mut s = String::new();
            let ty = types.type_of(&self.operand0);
            write!(
                s,
                "{} = fdiv {} {}, {}",
                &self.dest.show(types),
                ty.show(types),
                &self.operand0.show(types),
                &self.operand1.show(types)
            )
            .unwrap();
            s
        }
    }
    impl Show for instruction::FRem {
        fn show(&self, types: &Types) -> String {
            let mut s = String::new();
            let ty = types.type_of(&self.operand0);
            write!(
                s,
                "{} = frem {} {}, {}",
                &self.dest.show(types),
                ty.show(types),
                &self.operand0.show(types),
                &self.operand1.show(types)
            )
            .unwrap();
            s
        }
    }
    impl Show for instruction::FNeg {
        fn show(&self, types: &Types) -> String {
            let mut s = String::new();
            let ty = types.type_of(&self.operand);
            write!(
                s,
                "{} = fneg {} {}",
                &self.dest.show(types),
                ty.show(types),
                &self.operand.show(types)
            )
            .unwrap();
            s
        }
    }
    impl Show for instruction::ExtractElement {
        fn show(&self, types: &Types) -> String {
            let mut s = String::new();
            let vec_ty = types.type_of(&self.vector);
            write!(
                s,
                "{} = extractelement {} {}, {}",
                &self.dest.show(types),
                vec_ty.show(types),
                &self.vector.show(types),
                &self.index.show(types)
            )
            .unwrap();
            s
        }
    }
    impl Show for instruction::InsertElement {
        fn show(&self, types: &Types) -> String {
            let mut s = String::new();
            let vec_ty = types.type_of(&self.vector);
            write!(
                s,
                "{} = insertelement {} {}, {}, {}",
                &self.dest.show(types),
                vec_ty.show(types),
                &self.vector.show(types),
                &self.element.show(types),
                &self.index.show(types)
            )
            .unwrap();
            s
        }
    }
    impl Show for instruction::ShuffleVector {
        fn show(&self, types: &Types) -> String {
            let mut s = String::new();
            let vec_ty = types.type_of(&self.operand0);
            write!(
                s,
                "{} = shufflevector {} {}, {}, {}",
                &self.dest.show(types),
                vec_ty.show(types),
                &self.operand0.show(types),
                &self.operand1.show(types),
                &self.mask.show(types)
            )
            .unwrap();
            s
        }
    }
    impl Show for instruction::ExtractValue {
        fn show(&self, types: &Types) -> String {
            let mut s = String::new();
            let agg_ty = types.type_of(&self.aggregate);
            write!(
                s,
                "{} = extractvalue {} {}, {}",
                &self.dest.show(types),
                agg_ty.show(types),
                &self.aggregate.show(types),
                self.indices.first().expect("ExtractValue with no indices")
            )
            .unwrap();
            for idx in &self.indices[1..] {
                write!(s, ", {idx}").unwrap();
            }
            s
        }
    }
    impl Show for instruction::InsertValue {
        fn show(&self, types: &Types) -> String {
            let mut s = String::new();
            let agg_ty = types.type_of(&self.aggregate);
            write!(
                s,
                "{} = insertvalue {} {}, {}, {}",
                &self.dest.show(types),
                agg_ty.show(types),
                &self.aggregate.show(types),
                &self.element.show(types),
                self.indices.first().expect("InsertValue with no indices")
            )
            .unwrap();
            for idx in &self.indices[1..] {
                write!(s, ", {idx}").unwrap();
            }
            s
        }
    }
    impl Show for instruction::Alloca {
        fn show(&self, types: &Types) -> String {
            let mut s = String::new();
            write!(
                s,
                "{} = alloca {}",
                &self.dest.show(types),
                &self.allocated_type.show(types)
            )
            .unwrap();
            if let Some(Constant::Int { value: 1, .. }) = self.num_elements.as_constant() {
            } else {
                write!(s, ", {}", &self.num_elements.show(types)).unwrap();
            }
            write!(s, ", align {}", &self.alignment).unwrap();
            s
        }
    }
    impl Show for instruction::Load {
        fn show(&self, types: &Types) -> String {
            let mut s = String::new();
            write!(s, "{} = load ", &self.dest.show(types)).unwrap();
            if self.atomicity.is_some() {
                write!(s, "atomic ").unwrap();
            }
            if self.volatile {
                write!(s, "volatile ").unwrap();
            }
            write!(s, "{}, {}", &self.loaded_ty.show(types), &self.address.show(types)).unwrap();
            if let Some(a) = &self.atomicity {
                write!(s, " {}", a.show(types)).unwrap();
            }
            write!(s, ", align {}", &self.alignment).unwrap();
            s
        }
    }
    impl Show for instruction::Store {
        fn show(&self, types: &Types) -> String {
            let mut s = String::new();
            write!(s, "store ").unwrap();
            if self.atomicity.is_some() {
                write!(s, "atomic ").unwrap();
            }
            if self.volatile {
                write!(s, "volatile ").unwrap();
            }
            write!(
                s,
                "{}, {}",
                &self.value.show(types),
                &self.address.show(types)
            )
            .unwrap();
            if let Some(a) = &self.atomicity {
                write!(s, " {}", a.show(types)).unwrap();
            }
            write!(s, ", align {}", &self.alignment).unwrap();
            s
        }
    }
    impl Show for instruction::Fence {
        fn show(&self, types: &Types) -> String {
            let mut s = String::new();
            write!(s, "fence {}", &self.atomicity.show(types)).unwrap();
            s
        }
    }
    impl Show for instruction::CmpXchg {
        fn show(&self, types: &Types) -> String {
            let mut s = String::new();
            write!(s, "{} = cmpxchg ", &self.dest.show(types)).unwrap();
            if self.weak {
                write!(s, "weak ").unwrap();
            }
            if self.volatile {
                write!(s, "volatile ").unwrap();
            }
            write!(
                s,
                "{}, {}, {} {} {}",
                &self.address.show(types),
                &self.expected.show(types),
                &self.replacement.show(types),
                &self.atomicity.show(types),
                &self.failure_memory_ordering.show(types)
            )
            .unwrap();
            s
        }
    }
    impl Show for instruction::AtomicRMW {
        fn show(&self, types: &Types) -> String {
            let mut s = String::new();
            write!(s, "{} = atomicrmw ", &self.dest.show(types)).unwrap();
            if self.volatile {
                write!(s, "volatile ").unwrap();
            }
            write!(
                s,
                "{} {}, {} {}",
                &self.operation.show(types),
                &self.address.show(types),
                &self.value.show(types),
                &self.atomicity.show(types)
            )
            .unwrap();
            s
        }
    }
    impl Show for instruction::GetElementPtr {
        fn show(&self, types: &Types) -> String {
            let mut s = String::new();
            write!(s, "{} = getelementptr ", &self.dest.show(types)).unwrap();
            if self.in_bounds {
                write!(s, "inbounds ").unwrap();
            }
            write!(
                s,
                "{}, {}",
                &self.source_element_type.show(types),
                &self.address.show(types)
            )
            .unwrap();
            for idx in &self.indices {
                write!(s, ", {}", idx.show(types)).unwrap();
            }
            s
        }
    }
    impl Show for instruction::Trunc {
        fn show(&self, types: &Types) -> String {
            let mut s = String::new();
            let from_ty = types.type_of(&self.operand);
            write!(
                s,
                "{} = trunc {} {} to {}",
                &self.dest.show(types),
                from_ty.show(types),
                &self.operand.show(types),
                &self.to_type.show(types)
            )
            .unwrap();
            s
        }
    }
    impl Show for instruction::ZExt {
        fn show(&self, types: &Types) -> String {
            let mut s = String::new();
            let from_ty = types.type_of(&self.operand);
            write!(s, "{} = zext", &self.dest.show(types)).unwrap();
            if self.nneg {
                write!(s, " nneg").unwrap();
            }
            write!(
                s,
                " {} {} to {}",
                from_ty.show(types),
                &self.operand.show(types),
                &self.to_type.show(types)
            )
            .unwrap();
            s
        }
    }
    impl Show for instruction::SExt {
        fn show(&self, types: &Types) -> String {
            let mut s = String::new();
            let from_ty = types.type_of(&self.operand);
            write!(
                s,
                "{} = sext {} {} to {}",
                &self.dest.show(types),
                from_ty.show(types),
                &self.operand.show(types),
                &self.to_type.show(types)
            )
            .unwrap();
            s
        }
    }
    impl Show for instruction::FPTrunc {
        fn show(&self, types: &Types) -> String {
            let mut s = String::new();
            let from_ty = types.type_of(&self.operand);
            write!(
                s,
                "{} = fptrunc {} {} to {}",
                &self.dest.show(types),
                from_ty.show(types),
                &self.operand.show(types),
                &self.to_type.show(types)
            )
            .unwrap();
            s
        }
    }
    impl Show for instruction::FPExt {
        fn show(&self, types: &Types) -> String {
            let mut s = String::new();
            let from_ty = types.type_of(&self.operand);
            write!(
                s,
                "{} = fpext {} {} to {}",
                &self.dest.show(types),
                from_ty.show(types),
                &self.operand.show(types),
                &self.to_type.show(types)
            )
            .unwrap();
            s
        }
    }
    impl Show for instruction::FPToUI {
        fn show(&self, types: &Types) -> String {
            let mut s = String::new();
            let from_ty = types.type_of(&self.operand);
            write!(
                s,
                "{} = fptoui {} {} to {}",
                &self.dest.show(types),
                from_ty.show(types),
                &self.operand.show(types),
                &self.to_type.show(types)
            )
            .unwrap();
            s
        }
    }
    impl Show for instruction::FPToSI {
        fn show(&self, types: &Types) -> String {
            let mut s = String::new();
            let from_ty = types.type_of(&self.operand);
            write!(
                s,
                "{} = fptosi {} {} to {}",
                &self.dest.show(types),
                from_ty.show(types),
                &self.operand.show(types),
                &self.to_type.show(types)
            )
            .unwrap();
            s
        }
    }
    impl Show for instruction::UIToFP {
        fn show(&self, types: &Types) -> String {
            let mut s = String::new();
            let from_ty = types.type_of(&self.operand);
            write!(
                s,
                "{} = uitofp {} {} to {}",
                &self.dest.show(types),
                from_ty.show(types),
                &self.operand.show(types),
                &self.to_type.show(types)
            )
            .unwrap();
            s
        }
    }
    impl Show for instruction::SIToFP {
        fn show(&self, types: &Types) -> String {
            let mut s = String::new();
            let from_ty = types.type_of(&self.operand);
            write!(
                s,
                "{} = sitofp {} {} to {}",
                &self.dest.show(types),
                from_ty.show(types),
                &self.operand.show(types),
                &self.to_type.show(types)
            )
            .unwrap();
            s
        }
    }
    impl Show for instruction::PtrToInt {
        fn show(&self, types: &Types) -> String {
            let mut s = String::new();
            let from_ty = types.type_of(&self.operand);
            write!(
                s,
                "{} = ptrtoint {} {} to {}",
                &self.dest.show(types),
                from_ty.show(types),
                &self.operand.show(types),
                &self.to_type.show(types)
            )
            .unwrap();
            s
        }
    }
    impl Show for instruction::IntToPtr {
        fn show(&self, types: &Types) -> String {
            let mut s = String::new();
            let from_ty = types.type_of(&self.operand);
            write!(
                s,
                "{} = inttoptr {} {} to {}",
                &self.dest.show(types),
                from_ty.show(types),
                &self.operand.show(types),
                &self.to_type.show(types)
            )
            .unwrap();
            s
        }
    }
    impl Show for instruction::BitCast {
        fn show(&self, types: &Types) -> String {
            let mut s = String::new();
            let from_ty = types.type_of(&self.operand);
            write!(
                s,
                "{} = bitcast {} {} to {}",
                &self.dest.show(types),
                from_ty.show(types),
                &self.operand.show(types),
                &self.to_type.show(types)
            )
            .unwrap();
            s
        }
    }
    impl Show for instruction::AddrSpaceCast {
        fn show(&self, types: &Types) -> String {
            let mut s = String::new();
            let from_ty = types.type_of(&self.operand);
            write!(
                s,
                "{} = addrspacecast {} {} to {}",
                &self.dest.show(types),
                from_ty.show(types),
                &self.operand.show(types),
                &self.to_type.show(types)
            )
            .unwrap();
            s
        }
    }
    impl Show for instruction::ICmp {
        fn show(&self, types: &Types) -> String {
            let mut s = String::new();
            let ty = types.type_of(&self.operand0);
            write!(
                s,
                "{} = icmp {} {} {}, {}",
                &self.dest.show(types),
                &self.predicate.show(types),
                ty.show(types),
                &self.operand0.show(types),
                &self.operand1.show(types)
            )
            .unwrap();
            s
        }
    }
    impl Show for instruction::FCmp {
        fn show(&self, types: &Types) -> String {
            let mut s = String::new();
            let ty = types.type_of(&self.operand0);
            write!(
                s,
                "{} = fcmp {} {} {}, {}",
                &self.dest.show(types),
                &self.predicate.show(types),
                ty.show(types),
                &self.operand0.show(types),
                &self.operand1.show(types)
            )
            .unwrap();
            s
        }
    }
    impl Show for instruction::Phi {
        fn show(&self, types: &Types) -> String {
            let mut s = String::new();
            let (first_val, first_label) = &self
                .incoming_values
                .get(0)
                .expect("Phi with no incoming values");
            write!(
                s,
                "{} = phi {} [ {}, {} ]",
                &self.dest.show(types),
                &self.to_type.show(types),
                first_val.show(types),
                first_label.show(types)
            )
            .unwrap();
            for (val, label) in &self.incoming_values[1..] {
                write!(s, ", [ {}, {} ]", val.show(types), label.show(types)).unwrap();
            }
            s
        }
    }
    impl Show for instruction::Select {
        fn show(&self, types: &Types) -> String {
            let mut s = String::new();
            let ty = types.type_of(&self.true_value);
            write!(
                s,
                "{} = select {} {}, {} {}, {} {}",
                &self.dest.show(types),
                ty.show(types),
                &self.condition.show(types),
                ty.show(types),
                &self.true_value.show(types),
                ty.show(types),
                &self.false_value.show(types)
            )
            .unwrap();
            s
        }
    }
    impl Show for instruction::Freeze {
        fn show(&self, types: &Types) -> String {
            let mut s = String::new();
            let ty = types.type_of(&self.operand);
            write!(
                s,
                "{} = freeze {} {}",
                &self.dest.show(types),
                ty.show(types),
                &self.operand.show(types)
            )
            .unwrap();
            s
        }
    }
    impl Show for instruction::Call {
        fn show(&self, types: &Types) -> String {
            let mut s = String::new();
            if let Some(dest) = &self.dest {
                write!(s, "{} = ", dest.show(types)).unwrap();
            }
            if self.is_tail_call {
                write!(s, "tail ").unwrap();
            }
            write!(
                s,
                "call {}(",
                match &self.function {
                    either::Either::Left(_) => "<inline assembly>".into(),
                    either::Either::Right(op) =>
                        format!("{} {}", types.type_of(self).show(types), op.show(types)),
                }
            )
            .unwrap();
            for (i, (arg, _)) in self.arguments.iter().enumerate() {
                if i == self.arguments.len() - 1 {
                    write!(s, "{}", arg.show(types)).unwrap();
                } else {
                    write!(s, "{}, ", arg.show(types)).unwrap();
                }
            }
            write!(s, ")").unwrap();
            s
        }
    }
    impl Show for instruction::VAArg {
        fn show(&self, types: &Types) -> String {
            let mut s = String::new();
            write!(
                s,
                "{} = va_arg {}, {}",
                &self.dest.show(types),
                &self.arg_list.show(types),
                &self.cur_type.show(types)
            )
            .unwrap();
            s
        }
    }
    impl Show for instruction::LandingPad {
        fn show(&self, types: &Types) -> String {
            let mut s = String::new();
            write!(
                s,
                "{} = landingpad {}",
                &self.dest.show(types),
                &self.result_type.show(types)
            )
            .unwrap();
            if self.cleanup {
                write!(s, " cleanup").unwrap();
            }
            s
        }
    }
    impl Show for instruction::CatchPad {
        fn show(&self, types: &Types) -> String {
            let mut s = String::new();
            write!(
                s,
                "{} = catchpad within {} [",
                &self.dest.show(types),
                &self.catch_switch.show(types)
            )
            .unwrap();
            for (i, arg) in self.args.iter().enumerate() {
                if i == self.args.len() - 1 {
                    write!(s, "{}", arg.show(types)).unwrap();
                } else {
                    write!(s, "{}, ", arg.show(types)).unwrap();
                }
            }
            write!(s, "]").unwrap();
            s
        }
    }
    impl Show for instruction::CleanupPad {
        fn show(&self, types: &Types) -> String {
            let mut s = String::new();
            write!(
                s,
                "{} = cleanuppad within {} [",
                &self.dest.show(types),
                &self.parent_pad.show(types)
            )
            .unwrap();
            for (i, arg) in self.args.iter().enumerate() {
                if i == self.args.len() - 1 {
                    write!(s, "{}", arg.show(types)).unwrap();
                } else {
                    write!(s, "{}, ", arg.show(types)).unwrap();
                }
            }
            write!(s, "]").unwrap();
            s
        }
    }
    impl Show for instruction::Atomicity {
        fn show(&self, types: &Types) -> String {
            todo!();
        }
    }
    impl Show for instruction::RMWBinOp {
        fn show(&self, types: &Types) -> String {
            todo!();
        }
    }
    impl Show for instruction::MemoryOrdering {
        fn show(&self, types: &Types) -> String {
            todo!();
        }
    }
    impl Show for instruction::Instruction {
        fn show(&self, types: &Types) -> String {
            match self {
                instruction::Instruction::Add(i) => i.show(types),
                instruction::Instruction::Sub(i) => i.show(types),
                instruction::Instruction::Mul(i) => i.show(types),
                instruction::Instruction::UDiv(i) => i.show(types),
                instruction::Instruction::SDiv(i) => i.show(types),
                instruction::Instruction::URem(i) => i.show(types),
                instruction::Instruction::SRem(i) => i.show(types),
                instruction::Instruction::And(i) => i.show(types),
                instruction::Instruction::Or(i) => i.show(types),
                instruction::Instruction::Xor(i) => i.show(types),
                instruction::Instruction::Shl(i) => i.show(types),
                instruction::Instruction::LShr(i) => i.show(types),
                instruction::Instruction::AShr(i) => i.show(types),
                instruction::Instruction::FAdd(i) => i.show(types),
                instruction::Instruction::FSub(i) => i.show(types),
                instruction::Instruction::FMul(i) => i.show(types),
                instruction::Instruction::FDiv(i) => i.show(types),
                instruction::Instruction::FRem(i) => i.show(types),
                instruction::Instruction::FNeg(i) => i.show(types),
                instruction::Instruction::ExtractElement(i) => i.show(types),
                instruction::Instruction::InsertElement(i) => i.show(types),
                instruction::Instruction::ShuffleVector(i) => i.show(types),
                instruction::Instruction::ExtractValue(i) => i.show(types),
                instruction::Instruction::InsertValue(i) => i.show(types),
                instruction::Instruction::Alloca(i) => i.show(types),
                instruction::Instruction::Load(i) => i.show(types),
                instruction::Instruction::Store(i) => i.show(types),
                instruction::Instruction::Fence(i) => i.show(types),
                instruction::Instruction::CmpXchg(i) => i.show(types),
                instruction::Instruction::AtomicRMW(i) => i.show(types),
                instruction::Instruction::GetElementPtr(i) => i.show(types),
                instruction::Instruction::Trunc(i) => i.show(types),
                instruction::Instruction::ZExt(i) => i.show(types),
                instruction::Instruction::SExt(i) => i.show(types),
                instruction::Instruction::FPTrunc(i) => i.show(types),
                instruction::Instruction::FPExt(i) => i.show(types),
                instruction::Instruction::FPToUI(i) => i.show(types),
                instruction::Instruction::FPToSI(i) => i.show(types),
                instruction::Instruction::UIToFP(i) => i.show(types),
                instruction::Instruction::SIToFP(i) => i.show(types),
                instruction::Instruction::PtrToInt(i) => i.show(types),
                instruction::Instruction::IntToPtr(i) => i.show(types),
                instruction::Instruction::BitCast(i) => i.show(types),
                instruction::Instruction::AddrSpaceCast(i) => i.show(types),
                instruction::Instruction::ICmp(i) => i.show(types),
                instruction::Instruction::FCmp(i) => i.show(types),
                instruction::Instruction::Phi(i) => i.show(types),
                instruction::Instruction::Select(i) => i.show(types),
                instruction::Instruction::Freeze(i) => i.show(types),
                instruction::Instruction::Call(i) => i.show(types),
                instruction::Instruction::VAArg(i) => i.show(types),
                instruction::Instruction::LandingPad(i) => i.show(types),
                instruction::Instruction::CatchPad(i) => i.show(types),
                instruction::Instruction::CleanupPad(i) => i.show(types),
            }
        }
    }
}

mod types_show {
    use crate::llvm_ir::types;
    use crate::llvm_ir::types::{NamedStructDef, Types};
    use std::fmt::Write;

    use super::Show;

    impl Show for types::NamedStructDef {
        fn show(&self, types: &Types) -> String {
            let mut s = String::new();
            match self {
                NamedStructDef::Opaque => write!(s, "type opaque").unwrap(),
                NamedStructDef::Defined(ty) => write!(s, "type {}", ty.show(types)).unwrap(),
            };
            s
        }
    }

    impl Show for types::LLVMType {
        fn show(&self, types: &Types) -> String {
            todo!();
        }
    }
    impl Show for types::FPType {
        fn show(&self, types: &Types) -> String {
            todo!();
        }
    }
    impl Show for types::TypeRef {
        fn show(&self, types: &Types) -> String {
            todo!();
        }
    }
}

mod constant_show {
    use crate::llvm_ir::constant;
    use crate::llvm_ir::name::Name;
    use crate::llvm_ir::types::Types;
    use std::fmt::Write;

    use super::Show;

    impl Show for constant::Add {
        fn show(&self, types: &Types) -> String {
            todo!();
        }
    }
    impl Show for constant::Sub {
        fn show(&self, types: &Types) -> String {
            todo!();
        }
    }
    impl Show for constant::Mul {
        fn show(&self, types: &Types) -> String {
            todo!();
        }
    }
    impl Show for constant::Xor {
        fn show(&self, types: &Types) -> String {
            todo!();
        }
    }
    impl Show for constant::FRem {
        fn show(&self, types: &Types) -> String {
            format!(
                "{} frem ({}, {})",
                types.type_of(&self.operand0).show(types),
                &self.operand0.show(types),
                &self.operand1.show(types)
            )
        }
    }
    impl Show for constant::ExtractElement {
        fn show(&self, types: &Types) -> String {
            todo!();
        }
    }
    impl Show for constant::InsertElement {
        fn show(&self, types: &Types) -> String {
            todo!();
        }
    }
    impl Show for constant::ShuffleVector {
        fn show(&self, types: &Types) -> String {
            todo!();
        }
    }
    impl Show for constant::GetElementPtr {
        fn show(&self, types: &Types) -> String {
            todo!();
        }
    }
    impl Show for constant::Trunc {
        fn show(&self, types: &Types) -> String {
            todo!();
        }
    }
    impl Show for constant::PtrToInt {
        fn show(&self, types: &Types) -> String {
            todo!();
        }
    }
    impl Show for constant::IntToPtr {
        fn show(&self, types: &Types) -> String {
            todo!();
        }
    }
    impl Show for constant::BitCast {
        fn show(&self, types: &Types) -> String {
            todo!();
        }
    }
    impl Show for constant::AddrSpaceCast {
        fn show(&self, types: &Types) -> String {
            todo!();
        }
    }
    impl Show for constant::ConstantRef {
        fn show(&self, types: &Types) -> String {
            self.as_ref().show(types)
        }
    }
    impl Show for constant::Float {
        fn show(&self, types: &Types) -> String {
            todo!()
        }
    }
    impl Show for constant::Constant {
        fn show(&self, types: &Types) -> String {
            let mut s = String::new();
            match self {
                constant::Constant::Int { bits, value } => {
                    if *bits == 1 {
                        if *value == 0 {
                            write!(s, "false").unwrap()
                        } else {
                            write!(s, "true").unwrap()
                        }
                    } else {
                        match *bits {
                            16 => write!(s, "{}", (*value & 0xFFFF) as i16).unwrap(),
                            32 => write!(s, "{}", (*value & 0xFFFF_FFFF) as i32).unwrap(),
                            64 => write!(s, "{}", *value as i64).unwrap(),
                            _ => write!(s, "{}", value).unwrap(),
                        }
                    }
                }
                constant::Constant::Float(f) => write!(s, "{}", f.show(types)).unwrap(),
                constant::Constant::Null(_) => write!(s, "null").unwrap(),
                constant::Constant::AggregateZero(_) => write!(s, "zeroinitializer").unwrap(),
                constant::Constant::Undef(_) => write!(s, "undef").unwrap(),
                constant::Constant::Poison(_) => write!(s, "poison").unwrap(),
                constant::Constant::BlockAddress => write!(s, "blockaddr").unwrap(),
                constant::Constant::GlobalReference { name, ty: _ } => match name {
                    Name::Name(n) => write!(s, "@{}", n).unwrap(),
                    Name::Number(n) => write!(s, "@{}", n).unwrap(),
                },
                constant::Constant::TokenNone => write!(s, "none").unwrap(),
                constant::Constant::Struct {
                    name,
                    values,
                    is_packed,
                } => todo!(),
                constant::Constant::Array {
                    element_type,
                    elements,
                } => todo!(),
                constant::Constant::Vector(constant_refs) => todo!(),
                constant::Constant::PtrAuth {
                    ptr,
                    key,
                    disc,
                    addr_disc,
                } => todo!(),
                constant::Constant::Add(add) => todo!(),
                constant::Constant::Sub(sub) => todo!(),
                constant::Constant::Mul(mul) => todo!(),
                constant::Constant::Xor(xor) => todo!(),
                constant::Constant::ExtractElement(extract_element) => todo!(),
                constant::Constant::InsertElement(insert_element) => todo!(),
                constant::Constant::ShuffleVector(shuffle_vector) => todo!(),
                constant::Constant::GetElementPtr(get_element_ptr) => todo!(),
                constant::Constant::Trunc(trunc) => todo!(),
                constant::Constant::PtrToInt(ptr_to_int) => todo!(),
                constant::Constant::IntToPtr(int_to_ptr) => todo!(),
                constant::Constant::BitCast(bit_cast) => todo!(),
                constant::Constant::AddrSpaceCast(addr_space_cast) => todo!(),
            }
            s
        }
    }
}

mod terminator_show {
    use crate::llvm_ir::terminator;
    use crate::llvm_ir::types::Types;
    use std::fmt::Write;

    use super::Show;

    impl Show for terminator::Ret {
        fn show(&self, types: &Types) -> String {
            let mut s = String::new();
            write!(s, "ret ").unwrap();
            match &self.return_operand {
                None => write!(s, "void").unwrap(),
                Some(op) => write!(s, "{}", op.show(types)).unwrap(),
            }
            s
        }
    }
    impl Show for terminator::Br {
        fn show(&self, types: &Types) -> String {
            format!("br label {}", &self.dest.show(types))
        }
    }
    impl Show for terminator::CondBr {
        fn show(&self, types: &Types) -> String {
            let mut s = String::new();
            write!(
                s,
                "br i1 {}, label {}, label {}",
                &self.condition.show(types),
                &self.true_dest.show(types),
                &self.false_dest.show(types)
            )
            .unwrap();
            s
        }
    }
    impl Show for terminator::Switch {
        fn show(&self, types: &Types) -> String {
            let mut s = String::new();
            write!(
                s,
                "switch {}, label {} [ ",
                &self.operand.show(types),
                &self.default_dest.show(types)
            )
            .unwrap();
            for (val, label) in &self.dests {
                write!(s, "{}, label {}; ", val.show(types), label.show(types)).unwrap();
            }
            write!(s, "]").unwrap();
            s
        }
    }
    impl Show for terminator::IndirectBr {
        fn show(&self, types: &Types) -> String {
            let mut s = String::new();
            write!(
                s,
                "indirectbr {}, [ label {}",
                &self.operand.show(types),
                self.possible_dests
                    .get(0)
                    .expect("IndirectBr with no possible dests")
                    .show(types)
            )
            .unwrap();
            for dest in &self.possible_dests[1..] {
                write!(s, ", label {}", dest.show(types)).unwrap();
            }
            write!(s, " ]").unwrap();
            s
        }
    }
    impl Show for terminator::Invoke {
        fn show(&self, types: &Types) -> String {
            let mut s = String::new();
            write!(
                s,
                "{} = invoke {}(",
                &self.result.show(types),
                match &self.function {
                    either::Either::Left(_) => "<inline assembly>".into(),
                    either::Either::Right(op) => format!("{}", op.show(types)),
                }
            )
            .unwrap();
            for (i, (arg, _)) in self.arguments.iter().enumerate() {
                if i == self.arguments.len() - 1 {
                    write!(s, "{}", arg.show(types)).unwrap();
                } else {
                    write!(s, "{}, ", arg.show(types)).unwrap();
                }
            }
            write!(
                s,
                ") to label {} unwind label {}",
                &self.return_label.show(types),
                &self.exception_label.show(types)
            )
            .unwrap();
            s
        }
    }
    impl Show for terminator::Resume {
        fn show(&self, types: &Types) -> String {
            format!("resume {}", &self.operand.show(types))
        }
    }
    impl Show for terminator::Unreachable {
        fn show(&self, _types: &Types) -> String {
            "unreachable".to_string()
        }
    }
    impl Show for terminator::CleanupRet {
        fn show(&self, types: &Types) -> String {
            let mut s = String::new();
            write!(
                s,
                "cleanupret from {} unwind ",
                &self.cleanup_pad.show(types)
            )
            .unwrap();
            match &self.unwind_dest {
                None => write!(s, "to caller").unwrap(),
                Some(dest) => write!(s, "label {}", dest.show(types)).unwrap(),
            }
            s
        }
    }
    impl Show for terminator::CatchRet {
        fn show(&self, types: &Types) -> String {
            format!(
                "catchret from {} to label {}",
                &self.catch_pad.show(types),
                &self.successor.show(types)
            )
        }
    }
    impl Show for terminator::CatchSwitch {
        fn show(&self, types: &Types) -> String {
            let mut s = String::new();
            write!(
                s,
                "{} = catchswitch within {} [ label {}",
                &self.result.show(types),
                &self.parent_pad.show(types),
                self.catch_handlers
                    .get(0)
                    .expect("CatchSwitch with no handlers")
                    .show(types)
            )
            .unwrap();
            for handler in &self.catch_handlers[1..] {
                write!(s, ", label {}", handler.show(types)).unwrap();
            }
            write!(
                s,
                " ] unwind {}",
                match &self.default_unwind_dest {
                    None => "to caller".into(),
                    Some(dest) => format!("label {}", dest.show(types)),
                }
            )
            .unwrap();
            s
        }
    }
    impl Show for terminator::CallBr {
        fn show(&self, types: &Types) -> String {
            let mut s = String::new();
            write!(
                s,
                "{} = callbr {}(",
                &self.result.show(types),
                match &self.function {
                    either::Either::Left(_) => "<inline assembly>".into(),
                    either::Either::Right(op) => format!("{}", op.show(types)),
                }
            )
            .unwrap();
            for (i, (arg, _)) in self.arguments.iter().enumerate() {
                if i == self.arguments.len() - 1 {
                    write!(s, "{}", arg.show(types)).unwrap();
                } else {
                    write!(s, "{}, ", arg.show(types)).unwrap();
                }
            }
            write!(s, ") to label {}", &self.return_label.show(types)).unwrap();
            s
        }
    }
    impl Show for terminator::Terminator {
        fn show(&self, types: &Types) -> String {
            match self {
                terminator::Terminator::Ret(t) => t.show(types),
                terminator::Terminator::Br(t) => t.show(types),
                terminator::Terminator::CondBr(t) => t.show(types),
                terminator::Terminator::Switch(t) => t.show(types),
                terminator::Terminator::IndirectBr(t) => t.show(types),
                terminator::Terminator::Invoke(t) => t.show(types),
                terminator::Terminator::Resume(t) => t.show(types),
                terminator::Terminator::Unreachable(t) => t.show(types),
                terminator::Terminator::CleanupRet(t) => t.show(types),
                terminator::Terminator::CatchRet(t) => t.show(types),
                terminator::Terminator::CatchSwitch(t) => t.show(types),
                terminator::Terminator::CallBr(t) => t.show(types),
            }
        }
    }
}

mod predicate_show {
    use super::Show;
    use crate::llvm_ir::predicates;
    use std::fmt::Write;

    impl Show for predicates::FPPredicate {
        fn show(&self, types: &crate::llvm_ir::types::Types) -> String {
            todo!()
        }
    }

    impl Show for predicates::IntPredicate {
        fn show(&self, types: &crate::llvm_ir::types::Types) -> String {
            todo!()
        }
    }
}

mod name_show {
    use super::Show;
    use crate::llvm_ir::name;
    use crate::llvm_ir::types::Types;
    use std::fmt::Write;

    impl Show for name::Name {
        fn show(&self, types: &Types) -> String {
            let mut s = String::new();
            match self {
                name::Name::Name(name) => write!(s, "%{}", name).unwrap(),
                name::Name::Number(num) => write!(s, "%{}", num).unwrap(),
            };
            s
        }
    }
}
