use crate::llvm_ir::types::Types;

pub trait Show {
    fn show(&self, types: &Types) -> String;
}

mod module_show {
    use crate::llvm_ir::module;
    use crate::llvm_ir::module::{
        DLLStorageClass, Linkage, ThreadLocalMode, Visibility,
    };
    use crate::llvm_ir::types::Types;
    use std::fmt::Write;

    use super::Show;

    impl Show for module::Module {
        fn show(&self, types: &Types) -> String {
            let mut parts: Vec<String> = Vec::new();

            let mut header = format!(
                "source_filename = \"{}\"",
                self.source_file_name
            );
            if !self.data_layout.layout_str.is_empty() {
                header.push_str(&format!(
                    "\ntarget datalayout = \"{}\"",
                    self.data_layout.show(types)
                ));
            }
            if !self.target_triple.is_empty() {
                header.push_str(&format!(
                    "\ntarget triple = \"{}\"",
                    self.target_triple
                ));
            }
            parts.push(header);

            let struct_names: Vec<String> =
                self.types.all_struct_names().cloned().collect();
            for name in &struct_names {
                if let Some(def) = self.types.named_struct_def(name) {
                    parts.push(format!(
                        "%{} = {}",
                        name,
                        def.show(types)
                    ));
                }
            }

            if !self.inline_assembly.is_empty() {
                parts.push(format!(
                    "module asm \"{}\"",
                    self.inline_assembly
                ));
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
                write!(s, "{} ", self.visibility.show(types))
                    .unwrap();
            }
            if self.dll_storage_class != DLLStorageClass::Default {
                write!(s, "{} ", self.dll_storage_class.show(types))
                    .unwrap();
            }
            if self.thread_local_mode
                != ThreadLocalMode::NotThreadLocal
            {
                write!(s, "{} ", self.thread_local_mode.show(types))
                    .unwrap();
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
                if self.is_constant { "constant" } else { "global" }
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
                write!(s, "{} ", self.visibility.show(types))
                    .unwrap();
            }
            if self.dll_storage_class != DLLStorageClass::Default {
                write!(s, "{} ", self.dll_storage_class.show(types))
                    .unwrap();
            }
            if self.thread_local_mode
                != ThreadLocalMode::NotThreadLocal
            {
                write!(s, "{} ", self.thread_local_mode.show(types))
                    .unwrap();
            }
            if let Some(ref unnamed_addr) = self.unnamed_addr {
                write!(s, "{} ", unnamed_addr.show(types)).unwrap();
            }
            write!(s, "alias {}", self.ty.show(types)).unwrap();
            if self.addr_space != 0 {
                write!(s, ", addrspace({})", self.addr_space)
                    .unwrap();
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
                write!(s, "{} ", self.visibility.show(types))
                    .unwrap();
            }
            write!(
                s,
                "ifunc {}, {}",
                self.ty.show(types),
                self.resolver_fn.show(types)
            )
            .unwrap();

            s
        }
    }

    impl Show for module::UnnamedAddr {
        fn show(&self, _types: &Types) -> String {
            match self {
                module::UnnamedAddr::Local => {
                    "local_unnamed_addr".to_string()
                },
                module::UnnamedAddr::Global => {
                    "unnamed_addr".to_string()
                },
            }
        }
    }

    impl Show for module::Linkage {
        fn show(&self, _types: &Types) -> String {
            match self {
                Linkage::Private => "private".to_string(),
                Linkage::Internal => "internal".to_string(),
                Linkage::External => String::new(),
                Linkage::ExternalWeak => "extern_weak".to_string(),
                Linkage::AvailableExternally => {
                    "available_externally".to_string()
                },
                Linkage::LinkOnceAny => "linkonce".to_string(),
                Linkage::LinkOnceODR => "linkonce_odr".to_string(),
                Linkage::LinkOnceODRAutoHide => {
                    "linkonce_odr auto_hide".to_string()
                },
                Linkage::WeakAny => "weak".to_string(),
                Linkage::WeakODR => "weak_odr".to_string(),
                Linkage::Common => "common".to_string(),
                Linkage::Appending => "appending".to_string(),
                Linkage::DLLImport => "dllimport".to_string(),
                Linkage::DLLExport => "dllexport".to_string(),
                Linkage::Ghost => "ghost".to_string(),
                Linkage::LinkerPrivate => {
                    "linker_private".to_string()
                },
                Linkage::LinkerPrivateWeak => {
                    "linker_private_weak".to_string()
                },
            }
        }
    }

    impl Show for module::Visibility {
        fn show(&self, _types: &Types) -> String {
            match self {
                Visibility::Default => String::new(),
                Visibility::Hidden => "hidden".to_string(),
                Visibility::Protected => "protected".to_string(),
            }
        }
    }

    impl Show for module::DLLStorageClass {
        fn show(&self, _types: &Types) -> String {
            match self {
                DLLStorageClass::Default => String::new(),
                DLLStorageClass::Import => "dllimport".to_string(),
                DLLStorageClass::Export => "dllexport".to_string(),
            }
        }
    }

    impl Show for module::ThreadLocalMode {
        fn show(&self, _types: &Types) -> String {
            match self {
                module::ThreadLocalMode::NotThreadLocal => {
                    String::new()
                },
                module::ThreadLocalMode::GeneralDynamic => {
                    "thread_local".to_string()
                },
                module::ThreadLocalMode::LocalDynamic => {
                    "thread_local(localdynamic)".to_string()
                },
                module::ThreadLocalMode::InitialExec => {
                    "thread_local(initialexec)".to_string()
                },
                module::ThreadLocalMode::LocalExec => {
                    "thread_local(localexec)".to_string()
                },
            }
        }
    }

