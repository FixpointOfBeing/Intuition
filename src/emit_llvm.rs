use either::Either;
use std::collections::HashMap;
use target_lexicon::Triple;

use crate::closure_conversion::ClosFnDef;
use crate::explicate_control::{CAtom, CExpr, CStmt, CTail};
use crate::gensym::Gensym;
use crate::llvm_ir::basicblock::BasicBlock;
use crate::llvm_ir::constant::{Constant, ConstantRef, Float};
use crate::llvm_ir::function::{
    CallingConvention, Function, FunctionDeclaration, Parameter, ParameterAttribute,
};
use crate::llvm_ir::instruction::*;
use crate::llvm_ir::module::*;
use crate::llvm_ir::name::Name;
use crate::llvm_ir::operand::Operand;
use crate::llvm_ir::terminator::*;
use crate::llvm_ir::types::{LLVMType, TypeRef, Types};
use crate::syntax::{BinOp, Ident, Type, UnaryOp};

pub fn emit_module(
    body: CTail,
    return_type: Type,
    module_name: String,
    source_file_name: String,
    fn_defs: &[(ClosFnDef, CTail)],
) -> Module {
    let types = Types::new();
    let mut gensym = Gensym::new();
    let env = HashMap::new();
    let mut func_declarations = Vec::new();

    let ret_ty = llvm_type(&return_type, &types);

    let param_tys = vec![];
    let program_fn_ty = types.func_type(ret_ty.clone(), param_tys, false);

    let entry_block_name = nn("entry");

    let body_blocks = compile_tail(&body, &types, &mut gensym, &env, &mut func_declarations);

    let mut blocks = Vec::with_capacity(1 + body_blocks.len());
    let mut entry = BasicBlock::new(entry_block_name);
    if let Some(first) = body_blocks.first() {
        entry.term = Terminator::Br(Br {
            dest: first.name.clone(),
        });
    }
    blocks.push(entry);
    blocks.extend(body_blocks);

    let program_fn = Function {
        name: "program".to_string(),
        parameters: vec![],
        is_var_arg: false,
        return_type: ret_ty,
        basic_blocks: blocks,
        function_attributes: vec![],
        return_attributes: vec![],
        linkage: Linkage::External,
        visibility: Visibility::Default,
        dll_storage_class: DLLStorageClass::Default,
        calling_convention: CallingConvention::C,
        section: None,
        comdat: None,
        alignment: 0,
        garbage_collector_name: None,
        personality_function: None,
    };

    let mut lifted_fns: Vec<Function> = Vec::new();
    for (fn_def, fn_ctail) in fn_defs {
        let lfn = compile_lifted_fn(
            fn_def,
            fn_ctail,
            &types,
            &mut gensym,
            &mut func_declarations,
        );
        lifted_fns.push(lfn);
    }

    let main_fn = create_c_main(&types, &program_fn_ty, &mut gensym);

    let mut all_fns = vec![program_fn];
    all_fns.extend(lifted_fns);
    all_fns.push(main_fn);

    Module {
        name: module_name,
        source_file_name: source_file_name,
        data_layout: DataLayout::minimal(),
        target_triple: target_triple(),
        functions: all_fns,
        func_declarations,
        global_vars: vec![],
        global_aliases: vec![],
        global_ifuncs: vec![],
        inline_assembly: String::new(),
        types,
    }
}
fn target_triple() -> String {
    let host_triple = Triple::host();
    host_triple.to_string()
}
fn compile_lifted_fn(
    fn_def: &ClosFnDef,
    ctail: &CTail,
    types: &Types,
    gensym: &mut Gensym,
    func_decls: &mut Vec<FunctionDeclaration>,
) -> Function {
    let env_param = Parameter {
        name: Name::Name(Box::new(fn_def.env_param.clone())),
        ty: types.pointer(),
        attributes: vec![],
    };

    let params: Vec<Parameter> = std::iter::once(env_param)
        .chain(
            fn_def
                .params
                .iter()
                .enumerate()
                .map(|(_, (name, ty))| Parameter {
                    name: Name::Name(Box::new(name.clone())),
                    ty: llvm_type(ty, types),
                    attributes: vec![],
                }),
        )
        .collect();

    let ret_ty = llvm_type(&fn_def.return_type, types);

    let env = HashMap::new();
    let body_blocks = compile_tail(ctail, types, gensym, &env, func_decls);

    let mut blocks = Vec::with_capacity(1 + body_blocks.len());
    let mut entry = BasicBlock::new(nn("entry"));
    if let Some(first) = body_blocks.first() {
        entry.term = Terminator::Br(Br {
            dest: first.name.clone(),
        });
    }
    blocks.push(entry);
    blocks.extend(body_blocks);

    Function {
        name: fn_def.name.clone(),
        parameters: params,
        is_var_arg: false,
        return_type: ret_ty,
        basic_blocks: blocks,
        function_attributes: vec![],
        return_attributes: vec![],
        linkage: Linkage::External,
        visibility: Visibility::Default,
        dll_storage_class: DLLStorageClass::Default,
        calling_convention: CallingConvention::C,
        section: None,
        comdat: None,
        alignment: 0,
        garbage_collector_name: None,
        personality_function: None,
    }
}

