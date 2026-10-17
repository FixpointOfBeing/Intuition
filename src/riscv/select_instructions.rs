use crate::{
    BinOp, CAtom, CBlock, CDef, CExpr, CProgram, CStmt, CTail, Gensym, GlobalValue, HasType,
    PrimInput, PrimOutput, Type, UnaryOp,
    riscv::rv_var::{
        RvVarBasicBlock, RvVarFunction, RvVarGlobal, RvVarInstr, RvVarInstrPseudo, RvVarLocation,
        RvVarProgram,
        location::{a0, a1, f_arg, fa0, ft0, ft1, ra, t0, t1, x_arg, x0, zero},
    },
};

use crate::riscv::rv64imfd::Imm12;
use crate::riscv::rv64imfd::Label;
use crate::riscv::rv64imfd::Rm;
use crate::riscv::rv64imfd::Sym;

fn bool_not(rd: RvVarLocation, rs1: RvVarLocation) -> RvVarInstr {
    RvVarInstr::Xori { rd, rs1: rs1, imm: Imm12::from_i16(1) }
}

fn is_heap_ptr(ty: &Type) -> bool {
    matches!(ty, Type::Tuple(_) | Type::Arrow(_, _))
}

// runtime.c的object layout
//
// | bit 0 : '1' = 真实对象（不是 forwarding 指针）
// | bits 1–6 : 字段个数 `n`
// | bit (7+i) : `1` 表示第 `i` 个字段是指向堆对象的指针
fn closure_tag(elems: &[Type]) -> i64 {
    let n = elems.len();
    debug_assert!(n <= 63,);

    let mut ptr_bits: i64 = 0;
    for (i, ty) in elems.iter().enumerate() {
        if i == 0 {
            continue;
        }
        if is_heap_ptr(ty) {
            ptr_bits |= 1i64 << i; // 置为1
        }
    }

    1 | ((n as i64) << 1) | (ptr_bits << 7)
}

fn tuple_tag(elems: &[Type]) -> i64 {
    let n = elems.len();
    debug_assert!(n <= 63,);

    let mut ptr_bits: i64 = 0;
    for (i, ty) in elems.iter().enumerate() {
        if is_heap_ptr(ty) {
            ptr_bits |= 1i64 << i; // 置为1
        }
    }

    1 | ((n as i64) << 1) | (ptr_bits << 7)
}

fn select_atom(atom: CAtom, dest: RvVarLocation, block: &mut RvVarBasicBlock) {
    match atom {
        CAtom::Unit => {
            let add = RvVarInstr::Addi { rd: dest, rs1: zero(), imm: Imm12::from_i16(0) };
            block.append_instr(add);
        },
        CAtom::Bool(b) => {
            let v = if b { 1 } else { 0 };
            let add = RvVarInstr::Addi { rd: dest, rs1: zero(), imm: Imm12::from_i16(v as i16) };
            block.append_instr(add);
        },
        CAtom::Int(i) => {
            let li = RvVarInstrPseudo::Li { rd: dest, imm: i };
            block.append_pesudo(li);
        },
        CAtom::Float(f) => {
            let bits = f.to_bits() as i64;
            let li = RvVarInstrPseudo::Li { rd: t0(), imm: bits };
            block
                .append_pesudo(li)
                .append_instr(RvVarInstr::FmvDX { rd: dest, rs1: t0() });
        },
        CAtom::Var(name, ty) => match ty {
            Type::Float => {
                let var = RvVarLocation::FVar(name);
                let mv = RvVarInstrPseudo::FmvD { rd: dest, rs: var };
                block.append_pesudo(mv);
            },
            Type::Int | Type::Unit | Type::Bool | Type::Tuple(_) | Type::Arrow(_, _) => {
                // 函数指针和Tuple指针都放到X寄存器
                let var = RvVarLocation::XVar(name);
                let mv = RvVarInstrPseudo::Mv { rd: dest, rs: var };
                block.append_pesudo(mv);
            },
            Type::Var(_) => unreachable!(),
            Type::Dummy => unreachable!(),
        },
    };
}

fn select_expr_binop_xreg(
    op: BinOp,
    left: CAtom,
    right: CAtom,
    dest: RvVarLocation,
    block: &mut RvVarBasicBlock,
) {
    debug_assert!(matches!(dest, RvVarLocation::XVar(_) | RvVarLocation::XReg(_)));

    match op {
        BinOp::Add => {
            select_atom(left, t0(), block);
            select_atom(right, t1(), block);
            let instr = RvVarInstr::Add { rd: dest.clone(), rs1: t0(), rs2: t1() };
            block.append_instr(instr);
        },
        BinOp::Sub => {
            select_atom(left, t0(), block);
            select_atom(right, t1(), block);
            let instr = RvVarInstr::Sub { rd: dest, rs1: t0(), rs2: t1() };
            block.append_instr(instr);
        },
        BinOp::Mul => {
            // 乘法溢出的处理（回绕）
            select_atom(left, t0(), block);
            select_atom(right, t1(), block);
            let instr = RvVarInstr::Mul { rd: dest, rs1: t0(), rs2: t1() };
            block.append_instr(instr);
        },
        BinOp::Div => {
            select_atom(left, t0(), block);
            select_atom(right, t1(), block);
            let instr = RvVarInstr::Div { rd: dest, rs1: t0(), rs2: t1() };
            block.append_instr(instr);
        },
        BinOp::Eq => {
            select_atom(left, t0(), block);
            select_atom(right, t1(), block);
            let sub = RvVarInstr::Sub { rd: dest.clone(), rs1: t0(), rs2: t1() };
            let set_bool = RvVarInstrPseudo::Seqz { rd: dest.clone(), rs: dest };
            block.append_instr(sub).append_pesudo(set_bool);
        },
        BinOp::Neq => {
            select_atom(left, t0(), block);
            select_atom(right, t1(), block);
            let sub = RvVarInstr::Sub { rd: dest.clone(), rs1: t0(), rs2: t1() };
            let set_bool = RvVarInstrPseudo::Snez { rd: dest.clone(), rs: dest };
            block.append_instr(sub).append_pesudo(set_bool);
        },
        BinOp::Lt => {
            select_atom(left, t0(), block);
            select_atom(right, t1(), block);
            let instr = RvVarInstr::Slt { rd: dest, rs1: t0(), rs2: t1() };
            block.append_instr(instr);
        },
        BinOp::Gt => {
            select_atom(left, t0(), block);
            select_atom(right, t1(), block);
            let instr = RvVarInstr::Slt { rd: dest, rs1: t1(), rs2: t0() };
            block.append_instr(instr);
        },
        BinOp::Leq => {
            // left <= right => !(left > right)
            select_atom(left, t0(), block);
            select_atom(right, t1(), block);
            let right_lt_left = RvVarInstr::Slt { rd: dest.clone(), rs1: t1(), rs2: t0() };
            let reverse = bool_not(dest.clone(), dest);
            block.append_instr(right_lt_left).append_instr(reverse);
        },
        BinOp::Geq => {
            // left >= right => !(left < right)
            select_atom(left, t0(), block);
            select_atom(right, t1(), block);
            let left_lt_right = RvVarInstr::Slt { rd: dest.clone(), rs1: t0(), rs2: t1() };

            let reverse = bool_not(dest.clone(), dest);
            block.append_instr(left_lt_right).append_instr(reverse);
        },
        _ => unreachable!(),
    }
}