    impl Show for module::Comdat {
        fn show(&self, _types: &Types) -> String {
            format!("comdat({})", self.name)
        }
    }

    impl Show for module::SelectionKind {
        fn show(&self, _types: &Types) -> String {
            match self {
                module::SelectionKind::Any => "any".to_string(),
                module::SelectionKind::ExactMatch => {
                    "exactmatch".to_string()
                },
                module::SelectionKind::Largest => {
                    "largest".to_string()
                },
                module::SelectionKind::NoDuplicates => {
                    "noduplicates".to_string()
                },
                module::SelectionKind::SameSize => {
                    "samesize".to_string()
                },
            }
        }
    }

    impl Show for module::DataLayout {
        fn show(&self, _types: &Types) -> String {
            self.layout_str.clone()
        }
    }

    impl Show for module::Endianness {
        fn show(&self, _types: &Types) -> String {
            match self {
                module::Endianness::LittleEndian => {
                    "little".to_string()
                },
                module::Endianness::BigEndian => "big".to_string(),
            }
        }
    }

    impl Show for module::Mangling {
        fn show(&self, _types: &Types) -> String {
            match self {
                module::Mangling::ELF => "e".to_string(),
                module::Mangling::MIPS => "m".to_string(),
                module::Mangling::MachO => "o".to_string(),
                module::Mangling::WindowsX86COFF => "w".to_string(),
                module::Mangling::WindowsCOFF => "x".to_string(),
                module::Mangling::XCOFF => "y".to_string(),
            }
        }
    }
}

mod function_show {
    use crate::llvm_ir::function;
    use crate::llvm_ir::function::{
        CallingConvention, FunctionAttribute,
    };
    use crate::llvm_ir::module::{
        DLLStorageClass, Linkage, Visibility,
    };
    use crate::llvm_ir::name::Name;
    use crate::llvm_ir::types::Types;
    use std::fmt::Write;

    use super::Show;

    impl Show for function::Function {
        fn show(&self, types: &Types) -> String {
            let mut s = String::new();
            for attr in &self.function_attributes {
                match attr {
                    FunctionAttribute::UnknownAttribute => {},
                    _ => write!(s, "{} ", attr.show(types)).unwrap(),
                }
            }
            write!(s, "define ").unwrap();
            if self.linkage != Linkage::External {
                write!(s, "{} ", self.linkage.show(types)).unwrap();
            }
            if self.visibility != Visibility::Default {
                write!(s, "{} ", self.visibility.show(types))
                    .unwrap();
            }
            if self.dll_storage_class != DLLStorageClass::Default {
                write!(s, "{} ", self.dll_storage_class.show(types))
                    .unwrap();
            }
            if self.calling_convention != CallingConvention::C {
                write!(s, "{} ", self.calling_convention.show(types))
                    .unwrap();
            }
            write!(
                s,
                "{} @{}(",
                self.return_type.show(types),
                self.name
            )
            .unwrap();
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
                write!(s, " personality {}", pers.show(types))
                    .unwrap();
            }
            writeln!(s, " {{").unwrap();
            for bb in &self.basic_blocks {
                match &bb.name {
                    Name::Name(name) => {
                        write!(s, "{}:\n", name).unwrap()
                    },
                    Name::Number(num) => {
                        write!(s, "{}:\n", num).unwrap()
                    },
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
                write!(s, "{}", self.return_type.show(types))
                    .unwrap();
            } else {
                write!(s, "{}", self.return_type.show(types))
                    .unwrap();
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
            match self {
                function::ParameterAttribute::ZeroExt => "zeroext".to_string(),
                function::ParameterAttribute::SignExt => "signext".to_string(),
                function::ParameterAttribute::InReg => "inreg".to_string(),
                function::ParameterAttribute::ByVal(ty) => format!("byval({})", ty.show(types)),
                function::ParameterAttribute::Preallocated(ty) => {
                    format!("preallocated({})", ty.show(types))
                },
                function::ParameterAttribute::InAlloca(ty) => {
                    format!("inalloca({})", ty.show(types))
                },
                function::ParameterAttribute::SRet(ty) => format!("sret({})", ty.show(types)),
                function::ParameterAttribute::Alignment(n) => format!("align {}", n),
                function::ParameterAttribute::NoAlias => "noalias".to_string(),
                function::ParameterAttribute::NoCapture => "nocapture".to_string(),
                function::ParameterAttribute::NoFree => "nofree".to_string(),
                function::ParameterAttribute::Nest => "nest".to_string(),
                function::ParameterAttribute::Returned => "returned".to_string(),
                function::ParameterAttribute::NonNull => "nonnull".to_string(),
                function::ParameterAttribute::Dereferenceable(n) => {
                    format!("dereferenceable({})", n)
                },
                function::ParameterAttribute::DereferenceableOrNull(n) => {
                    format!("dereferenceable_or_null({})", n)
                },
                function::ParameterAttribute::SwiftSelf => "swiftself".to_string(),
                function::ParameterAttribute::SwiftError => "swifterror".to_string(),
                function::ParameterAttribute::ImmArg => "immarg".to_string(),
                function::ParameterAttribute::NoUndef => "noundef".to_string(),
                function::ParameterAttribute::StringAttribute { kind, value } => {
                    if value.is_empty() {
                        format!("\"{}\"", kind)
                    } else {
                        format!("\"{}\"=\"{}\"", kind, value)
                    }
                },
                function::ParameterAttribute::UnknownAttribute => String::new(),
                function::ParameterAttribute::UnknownTypeAttribute(ty) => ty.show(types),
            }
        }
    }

