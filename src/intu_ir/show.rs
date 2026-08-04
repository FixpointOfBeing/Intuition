use crate::intu_ir::types::Types;

pub trait Show {
    fn show(&self, types: &Types) -> String;
}

mod module_show {
    use crate::intu_ir::module;
    use crate::intu_ir::types::Types;
    use std::fmt::Write;

    use super::Show;

    impl Show for module::Module {
        fn show(&self, types: &Types) -> String {
            let mut parts: Vec<String> = Vec::new();

            let header = format!(
                "source_filename = \"{}\"",
                self.source_file_name
            );

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

            for gv in &self.global_vars {
                parts.push(gv.show(types));
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
            s
        }
    }
}

mod function_show {
    use crate::intu_ir::function;
    use crate::intu_ir::name::Name;
    use crate::intu_ir::types::Types;
    use std::fmt::Write;

    use super::Show;

    impl Show for function::Function {
        fn show(&self, types: &Types) -> String {
            let mut s = String::new();
            write!(s, "define ").unwrap();

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

            write!(s, ")").unwrap();

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

            write!(s, " @{}(", self.name).unwrap();
            for (i, param) in self.parameters.iter().enumerate() {
                if i > 0 {
                    write!(s, ", ").unwrap();
                }
                write!(s, "{}", param.show(types)).unwrap();
            }
            writeln!(s, ")").unwrap();
            s
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
            s
        }
    }
}

mod operand_show {
    use crate::intu_ir::operand;
    use crate::intu_ir::operand::Operand;
    use crate::intu_ir::types::Types;
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
            }
            s
        }
    }
}

mod instruction_show {
    use crate::intu_ir::instruction;
    use crate::intu_ir::types::Types;
    use std::fmt::Write;

    use super::Show;
    impl Show for instruction::FPPredicate {
        fn show(&self, _types: &Types) -> String {
            match self {
                instruction::FPPredicate::False => {
                    "false".to_string()
                },
                instruction::FPPredicate::OEQ => "oeq".to_string(),
                instruction::FPPredicate::OGT => "ogt".to_string(),
                instruction::FPPredicate::OGE => "oge".to_string(),
                instruction::FPPredicate::OLT => "olt".to_string(),
                instruction::FPPredicate::OLE => "ole".to_string(),
                instruction::FPPredicate::ONE => "one".to_string(),
                instruction::FPPredicate::ORD => "ord".to_string(),
                instruction::FPPredicate::UNO => "uno".to_string(),
                instruction::FPPredicate::UEQ => "ueq".to_string(),
                instruction::FPPredicate::UGT => "ugt".to_string(),
                instruction::FPPredicate::UGE => "uge".to_string(),
                instruction::FPPredicate::ULT => "ult".to_string(),
                instruction::FPPredicate::ULE => "ule".to_string(),
                instruction::FPPredicate::UNE => "une".to_string(),
                instruction::FPPredicate::True => "true".to_string(),
            }
        }
    }

