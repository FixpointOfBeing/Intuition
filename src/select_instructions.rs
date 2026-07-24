// use crate::{
//     explicate_control::{CAtom, CExpr, CStmt, CTail},
//     gensym::Gensym,
//     syntax::{BinOp, Ident, UnaryOp},
// };
// use llvm_ir::{Constant, ConstantRef, Name, Operand, instruction::Sub};
// use llvm_ir::{constant::Float, types::TypeRef};

// fn const_int_ref(i: u64) -> ConstantRef {
//     let cst = Constant::Int { bits: 32, value: i };
//     ConstantRef::new(cst)
// }

// fn const_float_ref(f: f64) -> ConstantRef {
//     let f = Constant::Float(Float::Double(f));
//     ConstantRef::new(f)
// }

// fn select_atom(atom: &CAtom) -> Operand {
//     match atom {
//         CAtom::Unit => Operand::ConstantOperand(const_int_ref(0)),
//         CAtom::Bool(b) => {
//             let i = if *b {
//                 const_int_ref(1)
//             } else {
//                 const_int_ref(0)
//             };
//             Operand::ConstantOperand(i)
//         }
//         CAtom::Int(i) => {
//             let i = u64::from_ne_bytes((*i).to_ne_bytes());
//             Operand::ConstantOperand(const_int_ref(i))
//         }
//         CAtom::Float(f) => {
//             let f = const_float_ref(*f);
//             Operand::ConstantOperand(f)
//         }
//         CAtom::Var(name) => {
//             Operand::LocalOperand { name: Name::Name(Box::new(name.to_string())), ty: () }
//         },
//     }
// }

// fn select_expr(dest: Reg, e: &CExpr, out: &mut Vec<Instr>) {
//     match e {
//         CExpr::Atom(a) => {
//             let atom = select_atom(&a);
//             out.push(Instr::Copy(dest, atom));
//         }
//         CExpr::BinOp(op, l, r) => {
//             let l = select_atom(&l);
//             let r = select_atom(&r);
//             match op {
//                 BinOp::Add => out.push(Instr::Add(dest, l, r)),
//                 BinOp::Sub => out.push(Instr::Sub(dest, l, r)),
//                 BinOp::Mul => out.push(Instr::Mul(dest, l, r)),
//                 BinOp::Div => out.push(Instr::Div(dest, l, r)),
//                 BinOp::And => out.push(Instr::And(dest, l, r)),
//                 BinOp::Or => out.push(Instr::Or(dest, l, r)),
//                 BinOp::Lt => out.push(Instr::Slt(dest, l, r)),
//                 BinOp::Gt => out.push(Instr::Slt(dest, r, l)),
//                 BinOp::Eq => {
//                     out.push(Instr::Xor(dest.clone(), l, r));
//                     out.push(Instr::Seqz(dest.clone(), dest));
//                 }
//                 BinOp::Neq => {
//                     out.push(RvInstr::Xor(dest.clone(), l, r));
//                     out.push(RvInstr::Snez(dest.clone(), dest));
//                 }
//                 BinOp::Leq => {
//                     // l<=r <=> !(r<l)
//                     out.push(RvInstr::Slt(dest.clone(), r, l));
//                     out.push(RvInstr::Xor(dest.clone(), dest, Operand::Imm(1)));
//                 }
//                 BinOp::Geq => {
//                     // l>=r <=> !(l<r)
//                     out.push(RvInstr::Slt(dest.clone(), l, r));
//                     out.push(RvInstr::Xor(dest.clone(), dest, Operand::Imm(1)));
//                 }
//             }
//         }
//         CExpr::UnaryOp(op, a) => {
//             let a = select_atom(&a);
//             match op {
//                 UnaryOp::Neg => out.push(RvInstr::Sub(dest, Operand::Reg(Value::Zero), a)),
//                 UnaryOp::Not => out.push(RvInstr::Xor(dest, a, Operand::Imm(1))),
//             }
//         }
//         CExpr::Call(f, args) => {
//             select_call(&f, &args, out);
//             out.push(RvInstr::Mv(dest, Operand::Reg(Value::A0)));
//         }
//     }
// }

// fn select_call(f: &CAtom, args: &[CAtom], out: &mut Vec<RvInstr>) {
//     for (i, arg) in args.iter().enumerate() {
//         let arg = select_atom(arg);
//         if i < 8 {
//             out.push(RvInstr::Mv(Operand::Reg(Value::A0), arg));
//         } else {
//             todo!("参数超过 8 个，暂不支持");
//         }
//     }
//     match f {
//         CAtom::Var(name) => out.push(RvInstr::Call(name.clone())),
//         other => out.push(RvInstr::Jalr(select_atom(other))),
//     }
// }