fn llvm_type(ty: &Type, types: &Types) -> TypeRef {
    match ty {
        Type::Unit => types.struct_of(vec![], false),
        Type::Bool => types.bool(),
        Type::Int => types.i64(),
        Type::Float => types.double(),
        Type::Arrow(_, _) => types.pointer(),
        Type::Var(_) => panic!("type variable should have been resolved"),
    }
}

fn compile_catom(atom: &CAtom, types: &Types, env: &HashMap<Ident, Operand>) -> Operand {
    match atom {
        CAtom::Unit => {
            let unit_ty = types.struct_of(vec![], false);
            Operand::ConstantOperand(ConstantRef::new(Constant::AggregateZero(unit_ty)))
        }
        CAtom::Bool(b) => Operand::ConstantOperand(ConstantRef::new(Constant::Int {
            bits: 1,
            value: if *b { 1 } else { 0 },
        })),
        CAtom::Int(i) => Operand::ConstantOperand(ConstantRef::new(Constant::Int {
            bits: 64,
            value: *i as u64,
        })),
        CAtom::Float(f) => {
            Operand::ConstantOperand(ConstantRef::new(Constant::Float(Float::Double(*f))))
        }
        CAtom::Var(name, ty) => {
            if let Some(op) = env.get(name) {
                op.clone()
            } else {
                let ty_ref = llvm_type(ty, types);
                Operand::LocalOperand {
                    name: Name::Name(Box::new(name.clone())),
                    ty: ty_ref,
                }
            }
        }
    }
}

fn compile_cexpr(
    cexpr: &CExpr,
    types: &Types,
    gensym: &mut Gensym,
    env: &HashMap<Ident, Operand>,
    func_decls: &mut Vec<FunctionDeclaration>,
) -> (Vec<Instruction>, Operand) {
    match cexpr {
        CExpr::Atom(catom) => (vec![], compile_catom(catom, types, env)),
        CExpr::BinOp(op, left, right) => {
            let left_op = compile_catom(left, types, env);
            let right_op = compile_catom(right, types, env);
            let dest = fresh_name(gensym);
            let instr = compile_binop(op, left_op.clone(), right_op, dest.clone(), types);
            let result_ty = types.type_of(&instr);
            (
                vec![instr],
                Operand::LocalOperand {
                    name: dest,
                    ty: result_ty,
                },
            )
        }
        CExpr::UnaryOp(op, operand) => {
            let opnd = compile_catom(operand, types, env);
            let dest = fresh_name(gensym);
            let instr = compile_unaryop(op, opnd, dest.clone(), types);
            let result_ty = types.type_of(&instr);
            (
                vec![instr],
                Operand::LocalOperand {
                    name: dest,
                    ty: result_ty,
                },
            )
        }
        CExpr::Call(func, args) => {
            let (func_op, func_ty) = compile_function_operand(func, types, env, func_decls);
            let ret_ty = match func_ty.as_ref() {
                LLVMType::FuncType { result_type, .. } => result_type.clone(),
                _ => types.void(),
            };
            let dest = fresh_name(gensym);
            let call = compile_call(
                func_op,
                func_ty,
                args,
                Some(dest.clone()),
                false,
                types,
                env,
                func_decls,
            );
            (
                vec![Instruction::Call(call)],
                Operand::LocalOperand {
                    name: dest,
                    ty: ret_ty,
                },
            )
        }
        CExpr::MakeClosure(fn_ptr, captured, _closure_type) => {
            compile_make_closure(fn_ptr, captured, types, gensym, env, func_decls)
        }
        CExpr::Project(env_val, idx, field_type) => {
            compile_project(env_val, *idx, field_type, types, gensym, env)
        }
    }
}