    impl Show for function::Parameter {
        fn show(&self, types: &Types) -> String {
            let mut s = String::new();
            write!(
                s,
                "{} {}",
                self.ty.show(types),
                self.name.show(types)
            )
            .unwrap();
            for attr in &self.attributes {
                write!(s, " {}", attr.show(types)).unwrap();
            }
            s
        }
    }

    impl Show for function::FunctionAttribute {
        fn show(&self, _types: &Types) -> String {
            use function::MemoryEffect;

            match self {
                function::FunctionAttribute::AlignStack(n) => format!("alignstack({})", n),
                function::FunctionAttribute::AllocSize { elt_size, num_elts } => {
                    if let Some(n) = num_elts {
                        format!("allocsize({}, {})", elt_size, n)
                    } else {
                        format!("allocsize({})", elt_size)
                    }
                },
                function::FunctionAttribute::AlwaysInline => "alwaysinline".to_string(),
                function::FunctionAttribute::Builtin => "builtin".to_string(),
                function::FunctionAttribute::Cold => "cold".to_string(),
                function::FunctionAttribute::Convergent => "convergent".to_string(),
                function::FunctionAttribute::InaccessibleMemOnly => {
                    "inaccessiblememonly".to_string()
                },
                function::FunctionAttribute::InaccessibleMemOrArgMemOnly => {
                    "inaccessiblemem_or_argmemonly".to_string()
                },
                function::FunctionAttribute::InlineHint => "inlinehint".to_string(),
                function::FunctionAttribute::JumpTable => "jumptable".to_string(),
                function::FunctionAttribute::MinimizeSize => "minsize".to_string(),
                function::FunctionAttribute::Naked => "naked".to_string(),
                function::FunctionAttribute::NoBuiltin => "nobuiltin".to_string(),
                function::FunctionAttribute::NoCFCheck => "nocf_check".to_string(),
                function::FunctionAttribute::NoDuplicate => "noduplicate".to_string(),
                function::FunctionAttribute::NoFree => "nofree".to_string(),
                function::FunctionAttribute::NoImplicitFloat => "noimplicitfloat".to_string(),
                function::FunctionAttribute::NoInline => "noinline".to_string(),
                function::FunctionAttribute::NoMerge => "nomerge".to_string(),
                function::FunctionAttribute::NonLazyBind => "nonlazybind".to_string(),
                function::FunctionAttribute::NoRedZone => "noredzone".to_string(),
                function::FunctionAttribute::NoReturn => "noreturn".to_string(),
                function::FunctionAttribute::NoRecurse => "norecurse".to_string(),
                function::FunctionAttribute::WillReturn => "willreturn".to_string(),
                function::FunctionAttribute::ReturnsTwice => "returns_twice".to_string(),
                function::FunctionAttribute::NoSync => "nosync".to_string(),
                function::FunctionAttribute::NoUnwind => "nounwind".to_string(),
                function::FunctionAttribute::NullPointerIsValid => {
                    "null_pointer_is_valid".to_string()
                },
                function::FunctionAttribute::OptForFuzzing => "optforfuzzing".to_string(),
                function::FunctionAttribute::OptNone => "optnone".to_string(),
                function::FunctionAttribute::OptSize => "optsize".to_string(),
                function::FunctionAttribute::ReadNone => "readnone".to_string(),
                function::FunctionAttribute::ReadOnly => "readonly".to_string(),
                function::FunctionAttribute::WriteOnly => "writeonly".to_string(),
                function::FunctionAttribute::ArgMemOnly => "argmemonly".to_string(),
                function::FunctionAttribute::SafeStack => "safestack".to_string(),
                function::FunctionAttribute::SanitizeAddress => "sanitize_address".to_string(),
                function::FunctionAttribute::SanitizeMemory => "sanitize_memory".to_string(),
                function::FunctionAttribute::SanitizeThread => "sanitize_thread".to_string(),
                function::FunctionAttribute::SanitizeHWAddress => "sanitize_hwaddress".to_string(),
                function::FunctionAttribute::SanitizeMemTag => "sanitize_memtag".to_string(),
                function::FunctionAttribute::ShadowCallStack => "shadowcallstack".to_string(),
                function::FunctionAttribute::SpeculativeLoadHardening => {
                    "speculative_load_hardening".to_string()
                },
                function::FunctionAttribute::Speculatable => "speculatable".to_string(),
                function::FunctionAttribute::StackProtect => "ssp".to_string(),
                function::FunctionAttribute::StackProtectReq => "sspreq".to_string(),
                function::FunctionAttribute::StackProtectStrong => "sspstrong".to_string(),
                function::FunctionAttribute::StrictFP => "strictfp".to_string(),
                function::FunctionAttribute::UWTable => "uwtable".to_string(),
                function::FunctionAttribute::Memory {
                    default,
                    argmem,
                    inaccessible_mem,
                } => {
                    let mut s = String::new();
                    write!(s, "memory(").unwrap();
                    let mut first = true;
                    let write_part =
                        |w: &mut String, first: &mut bool, pre: &str, me: &MemoryEffect| match me {
                            MemoryEffect::None => {},
                            MemoryEffect::Read => {
                                if !*first {
                                    write!(w, ", ").unwrap();
                                }
                                write!(w, "{}read", pre).unwrap();
                                *first = false;
                            },
                            MemoryEffect::Write => {
                                if !*first {
                                    write!(w, ", ").unwrap();
                                }
                                write!(w, "{}write", pre).unwrap();
                                *first = false;
                            },
                            MemoryEffect::ReadWrite => {
                                if !*first {
                                    write!(w, ", ").unwrap();
                                }
                                write!(w, "{}readwrite", pre).unwrap();
                                *first = false;
                            },
                        };
                    write_part(&mut s, &mut first, "", default);
                    write_part(&mut s, &mut first, "argmem: ", argmem);
                    write_part(&mut s, &mut first, "inaccessiblemem: ", inaccessible_mem);
                    write!(s, ")").unwrap();
                    s
                },
                function::FunctionAttribute::StringAttribute { kind, value } => {
                    if value.is_empty() {
                        format!("\"{}\"", kind)
                    } else {
                        format!("\"{}\"=\"{}\"", kind, value)
                    }
                },
                function::FunctionAttribute::UnknownAttribute => String::new(),
            }
        }
    }