fn select_expr_binop_freg(
    op: BinOp,
    left: CAtom,
    right: CAtom,
    dest: RvVarLocation,
    block: &mut RvVarBasicBlock,
) {
    match op {
        BinOp::Add => {
            debug_assert!(matches!(dest, RvVarLocation::FVar(_) | RvVarLocation::FReg(_)));

            select_atom(left, ft0(), block);
            select_atom(right, ft1(), block);
            let instr = RvVarInstr::FaddD { rd: dest, rs1: ft0(), rs2: ft1(), rm: Rm::Rne };
            block.append_instr(instr);
        },
        BinOp::Sub => {
            debug_assert!(matches!(dest, RvVarLocation::FVar(_) | RvVarLocation::FReg(_)));

            select_atom(left, ft0(), block);
            select_atom(right, ft1(), block);
            let instr = RvVarInstr::FsubD { rd: dest, rs1: ft0(), rs2: ft1(), rm: Rm::Rne };
            block.append_instr(instr);
        },
        BinOp::Mul => {
            debug_assert!(matches!(dest, RvVarLocation::FVar(_) | RvVarLocation::FReg(_)));

            select_atom(left, ft0(), block);
            select_atom(right, ft1(), block);
            let instr = RvVarInstr::FmulD { rd: dest, rs1: ft0(), rs2: ft1(), rm: Rm::Rne };
            block.append_instr(instr);
        },
        BinOp::Div => {
            debug_assert!(matches!(dest, RvVarLocation::FVar(_) | RvVarLocation::FReg(_)));

            select_atom(left, ft0(), block);
            select_atom(right, ft1(), block);
            let instr = RvVarInstr::FdivD { rd: dest, rs1: ft0(), rs2: ft1(), rm: Rm::Rne };
            block.append_instr(instr);
        },
        BinOp::Eq => {
            debug_assert!(matches!(dest, RvVarLocation::XVar(_) | RvVarLocation::XReg(_)));

            select_atom(left, ft0(), block);
            select_atom(right, ft1(), block);
            let instr = RvVarInstr::FeqD { rd: dest, rs1: ft0(), rs2: ft1() };
            block.append_instr(instr);
        },
        BinOp::Neq => {
            debug_assert!(matches!(dest, RvVarLocation::XVar(_) | RvVarLocation::XReg(_)));

            select_atom(left, ft0(), block);
            select_atom(right, ft1(), block);
            let eq = RvVarInstr::FeqD { rd: dest.clone(), rs1: ft0(), rs2: ft1() };
            let reverse = bool_not(dest.clone(), dest);
            block.append_instr(eq).append_instr(reverse);
        },
        BinOp::Lt => {
            debug_assert!(matches!(dest, RvVarLocation::XVar(_) | RvVarLocation::XReg(_)));

            select_atom(left, ft0(), block);
            select_atom(right, ft1(), block);
            let instr = RvVarInstr::FltD { rd: dest, rs1: ft0(), rs2: ft1() };
            block.append_instr(instr);
        },
        BinOp::Gt => {
            debug_assert!(matches!(dest, RvVarLocation::XVar(_) | RvVarLocation::XReg(_)));

            // left > right => right < left
            select_atom(left, ft0(), block);
            select_atom(right, ft1(), block);
            let instr = RvVarInstr::FltD { rd: dest, rs1: ft1(), rs2: ft0() };
            block.append_instr(instr);
        },
        BinOp::Leq => {
            debug_assert!(matches!(dest, RvVarLocation::XVar(_) | RvVarLocation::XReg(_)));

            select_atom(left, ft0(), block);
            select_atom(right, ft1(), block);
            let instr = RvVarInstr::FleD { rd: dest, rs1: ft0(), rs2: ft1() };
            block.append_instr(instr);
        },
        BinOp::Geq => {
            debug_assert!(matches!(dest, RvVarLocation::XVar(_) | RvVarLocation::XReg(_)));

            // left >= right => right <= left
            select_atom(left, ft0(), block);
            select_atom(right, ft1(), block);
            let instr = RvVarInstr::FleD { rd: dest, rs1: ft1(), rs2: ft0() };
            block.append_instr(instr);
        },
        _ => unreachable!(),
    }
}