fn compile_make_closure(
    fn_ptr: &CAtom,
    captured: &[CAtom],
    types: &Types,
    gensym: &mut Gensym,
    env: &HashMap<Ident, Operand>,
    func_decls: &mut Vec<FunctionDeclaration>,
) -> (Vec<Instruction>, Operand) {
    let mut instrs = Vec::new();

    let fn_ptr_op = compile_catom(fn_ptr, types, env);
    let fn_ptr_cast_dest = fresh_name(gensym);
    let fn_ptr_ty = types.type_of(&fn_ptr_op);
    instrs.push(Instruction::BitCast(BitCast {
        operand: fn_ptr_op,
        to_type: types.pointer(),
        dest: fn_ptr_cast_dest.clone(),
    }));

    let mut struct_fields = vec![types.pointer()];
    for cap in captured {
        let cap_op = compile_catom(cap, types, env);
        struct_fields.push(types.type_of(&cap_op));
    }
    let closure_struct_ty = types.struct_of(struct_fields, false);

    let alloca_dest = fresh_name(gensym);
    instrs.push(Instruction::Alloca(Alloca {
        allocated_type: closure_struct_ty.clone(),
        num_elements: Operand::ConstantOperand(ConstantRef::new(Constant::Int {
            bits: 64,
            value: 1,
        })),
        dest: alloca_dest.clone(),
        alignment: 8,
    }));

    let fn_field_dest = fresh_name(gensym);
    let gep0 = GetElementPtr {
        address: Operand::LocalOperand {
            name: alloca_dest.clone(),
            ty: types.pointer(),
        },
        indices: vec![
            Operand::ConstantOperand(ConstantRef::new(Constant::Int { bits: 32, value: 0 })),
            Operand::ConstantOperand(ConstantRef::new(Constant::Int { bits: 32, value: 0 })),
        ],
        dest: fn_field_dest.clone(),
        in_bounds: true,
        source_element_type: closure_struct_ty.clone(),
    };
    instrs.push(Instruction::GetElementPtr(gep0));

    instrs.push(Instruction::Store(Store {
        address: Operand::LocalOperand {
            name: fn_field_dest,
            ty: types.pointer(),
        },
        value: Operand::LocalOperand {
            name: fn_ptr_cast_dest,
            ty: types.pointer(),
        },
        volatile: false,
        atomicity: None,
        alignment: 8,
    }));

    for (i, cap) in captured.iter().enumerate() {
        let cap_op = compile_catom(cap, types, env);
        let cap_ty = types.type_of(&cap_op);

        let field_idx = (i + 1) as u64;
        let field_dest = fresh_name(gensym);
        let gep = GetElementPtr {
            address: Operand::LocalOperand {
                name: alloca_dest.clone(),
                ty: types.pointer(),
            },
            indices: vec![
                Operand::ConstantOperand(ConstantRef::new(Constant::Int { bits: 32, value: 0 })),
                Operand::ConstantOperand(ConstantRef::new(Constant::Int {
                    bits: 32,
                    value: field_idx,
                })),
            ],
            dest: field_dest.clone(),
            in_bounds: true,
            source_element_type: closure_struct_ty.clone(),
        };
        instrs.push(Instruction::GetElementPtr(gep));

        instrs.push(Instruction::Store(Store {
            address: Operand::LocalOperand {
                name: field_dest,
                ty: types.pointer(),
            },
            value: cap_op,
            volatile: false,
            atomicity: None,
            alignment: 8,
        }));
    }

    let result = Operand::LocalOperand {
        name: alloca_dest,
        ty: types.pointer(),
    };

    (instrs, result)
}