    impl Show for function::CallingConvention {
        fn show(&self, _types: &Types) -> String {
            match self {
                CallingConvention::C => "ccc".to_string(),
                CallingConvention::Fast => "fastcc".to_string(),
                CallingConvention::Cold => "coldcc".to_string(),
                CallingConvention::GHC => "ghccc".to_string(),
                CallingConvention::HiPE => "cc 11".to_string(),
                CallingConvention::WebKit_JS => {
                    "webkit_jscc".to_string()
                },
                CallingConvention::AnyReg => "anyregcc".to_string(),
                CallingConvention::PreserveMost => {
                    "preserve_mostcc".to_string()
                },
                CallingConvention::PreserveAll => {
                    "preserve_allcc".to_string()
                },
                CallingConvention::Swift => "swiftcc".to_string(),
                CallingConvention::CXX_FastTLS => {
                    "cxx_fast_tlscc".to_string()
                },
                CallingConvention::X86_StdCall => {
                    "x86_stdcallcc".to_string()
                },
                CallingConvention::X86_FastCall => {
                    "x86_fastcallcc".to_string()
                },
                CallingConvention::X86_RegCall => {
                    "x86_regcallcc".to_string()
                },
                CallingConvention::X86_ThisCall => {
                    "x86_thiscallcc".to_string()
                },
                CallingConvention::X86_VectorCall => {
                    "x86_vectorcallcc".to_string()
                },
                CallingConvention::X86_Intr => {
                    "x86_intrcc".to_string()
                },
                CallingConvention::X86_64_SysV => {
                    "x86_64_sysvcc".to_string()
                },
                CallingConvention::ARM_APCS => {
                    "arm_apcscc".to_string()
                },
                CallingConvention::ARM_AAPCS => {
                    "arm_aapcscc".to_string()
                },
                CallingConvention::ARM_AAPCS_VFP => {
                    "arm_aapcs_vfpcc".to_string()
                },
                CallingConvention::MSP430_INTR => {
                    "msp430_intrcc".to_string()
                },
                CallingConvention::MSP430_Builtin => {
                    "msp430_builtincc".to_string()
                },
                CallingConvention::PTX_Kernel => {
                    "ptx_kernel".to_string()
                },
                CallingConvention::PTX_Device => {
                    "ptx_device".to_string()
                },
                CallingConvention::SPIR_FUNC => {
                    "spir_func".to_string()
                },
                CallingConvention::SPIR_KERNEL => {
                    "spir_kernel".to_string()
                },
                CallingConvention::Intel_OCL_BI => {
                    "intel_ocl_bicc".to_string()
                },
                CallingConvention::Win64 => "win64cc".to_string(),
                CallingConvention::HHVM => "hhvmcc".to_string(),
                CallingConvention::HHVM_C => "hhvm_c".to_string(),
                CallingConvention::AVR_Intr => {
                    "avr_intrcc".to_string()
                },
                CallingConvention::AVR_Signal => {
                    "avr_signcc".to_string()
                },
                CallingConvention::AVR_Builtin => {
                    "avr_builtincc".to_string()
                },
                CallingConvention::AMDGPU_CS => {
                    "amdgpu_cs".to_string()
                },
                CallingConvention::AMDGPU_ES => {
                    "amdgpu_es".to_string()
                },
                CallingConvention::AMDGPU_GS => {
                    "amdgpu_gs".to_string()
                },
                CallingConvention::AMDGPU_HS => {
                    "amdgpu_hs".to_string()
                },
                CallingConvention::AMDGPU_LS => {
                    "amdgpu_ls".to_string()
                },
                CallingConvention::AMDGPU_PS => {
                    "amdgpu_ps".to_string()
                },
                CallingConvention::AMDGPU_VS => {
                    "amdgpu_vs".to_string()
                },
                CallingConvention::AMDGPU_Kernel => {
                    "amdgpu_kernel".to_string()
                },
                CallingConvention::Numbered(n) => format!("cc {}", n),
            }
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
                Operand::LocalOperand { name, ty: _ } => {
                    write!(s, "{}", name.show(types)).unwrap()
                },
                Operand::ConstantOperand(cref) => {
                    write!(s, "{}", cref.show(types)).unwrap()
                },
                Operand::MetadataOperand => {
                    write!(s, "<metadata>").unwrap()
                },
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
                self.indices
                    .first()
                    .expect("ExtractValue with no indices")
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
                self.indices
                    .first()
                    .expect("InsertValue with no indices")
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
            if let Some(Constant::Int { value: 1, .. }) =
                self.num_elements.as_constant()
            {
            } else {
                write!(s, ", {}", &self.num_elements.show(types))
                    .unwrap();
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
            write!(
                s,
                "{}, {}",
                &self.loaded_ty.show(types),
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
            write!(s, "fence {}", &self.atomicity.show(types))
                .unwrap();
            s
        }
    }

    impl Show for instruction::CmpXchg {
        fn show(&self, types: &Types) -> String {
            let mut s = String::new();
            write!(s, "{} = cmpxchg ", &self.dest.show(types))
                .unwrap();
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
            write!(s, "{} = atomicrmw ", &self.dest.show(types))
                .unwrap();
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
            write!(s, "{} = getelementptr ", &self.dest.show(types))
                .unwrap();
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
                write!(
                    s,
                    ", [ {}, {} ]",
                    val.show(types),
                    label.show(types)
                )
                .unwrap();
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
                    either::Either::Left(_) =>
                        "<inline assembly>".into(),
                    either::Either::Right(op) => format!(
                        "{} {}",
                        types.type_of(self).show(types),
                        op.show(types)
                    ),
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
            let mut s = String::new();
            use instruction::SynchronizationScope;
            match self.synch_scope {
                SynchronizationScope::SingleThread => {
                    write!(s, "syncscope(\"singlethread\") ")
                        .unwrap();
                },
                SynchronizationScope::System => {},
            }
            write!(s, "{}", self.mem_ordering.show(types)).unwrap();
            s
        }
    }

    impl Show for instruction::RMWBinOp {
        fn show(&self, _types: &Types) -> String {
            match self {
                instruction::RMWBinOp::Xchg => "xchg".to_string(),
                instruction::RMWBinOp::Add => "add".to_string(),
                instruction::RMWBinOp::Sub => "sub".to_string(),
                instruction::RMWBinOp::And => "and".to_string(),
                instruction::RMWBinOp::Nand => "nand".to_string(),
                instruction::RMWBinOp::Or => "or".to_string(),
                instruction::RMWBinOp::Xor => "xor".to_string(),
                instruction::RMWBinOp::Max => "max".to_string(),
                instruction::RMWBinOp::Min => "min".to_string(),
                instruction::RMWBinOp::UMax => "umax".to_string(),
                instruction::RMWBinOp::UMin => "umin".to_string(),
                instruction::RMWBinOp::FAdd => "fadd".to_string(),
                instruction::RMWBinOp::FSub => "fsub".to_string(),
                instruction::RMWBinOp::FMax => "fmax".to_string(),
                instruction::RMWBinOp::FMin => "fmin".to_string(),
                instruction::RMWBinOp::UIncWrap => {
                    "uinc_wrap".to_string()
                },
                instruction::RMWBinOp::UDecWrap => {
                    "udec_wrap".to_string()
                },
            }
        }
    }

    impl Show for instruction::MemoryOrdering {
        fn show(&self, _types: &Types) -> String {
            match self {
                instruction::MemoryOrdering::Unordered => "unordered".to_string(),
                instruction::MemoryOrdering::Monotonic => "monotonic".to_string(),
                instruction::MemoryOrdering::Acquire => "acquire".to_string(),
                instruction::MemoryOrdering::Release => "release".to_string(),
                instruction::MemoryOrdering::AcquireRelease => "acq_rel".to_string(),
                instruction::MemoryOrdering::SequentiallyConsistent => "seq_cst".to_string(),
                instruction::MemoryOrdering::NotAtomic => "not_atomic".to_string(),
            }
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
                instruction::Instruction::ExtractElement(i) => {
                    i.show(types)
                },
                instruction::Instruction::InsertElement(i) => {
                    i.show(types)
                },
                instruction::Instruction::ShuffleVector(i) => {
                    i.show(types)
                },
                instruction::Instruction::ExtractValue(i) => {
                    i.show(types)
                },
                instruction::Instruction::InsertValue(i) => {
                    i.show(types)
                },
                instruction::Instruction::Alloca(i) => i.show(types),
                instruction::Instruction::Load(i) => i.show(types),
                instruction::Instruction::Store(i) => i.show(types),
                instruction::Instruction::Fence(i) => i.show(types),
                instruction::Instruction::CmpXchg(i) => i.show(types),
                instruction::Instruction::AtomicRMW(i) => {
                    i.show(types)
                },
                instruction::Instruction::GetElementPtr(i) => {
                    i.show(types)
                },
                instruction::Instruction::Trunc(i) => i.show(types),
                instruction::Instruction::ZExt(i) => i.show(types),
                instruction::Instruction::SExt(i) => i.show(types),
                instruction::Instruction::FPTrunc(i) => i.show(types),
                instruction::Instruction::FPExt(i) => i.show(types),
                instruction::Instruction::FPToUI(i) => i.show(types),
                instruction::Instruction::FPToSI(i) => i.show(types),
                instruction::Instruction::UIToFP(i) => i.show(types),
                instruction::Instruction::SIToFP(i) => i.show(types),
                instruction::Instruction::PtrToInt(i) => {
                    i.show(types)
                },
                instruction::Instruction::IntToPtr(i) => {
                    i.show(types)
                },
                instruction::Instruction::BitCast(i) => i.show(types),
                instruction::Instruction::AddrSpaceCast(i) => {
                    i.show(types)
                },
                instruction::Instruction::ICmp(i) => i.show(types),
                instruction::Instruction::FCmp(i) => i.show(types),
                instruction::Instruction::Phi(i) => i.show(types),
                instruction::Instruction::Select(i) => i.show(types),
                instruction::Instruction::Freeze(i) => i.show(types),
                instruction::Instruction::Call(i) => i.show(types),
                instruction::Instruction::VAArg(i) => i.show(types),
                instruction::Instruction::LandingPad(i) => {
                    i.show(types)
                },
                instruction::Instruction::CatchPad(i) => {
                    i.show(types)
                },
                instruction::Instruction::CleanupPad(i) => {
                    i.show(types)
                },
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
                NamedStructDef::Opaque => {
                    write!(s, "type opaque").unwrap()
                },
                NamedStructDef::Defined(ty) => {
                    write!(s, "type {}", ty.show(types)).unwrap()
                },
            };
            s
        }
    }