// fn select_stmt(stmt: &CStmt, out: &mut Vec<RvInstr>) {
//     match stmt {
//         CStmt::Assign(name, cexpr) => {
//             let dest = Operand::Var(name.clone());
//             select_expr(dest, cexpr, out)
//         }
//     }
// }

// fn select_tail(tail: &CTail, gensym: &mut Gensym, out: &mut Vec<RvInstr>) {
//     match tail {
//         CTail::Return(cexpr) => {
//             let dest = Operand::Reg(Value::A0);
//             select_expr(dest, cexpr, out);
//         }
//         CTail::TailCall(func, args) => {
//             select_call(func, args, out);
//             out.push(RvInstr::Ret);
//         }
//         CTail::Seq(stmt, cont) => {
//             select_stmt(stmt, out);
//             select_tail(cont, gensym, out);
//         }
//         CTail::If(cond, thn, els) => {
//             let then_label = gensym.fresh_with_prefix("then");
//             let cond_arg = select_atom(cond);
//             out.push(RvInstr::Bnez(cond_arg, then_label.clone()));
//             select_tail(els, gensym, out);
//             out.push(RvInstr::Label(then_label));
//             select_tail(thn, gensym, out);
//         }
//     }
// }

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use crate::syntax::{BinOp, UnaryOp};

//     fn v(name: &str) -> Ident {
//         Ident::from(name)
//     }

//     fn dest() -> Operand {
//         Operand::Var(v("dst"))
//     }

//     // ---------- select_atom ----------

//     #[test]
//     fn atom_unit_is_zero_imm() {
//         assert_eq!(select_atom(&CAtom::Unit), Operand::Imm(0));
//     }

//     #[test]
//     fn atom_bool_true_is_one() {
//         assert_eq!(select_atom(&CAtom::Bool(true)), Operand::Imm(1));
//     }

//     #[test]
//     fn atom_bool_false_is_zero() {
//         assert_eq!(select_atom(&CAtom::Bool(false)), Operand::Imm(0));
//     }

//     #[test]
//     fn atom_int_is_imm() {
//         assert_eq!(select_atom(&CAtom::Int(42)), Operand::Imm(42));
//     }

//     #[test]
//     fn atom_int_negative_is_imm() {
//         assert_eq!(select_atom(&CAtom::Int(-7)), Operand::Imm(-7));
//     }

//     #[test]
//     fn atom_var_is_rvvar() {
//         assert_eq!(select_atom(&CAtom::Var(v("x"))), Operand::Var(v("x")));
//     }

//     // ---------- select_expr: Atom ----------

//     #[test]
//     fn expr_atom_int_emits_li() {
//         let mut out = vec![];
//         select_expr(dest(), &CExpr::Atom(CAtom::Int(5)), &mut out);
//         assert_eq!(out, vec![RvInstr::Li(dest(), 5)]);
//     }

//     #[test]
//     fn expr_atom_var_emits_mv() {
//         let mut out = vec![];
//         select_expr(dest(), &CExpr::Atom(CAtom::Var(v("x"))), &mut out);
//         assert_eq!(out, vec![RvInstr::Mv(dest(), Operand::Var(v("x")))]);
//     }

//     #[test]
//     fn expr_atom_bool_emits_li() {
//         let mut out = vec![];
//         select_expr(dest(), &CExpr::Atom(CAtom::Bool(true)), &mut out);
//         assert_eq!(out, vec![RvInstr::Li(dest(), 1)]);
//     }

//     #[test]
//     fn expr_atom_unit_emits_li_zero() {
//         let mut out = vec![];
//         select_expr(dest(), &CExpr::Atom(CAtom::Unit), &mut out);
//         assert_eq!(out, vec![RvInstr::Li(dest(), 0)]);
//     }

//     // ---------- select_expr: BinOp (simple, single-instr) ----------

//     fn bin(op: BinOp) -> CExpr {
//         CExpr::BinOp(op, CAtom::Var(v("a")), CAtom::Var(v("b")))
//     }

//     fn a_arg() -> Operand {
//         Operand::Var(v("a"))
//     }
//     fn b_arg() -> Operand {
//         Operand::Var(v("b"))
//     }

//     #[test]
//     fn binop_add() {
//         let mut out = vec![];
//         select_expr(dest(), &bin(BinOp::Add), &mut out);
//         assert_eq!(out, vec![RvInstr::Add(dest(), a_arg(), b_arg())]);
//     }

//     #[test]
//     fn binop_sub() {
//         let mut out = vec![];
//         select_expr(dest(), &bin(BinOp::Sub), &mut out);
//         assert_eq!(out, vec![RvInstr::Sub(dest(), a_arg(), b_arg())]);
//     }

//     #[test]
//     fn binop_mul() {
//         let mut out = vec![];
//         select_expr(dest(), &bin(BinOp::Mul), &mut out);
//         assert_eq!(out, vec![RvInstr::Mul(dest(), a_arg(), b_arg())]);
//     }