fn compile_project(
    env_val: &CAtom,
    idx: usize,
    field_type: &Type,
    types: &Types,
    gensym: &mut Gensym,
    env: &HashMap<Ident, Operand>,
) -> (Vec<Instruction>, Operand) {
    let mut instrs = Vec::new();

    let env_op = compile_catom(env_val, types, env);

    let field_llvm_ty = llvm_type(field_type, types);

    let gep_dest = fresh_name(gensym);

    let struct_ty = types.pointer();

    let gep = GetElementPtr {
        address: env_op,
        indices: vec![
            Operand::ConstantOperand(ConstantRef::new(Constant::Int { bits: 32, value: 0 })),
            Operand::ConstantOperand(ConstantRef::new(Constant::Int {
                bits: 32,
                value: idx as u64,
            })),
        ],
        dest: gep_dest.clone(),
        in_bounds: true,
        source_element_type: struct_ty,
    };
    instrs.push(Instruction::GetElementPtr(gep));

    let load_dest = fresh_name(gensym);
    instrs.push(Instruction::Load(Load {
        address: Operand::LocalOperand {
            name: gep_dest,
            ty: types.pointer(),
        },
        dest: load_dest.clone(),
        loaded_ty: field_llvm_ty.clone(),
        volatile: false,
        atomicity: None,
        alignment: 8,
    }));

    let result = Operand::LocalOperand {
        name: load_dest,
        ty: field_llvm_ty,
    };

    (instrs, result)
}