    impl Show for types::LLVMType {
        fn show(&self, types: &Types) -> String {
            match self {
                types::LLVMType::VoidType => "void".to_string(),
                types::LLVMType::IntegerType { bits } => {
                    format!("i{}", bits)
                },
                types::LLVMType::PointerType { .. } => {
                    "ptr".to_string()
                },
                types::LLVMType::FPType(fpt) => fpt.show(types),
                types::LLVMType::FuncType {
                    result_type,
                    param_types,
                    is_var_arg,
                } => {
                    let mut s = String::new();
                    write!(s, "{} (", result_type.show(types))
                        .unwrap();
                    for (i, param_ty) in
                        param_types.iter().enumerate()
                    {
                        if i == param_types.len() - 1 {
                            write!(s, "{}", param_ty.show(types))
                                .unwrap();
                        } else {
                            write!(s, "{}, ", param_ty.show(types))
                                .unwrap();
                        }
                    }
                    if *is_var_arg {
                        write!(s, ", ...").unwrap();
                    }
                    write!(s, ")").unwrap();
                    s
                },
                types::LLVMType::VectorType {
                    element_type,
                    num_elements,
                    scalable,
                } => {
                    if *scalable {
                        format!(
                            "<vscale x {} x {}>",
                            num_elements,
                            element_type.show(types)
                        )
                    } else {
                        format!(
                            "<{} x {}>",
                            num_elements,
                            element_type.show(types)
                        )
                    }
                },
                types::LLVMType::ArrayType {
                    element_type,
                    num_elements,
                } => format!(
                    "[{} x {}]",
                    num_elements,
                    element_type.show(types)
                ),
                types::LLVMType::StructType {
                    element_types,
                    is_packed,
                } => {
                    let mut s = String::new();
                    if *is_packed {
                        write!(s, "<").unwrap();
                    }
                    write!(s, "{{ ").unwrap();
                    for (i, element_ty) in
                        element_types.iter().enumerate()
                    {
                        if i == element_types.len() - 1 {
                            write!(s, "{}", element_ty.show(types))
                                .unwrap();
                        } else {
                            write!(s, "{}, ", element_ty.show(types))
                                .unwrap();
                        }
                    }
                    write!(s, " }}").unwrap();
                    if *is_packed {
                        write!(s, ">").unwrap();
                    }
                    s
                },
                types::LLVMType::NamedStructType { name } => {
                    format!("%{}", name)
                },
                types::LLVMType::X86_MMXType => "x86_mmx".to_string(),
                types::LLVMType::X86_AMXType => "x86_amx".to_string(),
                types::LLVMType::MetadataType => {
                    "metadata".to_string()
                },
                types::LLVMType::LabelType => "label".to_string(),
                types::LLVMType::TokenType => "token".to_string(),
                types::LLVMType::TargetExtType => {
                    "target()".to_string()
                },
            }
        }
    }