    impl Show for instruction::IntPredicate {
        fn show(&self, _types: &Types) -> String {
            match self {
                instruction::IntPredicate::EQ => "eq".to_string(),
                instruction::IntPredicate::NE => "ne".to_string(),
                instruction::IntPredicate::UGT => "ugt".to_string(),
                instruction::IntPredicate::UGE => "uge".to_string(),
                instruction::IntPredicate::ULT => "ult".to_string(),
                instruction::IntPredicate::ULE => "ule".to_string(),
                instruction::IntPredicate::SGT => "sgt".to_string(),
                instruction::IntPredicate::SGE => "sge".to_string(),
                instruction::IntPredicate::SLT => "slt".to_string(),
                instruction::IntPredicate::SLE => "sle".to_string(),
            }
        }
    }
    impl Show for instruction::Instruction {
        fn show(&self, types: &Types) -> String {
            use instruction::Instruction;
            match self {
                Instruction::Add { operand0, operand1, dest } => {
                    let mut s = String::new();
                    let ty = types.type_of(operand0);
                    write!(s, "{} = add", dest.show(types)).unwrap();

                    write!(
                        s,
                        " {} {}, {}",
                        ty.show(types),
                        operand0.show(types),
                        operand1.show(types)
                    )
                    .unwrap();
                    s
                },
                Instruction::Sub { operand0, operand1, dest } => {
                    let mut s = String::new();
                    let ty = types.type_of(operand0);
                    write!(s, "{} = sub", dest.show(types)).unwrap();

                    write!(
                        s,
                        " {} {}, {}",
                        ty.show(types),
                        operand0.show(types),
                        operand1.show(types)
                    )
                    .unwrap();
                    s
                },
                Instruction::Mul { operand0, operand1, dest } => {
                    let mut s = String::new();
                    let ty = types.type_of(operand0);
                    write!(s, "{} = mul", dest.show(types)).unwrap();

                    write!(
                        s,
                        " {} {}, {}",
                        ty.show(types),
                        operand0.show(types),
                        operand1.show(types)
                    )
                    .unwrap();
                    s
                },
                Instruction::UDiv { operand0, operand1, dest } => {
                    let mut s = String::new();
                    let ty = types.type_of(operand0);
                    write!(s, "{} = udiv", dest.show(types)).unwrap();

                    write!(
                        s,
                        " {} {}, {}",
                        ty.show(types),
                        operand0.show(types),
                        operand1.show(types)
                    )
                    .unwrap();
                    s
                },
                Instruction::SDiv { operand0, operand1, dest } => {
                    let mut s = String::new();
                    let ty = types.type_of(operand0);
                    write!(s, "{} = sdiv", dest.show(types)).unwrap();

                    write!(
                        s,
                        " {} {}, {}",
                        ty.show(types),
                        operand0.show(types),
                        operand1.show(types)
                    )
                    .unwrap();
                    s
                },
                Instruction::URem { operand0, operand1, dest } => {
                    let mut s = String::new();
                    let ty = types.type_of(operand0);
                    write!(
                        s,
                        "{} = urem {} {}, {}",
                        dest.show(types),
                        ty.show(types),
                        operand0.show(types),
                        operand1.show(types)
                    )
                    .unwrap();
                    s
                },
                Instruction::SRem { operand0, operand1, dest } => {
                    let mut s = String::new();
                    let ty = types.type_of(operand0);
                    write!(
                        s,
                        "{} = srem {} {}, {}",
                        dest.show(types),
                        ty.show(types),
                        operand0.show(types),
                        operand1.show(types)
                    )
                    .unwrap();
                    s
                },
                Instruction::And { operand0, operand1, dest } => {
                    let mut s = String::new();
                    let ty = types.type_of(operand0);
                    write!(
                        s,
                        "{} = and {} {}, {}",
                        dest.show(types),
                        ty.show(types),
                        operand0.show(types),
                        operand1.show(types)
                    )
                    .unwrap();
                    s
                },
                Instruction::Or { operand0, operand1, dest } => {
                    let mut s = String::new();
                    let ty = types.type_of(operand0);
                    write!(s, "{} = or", dest.show(types)).unwrap();

                    write!(
                        s,
                        " {} {}, {}",
                        ty.show(types),
                        operand0.show(types),
                        operand1.show(types)
                    )
                    .unwrap();
                    s
                },
                Instruction::Xor { operand0, operand1, dest } => {
                    let mut s = String::new();
                    let ty = types.type_of(operand0);
                    write!(
                        s,
                        "{} = xor {} {}, {}",
                        dest.show(types),
                        ty.show(types),
                        operand0.show(types),
                        operand1.show(types)
                    )
                    .unwrap();
                    s
                },
                Instruction::Shl { operand0, operand1, dest } => {
                    let mut s = String::new();
                    let ty = types.type_of(operand0);
                    write!(s, "{} = shl", dest.show(types)).unwrap();

                    write!(
                        s,
                        " {} {}, {}",
                        ty.show(types),
                        operand0.show(types),
                        operand1.show(types)
                    )
                    .unwrap();
                    s
                },
                Instruction::LShr { operand0, operand1, dest } => {
                    let mut s = String::new();
                    let ty = types.type_of(operand0);
                    write!(s, "{} = lshr", dest.show(types)).unwrap();

                    write!(
                        s,
                        " {} {}, {}",
                        ty.show(types),
                        operand0.show(types),
                        operand1.show(types)
                    )
                    .unwrap();
                    s
                },
                Instruction::AShr { operand0, operand1, dest } => {
                    let mut s = String::new();
                    let ty = types.type_of(operand0);
                    write!(s, "{} = ashr", dest.show(types)).unwrap();

                    write!(
                        s,
                        " {} {}, {}",
                        ty.show(types),
                        operand0.show(types),
                        operand1.show(types)
                    )
                    .unwrap();
                    s
                },
                Instruction::FAdd { operand0, operand1, dest } => {
                    let mut s = String::new();
                    let ty = types.type_of(operand0);
                    write!(
                        s,
                        "{} = fadd {} {}, {}",
                        dest.show(types),
                        ty.show(types),
                        operand0.show(types),
                        operand1.show(types)
                    )
                    .unwrap();
                    s
                },
                Instruction::FSub { operand0, operand1, dest } => {
                    let mut s = String::new();
                    let ty = types.type_of(operand0);
                    write!(
                        s,
                        "{} = fsub {} {}, {}",
                        dest.show(types),
                        ty.show(types),
                        operand0.show(types),
                        operand1.show(types)
                    )
                    .unwrap();
                    s
                },
                Instruction::FMul { operand0, operand1, dest } => {
                    let mut s = String::new();
                    let ty = types.type_of(operand0);
                    write!(
                        s,
                        "{} = fmul {} {}, {}",
                        dest.show(types),
                        ty.show(types),
                        operand0.show(types),
                        operand1.show(types)
                    )
                    .unwrap();
                    s
                },
                Instruction::FDiv { operand0, operand1, dest } => {
                    let mut s = String::new();
                    let ty = types.type_of(operand0);
                    write!(
                        s,
                        "{} = fdiv {} {}, {}",
                        dest.show(types),
                        ty.show(types),
                        operand0.show(types),
                        operand1.show(types)
                    )
                    .unwrap();
                    s
                },
                Instruction::FRem { operand0, operand1, dest } => {
                    let mut s = String::new();
                    let ty = types.type_of(operand0);
                    write!(
                        s,
                        "{} = frem {} {}, {}",
                        dest.show(types),
                        ty.show(types),
                        operand0.show(types),
                        operand1.show(types)
                    )
                    .unwrap();
                    s
                },
                Instruction::FNeg { operand, dest } => {
                    let mut s = String::new();
                    let ty = types.type_of(operand);
                    write!(
                        s,
                        "{} = fneg {} {}",
                        dest.show(types),
                        ty.show(types),
                        operand.show(types)
                    )
                    .unwrap();
                    s
                },
                Instruction::Alloca {
                    allocated_type,
                    num_elements,
                    dest,
                    alignment,
                } => {
                    let mut s = String::new();
                    write!(
                        s,
                        "{} = alloca {}",
                        dest.show(types),
                        allocated_type.show(types)
                    )
                    .unwrap();

                    write!(
                        s,
                        ", {} {}",
                        types.type_of(num_elements).show(types),
                        num_elements.show(types)
                    )
                    .unwrap();

                    write!(s, ", align {}", alignment).unwrap();

                    s
                },
                Instruction::Load {
                    address,
                    dest,
                    loaded_ty,
                    alignment,
                } => {
                    let mut s = String::new();
                    write!(s, "{} = load ", dest.show(types))
                        .unwrap();
                    write!(
                        s,
                        "{}, {} {}",
                        loaded_ty.show(types),
                        types.type_of(address).show(types),
                        address.show(types)
                    )
                    .unwrap();

                    write!(s, ", align {}", alignment).unwrap();
                    s
                },
                Instruction::Store { address, value, alignment } => {
                    let mut s = String::new();
                    write!(s, "store ").unwrap();
                    write!(
                        s,
                        "{} {}, {} {}",
                        types.type_of(value).show(types),
                        value.show(types),
                        types.type_of(address).show(types),
                        address.show(types)
                    )
                    .unwrap();

                    write!(s, ", align {}", alignment).unwrap();
                    s
                },
                Instruction::GetElementPtr {
                    address,
                    indices,
                    dest,
                    source_element_type,
                } => {
                    let mut s = String::new();
                    write!(
                        s,
                        "{} = getelementptr ",
                        dest.show(types)
                    )
                    .unwrap();
                    write!(
                        s,
                        "{}, {} {}",
                        source_element_type.show(types),
                        types.type_of(address).show(types),
                        address.show(types)
                    )
                    .unwrap();
                    for idx in indices {
                        write!(
                            s,
                            ", {} {}",
                            types.type_of(idx).show(types),
                            idx.show(types)
                        )
                        .unwrap();
                    }
                    s
                },
                Instruction::Trunc { operand, to_type, dest } => {
                    let mut s = String::new();
                    let from_ty = types.type_of(operand);
                    write!(
                        s,
                        "{} = trunc {} {} to {}",
                        dest.show(types),
                        from_ty.show(types),
                        operand.show(types),
                        to_type.show(types)
                    )
                    .unwrap();
                    s
                },
                Instruction::ZExt { operand, to_type, dest } => {
                    let mut s = String::new();
                    let from_ty = types.type_of(operand);
                    write!(s, "{} = zext", dest.show(types)).unwrap();

                    write!(
                        s,
                        " {} {} to {}",
                        from_ty.show(types),
                        operand.show(types),
                        to_type.show(types)
                    )
                    .unwrap();
                    s
                },
                Instruction::SExt { operand, to_type, dest } => {
                    let mut s = String::new();
                    let from_ty = types.type_of(operand);
                    write!(
                        s,
                        "{} = sext {} {} to {}",
                        dest.show(types),
                        from_ty.show(types),
                        operand.show(types),
                        to_type.show(types)
                    )
                    .unwrap();
                    s
                },
                Instruction::FPTrunc { operand, to_type, dest } => {
                    let mut s = String::new();
                    let from_ty = types.type_of(operand);
                    write!(
                        s,
                        "{} = fptrunc {} {} to {}",
                        dest.show(types),
                        from_ty.show(types),
                        operand.show(types),
                        to_type.show(types)
                    )
                    .unwrap();
                    s
                },
                Instruction::FPExt { operand, to_type, dest } => {
                    let mut s = String::new();
                    let from_ty = types.type_of(operand);
                    write!(
                        s,
                        "{} = fpext {} {} to {}",
                        dest.show(types),
                        from_ty.show(types),
                        operand.show(types),
                        to_type.show(types)
                    )
                    .unwrap();
                    s
                },
                Instruction::FPToUI { operand, to_type, dest } => {
                    let mut s = String::new();
                    let from_ty = types.type_of(operand);
                    write!(
                        s,
                        "{} = fptoui {} {} to {}",
                        dest.show(types),
                        from_ty.show(types),
                        operand.show(types),
                        to_type.show(types)
                    )
                    .unwrap();
                    s
                },
                Instruction::FPToSI { operand, to_type, dest } => {
                    let mut s = String::new();
                    let from_ty = types.type_of(operand);
                    write!(
                        s,
                        "{} = fptosi {} {} to {}",
                        dest.show(types),
                        from_ty.show(types),
                        operand.show(types),
                        to_type.show(types)
                    )
                    .unwrap();
                    s
                },
                Instruction::UIToFP { operand, to_type, dest } => {
                    let mut s = String::new();
                    let from_ty = types.type_of(operand);
                    write!(
                        s,
                        "{} = uitofp {} {} to {}",
                        dest.show(types),
                        from_ty.show(types),
                        operand.show(types),
                        to_type.show(types)
                    )
                    .unwrap();
                    s
                },
                Instruction::SIToFP { operand, to_type, dest } => {
                    let mut s = String::new();
                    let from_ty = types.type_of(operand);
                    write!(
                        s,
                        "{} = sitofp {} {} to {}",
                        dest.show(types),
                        from_ty.show(types),
                        operand.show(types),
                        to_type.show(types)
                    )
                    .unwrap();
                    s
                },
                Instruction::PtrToInt { operand, to_type, dest } => {
                    let mut s = String::new();
                    let from_ty = types.type_of(operand);
                    write!(
                        s,
                        "{} = ptrtoint {} {} to {}",
                        dest.show(types),
                        from_ty.show(types),
                        operand.show(types),
                        to_type.show(types)
                    )
                    .unwrap();
                    s
                },
                Instruction::IntToPtr { operand, to_type, dest } => {
                    let mut s = String::new();
                    let from_ty = types.type_of(operand);
                    write!(
                        s,
                        "{} = inttoptr {} {} to {}",
                        dest.show(types),
                        from_ty.show(types),
                        operand.show(types),
                        to_type.show(types)
                    )
                    .unwrap();
                    s
                },
                Instruction::BitCast { operand, to_type, dest } => {
                    let mut s = String::new();
                    let from_ty = types.type_of(operand);
                    write!(
                        s,
                        "{} = bitcast {} {} to {}",
                        dest.show(types),
                        from_ty.show(types),
                        operand.show(types),
                        to_type.show(types)
                    )
                    .unwrap();
                    s
                },
                Instruction::ICmp {
                    predicate,
                    operand0,
                    operand1,
                    dest,
                } => {
                    let mut s = String::new();
                    let ty = types.type_of(operand0);
                    write!(
                        s,
                        "{} = icmp {} {} {}, {}",
                        dest.show(types),
                        predicate.show(types),
                        ty.show(types),
                        operand0.show(types),
                        operand1.show(types)
                    )
                    .unwrap();
                    s
                },
                Instruction::FCmp {
                    predicate,
                    operand0,
                    operand1,
                    dest,
                } => {
                    let mut s = String::new();
                    let ty = types.type_of(operand0);
                    write!(
                        s,
                        "{} = fcmp {} {} {}, {}",
                        dest.show(types),
                        predicate.show(types),
                        ty.show(types),
                        operand0.show(types),
                        operand1.show(types)
                    )
                    .unwrap();
                    s
                },
                Instruction::Call {
                    function,
                    function_ty,
                    arguments,
                    dest,
                    is_tail_call,
                } => {
                    let mut s = String::new();
                    if let Some(dest) = dest {
                        write!(s, "{} = ", dest.show(types)).unwrap();
                    }
                    if *is_tail_call {
                        write!(s, "tail ").unwrap();
                    }
                    write!(
                        s,
                        "call {}(",
                        format!(
                            "{} {}",
                            types.type_of(self).show(types),
                            function.show(types)
                        ),
                    )
                    .unwrap();
                    for (i, arg) in arguments.iter().enumerate() {
                        if i == arguments.len() - 1 {
                            write!(s, "{}", arg.show(types)).unwrap();
                        } else {
                            write!(s, "{}, ", arg.show(types))
                                .unwrap();
                        }
                    }
                    write!(s, ")").unwrap();
                    s
                },
                Instruction::ExtractValue {
                    aggregate,
                    indices,
                    dest,
                } => {
                    let mut s = String::new();
                    let agg_ty = types.type_of(aggregate);
                    write!(
                        s,
                        "{} = extractvalue {} {}, {}",
                        dest.show(types),
                        agg_ty.show(types),
                        aggregate.show(types),
                        indices
                            .first()
                            .expect("ExtractValue with no indices")
                    )
                    .unwrap();
                    for idx in &indices[1..] {
                        write!(s, ", {idx}").unwrap();
                    }
                    s
                },
                Instruction::InsertValue {
                    aggregate,
                    element,
                    indices,
                    dest,
                } => {
                    let mut s = String::new();
                    let agg_ty = types.type_of(aggregate);
                    write!(
                        s,
                        "{} = insertvalue {} {}, {}, {}",
                        dest.show(types),
                        agg_ty.show(types),
                        aggregate.show(types),
                        element.show(types),
                        indices
                            .first()
                            .expect("InsertValue with no indices")
                    )
                    .unwrap();
                    for idx in &indices[1..] {
                        write!(s, ", {idx}").unwrap();
                    }
                    s
                },
            }
        }
    }
}

