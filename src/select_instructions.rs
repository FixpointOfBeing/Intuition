use crate::{
    explicate_control::{CAtom, CExpr, CStmt, CTail},
    gensym::Gensym,
    riscv_var::{
        basicblock::RvVarBasicBlock,
        instruction::{RvVarInstr, li, mv, seqz, snez},
        label::Label,
        location::{RvVarLocation, a0, ra, t0, t1, x0, zero},
        program::RvVarProgram,
    },
    syntax::{BinOp, UnaryOp},
};

fn bool_not(rd: RvVarLocation, rs1: RvVarLocation) -> RvVarInstr {
    RvVarInstr::Xori { rd, rs1: rs1, imm: 1 }
}

fn select_atom(
    atom: CAtom,
    instrs: &mut Vec<RvVarInstr>,
    dest: RvVarLocation,
) {
    match atom {
        CAtom::Unit => {
            let instr =
                RvVarInstr::Addi { rd: dest, rs1: zero(), imm: 0 };
            instrs.push(instr);
        },
        CAtom::Bool(b) => {
            let v = if b { 1 } else { 0 };
            let instr =
                RvVarInstr::Addi { rd: dest, rs1: zero(), imm: v };
            instrs.push(instr);
        },
        CAtom::Int(i) => instrs.append(&mut li(dest, i)),
        CAtom::Float(f) => {
            todo!()
        },
        CAtom::Var(name, _) => {
            let location = RvVarLocation::Var(name);
            let instr = mv(dest, location);

            instrs.push(instr);
        },
    };
}

fn select_expr(
    expr: CExpr,
    instrs: &mut Vec<RvVarInstr>,
    dest: RvVarLocation,
) {
    match expr {
        CExpr::Atom(catom) => {
            select_atom(catom, instrs, dest);
        },
        CExpr::BinOp(op, left, right) => match op {
            BinOp::Add => {
                select_atom(left, instrs, t0());
                select_atom(right, instrs, t1());
                let instr = RvVarInstr::Add {
                    rd: dest.clone(),
                    rs1: t0(),
                    rs2: t1(),
                };
                instrs.push(instr);
            },
            BinOp::Sub => {
                select_atom(left, instrs, t0());
                select_atom(right, instrs, t1());
                let instr = RvVarInstr::Sub {
                    rd: dest,
                    rs1: t0(),
                    rs2: t1(),
                };
                instrs.push(instr);
            },
            BinOp::Mul => {
                // 乘法溢出的处理（回绕）
                select_atom(left, instrs, t0());
                select_atom(right, instrs, t1());
                let instr = RvVarInstr::Mul {
                    rd: dest,
                    rs1: t0(),
                    rs2: t1(),
                };
                instrs.push(instr);
            },
            BinOp::Div => {
                select_atom(left, instrs, t0());
                select_atom(right, instrs, t1());
                let instr = RvVarInstr::Div {
                    rd: dest,
                    rs1: t0(),
                    rs2: t1(),
                };
                instrs.push(instr);
            },
            BinOp::And => {
                select_atom(left, instrs, t0());
                select_atom(right, instrs, t1());
                let instr = RvVarInstr::And {
                    rd: dest,
                    rs1: t0(),
                    rs2: t1(),
                };
                instrs.push(instr);
            },
            BinOp::Or => {
                select_atom(left, instrs, t0());
                select_atom(right, instrs, t1());
                let instr =
                    RvVarInstr::Or { rd: dest, rs1: t0(), rs2: t1() };
                instrs.push(instr);
            },
            BinOp::Eq => {
                select_atom(left, instrs, t0());
                select_atom(right, instrs, t1());
                let sub = RvVarInstr::Sub {
                    rd: dest.clone(),
                    rs1: t0(),
                    rs2: t1(),
                };
                let set_bool = seqz(dest.clone(), dest);
                instrs.push(sub);
                instrs.push(set_bool);
            },
            BinOp::Neq => {
                select_atom(left, instrs, t0());
                select_atom(right, instrs, t1());
                let sub = RvVarInstr::Sub {
                    rd: dest.clone(),
                    rs1: t0(),
                    rs2: t1(),
                };
                let set_bool = snez(dest.clone(), dest);
                instrs.push(sub);
                instrs.push(set_bool);
            },
            BinOp::Lt => {
                select_atom(left, instrs, t0());
                select_atom(right, instrs, t1());
                let instr = RvVarInstr::Slt {
                    rd: dest,
                    rs1: t0(),
                    rs2: t1(),
                };
                instrs.push(instr);
            },
            BinOp::Gt => {
                select_atom(left, instrs, t0());
                select_atom(right, instrs, t0());
                let instr = RvVarInstr::Slt {
                    rd: dest,
                    rs1: t1(),
                    rs2: t0(),
                };
                instrs.push(instr);
            },
            BinOp::Leq => {
                // left <= right => !(left > right)
                select_atom(left, instrs, t0());
                select_atom(right, instrs, t1());
                let right_lt_left = RvVarInstr::Slt {
                    rd: dest.clone(),
                    rs1: t1(),
                    rs2: t0(),
                };
                let reverse = bool_not(dest.clone(), dest);
                instrs.push(right_lt_left);
                instrs.push(reverse);
            },
            BinOp::Geq => {
                // left >= right => !(left < right)
                select_atom(left, instrs, t0());
                select_atom(right, instrs, t1());
                let left_lt_right = RvVarInstr::Slt {
                    rd: dest.clone(),
                    rs1: t0(),
                    rs2: t1(),
                };

                let reverse = bool_not(dest.clone(), dest);
                instrs.push(left_lt_right);
                instrs.push(reverse);
            },
        },
        CExpr::UnaryOp(op, catom) => match op {
            UnaryOp::Neg => {
                select_atom(catom, instrs, dest.clone());
                let sub = RvVarInstr::Sub {
                    rd: dest.clone(),
                    rs1: zero(),
                    rs2: dest,
                };
                instrs.push(sub);
            },
            UnaryOp::Not => {
                select_atom(catom, instrs, dest.clone());
                let reverse = bool_not(dest.clone(), dest);
                instrs.push(reverse);
            },
        },
        CExpr::Call(func, args) => todo!(),
        CExpr::MakeClosure(catom, catoms, _) => todo!(),
        CExpr::Project(catom, _, _) => todo!(),
    }
}