    impl Show for types::FPType {
        fn show(&self, _types: &Types) -> String {
            match self {
                types::FPType::Half => "half".to_string(),
                types::FPType::BFloat => "bfloat".to_string(),
                types::FPType::Single => "float".to_string(),
                types::FPType::Double => "double".to_string(),
                types::FPType::FP128 => "fp128".to_string(),
                types::FPType::X86_FP80 => "x86_fp80".to_string(),
                types::FPType::PPC_FP128 => "ppc_fp128".to_string(),
            }
        }
    }

    impl Show for types::TypeRef {
        fn show(&self, types: &Types) -> String {
            self.as_ref().show(types)
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
            format!(
                "add ({}, {})",
                &self.operand0.show(types),
                &self.operand1.show(types)
            )
        }
    }

    impl Show for constant::Sub {
        fn show(&self, types: &Types) -> String {
            format!(
                "sub ({}, {})",
                &self.operand0.show(types),
                &self.operand1.show(types)
            )
        }
    }

    impl Show for constant::Mul {
        fn show(&self, types: &Types) -> String {
            format!(
                "mul ({}, {})",
                &self.operand0.show(types),
                &self.operand1.show(types)
            )
        }
    }

    impl Show for constant::Xor {
        fn show(&self, types: &Types) -> String {
            format!(
                "xor ({}, {})",
                &self.operand0.show(types),
                &self.operand1.show(types)
            )
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
            format!(
                "extractelement ({}, {})",
                &self.vector.show(types),
                &self.index.show(types)
            )
        }
    }

    impl Show for constant::InsertElement {
        fn show(&self, types: &Types) -> String {
            format!(
                "insertelement ({}, {}, {})",
                &self.vector.show(types),
                &self.element.show(types),
                &self.index.show(types)
            )
        }
    }

    impl Show for constant::ShuffleVector {
        fn show(&self, types: &Types) -> String {
            format!(
                "shufflevector ({}, {}, {})",
                &self.operand0.show(types),
                &self.operand1.show(types),
                &self.mask.show(types)
            )
        }
    }