fn compile_binop(
    op: &BinOp,
    left: Operand,
    right: Operand,
    dest: Name,
    types: &Types,
) -> Instruction {
    let left_ty = types.type_of(&left);
    let is_float = matches!(left_ty.as_ref(), LLVMType::FPType(_));

    match op {
        BinOp::Add => {
            if is_float {
                Instruction::FAdd(FAdd {
                    operand0: left,
                    operand1: right,
                    dest,
                })
            } else {
                Instruction::Add(Add {
                    operand0: left,
                    operand1: right,
                    dest,
                    nuw: false,
                    nsw: false,
                })
            }
        }
        BinOp::Sub => {
            if is_float {
                Instruction::FSub(FSub {
                    operand0: left,
                    operand1: right,
                    dest,
                })
            } else {
                Instruction::Sub(Sub {
                    operand0: left,
                    operand1: right,
                    dest,
                    nuw: false,
                    nsw: false,
                })
            }
        }
        BinOp::Mul => {
            if is_float {
                Instruction::FMul(FMul {
                    operand0: left,
                    operand1: right,
                    dest,
                })
            } else {
                Instruction::Mul(Mul {
                    operand0: left,
                    operand1: right,
                    dest,
                    nuw: false,
                    nsw: false,
                })
            }
        }
        BinOp::Div => {
            if is_float {
                Instruction::FDiv(FDiv {
                    operand0: left,
                    operand1: right,
                    dest,
                })
            } else {
                Instruction::SDiv(SDiv {
                    operand0: left,
                    operand1: right,
                    dest,
                    exact: false,
                })
            }
        }
        BinOp::And => Instruction::And(And {
            operand0: left,
            operand1: right,
            dest,
        }),
        BinOp::Or => Instruction::Or(Or {
            operand0: left,
            operand1: right,
            dest,
            disjoint: false,
        }),
        BinOp::Eq => {
            if is_float {
                Instruction::FCmp(FCmp {
                    predicate: crate::llvm_ir::predicates::FPPredicate::OEQ,
                    operand0: left,
                    operand1: right,
                    dest,
                })
            } else {
                Instruction::ICmp(ICmp {
                    predicate: crate::llvm_ir::predicates::IntPredicate::EQ,
                    operand0: left,
                    operand1: right,
                    dest,
                })
            }
        }
        BinOp::Neq => {
            if is_float {
                Instruction::FCmp(FCmp {
                    predicate: crate::llvm_ir::predicates::FPPredicate::ONE,
                    operand0: left,
                    operand1: right,
                    dest,
                })
            } else {
                Instruction::ICmp(ICmp {
                    predicate: crate::llvm_ir::predicates::IntPredicate::NE,
                    operand0: left,
                    operand1: right,
                    dest,
                })
            }
        }
        BinOp::Lt => {
            if is_float {
                Instruction::FCmp(FCmp {
                    predicate: crate::llvm_ir::predicates::FPPredicate::OLT,
                    operand0: left,
                    operand1: right,
                    dest,
                })
            } else {
                Instruction::ICmp(ICmp {
                    predicate: crate::llvm_ir::predicates::IntPredicate::SLT,
                    operand0: left,
                    operand1: right,
                    dest,
                })
            }
        }
        BinOp::Gt => {
            if is_float {
                Instruction::FCmp(FCmp {
                    predicate: crate::llvm_ir::predicates::FPPredicate::OGT,
                    operand0: left,
                    operand1: right,
                    dest,
                })
            } else {
                Instruction::ICmp(ICmp {
                    predicate: crate::llvm_ir::predicates::IntPredicate::SGT,
                    operand0: left,
                    operand1: right,
                    dest,
                })
            }
        }
        BinOp::Leq => {
            if is_float {
                Instruction::FCmp(FCmp {
                    predicate: crate::llvm_ir::predicates::FPPredicate::OLE,
                    operand0: left,
                    operand1: right,
                    dest,
                })
            } else {
                Instruction::ICmp(ICmp {
                    predicate: crate::llvm_ir::predicates::IntPredicate::SLE,
                    operand0: left,
                    operand1: right,
                    dest,
                })
            }
        }
        BinOp::Geq => {
            if is_float {
                Instruction::FCmp(FCmp {
                    predicate: crate::llvm_ir::predicates::FPPredicate::OGE,
                    operand0: left,
                    operand1: right,
                    dest,
                })
            } else {
                Instruction::ICmp(ICmp {
                    predicate: crate::llvm_ir::predicates::IntPredicate::SGE,
                    operand0: left,
                    operand1: right,
                    dest,
                })
            }
        }
    }
}

fn compile_unaryop(op: &UnaryOp, operand: Operand, dest: Name, types: &Types) -> Instruction {
    let op_ty = types.type_of(&operand);
    let is_float = matches!(op_ty.as_ref(), LLVMType::FPType(_));

    match op {
        UnaryOp::Neg => {
            if is_float {
                Instruction::FNeg(FNeg { operand, dest })
            } else {
                Instruction::Sub(Sub {
                    operand0: Operand::ConstantOperand(ConstantRef::new(Constant::Int {
                        bits: 64,
                        value: 0,
                    })),
                    operand1: operand,
                    dest,
                    nuw: false,
                    nsw: false,
                })
            }
        }
        UnaryOp::Not => Instruction::Xor(Xor {
            operand0: operand,
            operand1: Operand::ConstantOperand(ConstantRef::new(Constant::Int {
                bits: 1,
                value: 1,
            })),
            dest,
        }),
    }
}