fn select_stmt(stmt: CStmt, instrs: &mut Vec<RvVarInstr>) {
    match stmt {
        CStmt::Assign(name, cexpr, _) => {
            let var = RvVarLocation::Var(name);
            select_expr(cexpr, instrs, var);
        },
    }
}
pub fn select_tail(
    tail: CTail,
    gensym: &mut Gensym,
    mut current: RvVarBasicBlock,
    prog: &mut RvVarProgram,
) {
    let instrs = &mut current.instrs;
    match tail {
        CTail::Return(cexpr) => {
            select_expr(cexpr, instrs, a0());
            let instr =
                RvVarInstr::Jalr { rd: x0(), rs1: ra(), imm: 0 };
            instrs.push(instr);
        },
        CTail::TailCall(func, args) => {
            // select_atom(func, instrs);
            // for arg in args {
            //     select_atom(arg, instrs);
            // }
            todo!()
        },
        CTail::Seq(cstmt, ctail) => {
            select_stmt(cstmt, instrs);
            select_tail(*ctail, gensym, current, prog)
        },
        CTail::If(catom, then_tail, else_tail) => {
            select_atom(catom, instrs, a0());

            let then_name = gensym.fresh_with_prefix(".Lthen");
            let else_name = gensym.fresh_with_prefix(".Lelse");
            let then_label = Label::new(then_name);
            let else_label = Label::new(else_name);

            let branch = RvVarInstr::Beq {
                rs1: t0(),
                rs2: zero(),
                label: else_label.clone(),
            };
            instrs.push(branch);

            prog.append_basic_block(current);

            let then_block = RvVarBasicBlock::new(then_label);
            select_tail(*then_tail, gensym, then_block, prog);

            let else_block = RvVarBasicBlock::new(else_label);
            select_tail(*else_tail, gensym, else_block, prog);
        },
    }
}