fn select_expr(expr: CExpr, dest: RvVarLocation, block: &mut RvVarBasicBlock) {
    match expr {
        CExpr::Atom(catom, _) => {
            select_atom(catom, dest, block);
        },
        CExpr::BinOp(op, left, right, ty) => match op {
            BinOp::And => {
                select_atom(left, t0(), block);
                select_atom(right, t1(), block);
                let and = RvVarInstr::And { rd: dest, rs1: t0(), rs2: t1() };
                block.append_instr(and);
            },
            BinOp::Or => {
                select_atom(left, t0(), block);
                select_atom(right, t1(), block);
                let or = RvVarInstr::Or { rd: dest, rs1: t0(), rs2: t1() };
                block.append_instr(or);
            },
            _ => {
                if ty.is_float_type() {
                    select_expr_binop_freg(op, left, right, dest, block);
                } else {
                    select_expr_binop_xreg(op, left, right, dest, block);
                }
            },
        },
        CExpr::UnaryOp(op, catom, ty) => match op {
            UnaryOp::Neg => {
                if ty.is_float_type() {
                    select_atom(catom, dest.clone(), block);
                    let neg = RvVarInstrPseudo::FnegD { rd: dest.clone(), rs: dest };
                    block.append_pesudo(neg);
                } else {
                    debug_assert!(ty.is_int_type());
                    select_atom(catom, dest.clone(), block);
                    let sub = RvVarInstr::Sub { rd: dest.clone(), rs1: zero(), rs2: dest };
                    block.append_instr(sub);
                }
            },
            UnaryOp::Not => {
                select_atom(catom, dest.clone(), block);
                let reverse = bool_not(dest.clone(), dest);
                block.append_instr(reverse);
            },
        },
        CExpr::Call(func, args, ty) => {
            let mut x_args = 0u8;
            let mut f_args = 0u8;
            for arg in args {
                let dest = if arg.type_of().is_float_type() {
                    let r = f_arg(f_args);
                    f_args += 1;
                    r
                } else {
                    debug_assert!(arg.type_of().is_int_type());
                    let r = x_arg(x_args);
                    x_args += 1;
                    r
                };
                select_atom(arg, dest, block);
            }
            debug_assert!(matches!(func, CAtom::Var(_, Type::Arrow(_, _))));
            select_atom(func, t0(), block);

            let jr = RvVarInstr::Jalr { rd: ra(), rs1: t0(), imm: Imm12::from_i16(0) };
            block.append_instr(jr);

            let mv = if ty.is_float_type() {
                RvVarInstrPseudo::FmvD { rd: dest, rs: fa0() }
            } else {
                RvVarInstrPseudo::Mv { rd: dest, rs: a0() }
            };
            block.append_pesudo(mv);
        },
        CExpr::PrimInput(prim, ty) => {
            let sym = match prim {
                PrimInput::ReadInt => "read_int",
                PrimInput::ReadFloat => "read_float",
            };

            let call = RvVarInstrPseudo::Call { sym: Sym::new(sym) };
            let mv = if ty.is_float_type() {
                debug_assert!(matches!(dest, RvVarLocation::FVar(_) | RvVarLocation::FReg(_)));
                RvVarInstrPseudo::Mv { rd: dest, rs: fa0() }
            } else {
                debug_assert!(matches!(dest, RvVarLocation::XVar(_) | RvVarLocation::XReg(_)));
                RvVarInstrPseudo::Mv { rd: dest, rs: a0() }
            };
            block.append_pesudo(call).append_pesudo(mv);
        },
        CExpr::TupleProj(tuple, index, ty) => {
            select_atom(tuple, t0(), block);

            let offset = (index + 1) * 8;
            let imm = Imm12::from_i16(offset as i16);

            if ty.is_float_type() {
                debug_assert!(matches!(dest, RvVarLocation::FVar(_) | RvVarLocation::FReg(_)));
                let fld = RvVarInstr::Fld { rd: dest, rs1: t0(), imm };
                block.append_instr(fld);
            } else {
                debug_assert!(matches!(dest, RvVarLocation::XVar(_) | RvVarLocation::XReg(_)));
                let ld = RvVarInstr::Ld { rd: dest, rs1: t0(), imm };
                block.append_instr(ld);
            }
        },
        CExpr::Allocate(bytes, ty) => {
            debug_assert!(is_heap_ptr(&ty));
            debug_assert!(matches!(dest, RvVarLocation::XVar(_) | RvVarLocation::XReg(_)));

            let li = RvVarInstrPseudo::Li { rd: a0(), imm: bytes as i64 };
            let call = RvVarInstrPseudo::Call { sym: Sym::new("allocate") };
            let mv = RvVarInstrPseudo::Mv { rd: dest, rs: a0() };

            block.append_pesudo(li).append_pesudo(call).append_pesudo(mv);
        },
        CExpr::AllocateClosure(bytes, _, ty) => {
            debug_assert!(is_heap_ptr(&ty));
            debug_assert!(matches!(dest, RvVarLocation::XVar(_) | RvVarLocation::XReg(_)));

            let li = RvVarInstrPseudo::Li { rd: a0(), imm: bytes as i64 };
            let call = RvVarInstrPseudo::Call { sym: Sym::new("allocate") };
            let mv = RvVarInstrPseudo::Mv { rd: dest, rs: a0() };

            block.append_pesudo(li).append_pesudo(call).append_pesudo(mv);
        },
        CExpr::FunRef(name, _, _) => {
            debug_assert!(matches!(dest, RvVarLocation::XVar(_) | RvVarLocation::XReg(_)));

            let lla = RvVarInstrPseudo::Lla { rd: dest, sym: Sym::new(name) };

            block.append_pesudo(lla);
        },
        CExpr::GlobalValue(global_value) => {
            debug_assert!(matches!(dest, RvVarLocation::XVar(_) | RvVarLocation::XReg(_)));
            let sym = match &global_value {
                GlobalValue::FreePtr => "free_ptr",
                GlobalValue::FromspaceEnd => "fromspace_end",
            };

            let la = RvVarInstrPseudo::La { rd: t0(), sym: Sym::new(sym) };
            let ld = RvVarInstr::Ld { rd: dest, rs1: t0(), imm: Imm12::from_i16(0) };

            block.append_pesudo(la).append_instr(ld);
        },
        CExpr::Global(name, ty) => {
            let la = RvVarInstrPseudo::Lla { rd: t0(), sym: Sym::new(name) };
            block.append_pesudo(la);
            if ty.is_float_type() {
                debug_assert!(matches!(dest, RvVarLocation::FVar(_) | RvVarLocation::FReg(_)));
                let fld = RvVarInstr::Fld { rd: dest, rs1: t0(), imm: Imm12::from_i16(0) };
                block.append_instr(fld);
            } else {
                debug_assert!(matches!(dest, RvVarLocation::XVar(_) | RvVarLocation::XReg(_)));
                let ld = RvVarInstr::Ld { rd: dest, rs1: t0(), imm: Imm12::from_i16(0) };
                block.append_instr(ld);
            }
        },
    }
}