//     #[test]
//     fn binop_div() {
//         let mut out = vec![];
//         select_expr(dest(), &bin(BinOp::Div), &mut out);
//         assert_eq!(out, vec![RvInstr::Div(dest(), a_arg(), b_arg())]);
//     }

//     #[test]
//     fn binop_and() {
//         let mut out = vec![];
//         select_expr(dest(), &bin(BinOp::And), &mut out);
//         assert_eq!(out, vec![RvInstr::And(dest(), a_arg(), b_arg())]);
//     }

//     #[test]
//     fn binop_or() {
//         let mut out = vec![];
//         select_expr(dest(), &bin(BinOp::Or), &mut out);
//         assert_eq!(out, vec![RvInstr::Or(dest(), a_arg(), b_arg())]);
//     }

//     #[test]
//     fn binop_lt_is_slt() {
//         let mut out = vec![];
//         select_expr(dest(), &bin(BinOp::Lt), &mut out);
//         assert_eq!(out, vec![RvInstr::Slt(dest(), a_arg(), b_arg())]);
//     }

//     #[test]
//     fn binop_gt_is_slt_with_swapped_operands() {
//         let mut out = vec![];
//         select_expr(dest(), &bin(BinOp::Gt), &mut out);
//         // a > b  <=>  b < a
//         assert_eq!(out, vec![RvInstr::Slt(dest(), b_arg(), a_arg())]);
//     }

//     // ---------- select_expr: BinOp (multi-instr) ----------

//     #[test]
//     fn binop_eq_is_xor_then_seqz() {
//         let mut out = vec![];
//         select_expr(dest(), &bin(BinOp::Eq), &mut out);
//         assert_eq!(
//             out,
//             vec![
//                 RvInstr::Xor(dest(), a_arg(), b_arg()),
//                 RvInstr::Seqz(dest(), dest()),
//             ]
//         );
//     }

//     #[test]
//     fn binop_neq_is_xor_then_snez() {
//         let mut out = vec![];
//         select_expr(dest(), &bin(BinOp::Neq), &mut out);
//         assert_eq!(
//             out,
//             vec![
//                 RvInstr::Xor(dest(), a_arg(), b_arg()),
//                 RvInstr::Snez(dest(), dest()),
//             ]
//         );
//     }

//     #[test]
//     fn binop_leq_is_slt_swapped_then_xor_one() {
//         let mut out = vec![];
//         select_expr(dest(), &bin(BinOp::Leq), &mut out);
//         // a<=b <=> !(b<a)
//         assert_eq!(
//             out,
//             vec![
//                 RvInstr::Slt(dest(), b_arg(), a_arg()),
//                 RvInstr::Xor(dest(), dest(), Operand::Imm(1)),
//             ]
//         );
//     }

//     #[test]
//     fn binop_geq_is_slt_then_xor_one() {
//         let mut out = vec![];
//         select_expr(dest(), &bin(BinOp::Geq), &mut out);
//         // a>=b <=> !(a<b)
//         assert_eq!(
//             out,
//             vec![
//                 RvInstr::Slt(dest(), a_arg(), b_arg()),
//                 RvInstr::Xor(dest(), dest(), Operand::Imm(1)),
//             ]
//         );
//     }

//     // ---------- select_expr: UnaryOp ----------

//     #[test]
//     fn unary_neg_is_sub_from_zero() {
//         let mut out = vec![];
//         select_expr(
//             dest(),
//             &CExpr::UnaryOp(UnaryOp::Neg, CAtom::Var(v("a"))),
//             &mut out,
//         );
//         assert_eq!(
//             out,
//             vec![RvInstr::Sub(dest(), Operand::Reg(Value::Zero), a_arg())]
//         );
//     }

//     #[test]
//     fn unary_not_is_xor_one() {
//         let mut out = vec![];
//         select_expr(
//             dest(),
//             &CExpr::UnaryOp(UnaryOp::Not, CAtom::Var(v("a"))),
//             &mut out,
//         );
//         assert_eq!(out, vec![RvInstr::Xor(dest(), a_arg(), Operand::Imm(1))]);
//     }

//     // ---------- select_expr / select_call: Call ----------

//     #[test]
//     fn call_named_function_moves_args_and_calls() {
//         let mut out = vec![];
//         let f = CAtom::Var(v("f"));
//         let args = vec![CAtom::Var(v("a")), CAtom::Var(v("b"))];
//         select_expr(dest(), &CExpr::Call(f, args), &mut out);

//         assert_eq!(
//             out,
//             vec![
//                 RvInstr::Mv(Operand::Reg(Value::A0), a_arg()),
//                 RvInstr::Mv(Operand::Reg(Value::A0), b_arg()),
//                 RvInstr::Call(v("f")),
//                 RvInstr::Mv(dest(), Operand::Reg(Value::A0)),
//             ]
//         );
//     }