fn compile_function_operand(
    func: &CAtom,
    types: &Types,
    env: &HashMap<Ident, Operand>,
    func_decls: &mut Vec<FunctionDeclaration>,
) -> (Operand, TypeRef) {
    match func {
        CAtom::Var(name, arrow_ty) => {
            let func_ty = flatten_arrow(arrow_ty, types);
            if let Some(op) = env.get(name) {
                (op.clone(), func_ty)
            } else {
                ensure_func_decl(name, arrow_ty, types, func_decls);
                let global_ref =
                    Operand::ConstantOperand(ConstantRef::new(Constant::GlobalReference {
                        name: Name::Name(Box::new(name.clone())),
                        ty: func_ty.clone(),
                    }));
                (global_ref, func_ty)
            }
        }
        _ => panic!("function operand must be a variable with arrow type"),
    }
}

fn flatten_arrow(arrow_ty: &Type, types: &Types) -> TypeRef {
    match arrow_ty {
        Type::Arrow(param_ty, ret_ty) => {
            let mut param_tys = vec![llvm_type(param_ty, types)];
            let mut cur = ret_ty.as_ref();
            loop {
                match cur {
                    Type::Arrow(p, r) => {
                        param_tys.push(llvm_type(p, types));
                        cur = r.as_ref();
                    }
                    _ => {
                        return types.func_type(llvm_type(cur, types), param_tys, false);
                    }
                }
            }
        }
        _ => panic!("expected arrow type, got {:?}", arrow_ty),
    }
}

fn ensure_func_decl(
    name: &str,
    arrow_ty: &Type,
    types: &Types,
    func_decls: &mut Vec<FunctionDeclaration>,
) {
    if func_decls.iter().any(|d| d.name == name) {
        return;
    }
    let (param_tys, ret_ty) = extract_arrow_params_and_ret(arrow_ty, types);
    let decl = FunctionDeclaration {
        name: name.to_string(),
        parameters: param_tys
            .iter()
            .enumerate()
            .map(|(i, ty)| Parameter {
                name: Name::Number(i),
                ty: ty.clone(),
                attributes: vec![],
            })
            .collect(),
        is_var_arg: false,
        return_type: ret_ty,
        return_attributes: vec![],
        linkage: Linkage::External,
        visibility: Visibility::Default,
        dll_storage_class: DLLStorageClass::Default,
        calling_convention: CallingConvention::C,
        alignment: 0,
        garbage_collector_name: None,
    };
    func_decls.push(decl);
}

fn extract_arrow_params_and_ret(arrow_ty: &Type, types: &Types) -> (Vec<TypeRef>, TypeRef) {
    match arrow_ty {
        Type::Arrow(param_ty, ret_ty) => {
            let mut param_tys = vec![llvm_type(param_ty, types)];
            let mut cur = ret_ty.as_ref();
            loop {
                match cur {
                    Type::Arrow(p, r) => {
                        param_tys.push(llvm_type(p, types));
                        cur = r.as_ref();
                    }
                    _ => {
                        return (param_tys, llvm_type(cur, types));
                    }
                }
            }
        }
        _ => panic!("expected arrow type"),
    }
}

fn compile_call(
    func_op: Operand,
    func_ty: TypeRef,
    args: &[CAtom],
    dest: Option<Name>,
    is_tail_call: bool,
    types: &Types,
    env: &HashMap<Ident, Operand>,
    func_decls: &mut Vec<FunctionDeclaration>,
) -> Call {
    let arguments: Vec<(Operand, Vec<ParameterAttribute>)> = args
        .iter()
        .map(|arg| {
            let (instrs, op) = compile_cexpr(
                &CExpr::Atom(arg.clone()),
                types,
                &mut Gensym::new(),
                env,
                func_decls,
            );
            assert!(instrs.is_empty(), "atom should produce no instructions");
            (op, vec![])
        })
        .collect();

    Call {
        function: Either::Right(func_op),
        function_ty: func_ty,
        arguments,
        return_attributes: vec![],
        dest,
        function_attributes: vec![],
        is_tail_call,
        calling_convention: CallingConvention::C,
    }
}