fn select_stmt(stmt: CStmt, block: &mut RvVarBasicBlock) {
    match stmt {
        CStmt::Assign(name, cexpr) => {
            let var = if cexpr.type_of().is_float_type() {
                RvVarLocation::FVar(name)
            } else {
                RvVarLocation::XVar(name)
            };
            select_expr(cexpr, var, block);
        },
        CStmt::TupleSet(tuple, elem, index) => {
            let offset = (index + 1) * 8;
            let imm = Imm12::from_i16(offset as i16);

            if elem.type_of().is_float_type() {
                select_atom(elem, ft1(), block); // 先算元素，避免 t0 被浮点字面量覆盖
                select_atom(tuple, t0(), block);
                let fsd = RvVarInstr::Fsd { rs2: t1(), rs1: t0(), imm };
                block.append_instr(fsd);
            } else {
                select_atom(elem, t1(), block);
                select_atom(tuple, t0(), block);
                let sd = RvVarInstr::Sd { rs2: t1(), rs1: t0(), imm };
                block.append_instr(sd);
            }
        },
        CStmt::PrimOutput(prim, catom) => match prim {
            PrimOutput::PrintInt => {
                select_atom(catom, a0(), block);
                let sym = "print_int";
                let call = RvVarInstrPseudo::Call { sym: Sym::new(sym) };
                block.append_pesudo(call);
            },
            PrimOutput::PrintFloat => {
                select_atom(catom, fa0(), block);
                let sym = "print_float";
                let call = RvVarInstrPseudo::Call { sym: Sym::new(sym) };
                block.append_pesudo(call);
            },
            PrimOutput::PrintBool => {
                select_atom(catom, a0(), block);
                let sym = "print_bool";
                let call = RvVarInstrPseudo::Call { sym: Sym::new(sym) };
                block.append_pesudo(call);
            },
        },
        CStmt::Collect(bytes) => {
            // todo: 有问题
            let la = RvVarInstrPseudo::La { rd: t0(), sym: Sym::new("rootstack_begin") };
            let ld = RvVarInstr::Ld { rd: a0(), rs1: t0(), imm: Imm12::from_i16(0) };
            let li = RvVarInstrPseudo::Li { rd: a1(), imm: bytes as i64 };
            let call = RvVarInstrPseudo::Call { sym: Sym::new("collect") };
            block
                .append_pesudo(la)
                .append_instr(ld)
                .append_pesudo(li)
                .append_pesudo(call);
        },
    }
}

