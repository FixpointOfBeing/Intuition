use crate::{
    a_normal_form::{AExpr, AnfExpr, CompExpr},
    expose_allocation::GlobalValue,
    syntax::{BinOp, HasType, Ident, PrimIO, Type, UnaryOp, ty_unit},
};

/*
 * For the integers and variables, we needed assignment and tail positions. The if expressions introduced predicate positions. For While , the begin expression introduces yet another kind of position: effect position.
 */
#[derive(Debug, Clone, PartialEq)]
pub enum CAtom {
    Unit,
    Bool(bool),
    Int(i64),
    Float(f64),
    Var(Ident, Type),
}

impl HasType for CAtom {
    fn type_of(&self) -> Type {
        todo!()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum CExpr {
    Atom(CAtom, Type),
    BinOp(BinOp, CAtom, CAtom, Type),
    UnaryOp(UnaryOp, CAtom, Type),
    TupleProj(CAtom, usize, Type),
    Allocate(usize, Type),
    AllocateClosure(usize, usize, Type),
    Call(CAtom, Vec<CAtom>, Type),
    FunRef(Ident, usize, Type),
    GlobalValue(GlobalValue),
}

impl HasType for CExpr {
    fn type_of(&self) -> Type {
        todo!()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum CStmt {
    Assign(Ident, CExpr),
    TupleSet(CAtom, CAtom, usize),
    PrimIO(PrimIO, Option<CAtom>),
    Collect(usize),
    Effect(CExpr),
    If(CAtom, Vec<CStmt>, Vec<CStmt>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum CTail {
    Return(CExpr),
    // Goto(Ident),
    TailCall(CAtom, Vec<CAtom>),
    TailSeq(Vec<CStmt>, Box<CTail>),
    TailIf(CAtom, Box<CTail>, Box<CTail>),
}

pub fn c_unit() -> CAtom {
    CAtom::Unit
}

pub fn c_bool(b: bool) -> CAtom {
    CAtom::Bool(b)
}

pub fn c_int(n: i64) -> CAtom {
    CAtom::Int(n)
}

pub fn c_float(f: f64) -> CAtom {
    CAtom::Float(f)
}

pub fn c_var(name: impl Into<Ident>, ty: Type) -> CAtom {
    CAtom::Var(name.into(), ty)
}

pub fn c_atom(atom: CAtom, ty: Type) -> CExpr {
    CExpr::Atom(atom, ty)
}

pub fn c_bin_op(op: BinOp, left: CAtom, right: CAtom, ty: Type) -> CExpr {
    CExpr::BinOp(op, left, right, ty)
}

pub fn c_unary(op: UnaryOp, catom: CAtom, ty: Type) -> CExpr {
    CExpr::UnaryOp(op, catom, ty)
}

pub fn c_tuple_proj(tuple: CAtom, index: usize, ty: Type) -> CExpr {
    CExpr::TupleProj(tuple, index, ty)
}

pub fn c_allocate(bytes: usize, ty: Type) -> CExpr {
    CExpr::Allocate(bytes, ty)
}

pub fn c_allocate_closure(bytes: usize, arity: usize, ty: Type) -> CExpr {
    CExpr::AllocateClosure(bytes, arity, ty)
}

pub fn c_call(func: CAtom, args: Vec<CAtom>, ty: Type) -> CExpr {
    CExpr::Call(func, args, ty)
}

pub fn c_fun_ref(name: impl Into<Ident>, arity: usize, ty: Type) -> CExpr {
    CExpr::FunRef(name.into(), arity, ty)
}

pub fn c_global_value(value: GlobalValue) -> CExpr {
    CExpr::GlobalValue(value)
}

pub fn c_assign(name: impl Into<Ident>, cexpr: CExpr) -> CStmt {
    CStmt::Assign(name.into(), cexpr)
}

pub fn c_tuple_set(tuple: CAtom, element: CAtom, index: usize) -> CStmt {
    CStmt::TupleSet(tuple, element, index)
}

pub fn c_prim_io(prim: PrimIO, expr: Option<CAtom>) -> CStmt {
    CStmt::PrimIO(prim, expr)
}

pub fn c_collect(bytes: usize) -> CStmt {
    CStmt::Collect(bytes)
}

pub fn c_effect(expr: CExpr) -> CStmt {
    CStmt::Effect(expr)
}

pub fn c_return(expr: CExpr) -> CTail {
    CTail::Return(expr)
}

// pub fn c_goto(label: impl Into<Ident>) -> CTail {
//     CTail::Goto(label.into())
// }

pub fn c_tail_call(func: CAtom, args: Vec<CAtom>) -> CTail {
    CTail::TailCall(func, args)
}

pub fn c_tail_seq(stmts: Vec<CStmt>, tail: CTail) -> CTail {
    CTail::TailSeq(stmts, Box::new(tail))
}

pub fn c_tail_if(cond: CAtom, thn: CTail, els: CTail) -> CTail {
    CTail::TailIf(cond, Box::new(thn), Box::new(els))
}

pub fn explicate_assign_anf(name: String, anf:AnfExpr, cont:CTail) -> CTail {
    match anf {
        AnfExpr::Complex(cexpr, _) => {
            explicate_assign_complex(name, cexpr, cont)
        }
        AnfExpr::Let(let_name, rhs, body, _) => {
            let inner_cont = explicate_assign_anf(name, *body, cont);
            explicate_assign_complex(let_name, rhs, inner_cont)
        }
    }
}

pub fn explicate_assign_complex(name: String, cexpr: CompExpr, cont: CTail) -> CTail {
    let dummy = "_".to_string();
    match cexpr {
        CompExpr::Atom(aexpr, ty) => {
            let catom = aexpr_to_catom(aexpr);
            let cexpr = c_atom(catom, ty);
            let stmt = c_assign(name, cexpr);
            c_tail_seq(vec![stmt], cont)
        },
        CompExpr::BinOp(op, left, right, ty) => {
            let left_catom = aexpr_to_catom(left);
            let right_catom = aexpr_to_catom(right);
            let cexpr = c_bin_op(op, left_catom, right_catom, ty);
            let stmt = c_assign(name, cexpr);
            c_tail_seq(vec![stmt], cont)
        },
        CompExpr::UnaryOp(op, aexpr, ty) => {
            let catom = aexpr_to_catom(aexpr);
            let cexpr = c_unary(op, catom, ty);
            let stmt = c_assign(name, cexpr);
            c_tail_seq(vec![stmt], cont)
        },
        CompExpr::TupleProj(tuple, idx, ty) => {
            let catom = aexpr_to_catom(tuple);
            let cexpr = c_tuple_proj(catom, idx, ty);
            let stmt = c_assign(name, cexpr);
            c_tail_seq(vec![stmt], cont)
        },
        CompExpr::Allocate(bytes, ty) => {
            let cexpr = c_allocate(bytes, ty);
            let stmt = c_assign(name, cexpr);
            c_tail_seq(vec![stmt], cont)
        },
        CompExpr::AllocateClosure(bytes, arity, ty) => {
            let cexpr = c_allocate_closure(bytes, arity, ty);
            let stmt = c_assign(name, cexpr);
            c_tail_seq(vec![stmt], cont)
        },
        CompExpr::PrimIO(prim, aexpr, ty) => {
            todo!()
        },
        CompExpr::App(func, args, ty) => {
            let func_catom = aexpr_to_catom(func);
            let args_catom = args.into_iter().map(aexpr_to_catom).collect();
            let cexpr = c_call(func_catom, args_catom, ty);
            let stmt = c_assign(name, cexpr);
            c_tail_seq(vec![stmt], cont)
        }
        CompExpr::If(cond, thn, els, _) => {
            let cond_catom = aexpr_to_catom(cond);
            // todo! cont会复制
            let thn_cont = explicate_assign_anf(name.clone(), *thn, cont.clone());
            let els_cont = explicate_assign_anf(name, *els, cont);
            c_tail_if(cond_catom, thn_cont, els_cont)
        },
        CompExpr::Collect(bytes) => {
            assert!(name == dummy);
            let stmt = c_collect(bytes);
            c_tail_seq(vec![stmt], cont)
        },
        CompExpr::TupleSet(tuple, elem, idx) => {
            assert!(name == dummy);
            let tuple_catom = aexpr_to_catom(tuple);
            let elem_catom = aexpr_to_catom(elem);
            let stmt = c_tuple_set(tuple_catom, elem_catom, idx);
            c_tail_seq(vec![stmt], cont)
        },
        CompExpr::GlobalValue(value) => {
            let cexpr = c_global_value(value);
            let stmt = c_assign(name, cexpr);
            c_tail_seq(vec![stmt], cont)
        },
        CompExpr::FunRef(func_name, arity, ty) => {
            let cexpr = c_fun_ref(func_name, arity, ty);
            let stmt = c_assign(name, cexpr);
            c_tail_seq(vec![stmt], cont)
        },
    }
}

fn aexpr_to_catom(aexpr: AExpr) -> CAtom {
    match aexpr {
        AExpr::Unit => CAtom::Unit,
        AExpr::Bool(b) => CAtom::Bool(b),
        AExpr::Int(i) => CAtom::Int(i),
        AExpr::Float(f) => CAtom::Float(f),
        AExpr::Var(name, ty) => CAtom::Var(name, ty),
    }
}

fn explicate_tail(anf: AnfExpr) -> CTail {
    match anf {
        AnfExpr::Complex(comp_expr, ty) => match comp_expr {
            CompExpr::Atom(aexpr, ty) => {
                let catom = aexpr_to_catom(aexpr);
                c_return(c_atom(catom, ty))
            },
            CompExpr::BinOp(op, left, right, ty) => {
                let left_atom = aexpr_to_catom(left);
                let right_atom = aexpr_to_catom(right);
                let cexpr = c_bin_op(op, left_atom, right_atom, ty);
                c_return(cexpr)
            },
            CompExpr::UnaryOp(op, aexpr, ty) => {
                let atom = aexpr_to_catom(aexpr);
                let cexpr = c_unary(op, atom, ty);
                c_return(cexpr)
            },
            CompExpr::TupleProj(aexpr, idx, ty) => {
                let atom = aexpr_to_catom(aexpr);
                let cexpr = c_tuple_proj(atom, idx, ty);
                c_return(cexpr)
            },
            CompExpr::Allocate(bytes, ty) => {
                let cexpr = c_allocate(bytes, ty);
                c_return(cexpr)
            },
            CompExpr::AllocateClosure(bytes, arity, ty) => {
                let cexpr = c_allocate_closure(bytes, arity, ty);
                c_return(cexpr)
            },
            CompExpr::PrimIO(prim, aexpr, _) => {
                let atom = aexpr.map(|e| aexpr_to_catom(e));
                let stmt = c_prim_io(prim, atom);

                todo!()
            },
            CompExpr::App(func, args, _) => {
                let func_atom = aexpr_to_catom(func);
                let args_atom = args.into_iter().map(|e| aexpr_to_catom(e)).collect::<Vec<_>>();
                c_tail_call(func_atom, args_atom)
            },
            CompExpr::If(cond, thn, els, _) => {
                let cond_atom = aexpr_to_catom(cond);
                let thn_atom = explicate_tail(*thn);
                let els_atom = explicate_tail(*els);
                c_tail_if(cond_atom, thn_atom, els_atom)
            },
            CompExpr::Collect(bytes) => {
                let stmt = c_collect(bytes);
                let last = c_return(c_atom(c_unit(), ty_unit()));
                c_tail_seq(vec![stmt], last)
            },
            CompExpr::TupleSet(tuple, elem, idx) => {
                let tuple_atom = aexpr_to_catom(tuple);
                let elem_atom = aexpr_to_catom(elem);
                let stmt = c_tuple_set(tuple_atom, elem_atom, idx);
                let last = c_return(c_atom(c_unit(), ty_unit()));
                c_tail_seq(vec![stmt], last)
            },
            CompExpr::GlobalValue(value) => c_return(c_global_value(value)),
            CompExpr::FunRef(name, arity, ty) => c_return(c_fun_ref(name, arity, ty)),
        },
        AnfExpr::Let(name, rhs, body, _) => {
            let tail = explicate_tail(*body);
            explicate_assign_complex(name, rhs, tail)
        },
    }
    
}
//
// pub fn explicate_control_convert(clos: ClosExpr) -> CTail {
//     explicate_tail(clos)
// }

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use crate::a_normal_form::AExpr;
//     use crate::closure_conversion::{ClosCompExpr, ClosExpr};
//     use crate::syntax::BinOp;
//     use crate::syntax::UnaryOp;
//     use rand::seq::IndexedRandom;
//
//     fn anf_var(name: &str, ty: Type) -> AExpr {
//         AExpr::Var(name.to_string(), ty)
//     }
//
//     fn random_type_with_depth(depth: usize) -> Type {
//         let mut rng = rand::rng();
//
//         if depth == 0 {
//             let leaf_variants: &[fn() -> Type] = &[
//                 || Type::Unit,
//                 || Type::Bool,
//                 || Type::Float,
//                 || Type::Int,
//                 || Type::Var("$dummy".to_string()),
//             ];
//             return leaf_variants.choose(&mut rng).unwrap()();
//         }
//
//         let all_variants: &[fn(usize) -> Type] = &[
//             |_| Type::Unit,
//             |_| Type::Bool,
//             |_| Type::Float,
//             |_| Type::Int,
//             |d| {
//                 Type::Arrow(
//                     Box::new(random_type_with_depth(d - 1)),
//                     Box::new(random_type_with_depth(d - 1)),
//                 )
//             },
//             |_| Type::Var("$dummy".to_string()),
//         ];
//         all_variants.choose(&mut rng).unwrap()(depth)
//     }
//
//     fn random_type() -> Type {
//         random_type_with_depth(5)
//     }
//
//     fn anf_int(n: i64) -> AExpr {
//         AExpr::Int(n)
//     }
//
//     fn anf_bool(b: bool) -> AExpr {
//         AExpr::Bool(b)
//     }
//
//     fn anf_atom_to_comp(a: AExpr) -> ClosCompExpr {
//         ClosCompExpr::Atom(a)
//     }
//
//     fn anf_atom_to_anf(a: AExpr) -> ClosExpr {
//         ClosExpr::Complex(ClosCompExpr::Atom(a))
//     }
//
//     fn anf_let(name: &str, rhs: ClosCompExpr, body: ClosExpr) -> ClosExpr {
//         ClosExpr::Let(name.to_string(), rhs, Box::new(body))
//     }
//
//     fn c_var(name: &str, ty: Type) -> CAtom {
//         CAtom::Var(name.to_string(), ty)
//     }
//
//     fn c_int(n: i64) -> CAtom {
//         CAtom::Int(n)
//     }
//
//     fn c_bool(b: bool) -> CAtom {
//         CAtom::Bool(b)
//     }
//
//     fn c_assign(name: &str, e: CExpr, cont: CTail) -> CTail {
//         let ty = type_of_cexpr(&e);
//         CTail::Seq(CStmt::Assign(name.to_string(), e, ty), Box::new(cont))
//     }
//
//     #[test]
//     fn tail_atom_int() {
//         let input = anf_atom_to_anf(anf_int(5));
//         let expected = CTail::Return(CExpr::Atom(CAtom::Int(5)));
//         assert_eq!(explicate_tail(input), expected);
//     }
//
//     #[test]
//     fn tail_atom_var() {
//         let ty = random_type();
//         let input = anf_atom_to_anf(anf_var("x", ty.clone()));
//         let expected = CTail::Return(CExpr::Atom(c_var("x", ty)));
//         assert_eq!(explicate_tail(input), expected);
//     }
//
//     #[test]
//     fn tail_binop() {
//         let ty = Type::Int;
//         let input = ClosExpr::Complex(ClosCompExpr::BinOp(
//             BinOp::Add,
//             anf_var("x", ty.clone()),
//             anf_int(1),
//         ));
//         let expected = CTail::Return(CExpr::BinOp(BinOp::Add, c_var("x", ty), c_int(1)));
//         assert_eq!(explicate_tail(input), expected);
//     }
//
//     #[test]
//     fn tail_unaryop() {
//         let ty = Type::Float;
//         let input =
//             ClosExpr::Complex(ClosCompExpr::UnaryOp(UnaryOp::Neg, anf_var("x", ty.clone())));
//         let expected = CTail::Return(CExpr::UnaryOp(UnaryOp::Neg, c_var("x", ty)));
//         assert_eq!(explicate_tail(input), expected);
//     }
//
//     #[test]
//     fn tail_app_becomes_tailcall() {
//         let x_ty = random_type();
//         let y_ty = random_type();
//         let fn_ty = Type::Arrow(
//             Box::new(x_ty.clone()),
//             Box::new(Type::Arrow(Box::new(y_ty.clone()), Box::new(Type::Unit))),
//         );
//         let input = ClosExpr::Complex(ClosCompExpr::App(
//             anf_var("f", fn_ty.clone()),
//             vec![anf_var("x", x_ty.clone()), anf_var("y", y_ty.clone())],
//         ));
//         let expected = CTail::TailCall(c_var("f", fn_ty), vec![c_var("x", x_ty), c_var("y", y_ty)]);
//         assert_eq!(explicate_tail(input), expected);
//     }
//
//     #[test]
//     fn tail_if_both_branches_are_tail() {
//         let input = ClosExpr::Complex(ClosCompExpr::If(
//             anf_var("c", Type::Bool),
//             Box::new(anf_atom_to_anf(anf_int(1))),
//             Box::new(anf_atom_to_anf(anf_int(2))),
//         ));
//         let expected = CTail::If(
//             c_var("c", Type::Bool),
//             Box::new(CTail::Return(CExpr::Atom(c_int(1)))),
//             Box::new(CTail::Return(CExpr::Atom(c_int(2)))),
//         );
//         assert_eq!(explicate_tail(input), expected);
//     }
//
//     #[test]
//     fn tail_let_single() {
//         let input =
//             anf_let("x", anf_atom_to_comp(anf_int(1)), anf_atom_to_anf(anf_var("x", Type::Int)));
//         let expected =
//             c_assign("x", CExpr::Atom(c_int(1)), CTail::Return(CExpr::Atom(c_var("x", Type::Int))));
//         assert_eq!(explicate_tail(input), expected);
//     }
//
//     #[test]
//     fn tail_let_nested_order_is_preserved() {
//         let input = anf_let(
//             "x",
//             anf_atom_to_comp(anf_int(1)),
//             anf_let(
//                 "y",
//                 ClosCompExpr::BinOp(BinOp::Add, anf_var("x", Type::Int), anf_int(1)),
//                 anf_atom_to_anf(anf_var("y", Type::Int)),
//             ),
//         );
//         let expected = c_assign(
//             "x",
//             CExpr::Atom(c_int(1)),
//             c_assign(
//                 "y",
//                 CExpr::BinOp(BinOp::Add, c_var("x", Type::Int), c_int(1)),
//                 CTail::Return(CExpr::Atom(c_var("y", Type::Int))),
//             ),
//         );
//         assert_eq!(explicate_tail(input), expected);
//     }
//
//     #[test]
//     fn tail_let_with_if_rhs() {
//         let input = anf_let(
//             "x",
//             ClosCompExpr::If(
//                 anf_var("c", Type::Bool),
//                 Box::new(anf_atom_to_anf(anf_int(1))),
//                 Box::new(anf_atom_to_anf(anf_int(2))),
//             ),
//             anf_atom_to_anf(anf_var("x", Type::Int)),
//         );
//         let cont = CTail::Return(CExpr::Atom(c_var("x", Type::Int)));
//         let expected = CTail::If(
//             c_var("c", Type::Bool),
//             Box::new(c_assign("x", CExpr::Atom(c_int(1)), cont.clone())),
//             Box::new(c_assign("x", CExpr::Atom(c_int(2)), cont)),
//         );
//         assert_eq!(explicate_tail(input), expected);
//     }
//
//     #[test]
//     fn assign_atom() {
//         let input = anf_atom_to_anf(anf_int(5));
//         let cont = CTail::Return(CExpr::Atom(c_var("x", Type::Int)));
//         let expected = c_assign("x", CExpr::Atom(c_int(5)), cont.clone());
//         assert_eq!(explicate_assign(input, "x", cont), expected);
//     }
//
//     #[test]
//     fn assign_binop() {
//         let input = ClosExpr::Complex(ClosCompExpr::BinOp(
//             BinOp::Mul,
//             anf_var("a", Type::Float),
//             anf_var("b", Type::Float),
//         ));
//         let cont = CTail::Return(CExpr::Atom(c_var("x", Type::Float)));
//         let expected = c_assign(
//             "x",
//             CExpr::BinOp(BinOp::Mul, c_var("a", Type::Float), c_var("b", Type::Float)),
//             cont.clone(),
//         );
//         assert_eq!(explicate_assign(input, "x", cont), expected);
//     }
//
//     #[test]
//     fn assign_unaryop() {
//         let input =
//             ClosExpr::Complex(ClosCompExpr::UnaryOp(UnaryOp::Not, anf_var("a", Type::Bool)));
//         let cont = CTail::Return(CExpr::Atom(c_var("x", Type::Bool)));
//         let expected =
//             c_assign("x", CExpr::UnaryOp(UnaryOp::Not, c_var("a", Type::Bool)), cont.clone());
//         assert_eq!(explicate_assign(input, "x", cont), expected);
//     }
//
//     #[test]
//     fn assign_app_is_non_tail_call() {
//         let a_ty = Type::Int;
//         let b_ty = Type::Int;
//         let result_ty = Type::Int;
//         let f_ty = Type::Arrow(
//             Box::new(a_ty.clone()),
//             Box::new(Type::Arrow(Box::new(b_ty.clone()), Box::new(result_ty.clone()))),
//         );
//         let input = ClosExpr::Complex(ClosCompExpr::App(
//             anf_var("f", f_ty.clone()),
//             vec![anf_var("a", a_ty.clone()), anf_var("b", b_ty.clone())],
//         ));
//         let cont = CTail::Return(CExpr::Atom(c_var("x", result_ty.clone())));
//         let expected = c_assign(
//             "x",
//             CExpr::Call(c_var("f", f_ty), vec![c_var("a", a_ty), c_var("b", b_ty)]),
//             cont.clone(),
//         );
//         assert_eq!(explicate_assign(input, "x", cont), expected);
//     }
//
//     #[test]
//     fn assign_if_both_branches_assign_and_share_cont() {
//         let input = ClosExpr::Complex(ClosCompExpr::If(
//             anf_var("c", Type::Bool),
//             Box::new(anf_atom_to_anf(anf_int(1))),
//             Box::new(anf_atom_to_anf(anf_int(2))),
//         ));
//         let cont = CTail::Return(CExpr::Atom(c_var("x", Type::Int)));
//         let expected = CTail::If(
//             c_var("c", Type::Bool),
//             Box::new(c_assign("x", CExpr::Atom(c_int(1)), cont.clone())),
//             Box::new(c_assign("x", CExpr::Atom(c_int(2)), cont.clone())),
//         );
//         assert_eq!(explicate_assign(input, "x", cont.clone()), expected);
//     }
//
//     #[test]
//     fn assign_if_nested_inside_let_rhs_and_outer_let() {
//         let inner_let = anf_let(
//             "x",
//             ClosCompExpr::If(
//                 anf_var("c", Type::Bool),
//                 Box::new(anf_atom_to_anf(anf_int(1))),
//                 Box::new(anf_atom_to_anf(anf_int(2))),
//             ),
//             anf_atom_to_anf(anf_var("x", Type::Int)),
//         );
//
//         let final_cont = CTail::Return(CExpr::Atom(c_var("y", Type::Int)));
//         let result = explicate_assign(inner_let, "y", final_cont.clone());
//
//         let expected = CTail::If(
//             c_var("c", Type::Bool),
//             Box::new(c_assign(
//                 "x",
//                 CExpr::Atom(c_int(1)),
//                 c_assign("y", CExpr::Atom(c_var("x", Type::Int)), final_cont.clone()),
//             )),
//             Box::new(c_assign(
//                 "x",
//                 CExpr::Atom(c_int(2)),
//                 c_assign("y", CExpr::Atom(c_var("x", Type::Int)), final_cont),
//             )),
//         );
//         assert_eq!(result, expected);
//     }
//
//     #[test]
//     fn assign_let_forwards_correctly() {
//         let input =
//             anf_let("a", anf_atom_to_comp(anf_int(1)), anf_atom_to_anf(anf_var("a", Type::Int)));
//         let cont = CTail::Return(CExpr::Atom(c_var("x", Type::Int)));
//         let expected = c_assign(
//             "a",
//             CExpr::Atom(c_int(1)),
//             c_assign("x", CExpr::Atom(c_var("a", Type::Int)), cont.clone()),
//         );
//         assert_eq!(explicate_assign(input, "x", cont), expected);
//     }
//
//     #[test]
//     fn assign_true_false_atoms() {
//         let input = anf_atom_to_anf(anf_bool(true));
//         let cont = CTail::Return(CExpr::Atom(c_var("x", Type::Bool)));
//         let expected = c_assign("x", CExpr::Atom(c_bool(true)), cont.clone());
//         assert_eq!(explicate_assign(input, "x", cont), expected);
//     }
// }