fn compile_tail(
    tail: &CTail,
    types: &Types,
    gensym: &mut Gensym,
    env: &HashMap<Ident, Operand>,
    func_decls: &mut Vec<FunctionDeclaration>,
) -> Vec<BasicBlock> {
    match tail {
        CTail::Return(cexpr) => {
            let (instrs, result) = compile_cexpr(cexpr, types, gensym, env, func_decls);
            let mut bb = BasicBlock::new(block_name(gensym, "ret"));
            bb.instrs = instrs;
            bb.term = Terminator::Ret(Ret {
                return_operand: Some(result),
            });
            vec![bb]
        }
        CTail::TailCall(func, args) => {
            let (func_op, func_ty) = compile_function_operand(func, types, env, func_decls);
            let call_dest = fresh_name(gensym);
            let call = compile_call(
                func_op,
                func_ty,
                args,
                Some(call_dest.clone()),
                true,
                types,
                env,
                func_decls,
            );
            let ret_ty = match call.function_ty.as_ref() {
                LLVMType::FuncType { result_type, .. } => result_type.clone(),
                _ => types.void(),
            };
            let mut bb = BasicBlock::new(block_name(gensym, "tailcall"));
            bb.instrs = vec![Instruction::Call(call)];
            bb.term = Terminator::Ret(Ret {
                return_operand: Some(Operand::LocalOperand {
                    name: call_dest,
                    ty: ret_ty,
                }),
            });
            vec![bb]
        }
        CTail::Seq(stmt, cont) => {
            let CStmt::Assign(name, cexpr, _ty) = stmt;
            let (instrs, result) = compile_cexpr(cexpr, types, gensym, env, func_decls);
            let mut env_with = env.clone();
            env_with.insert(name.clone(), result);
            let mut blocks = compile_tail(cont, types, gensym, &env_with, func_decls);
            if blocks.is_empty() {
                let mut bb = BasicBlock::new(block_name(gensym, "seq"));
                bb.instrs = instrs;
                bb.term = Terminator::Unreachable(Unreachable {});
                vec![bb]
            } else {
                blocks[0].instrs.splice(0..0, instrs);
                blocks
            }
        }
        CTail::If(cond, thn, els) => {
            let cond_op = compile_catom(cond, types, env);

            let then_blocks = compile_tail(thn, types, gensym, env, func_decls);
            let else_blocks = compile_tail(els, types, gensym, env, func_decls);

            let then_label = then_blocks
                .first()
                .map(|b| b.name.clone())
                .unwrap_or_else(|| block_name(gensym, "then.empty"));
            let else_label = else_blocks
                .first()
                .map(|b| b.name.clone())
                .unwrap_or_else(|| block_name(gensym, "else.empty"));

            let mut entry = BasicBlock::new(block_name(gensym, "if.entry"));
            entry.term = Terminator::CondBr(CondBr {
                condition: cond_op,
                true_dest: then_label.clone(),
                false_dest: else_label.clone(),
            });

            let mut blocks = vec![entry];
            if then_blocks.is_empty() {
                let mut bb = BasicBlock::new(then_label);
                bb.term = Terminator::Unreachable(Unreachable {});
                blocks.push(bb);
            } else {
                blocks.extend(then_blocks);
            }
            if else_blocks.is_empty() {
                let mut bb = BasicBlock::new(else_label);
                bb.term = Terminator::Unreachable(Unreachable {});
                blocks.push(bb);
            } else {
                blocks.extend(else_blocks);
            }
            blocks
        }
    }
}