pub fn select_tail(
    tail: CTail,
    gensym: &mut Gensym,
    block: &mut RvVarBasicBlock,
    function: &mut RvVarFunction,
) {
    match tail {
        CTail::Goto(label) => {
            let j = RvVarInstrPseudo::J { label: Label::new(label) };
            block.append_pesudo(j);
            function.append_basic_block((*block).clone());
        },
        CTail::Return(cexpr) => {
            let dest = if cexpr.type_of().is_float_type() { fa0() } else { a0() };
            select_expr(cexpr, dest, block);

            let ret = RvVarInstrPseudo::Ret {};
            block.append_pesudo(ret);

            function.append_basic_block((*block).clone());
        },
        CTail::TailCall(func, args) => {
            let mut x_args = 0u8;
            let mut f_args = 0u8;
            for arg in args {
                let dest = if arg.type_of().is_float_type() {
                    let r = f_arg(f_args);
                    f_args += 1;
                    r
                } else {
                    let r = x_arg(x_args);
                    x_args += 1;
                    r
                };
                select_atom(arg, dest, block);
            }

            // func 是 CAtom::Var(name, Arrow(..))，加载函数指针
            debug_assert!(matches!(func, CAtom::Var(_, Type::Arrow(_, _))));
            select_atom(func, t0(), block);

            let jr = RvVarInstr::Jalr { rd: x0(), rs1: t0(), imm: Imm12::from_i16(0) };
            block.append_instr(jr);
            function.append_basic_block((*block).clone());
        },
        CTail::Seq(cstmts, ctail) => {
            for cstmt in cstmts {
                select_stmt(cstmt, block);
            }
            select_tail(*ctail, gensym, block, function)
        },

        CTail::TailIf(catom, then_tail, else_tail) => {
            let then_name = gensym.fresh_with_prefix(".Lthen");
            let else_name = gensym.fresh_with_prefix(".Lelse");
            let then_label = Label::new(then_name);
            let else_label = Label::new(else_name);

            select_atom(catom, t0(), block);

            let branch = RvVarInstr::Beq { rs1: t0(), rs2: zero(), label: else_label.clone() };
            block.append_instr(branch);
            function.append_basic_block((*block).clone());

            let mut then_block = RvVarBasicBlock::new(then_label);
            select_tail(*then_tail, gensym, &mut then_block, function);

            let mut else_block = RvVarBasicBlock::new(else_label);
            select_tail(*else_tail, gensym, &mut else_block, function);
        },
    }
}
pub fn select_def(def: CDef, var_prog: &mut RvVarProgram, inits: &mut Vec<(String, String, bool)>, gensym: &mut Gensym) {
    match def {
        CDef::ValDef(name, ty, body, blocks) => {
            var_prog.append_global(RvVarGlobal { name: name.clone(), ty: ty.clone() });

            // body 编译成零参 thunk，返回值落在 a0/fa0
            let thunk = gensym.fresh_with_prefix(&format!("{}_init", name));
            let entry = Label::new(thunk.clone());
            let mut f = RvVarFunction {
                fun_name: thunk.clone(),
                params: vec![],
                blocks: vec![],
                entry: entry.clone(),
            };
            let mut entry_block = RvVarBasicBlock::new(entry);
            select_tail(body, gensym, &mut entry_block, &mut f);
            for b in blocks {
                let mut bb = RvVarBasicBlock::new(Label::new(b.label));
                select_tail(b.tail, gensym, &mut bb, &mut f);
            }
            inits.push((thunk, name, ty.is_float_type()));
            var_prog.append_function(f);
        },
        /*
         * 指令选择阶段不加 prologue
         *
         *  栈帧大小此刻还不知道。
         *  prologue 里的 addi sp, sp, -N 里那个 N，取决于寄存器分配后 spill 出多少个StackSlot
         *  （allocate_registers.rs 里 location_to_reg 会把装不下的变量映射成 StackSlot { offset, size }）。
         *  分配没跑，N 就没法算，prologue 也无从写。
         * 
         *  prologue 是「物理寄存器 + ABI」层面的东西，不是指令选择。 
         *  select_instructions 的职责是把 C 层 IR 映射成带虚拟位置（XVar/FVar）的 RISC-V 指令。
         *  压 ra、存 callee-saved 寄存器、开栈帧，都是拿到物理寄存器/栈布局之后才能做的约定（calling convention）细节。
         */
        CDef::FunDef(name, params, _ret_ty, body, blocks) => {
            let params = params
                .into_iter()
                .map(|(n, t)| {
                    if t.is_float_type() { RvVarLocation::FVar(n) } else { RvVarLocation::XVar(n) }
                })
                .collect();
            let entry = Label::new(name.clone());
            let mut f =
                RvVarFunction { fun_name: name, params, blocks: vec![], entry: entry.clone() };
            let mut entry_block = RvVarBasicBlock::new(entry);
            select_tail(body, gensym, &mut entry_block, &mut f);
            for b in blocks {
                let mut bb = RvVarBasicBlock::new(Label::new(b.label));
                select_tail(b.tail, gensym, &mut bb, &mut f);
            }
            var_prog.append_function(f);
        },
    }
}
pub fn select_prog(prog: CProgram) -> RvVarProgram {
    let mut gensym = Gensym::new();
    let mut var_prog = RvVarProgram::new();
    // (thunk 符号, 全局符号, is_float)
    let mut inits: Vec<(String, String, bool)> = vec![];

    for def in prog.defs {
        select_def(def, &mut var_prog, &mut inits, &mut gensym);
    }

    // main → start：先跑全局初始化，再跑 main body
    let entry = Label::new("start".to_string());
    let mut start_fn = RvVarFunction {
        fun_name: "start".to_string(),
        params: vec![],
        blocks: vec![],
        entry: entry.clone(),
    };
    let mut entry_block = RvVarBasicBlock::new(entry);
    for (thunk, global, is_float) in inits {
        entry_block.append_pesudo(RvVarInstrPseudo::Call { sym: Sym::new(thunk) });
        entry_block.append_pesudo(RvVarInstrPseudo::Lla { rd: t0(), sym: Sym::new(global) });
        if is_float {
            entry_block.append_instr(RvVarInstr::Fsd {
                rs2: fa0(),
                rs1: t0(),
                imm: Imm12::from_i16(0),
            });
        } else {
            entry_block.append_instr(RvVarInstr::Sd {
                rs2: a0(),
                rs1: t0(),
                imm: Imm12::from_i16(0),
            });
        }
    }
    select_tail(prog.main, &mut gensym, &mut entry_block, &mut start_fn);
    for b in prog.main_blocks {
        let mut bb = RvVarBasicBlock::new(Label::new(b.label));
        select_tail(b.tail, &mut gensym, &mut bb, &mut start_fn);
    }
    var_prog.append_function(start_fn);

    var_prog
}

#[cfg(test)]
mod tests {
    use crate::syntax::{ty_bool, ty_float, ty_int};

    use super::*;

    #[derive(Debug, Clone, PartialEq)]
    enum Val {
        Int(i64),
        Float(f64),
    }

    fn sext(v: i64, bits: u32) -> i64 {
        (v << (64 - bits)) >> (64 - bits)
    }

    fn lookup(env: &[(RvVarLocation, Val)], loc: &RvVarLocation) -> Val {
        env.iter().rev().find(|(l, _)| l == loc).unwrap().1.clone()
    }

    fn lookup_int(env: &[(RvVarLocation, Val)], loc: &RvVarLocation) -> i64 {
        match lookup(env, loc) {
            Val::Int(i) => i,
            Val::Float(_) => {
                panic!("expected integer value at {loc}")
            },
        }
    }

    fn lookup_float(env: &[(RvVarLocation, Val)], loc: &RvVarLocation) -> f64 {
        match lookup(env, loc) {
            Val::Float(f) => f,
            Val::Int(_) => panic!("expected float value at {loc}"),
        }
    }

    fn store(env: &mut Vec<(RvVarLocation, Val)>, loc: &RvVarLocation, v: Val) {
        if let Some(pair) = env.iter_mut().find(|(l, _)| l == loc) {
            pair.1 = v;
        } else {
            env.push((loc.clone(), v));
        }
    }

