use std::collections::HashMap;

use crate::syntax::BinOp;
use crate::syntax::HasType;
use crate::syntax::Ident;
use crate::syntax::PrimIO;
use crate::syntax::Type;
use crate::syntax::UnaryOp;
use crate::typechecker::TypedDef;
use crate::typechecker::TypedExpr;
use crate::typechecker::TypedProgram;

type Fnames = HashMap<Ident, usize>; // name -> arity

#[derive(Debug, Clone, PartialEq)]
pub enum RevealExpr {
    Unit,
    Bool(bool),
    Int(i64),
    Float(f64),
    Var(Ident, Type),
    FunRef(Ident, usize, Type), // 新增：函数名 → label
    BinOp(BinOp, Box<RevealExpr>, Box<RevealExpr>, Type),
    Tuple(Vec<RevealExpr>, Type),
    TupleProj(Box<RevealExpr>, usize, Type),
    PrimIO(PrimIO, Option<Box<RevealExpr>>, Type),
    UnaryOp(UnaryOp, Box<RevealExpr>, Type),
    Ann(Box<RevealExpr>, Type),
    If(Box<RevealExpr>, Box<RevealExpr>, Box<RevealExpr>, Type),
    Let(Ident, Type, Box<RevealExpr>, Box<RevealExpr>, Type),
    LetRec(Ident, Vec<(Ident, Type)>, Type, Box<RevealExpr>, Box<RevealExpr>, Type),
    App(Box<RevealExpr>, Vec<RevealExpr>, Type),
    Lambda(Vec<(Ident, Type)>, Box<RevealExpr>, Type),
}