    impl Show for constant::GetElementPtr {
        fn show(&self, types: &Types) -> String {
            let mut s = String::new();
            write!(s, "getelementptr").unwrap();
            if self.in_bounds {
                write!(s, " inbounds").unwrap();
            }
            write!(s, " ({}", &self.address.show(types)).unwrap();
            for idx in &self.indices {
                write!(s, ", {}", idx.show(types)).unwrap();
            }
            write!(s, ")").unwrap();
            s
        }
    }

    impl Show for constant::Trunc {
        fn show(&self, types: &Types) -> String {
            format!(
                "trunc ({} to {})",
                &self.operand.show(types),
                &self.to_type.show(types)
            )
        }
    }

    impl Show for constant::PtrToInt {
        fn show(&self, types: &Types) -> String {
            format!(
                "ptrtoint ({} to {})",
                &self.operand.show(types),
                &self.to_type.show(types)
            )
        }
    }

    impl Show for constant::IntToPtr {
        fn show(&self, types: &Types) -> String {
            format!(
                "inttoptr ({} to {})",
                &self.operand.show(types),
                &self.to_type.show(types)
            )
        }
    }

    impl Show for constant::BitCast {
        fn show(&self, types: &Types) -> String {
            format!(
                "bitcast ({} to {})",
                &self.operand.show(types),
                &self.to_type.show(types)
            )
        }
    }

    impl Show for constant::AddrSpaceCast {
        fn show(&self, types: &Types) -> String {
            format!(
                "addrspacecast ({} to {})",
                &self.operand.show(types),
                &self.to_type.show(types)
            )
        }
    }

    impl Show for constant::ConstantRef {
        fn show(&self, types: &Types) -> String {
            self.as_ref().show(types)
        }
    }

    impl Show for constant::Float {
        fn show(&self, _types: &Types) -> String {
            match self {
                constant::Float::Half => "half".to_string(),
                constant::Float::BFloat => "bfloat".to_string(),
                constant::Float::Single(s) => format!("float {}", s),
                constant::Float::Double(d) => format!("double {}", d),
                constant::Float::Quadruple => "quadruple".to_string(),
                constant::Float::X86_FP80 => "x86_fp80".to_string(),
                constant::Float::PPC_FP128 => "ppc_fp128".to_string(),
            }
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
                            16 => write!(
                                s,
                                "{}",
                                (*value & 0xFFFF) as i16
                            )
                            .unwrap(),
                            32 => write!(
                                s,
                                "{}",
                                (*value & 0xFFFF_FFFF) as i32
                            )
                            .unwrap(),
                            64 => write!(s, "{}", *value as i64)
                                .unwrap(),
                            _ => write!(s, "{}", value).unwrap(),
                        }
                    }
                },
                constant::Constant::Float(f) => {
                    write!(s, "{}", f.show(types)).unwrap()
                },
                constant::Constant::Null(_) => {
                    write!(s, "null").unwrap()
                },
                constant::Constant::AggregateZero(_) => {
                    write!(s, "zeroinitializer").unwrap()
                },
                constant::Constant::Undef(_) => {
                    write!(s, "undef").unwrap()
                },
                constant::Constant::Poison(_) => {
                    write!(s, "poison").unwrap()
                },
                constant::Constant::BlockAddress => {
                    write!(s, "blockaddr").unwrap()
                },
                constant::Constant::GlobalReference {
                    name,
                    ty: _,
                } => match name {
                    Name::Name(n) => write!(s, "@{}", n).unwrap(),
                    Name::Number(n) => write!(s, "@{}", n).unwrap(),
                },
                constant::Constant::TokenNone => {
                    write!(s, "none").unwrap()
                },
                constant::Constant::Struct {
                    name: _,
                    values,
                    is_packed,
                } => {
                    if *is_packed {
                        write!(s, "<").unwrap();
                    }
                    write!(s, "{{ ").unwrap();
                    for (i, val) in values.iter().enumerate() {
                        if i == values.len() - 1 {
                            write!(s, "{}", val.show(types)).unwrap();
                        } else {
                            write!(s, "{}, ", val.show(types))
                                .unwrap();
                        }
                    }
                    write!(s, " }}").unwrap();
                    if *is_packed {
                        write!(s, ">").unwrap();
                    }
                },
                constant::Constant::Array {
                    element_type: _,
                    elements,
                } => {
                    write!(s, "[ ").unwrap();
                    for (i, elt) in elements.iter().enumerate() {
                        if i == elements.len() - 1 {
                            write!(s, "{}", elt.show(types)).unwrap();
                        } else {
                            write!(s, "{}, ", elt.show(types))
                                .unwrap();
                        }
                    }
                    write!(s, " ]").unwrap();
                },
                constant::Constant::Vector(constant_refs) => {
                    write!(s, "< ").unwrap();
                    for (i, elt) in constant_refs.iter().enumerate() {
                        if i == constant_refs.len() - 1 {
                            write!(s, "{}", elt.show(types)).unwrap();
                        } else {
                            write!(s, "{}, ", elt.show(types))
                                .unwrap();
                        }
                    }
                    write!(s, " >").unwrap();
                },
                constant::Constant::PtrAuth {
                    ptr,
                    key,
                    disc,
                    addr_disc,
                } => write!(
                    s,
                    "ptrauth({}, {}, {}, {})",
                    ptr.show(types),
                    key.show(types),
                    disc.show(types),
                    addr_disc.show(types)
                )
                .unwrap(),
                constant::Constant::Add(add) => {
                    write!(s, "{}", add.show(types)).unwrap()
                },
                constant::Constant::Sub(sub) => {
                    write!(s, "{}", sub.show(types)).unwrap()
                },
                constant::Constant::Mul(mul) => {
                    write!(s, "{}", mul.show(types)).unwrap()
                },
                constant::Constant::Xor(xor) => {
                    write!(s, "{}", xor.show(types)).unwrap()
                },
                constant::Constant::ExtractElement(
                    extract_element,
                ) => write!(s, "{}", extract_element.show(types))
                    .unwrap(),
                constant::Constant::InsertElement(insert_element) => {
                    write!(s, "{}", insert_element.show(types))
                        .unwrap()
                },
                constant::Constant::ShuffleVector(shuffle_vector) => {
                    write!(s, "{}", shuffle_vector.show(types))
                        .unwrap()
                },
                constant::Constant::GetElementPtr(
                    get_element_ptr,
                ) => write!(s, "{}", get_element_ptr.show(types))
                    .unwrap(),
                constant::Constant::Trunc(trunc) => {
                    write!(s, "{}", trunc.show(types)).unwrap()
                },
                constant::Constant::PtrToInt(ptr_to_int) => {
                    write!(s, "{}", ptr_to_int.show(types)).unwrap()
                },
                constant::Constant::IntToPtr(int_to_ptr) => {
                    write!(s, "{}", int_to_ptr.show(types)).unwrap()
                },
                constant::Constant::BitCast(bit_cast) => {
                    write!(s, "{}", bit_cast.show(types)).unwrap()
                },
                constant::Constant::AddrSpaceCast(
                    addr_space_cast,
                ) => write!(s, "{}", addr_space_cast.show(types))
                    .unwrap(),
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
                Some(op) => write!(
                    s,
                    "{} {}",
                    types.type_of(op).show(types),
                    op.show(types)
                )
                .unwrap(),
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
                write!(
                    s,
                    "{}, label {}; ",
                    val.show(types),
                    label.show(types)
                )
                .unwrap();
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
                    either::Either::Left(_) =>
                        "<inline assembly>".into(),
                    either::Either::Right(op) =>
                        format!("{}", op.show(types)),
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
                Some(dest) => {
                    write!(s, "label {}", dest.show(types)).unwrap()
                },
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
                    Some(dest) =>
                        format!("label {}", dest.show(types)),
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
                    either::Either::Left(_) =>
                        "<inline assembly>".into(),
                    either::Either::Right(op) =>
                        format!("{}", op.show(types)),
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
                ") to label {}",
                &self.return_label.show(types)
            )
            .unwrap();
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
                terminator::Terminator::IndirectBr(t) => {
                    t.show(types)
                },
                terminator::Terminator::Invoke(t) => t.show(types),
                terminator::Terminator::Resume(t) => t.show(types),
                terminator::Terminator::Unreachable(t) => {
                    t.show(types)
                },
                terminator::Terminator::CleanupRet(t) => {
                    t.show(types)
                },
                terminator::Terminator::CatchRet(t) => t.show(types),
                terminator::Terminator::CatchSwitch(t) => {
                    t.show(types)
                },
                terminator::Terminator::CallBr(t) => t.show(types),
            }
        }
    }
}