    fn exec_instr(instr: &RvVarInstr, env: &mut Vec<(RvVarLocation, Val)>) {
        match instr {
            RvVarInstr::Addi { rd, rs1, imm } => {
                let v = lookup_int(env, rs1).wrapping_add(imm.to_i16() as i64);
                store(env, rd, Val::Int(v));
            },
            RvVarInstr::Addiw { rd, rs1, imm } => {
                let v = lookup_int(env, rs1).wrapping_add(imm.to_i16() as i64);
                store(env, rd, Val::Int(sext(v, 32)));
            },
            RvVarInstr::Slli { rd, rs1, shamt } => {
                let v = lookup_int(env, rs1) << shamt.to_u8();
                store(env, rd, Val::Int(v));
            },
            RvVarInstr::Lui { rd, imm } => {
                let v = sext((imm.to_i32() >> 12) as i64, 20) << 12;
                store(env, rd, Val::Int(v));
            },
            RvVarInstr::Add { rd, rs1, rs2 } => {
                let v = lookup_int(env, rs1).wrapping_add(lookup_int(env, rs2));
                store(env, rd, Val::Int(v));
            },
            RvVarInstr::Sub { rd, rs1, rs2 } => {
                let v = lookup_int(env, rs1).wrapping_sub(lookup_int(env, rs2));
                store(env, rd, Val::Int(v));
            },
            RvVarInstr::Mul { rd, rs1, rs2 } => {
                let v = lookup_int(env, rs1).wrapping_mul(lookup_int(env, rs2));
                store(env, rd, Val::Int(v));
            },
            RvVarInstr::Div { rd, rs1, rs2 } => {
                let a = lookup_int(env, rs1);
                let b = lookup_int(env, rs2);
                let v = if b == 0 { -1 } else { a / b };
                store(env, rd, Val::Int(v));
            },
            RvVarInstr::Slt { rd, rs1, rs2 } => {
                let v = (lookup_int(env, rs1) < lookup_int(env, rs2)) as i64;
                store(env, rd, Val::Int(v));
            },
            RvVarInstr::Sltu { rd, rs1, rs2 } => {
                let a = lookup_int(env, rs1) as u64;
                let b = lookup_int(env, rs2) as u64;
                store(env, rd, Val::Int((a < b) as i64));
            },
            RvVarInstr::Sltiu { rd, rs1, imm } => {
                let a = lookup_int(env, rs1) as u64;
                let b = imm.to_i16() as u64;
                store(env, rd, Val::Int((a < b) as i64));
            },
            RvVarInstr::Xori { rd, rs1, imm } => {
                let v = lookup_int(env, rs1) ^ (imm.to_i16() as i64);
                store(env, rd, Val::Int(v));
            },
            RvVarInstr::And { rd, rs1, rs2 } => {
                let v = lookup_int(env, rs1) & lookup_int(env, rs2);
                store(env, rd, Val::Int(v));
            },
            RvVarInstr::Or { rd, rs1, rs2 } => {
                let v = lookup_int(env, rs1) | lookup_int(env, rs2);
                store(env, rd, Val::Int(v));
            },
            RvVarInstr::FaddD { rd, rs1, rs2, .. } => {
                let v = lookup_float(env, rs1) + lookup_float(env, rs2);
                store(env, rd, Val::Float(v));
            },
            RvVarInstr::FsubD { rd, rs1, rs2, .. } => {
                let v = lookup_float(env, rs1) - lookup_float(env, rs2);
                store(env, rd, Val::Float(v));
            },
            RvVarInstr::FmulD { rd, rs1, rs2, .. } => {
                let v = lookup_float(env, rs1) * lookup_float(env, rs2);
                store(env, rd, Val::Float(v));
            },
            RvVarInstr::FdivD { rd, rs1, rs2, .. } => {
                let v = lookup_float(env, rs1) / lookup_float(env, rs2);
                store(env, rd, Val::Float(v));
            },
            RvVarInstr::FeqD { rd, rs1, rs2 } => {
                let v = (lookup_float(env, rs1) == lookup_float(env, rs2)) as i64;
                store(env, rd, Val::Int(v));
            },
            RvVarInstr::FltD { rd, rs1, rs2 } => {
                let v = (lookup_float(env, rs1) < lookup_float(env, rs2)) as i64;
                store(env, rd, Val::Int(v));
            },
            RvVarInstr::FleD { rd, rs1, rs2 } => {
                let v = (lookup_float(env, rs1) <= lookup_float(env, rs2)) as i64;
                store(env, rd, Val::Int(v));
            },
            RvVarInstr::FmvDX { rd, rs1 } => {
                let bits = lookup_int(env, rs1) as u64;
                store(env, rd, Val::Float(f64::from_bits(bits)));
            },
            RvVarInstr::FsgnjD { rd, rs1, rs2 } => {
                let a = lookup_float(env, rs1);
                let b = lookup_float(env, rs2);
                store(env, rd, Val::Float(a.abs().copysign(b)));
            },
            RvVarInstr::FsgnjnD { rd, rs1, rs2 } => {
                let a = lookup_float(env, rs1);
                let b = lookup_float(env, rs2);
                store(env, rd, Val::Float(a.abs().copysign(-b)));
            },
            _ => panic!("unexpected instruction in interpreter: {instr:?}"),
        }
    }

    fn read_result(env: &[(RvVarLocation, Val)]) -> Val {
        if env.iter().rev().any(|(l, _)| l == &fa0()) {
            lookup(env, &fa0())
        } else {
            lookup(env, &a0())
        }
    }

