use std::collections::HashSet;

use crate::{
    gensym::Gensym,
    syntax::{BinOp, HasType, Ident, PrimIO, Type, UnaryOp, ty_arrow, ty_dummy, ty_tuple},
    typechecker::TypedExpr,
};

#[derive(Debug, Clone, PartialEq)]
pub struct Closure {
    arity: usize,
    func_name: Ident,
    func_ty: Type,
    free_vars: Vec<(Ident, Type)>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ClosureExpr {
    Unit,
    Bool(bool),
    Int(i64),
    Float(f64),
    Var(Ident, Type),
    BinOp(BinOp, Box<ClosureExpr>, Box<ClosureExpr>, Type),
    Closure(Closure, Type),
    Tuple(Vec<ClosureExpr>, Type),
    TupleProj(Box<ClosureExpr>, usize, Type),
    PrimIO(PrimIO, Option<Box<ClosureExpr>>, Type),
    UnaryOp(UnaryOp, Box<ClosureExpr>, Type),
    Ann(Box<ClosureExpr>, Type),
    If(Box<ClosureExpr>, Box<ClosureExpr>, Box<ClosureExpr>, Type),
    Let(
        Ident,
        Type, // rhs's type
        Box<ClosureExpr>,
        Box<ClosureExpr>,
        Type,
    ),
    LetRec(
        Ident,              // function name
        Vec<(Ident, Type)>, // function parameters with their types
        Type,               // function return type
        Box<ClosureExpr>,   // function body
        Box<ClosureExpr>,   // expression after the let rec
        Type,
    ),
    App(Box<ClosureExpr>, Vec<ClosureExpr>, Type),
}

impl HasType for ClosureExpr {
    fn type_of(&self) -> Type {
        match self {
            ClosureExpr::Unit => todo!(),
            ClosureExpr::Bool(_) => todo!(),
            ClosureExpr::Int(_) => todo!(),
            ClosureExpr::Float(_) => todo!(),
            ClosureExpr::Var(_, _) => todo!(),
            ClosureExpr::BinOp(bin_op, closure_expr, closure_expr1, _) => todo!(),
            ClosureExpr::Closure(_, ty) => (*ty).clone(),
            ClosureExpr::Tuple(closure_exprs, _) => todo!(),
            ClosureExpr::TupleProj(closure_expr, _, _) => todo!(),
            ClosureExpr::PrimIO(prim_io, closure_expr, _) => todo!(),
            ClosureExpr::UnaryOp(unary_op, closure_expr, _) => todo!(),
            ClosureExpr::Ann(closure_expr, _) => todo!(),
            ClosureExpr::If(closure_expr, closure_expr1, closure_expr2, _) => todo!(),
            ClosureExpr::Let(_, _, closure_expr, closure_expr1, _) => todo!(),
            ClosureExpr::LetRec(_, items, _, closure_expr, closure_expr1, _) => todo!(),
            ClosureExpr::App(closure_expr, closure_exprs, _) => todo!(),
        }
    }
}

pub fn clos_unit() -> ClosureExpr {
    ClosureExpr::Unit
}

pub fn clos_bool(b: bool) -> ClosureExpr {
    ClosureExpr::Bool(b)
}

pub fn clos_int(n: i64) -> ClosureExpr {
    ClosureExpr::Int(n)
}

pub fn clos_float(f: f64) -> ClosureExpr {
    ClosureExpr::Float(f)
}

pub fn clos_clos(clos: Closure, ty: Type) -> ClosureExpr {
    ClosureExpr::Closure(clos, ty)
}

pub fn clos_var(name: impl Into<Ident>, ty: Type) -> ClosureExpr {
    ClosureExpr::Var(name.into(), ty)
}

pub fn clos_tuple(elems: Vec<ClosureExpr>, ty: Type) -> ClosureExpr {
    ClosureExpr::Tuple(elems, ty)
}

pub fn clos_tuple_projection(expr: ClosureExpr, index: usize, ty: Type) -> ClosureExpr {
    ClosureExpr::TupleProj(Box::new(expr), index, ty)
}

pub fn clos_bin_op(op: BinOp, left: ClosureExpr, right: ClosureExpr, ty: Type) -> ClosureExpr {
    ClosureExpr::BinOp(op, Box::new(left), Box::new(right), ty)
}

pub fn clos_prim_io(prim: PrimIO, expr: Option<ClosureExpr>, ty: Type) -> ClosureExpr {
    ClosureExpr::PrimIO(prim, expr.map(Box::new), ty)
}

pub fn clos_unary(op: UnaryOp, expr: ClosureExpr, ty: Type) -> ClosureExpr {
    ClosureExpr::UnaryOp(op, Box::new(expr), ty)
}

pub fn clos_ann(expr: ClosureExpr, ty: Type) -> ClosureExpr {
    ClosureExpr::Ann(Box::new(expr), ty)
}

pub fn clos_if(cond: ClosureExpr, thn: ClosureExpr, els: ClosureExpr, ty: Type) -> ClosureExpr {
    ClosureExpr::If(Box::new(cond), Box::new(thn), Box::new(els), ty)
}

pub fn clos_let(
    name: impl Into<Ident>,
    ty: Type,
    rhs: ClosureExpr,
    body: ClosureExpr,
    let_ty: Type,
) -> ClosureExpr {
    ClosureExpr::Let(name.into(), ty, Box::new(rhs), Box::new(body), let_ty)
}

pub fn clos_let_rec(
    name: impl Into<Ident>,
    params: Vec<(Ident, Type)>,
    ret_ty: Type,
    fbody: ClosureExpr,
    body: ClosureExpr,
    ty: Type,
) -> ClosureExpr {
    ClosureExpr::LetRec(name.into(), params, ret_ty, Box::new(fbody), Box::new(body), ty)
}

pub fn clos_app(func: ClosureExpr, args: Vec<ClosureExpr>, ty: Type) -> ClosureExpr {
    ClosureExpr::App(Box::new(func), args, ty)
}
#[derive(Debug, Clone, PartialEq)]
pub enum ClosureDef {
    ValDef(
        Ident,        // name
        Option<Type>, // optional type annotation
        ClosureExpr,  // expresion
    ),
    FunDef(
        Ident,              // function name
        Vec<(Ident, Type)>, // function arguments with their types
        Type,               // function return type
        ClosureExpr,        // function body
    ),
}

#[derive(Debug, Clone, PartialEq)]
pub struct ClosureProgram {
    defs: Vec<ClosureDef>,
    main: ClosureExpr,
}

fn func_type_arity(ty: &Type) -> usize {
    match ty {
        Type::Arrow(_, to) => 1 + func_type_arity(&**to),
        _ => 0,
    }
}

fn make_closure_type(ty: &Type) -> Type {
    assert!(ty.is_func_type());
    ty_tuple(vec![ty_arrow(ty_tuple(vec![ty_dummy()]), (*ty).clone())])
}
pub fn convert_to_closure(
    typed_expr: TypedExpr,
    gensym: &mut Gensym,
    top_level_defs: &mut Vec<ClosureDef>,
) -> ClosureExpr {
    match typed_expr {
        TypedExpr::Unit => todo!(),
        TypedExpr::Bool(_) => todo!(),
        TypedExpr::Int(_) => todo!(),
        TypedExpr::Float(_) => todo!(),
        TypedExpr::Var(name, ty) => {
            if ty.is_func_type() {
                let clos_ty = make_closure_type(&ty);
                clos_clos(
                    Closure {
                        arity: func_type_arity(&ty),
                        func_name: name,
                        func_ty: ty,
                        free_vars: vec![],
                    },
                    clos_ty,
                )
            } else {
                clos_var(name, ty)
            }
        },
        TypedExpr::BinOp(bin_op, typed_expr, typed_expr1, _) => todo!(),
        TypedExpr::Tuple(typed_exprs, _) => todo!(),
        TypedExpr::TupleProj(typed_expr, _, _) => todo!(),
        TypedExpr::PrimIO(prim_io, typed_expr, _) => todo!(),
        TypedExpr::UnaryOp(unary_op, typed_expr, _) => todo!(),
        TypedExpr::Ann(typed_expr, _) => todo!(),
        TypedExpr::If(typed_expr, typed_expr1, typed_expr2, _) => todo!(),
        TypedExpr::Let(_, _, typed_expr, typed_expr1, _) => todo!(),
        TypedExpr::LetRec(_, items, _, typed_expr, typed_expr1, _) => todo!(),
        TypedExpr::App(func, args, ty) => {
            let clos_func = convert_to_closure(*func, gensym, top_level_defs);

            let func_ty = clos_func.type_of();
            // assert!(matches!(func_ty, Type::Arrow(Type::Tuple(Type::Dummy), _)));
            let tmp = gensym.fresh_with_prefix("$tmp");
            let tmp_var = clos_var(tmp.clone(), func_ty.clone());

            let new_func = clos_tuple_projection(
                tmp_var.clone(),
                0,
                ty_arrow(ty_tuple(vec![ty_dummy()]), ty.clone()),
            );

            let mut new_args = Vec::with_capacity(args.len() + 1);
            new_args.push(tmp_var.clone());
            for arg in args {
                new_args.push(convert_to_closure(arg, gensym, top_level_defs));
            }
            clos_let(tmp, func_ty, clos_func, clos_app(new_func, new_args, ty.clone()), ty)
        },
        TypedExpr::Lambda(params, body, lambda_ty) => {
            let bound = params
                .iter()
                .map(|(name, _)| name.to_string())
                .collect::<HashSet<Ident>>();
            let frees = free_vars(&*body, &bound);

            let clos_param_name = gensym.fresh_with_prefix("$clos");
            let clos_param_ty = {
                let mut free_tys = frees.iter().map(|(_, ty)| (*ty).clone()).collect::<Vec<Type>>();
                free_tys.insert(0, Type::Dummy);
                Type::Tuple(free_tys)
            };
            let clos_param = clos_var(clos_param_name.clone(), clos_param_ty.clone());

            let mut new_body = convert_to_closure(*body, gensym, top_level_defs);
            let rt_ty = new_body.type_of();

            new_body = frees.iter().enumerate().rfold(new_body, |acc, (idx, (name, ty))| {
                let rhs = clos_tuple_projection(clos_param.clone(), idx + 1, (*ty).clone());
                clos_let(name.to_string(), (*ty).clone(), rhs, acc, rt_ty.clone())
            });

            let def_name = gensym.fresh_with_prefix("$lambda");
            let mut def_params = Vec::with_capacity(params.len() + 1);
            def_params.push((clos_param_name, clos_param_ty));
            for param_ty in &params {
                def_params.push(param_ty.clone());
            }
            let def = ClosureDef::FunDef(def_name.clone(), def_params, rt_ty, new_body);
            top_level_defs.push(def);

            let arity = params.len();

            let clos_ty = make_closure_type(&lambda_ty);
            ClosureExpr::Closure(
                Closure { arity, func_name: def_name, func_ty: lambda_ty, free_vars: frees },
                clos_ty,
            )
        },
    }
}

fn free_vars(expr: &TypedExpr, bound: &HashSet<Ident>) -> Vec<(Ident, Type)> {
    todo!()
}