impl HasType for RevealExpr {
    fn type_of(&self) -> Type {
        match self {
            RevealExpr::Unit => Type::Unit,
            RevealExpr::Bool(_) => Type::Bool,
            RevealExpr::Int(_) => Type::Int,
            RevealExpr::Float(_) => Type::Float,
            RevealExpr::Var(_, ty) => ty.clone(),
            RevealExpr::FunRef(_, _, ty) => ty.clone(),
            RevealExpr::BinOp(_, _, _, ty) => ty.clone(),
            RevealExpr::Tuple(_, ty) => ty.clone(),
            RevealExpr::TupleProj(_, _, ty) => ty.clone(),
            RevealExpr::PrimIO(_, _, ty) => ty.clone(),
            RevealExpr::UnaryOp(_, _, ty) => ty.clone(),
            RevealExpr::Ann(_, ty) => ty.clone(),
            RevealExpr::If(_, _, _, ty) => ty.clone(),
            RevealExpr::Let(_, _, _, _, ty) => ty.clone(),
            RevealExpr::LetRec(_, _, _, _, _, ty) => ty.clone(),
            RevealExpr::App(_, _, ty) => ty.clone(),
            RevealExpr::Lambda(_, _, ty) => ty.clone(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum RevealDef {
    ValDef(Ident, Option<Type>, RevealExpr),
    FunDef(Ident, Vec<(Ident, Type)>, Type, RevealExpr),
}

impl RevealDef {
    pub fn name(&self) -> Ident {
        match self {
            RevealDef::ValDef(name, _, _) => name.to_string(),
            RevealDef::FunDef(name, _, _, _) => name.to_string(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct RevealProgram {
    pub defs: Vec<RevealDef>,
    pub main: RevealExpr,
}

pub fn r_unit() -> RevealExpr {
    RevealExpr::Unit
}

pub fn r_bool(b: bool) -> RevealExpr {
    RevealExpr::Bool(b)
}

pub fn r_int(n: i64) -> RevealExpr {
    RevealExpr::Int(n)
}

pub fn r_float(f: f64) -> RevealExpr {
    RevealExpr::Float(f)
}

pub fn r_var(name: impl Into<Ident>, ty: Type) -> RevealExpr {
    RevealExpr::Var(name.into(), ty)
}

pub fn r_fun_ref(name: impl Into<Ident>, arity: usize, ty: Type) -> RevealExpr {
    RevealExpr::FunRef(name.into(), arity, ty)
}

pub fn r_tuple(elems: Vec<RevealExpr>, ty: Type) -> RevealExpr {
    RevealExpr::Tuple(elems, ty)
}

pub fn r_tuple_projection(expr: RevealExpr, index: usize, ty: Type) -> RevealExpr {
    RevealExpr::TupleProj(Box::new(expr), index, ty)
}

pub fn r_bin_op(op: BinOp, left: RevealExpr, right: RevealExpr, ty: Type) -> RevealExpr {
    RevealExpr::BinOp(op, Box::new(left), Box::new(right), ty)
}

pub fn r_prim_io(prim: PrimIO, expr: Option<RevealExpr>, ty: Type) -> RevealExpr {
    RevealExpr::PrimIO(prim, expr.map(Box::new), ty)
}

pub fn r_unary(op: UnaryOp, expr: RevealExpr, ty: Type) -> RevealExpr {
    RevealExpr::UnaryOp(op, Box::new(expr), ty)
}

pub fn r_ann(expr: RevealExpr, ty: Type) -> RevealExpr {
    RevealExpr::Ann(Box::new(expr), ty)
}

pub fn r_if(cond: RevealExpr, thn: RevealExpr, els: RevealExpr, ty: Type) -> RevealExpr {
    RevealExpr::If(Box::new(cond), Box::new(thn), Box::new(els), ty)
}

pub fn r_let(
    name: impl Into<Ident>,
    ty: Type,
    rhs: RevealExpr,
    body: RevealExpr,
    let_ty: Type,
) -> RevealExpr {
    RevealExpr::Let(name.into(), ty, Box::new(rhs), Box::new(body), let_ty)
}

pub fn r_let_rec(
    name: impl Into<Ident>,
    params: Vec<(Ident, Type)>,
    ret_ty: Type,
    fbody: RevealExpr,
    body: RevealExpr,
    ty: Type,
) -> RevealExpr {
    RevealExpr::LetRec(name.into(), params, ret_ty, Box::new(fbody), Box::new(body), ty)
}

pub fn r_app(func: RevealExpr, args: Vec<RevealExpr>, ty: Type) -> RevealExpr {
    RevealExpr::App(Box::new(func), args, ty)
}

pub fn r_lambda(params: Vec<(Ident, Type)>, body: RevealExpr, lambda_ty: Type) -> RevealExpr {
    RevealExpr::Lambda(params, Box::new(body), lambda_ty)
}

pub fn reveal_expr(typed_expr: TypedExpr, fnames: &mut Fnames) -> RevealExpr {
    match typed_expr {
        TypedExpr::Unit => r_unit(),
        TypedExpr::Bool(b) => r_bool(b),
        TypedExpr::Int(i) => r_int(i),
        TypedExpr::Float(f) => r_float(f),
        TypedExpr::Var(name, ty) => {
            if let Some(arity) = fnames.get(&name) {
                r_fun_ref(name, *arity, ty)
            } else {
                r_var(name, ty)
            }
        },
        TypedExpr::BinOp(op, left, right, ty) => {
            let left = reveal_expr(*left, fnames);
            let right = reveal_expr(*right, fnames);
            r_bin_op(op, left, right, ty)
        },
        TypedExpr::Tuple(elems, ty) => {
            let elems = elems.into_iter().map(|e| reveal_expr(e, fnames)).collect();
            r_tuple(elems, ty)
        },
        TypedExpr::TupleProj(tuple, idx, ty) => {
            let tuple = reveal_expr(*tuple, fnames);
            r_tuple_projection(tuple, idx, ty)
        },
        TypedExpr::PrimIO(prim, expr, ty) => {
            let expr = expr.map(|e| reveal_expr(*e, fnames));
            r_prim_io(prim, expr, ty)
        },
        TypedExpr::UnaryOp(op, operand, ty) => {
            let operand = reveal_expr(*operand, fnames);
            r_unary(op, operand, ty)
        },
        TypedExpr::Ann(expr, ty) => {
            let expr = reveal_expr(*expr, fnames);
            r_ann(expr, ty)
        },
        TypedExpr::If(cond, thn, els, ty) => {
            let cond = reveal_expr(*cond, fnames);
            let thn = reveal_expr(*thn, fnames);
            let els = reveal_expr(*els, fnames);
            r_if(cond, thn, els, ty)
        },
        TypedExpr::Let(name, rhs_ty, rhs, body, let_ty) => {
            let rhs = reveal_expr(*rhs, fnames);
            let body = reveal_expr(*body, fnames);
            r_let(name, rhs_ty, rhs, body, let_ty)
        },
        TypedExpr::LetRec(fname, params, f_ret_ty, fbody, body, ty) => {
            let mut scoped = fnames.clone();
            scoped.insert(fname.clone(), params.len());
            let new_fbody = reveal_expr(*fbody, &mut scoped);
            let new_body = reveal_expr(*body, &mut scoped);
            r_let_rec(fname, params, f_ret_ty, new_fbody, new_body, ty)
        },
        TypedExpr::App(func, args, ty) => {
            let func = reveal_expr(*func, fnames);
            let args = args.into_iter().map(|a| reveal_expr(a, fnames)).collect();
            r_app(func, args, ty)
        },
        TypedExpr::Lambda(params, body, ty) => {
            let body = reveal_expr(*body, fnames);
            r_lambda(params, body, ty)
        },
    }
}

pub fn reveal_def(def: TypedDef, fnames: &mut Fnames) -> RevealDef {
    match def {
        TypedDef::ValDef(name, ty, typed_expr) => {
            let reveal = reveal_expr(typed_expr, fnames);
            RevealDef::ValDef(name, ty, reveal)
        },
        TypedDef::FunDef(name, params, ty, typed_expr) => {
            let reveal = reveal_expr(typed_expr, fnames);
            RevealDef::FunDef(name, params, ty, reveal)
        },
    }
}

pub fn reveal_program(prog: TypedProgram) -> RevealProgram {
    let mut fnames = Fnames::new();

    let mut reveal_defs = Vec::with_capacity(prog.defs.len());
    for def in prog.defs {
        if let TypedDef::FunDef(name, params, _, _) = &def {
            fnames.insert(name.clone(), params.len());
        }

        let reveal_def = reveal_def(def, &mut fnames);
        reveal_defs.push(reveal_def);
    }

    let reveal_main = reveal_expr(prog.main, &mut fnames);
    RevealProgram { defs: reveal_defs, main: reveal_main }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::syntax::{BinOp, Type, ty_arrow, ty_int};
    use crate::typechecker::{
        TypedDef, TypedExpr, TypedProgram, t_app, t_bin_op, t_bool, t_float, t_int, t_lambda,
        t_let_rec, t_unit, t_var,
    };

    fn int_to_int() -> Type {
        ty_arrow(ty_int(), ty_int())
    }

    #[test]
    fn reveals_atoms() {
        let mut fnames = Fnames::new();
        assert_eq!(reveal_expr(t_unit(), &mut fnames), r_unit());
        assert_eq!(reveal_expr(t_bool(true), &mut fnames), r_bool(true));
        assert_eq!(reveal_expr(t_int(42), &mut fnames), r_int(42));
        assert_eq!(reveal_expr(t_float(3.5), &mut fnames), r_float(3.5));
    }

    #[test]
    fn reveals_known_function_var_as_fun_ref() {
        let mut fnames = Fnames::new();
        fnames.insert("f".to_string(), 2);
        let ty = int_to_int();
        let got = reveal_expr(t_var("f", ty.clone()), &mut fnames);
        assert_eq!(got, r_fun_ref("f".to_string(), 2, ty));
    }

    #[test]
    fn keeps_unknown_var_as_var() {
        let mut fnames = Fnames::new();
        let ty = ty_int();
        let got = reveal_expr(t_var("x", ty.clone()), &mut fnames);
        assert_eq!(got, r_var("x".to_string(), ty));
    }

    #[test]
    fn reveals_bin_op_recursively() {
        let mut fnames = Fnames::new();
        let expr = t_bin_op(
            BinOp::Add,
            t_int(1),
            t_bin_op(BinOp::Mul, t_int(3), t_int(7), ty_int()),
            ty_int(),
        );
        let got = reveal_expr(expr, &mut fnames);
        assert_eq!(
            got,
            r_bin_op(
                BinOp::Add,
                r_int(1),
                r_bin_op(BinOp::Mul, r_int(3), r_int(7), ty_int()),
                ty_int(),
            )
        );
    }

    #[test]
    fn reveals_app_func_position() {
        let mut fnames = Fnames::new();
        fnames.insert("f".to_string(), 2);
        let fn_ty = int_to_int();
        let expr = t_app(t_var("f", fn_ty.clone()), vec![t_int(1)], ty_int());
        let got = reveal_expr(expr, &mut fnames);
        assert_eq!(got, r_app(r_fun_ref("f".to_string(), 2, fn_ty), vec![r_int(1)], ty_int(),));
    }

    #[test]
    fn let_rec_reveals_self_ref_but_does_not_leak() {
        let mut fnames = Fnames::new();
        let fn_ty = int_to_int();
        let fbody = t_app(
            t_var("fact", fn_ty.clone()),
            vec![t_bin_op(BinOp::Sub, t_var("n", ty_int()), t_int(1), ty_int())],
            ty_int(),
        );
        let body = t_app(t_var("fact", fn_ty.clone()), vec![t_int(1)], ty_int());
        let expr =
            t_let_rec("fact", vec![("n".to_string(), ty_int())], ty_int(), fbody, body, ty_int());

        let got = reveal_expr(expr, &mut fnames);

        match got {
            RevealExpr::LetRec(name, params, ret_ty, fbody, body, ty) => {
                assert_eq!(name, "fact".to_string());
                assert_eq!(params, vec![("n".to_string(), ty_int())]);
                assert_eq!(ret_ty, ty_int());
                assert_eq!(
                    *fbody,
                    r_app(
                        r_fun_ref("fact".to_string(), 1, fn_ty.clone()),
                        vec![r_bin_op(
                            BinOp::Sub,
                            r_var("n".to_string(), ty_int()),
                            r_int(1),
                            ty_int()
                        )],
                        ty_int(),
                    )
                );
                assert_eq!(
                    *body,
                    r_app(
                        r_fun_ref("fact".to_string(), 1, fn_ty.clone()),
                        vec![r_int(1)],
                        ty_int(),
                    )
                );
                assert_eq!(ty, ty_int());
            },
            other => panic!("unexpected reveal result: {:?}", other),
        }

        assert!(!fnames.contains_key("fact"));
    }

    #[test]
    fn reveal_def_preserves_fields() {
        let mut fnames = Fnames::new();

        let val =
            reveal_def(TypedDef::ValDef("x".to_string(), Some(ty_int()), t_int(7)), &mut fnames);
        assert_eq!(val, RevealDef::ValDef("x".to_string(), Some(ty_int()), r_int(7)));

        let fun = reveal_def(
            TypedDef::FunDef(
                "f".to_string(),
                vec![("n".to_string(), ty_int())],
                ty_int(),
                t_int(1),
            ),
            &mut fnames,
        );
        assert_eq!(
            fun,
            RevealDef::FunDef(
                "f".to_string(),
                vec![("n".to_string(), ty_int())],
                ty_int(),
                r_int(1),
            )
        );
    }

    #[test]
    fn reveal_program_reveals_fun_defs_in_main() {
        let fn_ty = int_to_int();
        let prog = TypedProgram {
            defs: vec![TypedDef::FunDef(
                "f".to_string(),
                vec![("n".to_string(), ty_int())],
                ty_int(),
                t_int(1),
            )],
            main: t_app(t_var("f", fn_ty.clone()), vec![t_int(1)], ty_int()),
        };

        let revealed = reveal_program(prog);
        assert_eq!(revealed.defs.len(), 1);
        assert_eq!(
            revealed.main,
            r_app(r_fun_ref("f".to_string(), 1, fn_ty), vec![r_int(1)], ty_int(),)
        );
    }

    #[test]
    fn reveal_program_keeps_val_def_lambda_as_var() {
        let fn_ty = int_to_int();
        let prog = TypedProgram {
            defs: vec![TypedDef::ValDef(
                "id".to_string(),
                Some(fn_ty.clone()),
                t_lambda(vec![("x".to_string(), ty_int())], t_var("x", ty_int()), fn_ty.clone()),
            )],
            main: t_var("id", fn_ty.clone()),
        };

        let revealed = reveal_program(prog);
        assert_eq!(revealed.main, r_var("id".to_string(), fn_ty));
    }
}