    fn interpret(prog: &RvVarProgram) -> Val {
        todo!()
        // let mut env: Vec<(RvVarLocation, Val)> = vec![(zero(), Val::Int(0))];
        // let mut pc = 0usize;
        // loop {
        //     let block = &prog.blocks[pc];
        //     let mut jumped = false;
        //     for instr in &block.instrs {
        //         match instr {
        //             RvVarInstr::Jalr { .. } => {
        //                 return read_result(&env);
        //             },
        //             RvVarInstr::Beq { rs1, rs2, label } => {
        //                 let taken = lookup_int(&env, rs1) == lookup_int(&env, rs2);
        //                 pc = if taken {
        //                     prog.blocks.iter().position(|b| b.name.name == label.name).unwrap()
        //                 } else {
        //                     pc + 1
        //                 };
        //                 jumped = true;
        //                 break;
        //             },
        //             _ => exec_instr(instr, &mut env),
        //         }
        //     }
        //     if !jumped {
        //         pc += 1;
        //     }
        // }
    }

    fn run_tail(tail: CTail) -> Val {
        todo!()
        // let mut gensym = Gensym::new();
        // let entry = RvVarBasicBlock::new(Label::new("entry".to_string()));
        // let mut prog = RvVarProgram::new();
        // select_tail(tail, &mut gensym, entry, &mut prog);
        // interpret(&prog)
    }

    fn c_int(i: i64) -> CAtom {
        CAtom::Int(i)
    }

    fn c_float(f: f64) -> CAtom {
        CAtom::Float(f)
    }

    fn c_bool(b: bool) -> CAtom {
        CAtom::Bool(b)
    }

    fn c_var(name: &str, ty: Type) -> CAtom {
        CAtom::Var(name.to_string(), ty)
    }

    fn c_atom(a: CAtom) -> CExpr {
        let ty = a.type_of();
        CExpr::Atom(a, ty)
    }

    fn c_binop(op: BinOp, l: CAtom, r: CAtom) -> CExpr {
        let ty = l.type_of();
        CExpr::BinOp(op, l, r, ty)
    }

    fn c_ret(e: CExpr) -> CTail {
        CTail::Return(e)
    }

    fn c_if(c: CAtom, t: CTail, e: CTail) -> CTail {
        CTail::TailIf(c, Box::new(t), Box::new(e))
    }

    fn c_seq(stmts: Vec<CStmt>, tail: CTail) -> CTail {
        CTail::Seq(stmts, Box::new(tail))
    }

    fn run_expr_int(e: CExpr) -> i64 {
        match run_tail(c_ret(e)) {
            Val::Int(i) => i,
            Val::Float(_) => panic!("expected integer result"),
        }
    }

    fn run_expr_float(e: CExpr) -> f64 {
        match run_tail(c_ret(e)) {
            Val::Float(f) => f,
            Val::Int(_) => panic!("expected float result"),
        }
    }

    // --- 整数加减乘除 ---

    #[test]
    fn int_add() {
        assert_eq!(run_expr_int(c_binop(BinOp::Add, c_int(1), c_int(2))), 3);
    }

    #[test]
    fn int_sub() {
        assert_eq!(run_expr_int(c_binop(BinOp::Sub, c_int(10), c_int(3))), 7);
    }

    #[test]
    fn int_mul() {
        assert_eq!(run_expr_int(c_binop(BinOp::Mul, c_int(6), c_int(7))), 42);
    }

    #[test]
    fn int_div() {
        assert_eq!(run_expr_int(c_binop(BinOp::Div, c_int(8), c_int(2))), 4);
    }

    #[test]
    fn int_div_truncates_toward_zero() {
        assert_eq!(run_expr_int(c_binop(BinOp::Div, c_int(7), c_int(2))), 3);
        assert_eq!(run_expr_int(c_binop(BinOp::Div, c_int(-7), c_int(2))), -3);
    }

    #[test]
    fn int_unary_neg() {
        assert_eq!(run_expr_int(CExpr::UnaryOp(UnaryOp::Neg, c_int(5), ty_int())), -5);
    }

    // --- 浮点加减乘除 ---

    #[test]
    fn float_add() {
        assert_eq!(run_expr_float(c_binop(BinOp::Add, c_float(1.5), c_float(2.5))), 4.0);
    }

    #[test]
    fn float_sub() {
        assert_eq!(run_expr_float(c_binop(BinOp::Sub, c_float(5.0), c_float(1.5))), 3.5);
    }

    #[test]
    fn float_mul() {
        assert_eq!(run_expr_float(c_binop(BinOp::Mul, c_float(2.0), c_float(3.0))), 6.0);
    }

    #[test]
    fn float_div() {
        assert_eq!(run_expr_float(c_binop(BinOp::Div, c_float(6.0), c_float(2.0))), 3.0);
    }

    #[test]
    fn float_unary_neg() {
        assert_eq!(run_expr_float(CExpr::UnaryOp(UnaryOp::Neg, c_float(1.5), ty_float())), -1.5);
    }

    // --- 整数比较 ---

    #[test]
    fn int_lt() {
        assert_eq!(run_expr_int(c_binop(BinOp::Lt, c_int(1), c_int(2))), 1);
        assert_eq!(run_expr_int(c_binop(BinOp::Lt, c_int(2), c_int(1))), 0);
    }

    #[test]
    fn int_gt() {
        assert_eq!(run_expr_int(c_binop(BinOp::Gt, c_int(2), c_int(1))), 1);
        assert_eq!(run_expr_int(c_binop(BinOp::Gt, c_int(1), c_int(2))), 0);
    }

    #[test]
    fn int_leq() {
        assert_eq!(run_expr_int(c_binop(BinOp::Leq, c_int(1), c_int(1))), 1);
        assert_eq!(run_expr_int(c_binop(BinOp::Leq, c_int(2), c_int(1))), 0);
    }

    #[test]
    fn int_geq() {
        assert_eq!(run_expr_int(c_binop(BinOp::Geq, c_int(2), c_int(2))), 1);
        assert_eq!(run_expr_int(c_binop(BinOp::Geq, c_int(1), c_int(2))), 0);
    }

