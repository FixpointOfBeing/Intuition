use crate::{
    explicate_control::{CAtom, CExpr, CStmt, CTail},
    gensym::Gensym,
    syntax::{BinOp, Ident, UnaryOp},
};

#[derive(Debug, Clone, PartialEq)]
pub enum RvReg {
    Zero,
    Ra,
    Sp,
    Gp,
    Tp,
    T0,
    T1,
    T2,
    T3,
    T4,
    T5,
    T6,
    S0,
    S1,
    S2,
    S3,
    S4,
    S5,
    S6,
    S7,
    S8,
    S9,
    S10,
    S11,
    A0,
    A1,
    A2,
    A3,
    A4,
    A5,
    A6,
    A7,
}

#[derive(Debug, Clone, PartialEq)]
pub enum RvArg {
    Reg(RvReg),
    Var(Ident),
    Imm(i64),
    Mem(RvReg, i64),
}

#[derive(Debug, Clone, PartialEq)]
pub enum RvInstr {
    Add(RvArg, RvArg, RvArg),
    Sub(RvArg, RvArg, RvArg),
    Mul(RvArg, RvArg, RvArg),
    Div(RvArg, RvArg, RvArg),
    And(RvArg, RvArg, RvArg),
    Or(RvArg, RvArg, RvArg),
    Xor(RvArg, RvArg, RvArg),
    Slt(RvArg, RvArg, RvArg),

    Seqz(RvArg, RvArg),
    Snez(RvArg, RvArg),

    Mv(RvArg, RvArg),
    Li(RvArg, i64),
    Call(Ident),
    Tail(Ident),
    Ret,

    Load(RvArg, RvArg, i64), // rd, base, offset
    Store(RvArg, RvArg, i64),

    Beqz(RvArg, Ident),
    Bnez(RvArg, Ident),
    J(Ident),
    Jalr(RvArg),
    Label(Ident),
}

pub struct Block {
    instrs: Vec<RvInstr>,
}

fn select_atom(atom: &CAtom) -> RvArg {
    match atom {
        CAtom::Unit => RvArg::Imm(0),
        CAtom::Bool(b) => RvArg::Imm(if *b { 1 } else { 0 }),
        CAtom::Int(i) => RvArg::Imm(*i),
        CAtom::Float(f) => todo!(),
        CAtom::Var(name) => RvArg::Var(name.into()),
    }
}

fn select_expr(dest: RvArg, e: &CExpr, out: &mut Vec<RvInstr>) {
    match e {
        CExpr::Atom(a) => match select_atom(&a) {
            RvArg::Imm(n) => out.push(RvInstr::Li(dest, n)),
            other => out.push(RvInstr::Mv(dest, other)),
        },
        CExpr::BinOp(op, l, r) => {
            let l = select_atom(&l);
            let r = select_atom(&r);
            match op {
                BinOp::Add => out.push(RvInstr::Add(dest, l, r)),
                BinOp::Sub => out.push(RvInstr::Sub(dest, l, r)),
                BinOp::Mul => out.push(RvInstr::Mul(dest, l, r)),
                BinOp::Div => out.push(RvInstr::Div(dest, l, r)),
                BinOp::And => out.push(RvInstr::And(dest, l, r)),
                BinOp::Or => out.push(RvInstr::Or(dest, l, r)),
                BinOp::Lt => out.push(RvInstr::Slt(dest, l, r)),
                BinOp::Gt => out.push(RvInstr::Slt(dest, r, l)),
                BinOp::Eq => {
                    out.push(RvInstr::Xor(dest.clone(), l, r));
                    out.push(RvInstr::Seqz(dest.clone(), dest));
                }
                BinOp::Neq => {
                    out.push(RvInstr::Xor(dest.clone(), l, r));
                    out.push(RvInstr::Snez(dest.clone(), dest));
                }
                BinOp::Leq => {
                    // l<=r <=> !(r<l)
                    out.push(RvInstr::Slt(dest.clone(), r, l));
                    out.push(RvInstr::Xor(dest.clone(), dest, RvArg::Imm(1)));
                }
                BinOp::Geq => {
                    // l>=r <=> !(l<r)
                    out.push(RvInstr::Slt(dest.clone(), l, r));
                    out.push(RvInstr::Xor(dest.clone(), dest, RvArg::Imm(1)));
                }
            }
        }
        CExpr::UnaryOp(op, a) => {
            let a = select_atom(&a);
            match op {
                UnaryOp::Neg => out.push(RvInstr::Sub(dest, RvArg::Reg(RvReg::Zero), a)),
                UnaryOp::Not => out.push(RvInstr::Xor(dest, a, RvArg::Imm(1))),
            }
        }
        CExpr::Call(f, args) => {
            select_call(&f, &args, out);
            out.push(RvInstr::Mv(dest, RvArg::Reg(RvReg::A0)));
        }
    }
}