//     #[test]
//     fn call_zero_args() {
//         let mut out = vec![];
//         let f = CAtom::Var(v("f"));
//         select_expr(dest(), &CExpr::Call(f, vec![]), &mut out);
//         assert_eq!(
//             out,
//             vec![
//                 RvInstr::Call(v("f")),
//                 RvInstr::Mv(dest(), Operand::Reg(Value::A0)),
//             ]
//         );
//     }

//     #[test]
//     fn call_via_non_var_callee_uses_jalr() {
//         // A CAtom other than Var (e.g. computed/int) falls through to Jalr.
//         let mut out = vec![];
//         select_call(&CAtom::Int(0), &[], &mut out);
//         assert_eq!(out, vec![RvInstr::Jalr(Operand::Imm(0))]);
//     }

//     // ---------- select_stmt ----------

//     #[test]
//     fn stmt_assign_dispatches_to_select_expr() {
//         let mut out = vec![];
//         select_stmt(&CStmt::Assign(v("x"), CExpr::Atom(CAtom::Int(9))), &mut out);
//         assert_eq!(out, vec![RvInstr::Li(Operand::Var(v("x")), 9)]);
//     }

//     // ---------- select_tail ----------

//     #[test]
//     fn tail_return_moves_result_into_a0() {
//         let mut out = vec![];
//         let mut gensym = Gensym::new();
//         select_tail(
//             &CTail::Return(CExpr::Atom(CAtom::Int(3))),
//             &mut gensym,
//             &mut out,
//         );
//         assert_eq!(out, vec![RvInstr::Li(Operand::Reg(Value::A0), 3)]);
//     }

//     #[test]
//     fn tail_tailcall_calls_then_rets() {
//         let mut out = vec![];
//         let mut gensym = Gensym::new();
//         select_tail(
//             &CTail::TailCall(CAtom::Var(v("f")), vec![CAtom::Var(v("a"))]),
//             &mut gensym,
//             &mut out,
//         );
//         assert_eq!(
//             out,
//             vec![
//                 RvInstr::Mv(Operand::Reg(Value::A0), a_arg()),
//                 RvInstr::Call(v("f")),
//                 RvInstr::Ret,
//             ]
//         );
//     }

//     #[test]
//     fn tail_seq_runs_stmt_then_continuation() {
//         let mut out = vec![];
//         let mut gensym = Gensym::new();
//         let tail = CTail::Seq(
//             CStmt::Assign(v("x"), CExpr::Atom(CAtom::Int(1))),
//             Box::new(CTail::Return(CExpr::Atom(CAtom::Var(v("x"))))),
//         );
//         select_tail(&tail, &mut gensym, &mut out);
//         assert_eq!(
//             out,
//             vec![
//                 RvInstr::Li(Operand::Var(v("x")), 1),
//                 RvInstr::Mv(Operand::Reg(Value::A0), Operand::Var(v("x"))),
//             ]
//         );
//     }

//     #[test]
//     fn tail_if_emits_bnez_else_label_then() {
//         let mut out = vec![];
//         let mut gensym = Gensym::new();
//         let tail = CTail::If(
//             CAtom::Var(v("c")),
//             Box::new(CTail::Return(CExpr::Atom(CAtom::Int(1)))),
//             Box::new(CTail::Return(CExpr::Atom(CAtom::Int(0)))),
//         );
//         select_tail(&tail, &mut gensym, &mut out);

//         assert_eq!(out.len(), 4);
//         match &out[0] {
//             RvInstr::Bnez(cond, label) => {
//                 assert_eq!(*cond, Operand::Var(v("c")));
//                 assert_eq!(out[2], RvInstr::Label(label.clone()));
//             }
//             other => panic!("expected Bnez as first instr, got {other:?}"),
//         }
//         assert_eq!(out[1], RvInstr::Li(Operand::Reg(Value::A0), 0));
//         assert_eq!(out[3], RvInstr::Li(Operand::Reg(Value::A0), 1));
//     }

//     #[test]
//     fn tail_if_nested_in_seq() {
//         let mut out = vec![];
//         let mut gensym = Gensym::new();
//         let tail = CTail::Seq(
//             CStmt::Assign(v("x"), CExpr::Atom(CAtom::Int(1))),
//             Box::new(CTail::If(
//                 CAtom::Var(v("x")),
//                 Box::new(CTail::Return(CExpr::Atom(CAtom::Int(1)))),
//                 Box::new(CTail::Return(CExpr::Atom(CAtom::Int(0)))),
//             )),
//         );
//         select_tail(&tail, &mut gensym, &mut out);
//         assert_eq!(out[0], RvInstr::Li(Operand::Var(v("x")), 1));
//         assert!(matches!(out[1], RvInstr::Bnez(_, _)));
//     }
// }