mod types_show {
    use crate::intu_ir::types;
    use crate::intu_ir::types::{NamedStructDef, Types};
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

    impl Show for types::InstType {
        fn show(&self, types: &Types) -> String {
            match self {
                types::InstType::VoidType => "void".to_string(),
                types::InstType::IntegerType { bits } => {
                    format!("i{}", bits)
                },
                types::InstType::PointerType { .. } => {
                    "ptr".to_string()
                },
                types::InstType::FPType(fpt) => fpt.show(types),
                types::InstType::FuncType {
                    result_type,
                    param_types,
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

                    write!(s, ")").unwrap();
                    s
                },
                types::InstType::VectorType {
                    element_type,
                    num_elements,
                } => {
                    format!(
                        "<{} x {}>",
                        num_elements,
                        element_type.show(types)
                    )
                },
                types::InstType::ArrayType {
                    element_type,
                    num_elements,
                } => format!(
                    "[{} x {}]",
                    num_elements,
                    element_type.show(types)
                ),
                types::InstType::StructType { element_types } => {
                    let mut s = String::new();
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
                    s
                },
                types::InstType::NamedStructType { name } => {
                    format!("%{}", name)
                },
            }
        }
    }

    impl Show for types::FPType {
        fn show(&self, _types: &Types) -> String {
            match self {
                types::FPType::Single => "float".to_string(),
                types::FPType::Double => "double".to_string(),
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
    use crate::intu_ir::constant;
    use crate::intu_ir::name::Name;
    use crate::intu_ir::types::Types;
    use std::fmt::Write;

    use super::Show;

    impl Show for constant::Float {
        fn show(&self, _types: &Types) -> String {
            match self {
                constant::Float::Single(s) => format!("float {}", s),
                constant::Float::Double(d) => format!("double {}", d),
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
            }
            s
        }
    }
}

mod terminator_show {
    use crate::intu_ir::terminator;
    use crate::intu_ir::types::Types;
    use std::fmt::Write;

    use super::Show;

        impl Show for terminator::Terminator {
        fn show(&self, types: &Types) -> String {
            use terminator::Terminator;
            match self {
                Terminator::Ret { return_operand } => {
                    let mut s = String::new();
                    write!(s, "ret ").unwrap();
                    match return_operand {
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
                },
                Terminator::Br { dest } => {
                    format!("br label {}", dest.show(types))
                },
                Terminator::CondBr {
                    condition,
                    true_dest,
                    false_dest,
                } => {
                    let mut s = String::new();
                    write!(
                        s,
                        "br i1 {}, label {}, label {}",
                        condition.show(types),
                        true_dest.show(types),
                        false_dest.show(types)
                    )
                    .unwrap();
                    s
                },
                Terminator::IndirectBr {
                    operand,
                    possible_dests,
                } => {
                    let mut s = String::new();
                    write!(
                        s,
                        "indirectbr {}, [ label {}",
                        operand.show(types),
                        possible_dests
                            .get(0)
                            .expect(
                                "IndirectBr with no possible dests"
                            )
                            .show(types)
                    )
                    .unwrap();
                    for dest in &possible_dests[1..] {
                        write!(s, ", label {}", dest.show(types))
                            .unwrap();
                    }
                    write!(s, " ]").unwrap();
                    s
                },
                Terminator::Unreachable => "unreachable".to_string(),
            }
        }
    }
}

mod name_show {
    use super::Show;
    use crate::intu_ir::name;
    use crate::intu_ir::types::Types;
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

#[cfg(test)]
mod tests {
}