fn select_call(f: &CAtom, args: &[CAtom], out: &mut Vec<RvInstr>) {
    for (i, arg) in args.iter().enumerate() {
        let arg = select_atom(arg);
        if i < 8 {
            out.push(RvInstr::Mv(RvArg::Reg(RvReg::A0), arg));
        } else {
            todo!("参数超过 8 个，暂不支持");
        }
    }
    match f {
        CAtom::Var(name) => out.push(RvInstr::Call(name.clone())),
        other => out.push(RvInstr::Jalr(select_atom(other))),
    }
}

fn select_stmt(stmt: &CStmt, out: &mut Vec<RvInstr>) {
    match stmt {
        CStmt::Assign(name, cexpr) => {
            let dest = RvArg::Var(name.clone());
            select_expr(dest, cexpr, out)
        }
    }
}

fn select_tail(tail: &CTail, gensym: &mut Gensym, out: &mut Vec<RvInstr>) {
    match tail {
        CTail::Return(cexpr) => {
            let dest = RvArg::Reg(RvReg::A0);
            select_expr(dest, cexpr, out);
        }
        CTail::TailCall(func, args) => {
            select_call(func, args, out);
            out.push(RvInstr::Ret);
        }
        CTail::Seq(stmt, cont) => {
            select_stmt(stmt, out);
            select_tail(cont, gensym, out);
        }
        CTail::If(cond, thn, els) => {
            let then_label = gensym.fresh_with_prefix("then");
            let cond_arg = select_atom(cond);
            out.push(RvInstr::Bnez(cond_arg, then_label.clone()));
            select_tail(els, gensym, out);
            out.push(RvInstr::Label(then_label));
            select_tail(thn, gensym, out);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::syntax::{BinOp, UnaryOp};

    fn v(name: &str) -> Ident {
        Ident::from(name)
    }

    fn dest() -> RvArg {
        RvArg::Var(v("dst"))
    }

    // ---------- select_atom ----------

    #[test]
    fn atom_unit_is_zero_imm() {
        assert_eq!(select_atom(&CAtom::Unit), RvArg::Imm(0));
    }

    #[test]
    fn atom_bool_true_is_one() {
        assert_eq!(select_atom(&CAtom::Bool(true)), RvArg::Imm(1));
    }

    #[test]
    fn atom_bool_false_is_zero() {
        assert_eq!(select_atom(&CAtom::Bool(false)), RvArg::Imm(0));
    }

    #[test]
    fn atom_int_is_imm() {
        assert_eq!(select_atom(&CAtom::Int(42)), RvArg::Imm(42));
    }

    #[test]
    fn atom_int_negative_is_imm() {
        assert_eq!(select_atom(&CAtom::Int(-7)), RvArg::Imm(-7));
    }

    #[test]
    fn atom_var_is_rvvar() {
        assert_eq!(select_atom(&CAtom::Var(v("x"))), RvArg::Var(v("x")));
    }

    // ---------- select_expr: Atom ----------

    #[test]
    fn expr_atom_int_emits_li() {
        let mut out = vec![];
        select_expr(dest(), &CExpr::Atom(CAtom::Int(5)), &mut out);
        assert_eq!(out, vec![RvInstr::Li(dest(), 5)]);
    }

    #[test]
    fn expr_atom_var_emits_mv() {
        let mut out = vec![];
        select_expr(dest(), &CExpr::Atom(CAtom::Var(v("x"))), &mut out);
        assert_eq!(out, vec![RvInstr::Mv(dest(), RvArg::Var(v("x")))]);
    }

    #[test]
    fn expr_atom_bool_emits_li() {
        let mut out = vec![];
        select_expr(dest(), &CExpr::Atom(CAtom::Bool(true)), &mut out);
        assert_eq!(out, vec![RvInstr::Li(dest(), 1)]);
    }

    #[test]
    fn expr_atom_unit_emits_li_zero() {
        let mut out = vec![];
        select_expr(dest(), &CExpr::Atom(CAtom::Unit), &mut out);
        assert_eq!(out, vec![RvInstr::Li(dest(), 0)]);
    }

    // ---------- select_expr: BinOp (simple, single-instr) ----------

    fn bin(op: BinOp) -> CExpr {
        CExpr::BinOp(op, CAtom::Var(v("a")), CAtom::Var(v("b")))
    }

    fn a_arg() -> RvArg {
        RvArg::Var(v("a"))
    }
    fn b_arg() -> RvArg {
        RvArg::Var(v("b"))
    }

    #[test]
    fn binop_add() {
        let mut out = vec![];
        select_expr(dest(), &bin(BinOp::Add), &mut out);
        assert_eq!(out, vec![RvInstr::Add(dest(), a_arg(), b_arg())]);
    }

    #[test]
    fn binop_sub() {
        let mut out = vec![];
        select_expr(dest(), &bin(BinOp::Sub), &mut out);
        assert_eq!(out, vec![RvInstr::Sub(dest(), a_arg(), b_arg())]);
    }

    #[test]
    fn binop_mul() {
        let mut out = vec![];
        select_expr(dest(), &bin(BinOp::Mul), &mut out);
        assert_eq!(out, vec![RvInstr::Mul(dest(), a_arg(), b_arg())]);
    }

    #[test]
    fn binop_div() {
        let mut out = vec![];
        select_expr(dest(), &bin(BinOp::Div), &mut out);
        assert_eq!(out, vec![RvInstr::Div(dest(), a_arg(), b_arg())]);
    }

    #[test]
    fn binop_and() {
        let mut out = vec![];
        select_expr(dest(), &bin(BinOp::And), &mut out);
        assert_eq!(out, vec![RvInstr::And(dest(), a_arg(), b_arg())]);
    }

    #[test]
    fn binop_or() {
        let mut out = vec![];
        select_expr(dest(), &bin(BinOp::Or), &mut out);
        assert_eq!(out, vec![RvInstr::Or(dest(), a_arg(), b_arg())]);
    }

    #[test]
    fn binop_lt_is_slt() {
        let mut out = vec![];
        select_expr(dest(), &bin(BinOp::Lt), &mut out);
        assert_eq!(out, vec![RvInstr::Slt(dest(), a_arg(), b_arg())]);
    }

    #[test]
    fn binop_gt_is_slt_with_swapped_operands() {
        let mut out = vec![];
        select_expr(dest(), &bin(BinOp::Gt), &mut out);
        // a > b  <=>  b < a
        assert_eq!(out, vec![RvInstr::Slt(dest(), b_arg(), a_arg())]);
    }

    // ---------- select_expr: BinOp (multi-instr) ----------

    #[test]
    fn binop_eq_is_xor_then_seqz() {
        let mut out = vec![];
        select_expr(dest(), &bin(BinOp::Eq), &mut out);
        assert_eq!(
            out,
            vec![
                RvInstr::Xor(dest(), a_arg(), b_arg()),
                RvInstr::Seqz(dest(), dest()),
            ]
        );
    }

    #[test]
    fn binop_neq_is_xor_then_snez() {
        let mut out = vec![];
        select_expr(dest(), &bin(BinOp::Neq), &mut out);
        assert_eq!(
            out,
            vec![
                RvInstr::Xor(dest(), a_arg(), b_arg()),
                RvInstr::Snez(dest(), dest()),
            ]
        );
    }

    #[test]
    fn binop_leq_is_slt_swapped_then_xor_one() {
        let mut out = vec![];
        select_expr(dest(), &bin(BinOp::Leq), &mut out);
        // a<=b <=> !(b<a)
        assert_eq!(
            out,
            vec![
                RvInstr::Slt(dest(), b_arg(), a_arg()),
                RvInstr::Xor(dest(), dest(), RvArg::Imm(1)),
            ]
        );
    }

    #[test]
    fn binop_geq_is_slt_then_xor_one() {
        let mut out = vec![];
        select_expr(dest(), &bin(BinOp::Geq), &mut out);
        // a>=b <=> !(a<b)
        assert_eq!(
            out,
            vec![
                RvInstr::Slt(dest(), a_arg(), b_arg()),
                RvInstr::Xor(dest(), dest(), RvArg::Imm(1)),
            ]
        );
    }

    // ---------- select_expr: UnaryOp ----------

    #[test]
    fn unary_neg_is_sub_from_zero() {
        let mut out = vec![];
        select_expr(
            dest(),
            &CExpr::UnaryOp(UnaryOp::Neg, CAtom::Var(v("a"))),
            &mut out,
        );
        assert_eq!(
            out,
            vec![RvInstr::Sub(dest(), RvArg::Reg(RvReg::Zero), a_arg())]
        );
    }

    #[test]
    fn unary_not_is_xor_one() {
        let mut out = vec![];
        select_expr(
            dest(),
            &CExpr::UnaryOp(UnaryOp::Not, CAtom::Var(v("a"))),
            &mut out,
        );
        assert_eq!(out, vec![RvInstr::Xor(dest(), a_arg(), RvArg::Imm(1))]);
    }

    // ---------- select_expr / select_call: Call ----------

    #[test]
    fn call_named_function_moves_args_and_calls() {
        let mut out = vec![];
        let f = CAtom::Var(v("f"));
        let args = vec![CAtom::Var(v("a")), CAtom::Var(v("b"))];
        select_expr(dest(), &CExpr::Call(f, args), &mut out);

        assert_eq!(
            out,
            vec![
                RvInstr::Mv(RvArg::Reg(RvReg::A0), a_arg()),
                RvInstr::Mv(RvArg::Reg(RvReg::A0), b_arg()),
                RvInstr::Call(v("f")),
                RvInstr::Mv(dest(), RvArg::Reg(RvReg::A0)),
            ]
        );
    }

    #[test]
    fn call_zero_args() {
        let mut out = vec![];
        let f = CAtom::Var(v("f"));
        select_expr(dest(), &CExpr::Call(f, vec![]), &mut out);
        assert_eq!(
            out,
            vec![
                RvInstr::Call(v("f")),
                RvInstr::Mv(dest(), RvArg::Reg(RvReg::A0)),
            ]
        );
    }

    #[test]
    fn call_via_non_var_callee_uses_jalr() {
        // A CAtom other than Var (e.g. computed/int) falls through to Jalr.
        let mut out = vec![];
        select_call(&CAtom::Int(0), &[], &mut out);
        assert_eq!(out, vec![RvInstr::Jalr(RvArg::Imm(0))]);
    }

    // ---------- select_stmt ----------

    #[test]
    fn stmt_assign_dispatches_to_select_expr() {
        let mut out = vec![];
        select_stmt(
            &CStmt::Assign(v("x"), CExpr::Atom(CAtom::Int(9))),
            &mut out,
        );
        assert_eq!(out, vec![RvInstr::Li(RvArg::Var(v("x")), 9)]);
    }

    // ---------- select_tail ----------

    #[test]
    fn tail_return_moves_result_into_a0() {
        let mut out = vec![];
        let mut gensym = Gensym::new();
        select_tail(&CTail::Return(CExpr::Atom(CAtom::Int(3))), &mut gensym, &mut out);
        assert_eq!(out, vec![RvInstr::Li(RvArg::Reg(RvReg::A0), 3)]);
    }

    #[test]
    fn tail_tailcall_calls_then_rets() {
        let mut out = vec![];
        let mut gensym = Gensym::new();
        select_tail(
            &CTail::TailCall(CAtom::Var(v("f")), vec![CAtom::Var(v("a"))]),
            &mut gensym,
            &mut out,
        );
        assert_eq!(
            out,
            vec![
                RvInstr::Mv(RvArg::Reg(RvReg::A0), a_arg()),
                RvInstr::Call(v("f")),
                RvInstr::Ret,
            ]
        );
    }

    #[test]
    fn tail_seq_runs_stmt_then_continuation() {
        let mut out = vec![];
        let mut gensym = Gensym::new();
        let tail = CTail::Seq(
            CStmt::Assign(v("x"), CExpr::Atom(CAtom::Int(1))),
            Box::new(CTail::Return(CExpr::Atom(CAtom::Var(v("x"))))),
        );
        select_tail(&tail, &mut gensym, &mut out);
        assert_eq!(
            out,
            vec![
                RvInstr::Li(RvArg::Var(v("x")), 1),
                RvInstr::Mv(RvArg::Reg(RvReg::A0), RvArg::Var(v("x"))),
            ]
        );
    }

    #[test]
    fn tail_if_emits_bnez_else_label_then() {
        let mut out = vec![];
        let mut gensym = Gensym::new();
        let tail = CTail::If(
            CAtom::Var(v("c")),
            Box::new(CTail::Return(CExpr::Atom(CAtom::Int(1)))),
            Box::new(CTail::Return(CExpr::Atom(CAtom::Int(0)))),
        );
        select_tail(&tail, &mut gensym, &mut out);

        assert_eq!(out.len(), 4);
        match &out[0] {
            RvInstr::Bnez(cond, label) => {
                assert_eq!(*cond, RvArg::Var(v("c")));
                assert_eq!(out[2], RvInstr::Label(label.clone()));
            }
            other => panic!("expected Bnez as first instr, got {other:?}"),
        }
        assert_eq!(out[1], RvInstr::Li(RvArg::Reg(RvReg::A0), 0));
        assert_eq!(out[3], RvInstr::Li(RvArg::Reg(RvReg::A0), 1));
    }

    #[test]
    fn tail_if_nested_in_seq() {
        let mut out = vec![];
        let mut gensym = Gensym::new();
        let tail = CTail::Seq(
            CStmt::Assign(v("x"), CExpr::Atom(CAtom::Int(1))),
            Box::new(CTail::If(
                CAtom::Var(v("x")),
                Box::new(CTail::Return(CExpr::Atom(CAtom::Int(1)))),
                Box::new(CTail::Return(CExpr::Atom(CAtom::Int(0)))),
            )),
        );
        select_tail(&tail, &mut gensym, &mut out);
        assert_eq!(out[0], RvInstr::Li(RvArg::Var(v("x")), 1));
        assert!(matches!(out[1], RvInstr::Bnez(_, _)));
    }
}