fn create_c_main(types: &Types, program_fn_ty: &TypeRef, gensym: &mut Gensym) -> Function {
    let i32_ty = types.i32();

    let program_op = Operand::ConstantOperand(ConstantRef::new(Constant::GlobalReference {
        name: Name::Name(Box::new("program".to_string())),
        ty: program_fn_ty.clone(),
    }));

    let call_dest = fresh_name(gensym);
    let call = Call {
        function: Either::Right(program_op),
        function_ty: program_fn_ty.clone(),
        arguments: vec![],
        return_attributes: vec![],
        dest: Some(call_dest.clone()),
        function_attributes: vec![],
        is_tail_call: false,
        calling_convention: CallingConvention::C,
    };

    let mut entry = BasicBlock::new(nn("entry"));
    entry.instrs = vec![Instruction::Call(call)];
    entry.term = Terminator::Ret(Ret {
        return_operand: Some(Operand::ConstantOperand(ConstantRef::new(Constant::Int {
            bits: 32,
            value: 0,
        }))),
    });

    Function {
        name: "main".to_string(),
        parameters: vec![],
        is_var_arg: false,
        return_type: i32_ty,
        basic_blocks: vec![entry],
        function_attributes: vec![],
        return_attributes: vec![],
        linkage: Linkage::External,
        visibility: Visibility::Default,
        dll_storage_class: DLLStorageClass::Default,
        calling_convention: CallingConvention::C,
        section: None,
        comdat: None,
        alignment: 0,
        garbage_collector_name: None,
        personality_function: None,
    }
}

fn fresh_name(gensym: &mut Gensym) -> Name {
    let name = gensym.fresh_with_prefix("tmp");
    Name::Name(Box::new(name))
}

fn block_name(gensym: &mut Gensym, prefix: &str) -> Name {
    Name::Name(Box::new(gensym.fresh_with_prefix(prefix)))
}

fn nn(s: &str) -> Name {
    Name::Name(Box::new(s.to_string()))
}

pub fn module_to_string(module: &Module) -> String {
    module.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::typechecker::typecheck;
    use lalrpop_util::lalrpop_mod;
    lalrpop_mod!(pub parser);

    fn emit_and_print(src: &str) -> String {
        let ast = parser::ExprParser::new().parse(src).unwrap();
        let (return_ty, typed_ast) = typecheck(*ast).expect("typecheck failed");
        let anf = crate::a_normal_form::anf_convert(typed_ast);
        let clos_prog = crate::closure_conversion::closure_convert(anf);
        let body_ctail = crate::explicate_control::explicate_control_convert(clos_prog.body);
        let fn_ctails: Vec<(crate::closure_conversion::ClosFnDef, _)> = clos_prog
            .fn_defs
            .iter()
            .map(|d| {
                (
                    d.clone(),
                    crate::explicate_control::explicate_control_convert(d.body.clone()),
                )
            })
            .collect();
        let module = emit_module(
            body_ctail,
            return_ty,
            "test_module".to_string(),
            "test_module.intu".to_string(),
            &fn_ctails,
        );
        module.to_string()
    }

    #[test]
    fn test_int_literal() {
        let ir = emit_and_print("42");
        println!("IR:\n{}", ir);
        assert!(ir.contains("@program"));
        assert!(ir.contains("ret i64 42"));
        assert!(ir.contains("@main"));
    }

    #[test]
    fn test_arithmetic() {
        let ir = emit_and_print("1 + 2 * 3");
        println!("IR:\n{}", ir);
        assert!(ir.contains("add"));
        assert!(ir.contains("mul"));
    }

    #[test]
    fn test_if_expr() {
        let ir = emit_and_print("if true then 1 else 2");
        println!("IR:\n{}", ir);
        assert!(ir.contains("br i1"));
    }

    #[test]
    fn test_let_expr() {
        let ir = emit_and_print("let x = 5 in x + 1");
        println!("IR:\n{}", ir);
        assert!(ir.contains("add"));
    }

    #[test]
    fn test_bool_true() {
        let ir = emit_and_print("true");
        println!("IR:\n{}", ir);
    }

    #[test]
    fn test_unit() {
        let ir = emit_and_print("()");
        println!("IR:\n{}", ir);
    }
}