mod predicate_show {
    use super::Show;
    use crate::llvm_ir::predicates;

    impl Show for predicates::FPPredicate {
        fn show(
            &self,
            _types: &crate::llvm_ir::types::Types,
        ) -> String {
            match self {
                predicates::FPPredicate::False => "false".to_string(),
                predicates::FPPredicate::OEQ => "oeq".to_string(),
                predicates::FPPredicate::OGT => "ogt".to_string(),
                predicates::FPPredicate::OGE => "oge".to_string(),
                predicates::FPPredicate::OLT => "olt".to_string(),
                predicates::FPPredicate::OLE => "ole".to_string(),
                predicates::FPPredicate::ONE => "one".to_string(),
                predicates::FPPredicate::ORD => "ord".to_string(),
                predicates::FPPredicate::UNO => "uno".to_string(),
                predicates::FPPredicate::UEQ => "ueq".to_string(),
                predicates::FPPredicate::UGT => "ugt".to_string(),
                predicates::FPPredicate::UGE => "uge".to_string(),
                predicates::FPPredicate::ULT => "ult".to_string(),
                predicates::FPPredicate::ULE => "ule".to_string(),
                predicates::FPPredicate::UNE => "une".to_string(),
                predicates::FPPredicate::True => "true".to_string(),
            }
        }
    }

    impl Show for predicates::IntPredicate {
        fn show(
            &self,
            _types: &crate::llvm_ir::types::Types,
        ) -> String {
            match self {
                predicates::IntPredicate::EQ => "eq".to_string(),
                predicates::IntPredicate::NE => "ne".to_string(),
                predicates::IntPredicate::UGT => "ugt".to_string(),
                predicates::IntPredicate::UGE => "uge".to_string(),
                predicates::IntPredicate::ULT => "ult".to_string(),
                predicates::IntPredicate::ULE => "ule".to_string(),
                predicates::IntPredicate::SGT => "sgt".to_string(),
                predicates::IntPredicate::SGE => "sge".to_string(),
                predicates::IntPredicate::SLT => "slt".to_string(),
                predicates::IntPredicate::SLE => "sle".to_string(),
            }
        }
    }
}

mod name_show {
    use super::Show;
    use crate::llvm_ir::name;
    use crate::llvm_ir::types::Types;
    use std::fmt::Write;

    impl Show for name::Name {
        fn show(&self, _types: &Types) -> String {
            let mut s = String::new();
            match self {
                name::Name::Name(name) => {
                    write!(s, "%{}", name).unwrap()
                },
                name::Name::Number(num) => {
                    write!(s, "%{}", num).unwrap()
                },
            };
            s
        }
    }
}