    #[test]
    fn int_eq() {
        assert_eq!(run_expr_int(c_binop(BinOp::Eq, c_int(1), c_int(1))), 1);
        assert_eq!(run_expr_int(c_binop(BinOp::Eq, c_int(1), c_int(2))), 0);
    }

    #[test]
    fn int_neq() {
        assert_eq!(run_expr_int(c_binop(BinOp::Neq, c_int(1), c_int(2))), 1);
        assert_eq!(run_expr_int(c_binop(BinOp::Neq, c_int(1), c_int(1))), 0);
    }

    // --- 浮点比较 ---

    #[test]
    fn float_lt() {
        assert_eq!(run_expr_int(c_binop(BinOp::Lt, c_float(1.0), c_float(2.0))), 1);
        assert_eq!(run_expr_int(c_binop(BinOp::Lt, c_float(2.0), c_float(1.0))), 0);
    }

    #[test]
    fn float_gt() {
        assert_eq!(run_expr_int(c_binop(BinOp::Gt, c_float(2.0), c_float(1.0))), 1);
        assert_eq!(run_expr_int(c_binop(BinOp::Gt, c_float(1.0), c_float(2.0))), 0);
    }

    #[test]
    fn float_leq() {
        assert_eq!(run_expr_int(c_binop(BinOp::Leq, c_float(1.0), c_float(1.0))), 1);
        assert_eq!(run_expr_int(c_binop(BinOp::Leq, c_float(2.0), c_float(1.0))), 0);
    }

    #[test]
    fn float_geq() {
        assert_eq!(run_expr_int(c_binop(BinOp::Geq, c_float(2.0), c_float(2.0))), 1);
        assert_eq!(run_expr_int(c_binop(BinOp::Geq, c_float(1.0), c_float(2.0))), 0);
    }

    #[test]
    fn float_eq() {
        assert_eq!(run_expr_int(c_binop(BinOp::Eq, c_float(1.0), c_float(1.0))), 1);
        assert_eq!(run_expr_int(c_binop(BinOp::Eq, c_float(1.0), c_float(2.0))), 0);
    }

    // --- 逻辑运算 ---

    #[test]
    fn logic_and() {
        assert_eq!(run_expr_int(c_binop(BinOp::And, c_bool(true), c_bool(false))), 0);
        assert_eq!(run_expr_int(c_binop(BinOp::And, c_bool(true), c_bool(true))), 1);
    }

    #[test]
    fn logic_or() {
        assert_eq!(run_expr_int(c_binop(BinOp::Or, c_bool(false), c_bool(true))), 1);
        assert_eq!(run_expr_int(c_binop(BinOp::Or, c_bool(false), c_bool(false))), 0);
    }

    #[test]
    fn logic_not() {
        assert_eq!(run_expr_int(CExpr::UnaryOp(UnaryOp::Not, c_bool(true), ty_bool())), 0);
        assert_eq!(run_expr_int(CExpr::UnaryOp(UnaryOp::Not, c_bool(false), ty_bool())), 1);
    }

    // --- if ---

    #[test]
    fn if_true_branch() {
        let tail = c_if(c_bool(true), c_ret(c_atom(c_int(1))), c_ret(c_atom(c_int(2))));
        assert_eq!(run_tail(tail), Val::Int(1));
    }

    #[test]
    fn if_false_branch() {
        let tail = c_if(c_bool(false), c_ret(c_atom(c_int(1))), c_ret(c_atom(c_int(2))));
        assert_eq!(run_tail(tail), Val::Int(2));
    }

    #[test]
    fn if_float_result() {
        let tail = c_if(c_bool(true), c_ret(c_atom(c_float(1.5))), c_ret(c_atom(c_float(2.5))));
        assert_eq!(run_tail(tail), Val::Float(1.5));
    }

    // x > 0 ? x : -x  （等价于绝对值）
    fn abs_via_if(x: i64) -> Val {
        let tail = c_seq(
            vec![
                CStmt::Assign("x".to_string(), c_atom(c_int(x)) /*Type::Int*/),
                CStmt::Assign(
                    "c".to_string(),
                    c_binop(BinOp::Gt, c_var("x", Type::Int), c_int(0)),
                    // Type::Bool,
                ),
            ],
            c_if(
                c_var("c", Type::Bool),
                c_ret(c_atom(c_var("x", Type::Int))),
                c_ret(CExpr::UnaryOp(UnaryOp::Neg, c_var("x", Type::Int), ty_int())),
            ),
        );
        run_tail(tail)
    }

    #[test]
    fn if_computed_condition() {
        assert_eq!(abs_via_if(5), Val::Int(5));
        assert_eq!(abs_via_if(-5), Val::Int(5));
        assert_eq!(abs_via_if(0), Val::Int(0));
    }

    // --- return ---

    #[test]
    fn return_int() {
        assert_eq!(run_tail(c_ret(c_atom(c_int(42)))), Val::Int(42));
    }

    #[test]
    fn return_bool() {
        assert_eq!(run_tail(c_ret(c_atom(c_bool(true)))), Val::Int(1));
    }

    #[test]
    fn return_float() {
        assert_eq!(run_tail(c_ret(c_atom(c_float(3.14)))), Val::Float(3.14));
    }

    #[test]
    fn return_composite_expr() {
        // (1 + 2) * 3 = 9
        let tail = c_seq(
            vec![
                CStmt::Assign(
                    "a".to_string(),
                    c_binop(BinOp::Add, c_int(1), c_int(2)), /*Type::Int*/
                ),
                CStmt::Assign(
                    "b".to_string(),
                    c_binop(BinOp::Mul, c_var("a", Type::Int), c_int(3)),
                    // Type::Int,
                ),
            ],
            c_ret(c_atom(c_var("b", Type::Int))),
        );
        assert_eq!(run_tail(tail), Val::Int(9));
    }
}
