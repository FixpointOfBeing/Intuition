use crate::closure_conversion::{ClosureDef, ClosureExpr, ClosureProgram, convert_fun_ptr_type};
use crate::gensym::Gensym;
use crate::syntax::HasType;
use crate::syntax::Ident;
use crate::syntax::PrimIO;
use crate::syntax::Type;
use crate::syntax::UnaryOp;
use crate::syntax::{BinOp, ty_bool, ty_float, ty_int, ty_unit};

#[derive(Debug, Clone, PartialEq)]
pub enum AllocExpr {
    Unit,
    Bool(bool),
    Int(i64),
    Float(f64),
    Var(Ident, Type),
    BinOp(BinOp, Box<AllocExpr>, Box<AllocExpr>, Type),
    Collect(usize), // bytes
    Allocate(
        usize, // bytes
        Type,
    ),
    AllocateClosure(
        usize, // bytes
        usize, // arity
        Type,
    ),
    FunRef(Ident, usize, Type),
    GlobalValue(GlobalValue),
    TupleProj(Box<AllocExpr>, usize, Type),
    TupleSet(
        Box<AllocExpr>, // tuple
        Box<AllocExpr>, // element
        usize,          // idx
    ),
    Seq(
        Vec<AllocExpr>, // sequence
        Box<AllocExpr>, // last expression
        Type,           // last expression's type
    ),
    PrimIO(PrimIO, Option<Box<AllocExpr>>, Type),
    UnaryOp(UnaryOp, Box<AllocExpr>, Type),
    If(Box<AllocExpr>, Box<AllocExpr>, Box<AllocExpr>, Type),
    Let(Ident, Type, Box<AllocExpr>, Box<AllocExpr>, Type),
    App(Box<AllocExpr>, Vec<AllocExpr>, Type),
}

impl HasType for AllocExpr {
    fn type_of(&self) -> Type {
        match self {
            AllocExpr::Unit => ty_unit(),
            AllocExpr::Bool(_) => ty_bool(),
            AllocExpr::Int(_) => ty_int(),
            AllocExpr::Float(_) => ty_float(),
            AllocExpr::Var(_, ty) => (*ty).clone(),
            AllocExpr::BinOp(_, _, _, ty) => (*ty).clone(),
            AllocExpr::Collect(_) => ty_unit(),
            AllocExpr::Allocate(_, ty) => (*ty).clone(),
            AllocExpr::AllocateClosure(_, _, ty) => (*ty).clone(),
            AllocExpr::FunRef(_, _, ty) => (*ty).clone(),
            AllocExpr::GlobalValue(gv) => gv.type_of(),
            AllocExpr::TupleProj(_, _, ty) => (*ty).clone(),
            AllocExpr::TupleSet(_, _, _) => ty_int(),
            AllocExpr::Seq(_, _, ty) => (*ty).clone(),
            AllocExpr::PrimIO(_, _, ty) => (*ty).clone(),
            AllocExpr::UnaryOp(_, _, ty) => (*ty).clone(),
            AllocExpr::If(_, _, _, ty) => (*ty).clone(),
            AllocExpr::Let(_, _, _, _, ty) => (*ty).clone(),
            AllocExpr::App(_, _, ty) => (*ty).clone(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum AllocDef {
    ValDef(
        Ident,        // name
        Option<Type>, // optional type annotation
        AllocExpr,    // expresion
    ),
    FunDef(
        Ident,              // function name
        Vec<(Ident, Type)>, // function arguments with their types
        Type,               // function return type
        AllocExpr,          // function body
    ),
}

#[derive(Debug, Clone, PartialEq)]
pub struct AllocProgram {
    pub defs: Vec<AllocDef>,
    pub main: AllocExpr,
}

#[derive(Debug, Clone, PartialEq)]
pub enum GlobalValue {
    FreePtr,
    FromspaceEnd,
}

impl HasType for GlobalValue {
    fn type_of(&self) -> Type {
        match self {
            GlobalValue::FreePtr => ty_int(),
            GlobalValue::FromspaceEnd => ty_int(),
        }
    }
}

pub fn alloc_unit() -> AllocExpr {
    AllocExpr::Unit
}

pub fn alloc_bool(b: bool) -> AllocExpr {
    AllocExpr::Bool(b)
}

pub fn alloc_int(n: i64) -> AllocExpr {
    AllocExpr::Int(n)
}

pub fn alloc_float(f: f64) -> AllocExpr {
    AllocExpr::Float(f)
}

pub fn alloc_var(name: impl Into<Ident>, ty: Type) -> AllocExpr {
    AllocExpr::Var(name.into(), ty)
}

pub fn alloc_fun_ref(name: impl Into<Ident>, arity: usize, ty: Type) -> AllocExpr {
    AllocExpr::FunRef(name.into(), arity, ty)
}

pub fn alloc_bin_op(op: BinOp, left: AllocExpr, right: AllocExpr, ty: Type) -> AllocExpr {
    AllocExpr::BinOp(op, Box::new(left), Box::new(right), ty)
}

pub fn collect(bytes: usize) -> AllocExpr {
    AllocExpr::Collect(bytes)
}

pub fn allocate(bytes: usize, ty: Type) -> AllocExpr {
    AllocExpr::Allocate(bytes, ty)
}

pub fn allocate_closure(len: usize, arity: usize, ty: Type) -> AllocExpr {
    AllocExpr::AllocateClosure(len, arity, ty)
}

pub fn global_value(value: GlobalValue) -> AllocExpr {
    AllocExpr::GlobalValue(value)
}

pub fn global_freeptr() -> AllocExpr {
    AllocExpr::GlobalValue(GlobalValue::FreePtr)
}

pub fn global_fromspace_end() -> AllocExpr {
    AllocExpr::GlobalValue(GlobalValue::FromspaceEnd)
}

pub fn alloc_tuple_set(tuple: AllocExpr, element: AllocExpr, idx: usize) -> AllocExpr {
    AllocExpr::TupleSet(Box::new(tuple), Box::new(element), idx)
}

pub fn alloc_tuple_proj(tuple: AllocExpr, idx: usize, ty: Type) -> AllocExpr {
    AllocExpr::TupleProj(Box::new(tuple), idx, ty)
}

pub fn alloc_seq(exprs: Vec<AllocExpr>, last: AllocExpr, ty: Type) -> AllocExpr {
    AllocExpr::Seq(exprs, Box::new(last), ty)
}

pub fn alloc_prim_io(prim: PrimIO, expr: Option<AllocExpr>, ty: Type) -> AllocExpr {
    AllocExpr::PrimIO(prim, expr.map(Box::new), ty)
}

pub fn alloc_unary(op: UnaryOp, expr: AllocExpr, ty: Type) -> AllocExpr {
    AllocExpr::UnaryOp(op, Box::new(expr), ty)
}

pub fn alloc_if_else(
    cond: AllocExpr,
    then_branch: AllocExpr,
    else_branch: AllocExpr,
    ty: Type,
) -> AllocExpr {
    AllocExpr::If(Box::new(cond), Box::new(then_branch), Box::new(else_branch), ty)
}

pub fn alloc_let(
    name: impl Into<Ident>,
    ty: Type,
    val: AllocExpr,
    body: AllocExpr,
    body_ty: Type,
) -> AllocExpr {
    AllocExpr::Let(name.into(), ty, Box::new(val), Box::new(body), body_ty)
}

pub fn alloc_app(func: AllocExpr, args: Vec<AllocExpr>, ty: Type) -> AllocExpr {
    AllocExpr::App(Box::new(func), args, ty)
}

// fn expose_allocate_closure(
//     try_collect: AllocExpr,
//     closure_name: Ident,
//     closure_ty: Type,
//     closure: AllocExpr,
//     allocate: AllocExpr,
//     tuple_inits: Vec<AllocExpr>,
// ) -> AllocExpr {
//     alloc_seq(
//         vec![try_collect],
//         alloc_let(
//             closure_name,
//             closure_ty.clone(),
//             allocate,
//             alloc_seq(tuple_inits, closure, closure_ty.clone()),
//             closure_ty.clone(),
//         ),
//         closure_ty,
//     )
// }

fn expose_allocate_tuple(
    try_collect: AllocExpr,
    tuple_name: Ident,
    tuple_ty: Type,
    tuple: AllocExpr,
    allocate: AllocExpr,
    tuple_inits: Vec<AllocExpr>,
) -> AllocExpr {
    alloc_seq(
        vec![try_collect],
        alloc_let(
            tuple_name,
            tuple_ty.clone(),
            allocate,
            alloc_seq(tuple_inits, tuple, tuple_ty.clone()),
            tuple_ty.clone(),
        ),
        tuple_ty,
    )
}

pub fn expose_allocation(clos_expr: ClosureExpr, gs: &mut Gensym) -> AllocExpr {
    match clos_expr {
        ClosureExpr::Unit => alloc_unit(),
        ClosureExpr::Bool(b) => alloc_bool(b),
        ClosureExpr::Int(i) => alloc_int(i),
        ClosureExpr::Float(f) => alloc_float(f),
        ClosureExpr::Var(name, clos_ty) => alloc_var(name, clos_ty.to_type()),
        ClosureExpr::BinOp(op, left, right, clos_ty) => {
            let alloc_left = expose_allocation(*left, gs);
            let alloc_right = expose_allocation(*right, gs);
            alloc_bin_op(op, alloc_left, alloc_right, clos_ty.to_type())
        },
        ClosureExpr::Closure(clos, _) => {
            // A closure is represented at runtime by a flat tuple:
            //
            //   slot 0: function pointer
            //   slot i + 1: free_vars[i]
            //
            //   bytes_needed = <size of slots>
            //
            // (Closure { func_name, func_arity, free_vars })
            // is compiled into:
            //
            //   if free_ptr + bytes_needed < fromspace_end
            //      then ()
            //      else Collect(bytes_needed);
            //   let closure = AllocateClosure(bytes_needed, func_arity) in
            //       closure[0] := FunRef(func_name, func_arity, fun_ptr_ty);
            //       closure
            let bytes_needed = 8 + clos
                .free_vars
                .iter()
                .rfold(0, |acc, (_, cty)| cty.to_type().bytes_of() + acc);

            let try_collect = {
                let ptr_after_alloc = alloc_bin_op(
                    BinOp::Add,
                    global_freeptr(),
                    alloc_int(bytes_needed as i64),
                    Type::Int,
                );
                let cond =
                    alloc_bin_op(BinOp::Lt, ptr_after_alloc, global_fromspace_end(), Type::Bool);

                alloc_if_else(cond, alloc_unit(), collect(bytes_needed), Type::Unit)
            };

            let fun_ptr_ty = {
                let fun_ptr_cty = convert_fun_ptr_type(&clos.func_ty);
                fun_ptr_cty.to_type()
            };

            let closure_name = gs.fresh_with_prefix("closure");
            let closure_ty = {
                let mut tuple_elem_tys = Vec::with_capacity(1 + clos.free_vars.len());
                tuple_elem_tys.push(fun_ptr_ty.clone());
                clos.free_vars.iter().for_each(|(_, cty)| {
                    tuple_elem_tys.push(cty.to_type());
                });
                Type::Tuple(tuple_elem_tys)
            };
            let closure = alloc_var(closure_name.clone(), closure_ty.clone());

            let allocate = allocate_closure(bytes_needed, clos.func_arity, closure_ty.clone());

            let tuple_inits = {
                let mut v = Vec::with_capacity(1 + clos.free_vars.len());
                let init_func = alloc_tuple_set(
                    closure.clone(),
                    alloc_fun_ref(clos.func_name.clone(), clos.func_arity, fun_ptr_ty),
                    0,
                );
                v.push(init_func);
                for (idx, (name, cty)) in clos.free_vars.iter().enumerate() {
                    let free_var = alloc_var(name.clone(), cty.to_type());
                    v.push(alloc_tuple_set(closure.clone(), free_var, idx + 1));
                }
                v
            };

            expose_allocate_tuple(
                try_collect,
                closure_name,
                closure_ty,
                closure,
                allocate,
                tuple_inits,
            )
        },
        ClosureExpr::ClosureFunPtr(expr, clos_ty) => {
            let alloc_expr = expose_allocation(*expr, gs);
            alloc_tuple_proj(alloc_expr, 0, clos_ty.to_type())
        },
        ClosureExpr::ClosureFreeVar(expr, idx, clos_ty) => {
            let alloc_expr = expose_allocation(*expr, gs);
            alloc_tuple_proj(alloc_expr, idx, clos_ty.to_type())
        },
        ClosureExpr::Tuple(exprs, clos_ty) => {
            // (Tuple exprs)
            //
            //   bytes_needed = <size of slots>
            //
            // is compiled into:
            //
            //   if free_ptr + bytes_needed < fromspace_end
            //      then ()
            //      else Collect(bytes_needed);
            //   let tuple = Allocate(bytes) in
            //       tuple[0] := exprs[0];
            //       tuple[1] := exprs[1];
            //       ...
            //       tuple

            let tuple_ty = clos_ty.to_type();

            let bytes_needed = tuple_ty.bytes_of();

            let try_collect = {
                let ptr_after_alloc = alloc_bin_op(
                    BinOp::Add,
                    global_freeptr(),
                    alloc_int(bytes_needed as i64),
                    Type::Int,
                );
                let cond =
                    alloc_bin_op(BinOp::Lt, ptr_after_alloc, global_fromspace_end(), Type::Bool);
                let collect = collect(bytes_needed);

                alloc_if_else(cond, alloc_unit(), collect, Type::Unit)
            };

            let allocate = allocate(bytes_needed, tuple_ty.clone());

            let tuple_name = gs.fresh_with_prefix("tuple");
            let tuple = alloc_var(tuple_name.clone(), tuple_ty.clone());

            let tuple_inits = {
                let mut v = Vec::with_capacity(exprs.len());
                let alloc_exprs = exprs
                    .into_iter()
                    .map(|e| expose_allocation(e, gs))
                    .collect::<Vec<AllocExpr>>();

                for (idx, expr) in alloc_exprs.into_iter().enumerate() {
                    let init_elem = alloc_tuple_set(tuple.clone(), expr, idx);
                    v.push(init_elem);
                }
                v
            };

            expose_allocate_tuple(try_collect, tuple_name, tuple_ty, tuple, allocate, tuple_inits)
        },
        ClosureExpr::TupleProj(expr, idx, clos_ty) => {
            let alloc_expr = expose_allocation(*expr, gs);
            alloc_tuple_proj(alloc_expr, idx, clos_ty.to_type())
        },
        ClosureExpr::PrimIO(prim_io, expr, clos_ty) => {
            let expr = expr.map(|e| expose_allocation(*e, gs));
            alloc_prim_io(prim_io, expr, clos_ty.to_type())
        },
        ClosureExpr::UnaryOp(unary_op, expr, clos_ty) => {
            let alloc_expr = expose_allocation(*expr, gs);
            alloc_unary(unary_op, alloc_expr, clos_ty.to_type())
        },
        ClosureExpr::If(cond, thn, els, clos_ty) => {
            let alloc_cond = expose_allocation(*cond, gs);
            let alloc_thn = expose_allocation(*thn, gs);
            let alloc_els = expose_allocation(*els, gs);
            alloc_if_else(alloc_cond, alloc_thn, alloc_els, clos_ty.to_type())
        },
        ClosureExpr::Let(name, rhs_clos_ty, rhs, body, body_clos_ty) => {
            let alloc_rhs = expose_allocation(*rhs, gs);
            let alloc_body = expose_allocation(*body, gs);
            alloc_let(name, rhs_clos_ty.to_type(), alloc_rhs, alloc_body, body_clos_ty.to_type())
        },
        ClosureExpr::App(func, args, clos_ty) => {
            let alloc_func = expose_allocation(*func, gs);
            let alloc_args = args.into_iter().map(|e| expose_allocation(e, gs)).collect();
            alloc_app(alloc_func, alloc_args, clos_ty.to_type())
        },
    }
}

pub fn expose_allocation_def(def: ClosureDef, gs: &mut Gensym) -> AllocDef {
    match def {
        ClosureDef::ValDef(name, clos_ty, clos_expr) => AllocDef::ValDef(
            name,
            clos_ty.map(|cty| cty.to_type()),
            expose_allocation(clos_expr, gs),
        ),
        ClosureDef::FunDef(name, params, ret_ty, clos_expr) => AllocDef::FunDef(
            name,
            params
                .into_iter()
                .map(|(name, cty)| (name, cty.to_type()))
                .collect::<Vec<_>>(),
            ret_ty.to_type(),
            expose_allocation(clos_expr, gs),
        ),
    }
}

pub fn expose_allocation_prog(prog: ClosureProgram) -> AllocProgram {
    let mut defs = Vec::with_capacity(prog.defs.len());
    let mut gs = Gensym::new();
    for def in prog.defs {
        defs.push(expose_allocation_def(def, &mut gs));
    }

    let main = expose_allocation(prog.main, &mut gs);
    AllocProgram { defs, main }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::closure_conversion::{
        Closure, ClosureDef, ClosureProgram, ClosureType, convert_fun_ptr_type,
    };
    use crate::syntax::{BinOp, PrimIO, Type, UnaryOp, ty_arrow, ty_int};

    fn run(expr: ClosureExpr) -> AllocExpr {
        expose_allocation(expr, &mut Gensym::new())
    }

    fn run_def(def: ClosureDef) -> AllocDef {
        expose_allocation_def(def, &mut Gensym::new())
    }

    fn closure_expr(
        name: &str,
        arity: usize,
        func_ty: Type,
        free_vars: Vec<(&str, ClosureType)>,
    ) -> ClosureExpr {
        let clos_ty = ClosureType::Closure(Box::new(func_ty.clone()));
        ClosureExpr::Closure(
            Closure {
                func_arity: arity,
                func_name: name.to_string(),
                func_ty,
                free_vars: free_vars
                    .into_iter()
                    .map(|(name, ty)| (name.to_string(), ty))
                    .collect(),
            },
            clos_ty,
        )
    }

    fn expect_seq(expr: &AllocExpr) -> (&[AllocExpr], &AllocExpr, &Type) {
        match expr {
            AllocExpr::Seq(items, last, ty) => (items, last, ty),
            other => panic!("expected Seq, got {:?}", other),
        }
    }

    fn expect_let(expr: &AllocExpr) -> (&Ident, &Type, &AllocExpr, &AllocExpr, &Type) {
        match expr {
            AllocExpr::Let(name, ty, val, body, body_ty) => (name, ty, val, body, body_ty),
            other => panic!("expected Let, got {:?}", other),
        }
    }

    fn expect_tuple_set(expr: &AllocExpr) -> (&AllocExpr, &AllocExpr, usize) {
        match expr {
            AllocExpr::TupleSet(tuple, element, idx) => (tuple, element, *idx),
            other => panic!("expected TupleElemInit, got {:?}", other),
        }
    }

    #[test]
    fn exposes_literals_and_vars() {
        assert_eq!(run(ClosureExpr::Unit), AllocExpr::Unit);
        assert_eq!(run(ClosureExpr::Bool(true)), AllocExpr::Bool(true));
        assert_eq!(run(ClosureExpr::Int(42)), AllocExpr::Int(42));
        assert_eq!(run(ClosureExpr::Float(1.5)), AllocExpr::Float(1.5));
        assert_eq!(
            run(ClosureExpr::Var("x".to_string(), ClosureType::Bool)),
            AllocExpr::Var("x".to_string(), Type::Bool)
        );
    }

    #[test]
    fn exposes_operations_and_projections() {
        assert_eq!(
            run(ClosureExpr::BinOp(
                BinOp::Add,
                Box::new(ClosureExpr::Int(1)),
                Box::new(ClosureExpr::Int(2)),
                ClosureType::Int,
            )),
            AllocExpr::BinOp(
                BinOp::Add,
                Box::new(AllocExpr::Int(1)),
                Box::new(AllocExpr::Int(2)),
                Type::Int,
            )
        );

        assert_eq!(
            run(ClosureExpr::UnaryOp(
                UnaryOp::Neg,
                Box::new(ClosureExpr::Int(3)),
                ClosureType::Int,
            )),
            AllocExpr::UnaryOp(UnaryOp::Neg, Box::new(AllocExpr::Int(3)), Type::Int)
        );

        assert_eq!(
            run(ClosureExpr::TupleProj(
                Box::new(ClosureExpr::Var(
                    "t".to_string(),
                    ClosureType::Tuple(vec![ClosureType::Int, ClosureType::Bool]),
                )),
                1,
                ClosureType::Bool,
            )),
            AllocExpr::TupleProj(
                Box::new(
                    AllocExpr::Var("t".to_string(), Type::Tuple(vec![Type::Int, Type::Bool]),)
                ),
                1,
                Type::Bool,
            )
        );
    }

    #[test]
    fn exposes_closure_accessors() {
        let func_ty = ty_arrow(ty_int(), ty_int());

        assert_eq!(
            run(ClosureExpr::ClosureFunPtr(
                Box::new(ClosureExpr::Var(
                    "c".to_string(),
                    ClosureType::Closure(Box::new(func_ty.clone())),
                )),
                ClosureType::Int,
            )),
            AllocExpr::TupleProj(
                Box::new(AllocExpr::Var("c".to_string(), func_ty.clone())),
                0,
                Type::Int,
            )
        );

        assert_eq!(
            run(ClosureExpr::ClosureFreeVar(
                Box::new(ClosureExpr::Var(
                    "c".to_string(),
                    ClosureType::Closure(Box::new(func_ty)),
                )),
                2,
                ClosureType::Bool,
            )),
            AllocExpr::TupleProj(
                Box::new(AllocExpr::Var("c".to_string(), ty_arrow(ty_int(), ty_int()))),
                2,
                Type::Bool,
            )
        );
    }

    #[test]
    fn exposes_tuple_allocation_shape() {
        let tuple_ty = Type::Tuple(vec![Type::Int, Type::Bool]);
        let expr = ClosureExpr::Tuple(
            vec![ClosureExpr::Int(1), ClosureExpr::Bool(true)],
            ClosureType::Tuple(vec![ClosureType::Int, ClosureType::Bool]),
        );

        let result = run(expr);
        let (collects, last, result_ty) = expect_seq(&result);

        assert_eq!(collects.len(), 1);
        assert!(matches!(&collects[0], AllocExpr::If(_, _, _, _)));
        assert_eq!(result_ty, &tuple_ty);

        let (name, bound_ty, allocate, inits_seq, body_ty) = expect_let(last);
        assert!(name.starts_with("tuple$"));
        assert_eq!(bound_ty, &tuple_ty);
        assert_eq!(body_ty, &tuple_ty);
        assert_eq!(allocate, &AllocExpr::Allocate(tuple_ty.bytes_of(), tuple_ty.clone()));

        let (inits, tuple_var, inits_ty) = expect_seq(inits_seq);
        assert_eq!(inits_ty, &tuple_ty);
        assert_eq!(inits.len(), 2);

        let (_, element, idx) = expect_tuple_set(&inits[0]);
        assert_eq!(idx, 0);
        assert_eq!(element, &AllocExpr::Int(1));

        let (_, element, idx) = expect_tuple_set(&inits[1]);
        assert_eq!(idx, 1);
        assert_eq!(element, &AllocExpr::Bool(true));

        assert_eq!(tuple_var, &AllocExpr::Var(name.clone(), tuple_ty));
    }

    #[test]
    fn exposes_closure_allocation_shape() {
        let func_ty = ty_arrow(ty_int(), ty_int());
        let fun_ptr_ty = convert_fun_ptr_type(&func_ty).to_type();
        let closure_ty = Type::Tuple(vec![fun_ptr_ty.clone(), Type::Int]);
        let expr = closure_expr("lambda$0", 1, func_ty, vec![("x", ClosureType::Int)]);

        let result = run(expr);
        let (collects, last, result_ty) = expect_seq(&result);

        assert_eq!(collects.len(), 1);
        assert!(matches!(&collects[0], AllocExpr::If(_, _, _, _)));
        assert_eq!(result_ty, &closure_ty);

        let (name, bound_ty, allocate, inits_seq, body_ty) = expect_let(last);
        assert!(name.starts_with("closure$"));
        assert_eq!(bound_ty, &closure_ty);
        assert_eq!(body_ty, &closure_ty);
        assert_eq!(allocate, &AllocExpr::AllocateClosure(16, 1, closure_ty.clone()));

        let (inits, closure_var, inits_ty) = expect_seq(inits_seq);
        assert_eq!(inits_ty, &closure_ty);
        assert_eq!(inits.len(), 2);

        let (_, element, idx) = expect_tuple_set(&inits[0]);
        assert_eq!(idx, 0);
        assert_eq!(element, &AllocExpr::FunRef("lambda$0".to_string(), 1, fun_ptr_ty));

        let (_, element, idx) = expect_tuple_set(&inits[1]);
        assert_eq!(idx, 1);
        assert_eq!(element, &AllocExpr::Var("x".to_string(), Type::Int));

        assert_eq!(closure_var, &AllocExpr::Var(name.clone(), closure_ty));
    }

    #[test]
    fn exposes_closure_without_free_vars_allocation_shape() {
        let func_ty = ty_arrow(ty_int(), ty_int());
        let fun_ptr_ty = convert_fun_ptr_type(&func_ty).to_type();
        let closure_ty = Type::Tuple(vec![fun_ptr_ty.clone()]);
        let expr = closure_expr("lambda$1", 2, func_ty, vec![]);

        let result = run(expr);
        let (_, last, _) = expect_seq(&result);
        let (_, _, allocate, inits_seq, _) = expect_let(last);

        assert_eq!(allocate, &AllocExpr::AllocateClosure(8, 2, closure_ty.clone()));

        let (inits, _, _) = expect_seq(inits_seq);
        assert_eq!(inits.len(), 1);
        let (_, element, idx) = expect_tuple_set(&inits[0]);
        assert_eq!(idx, 0);
        assert_eq!(element, &AllocExpr::FunRef("lambda$1".to_string(), 2, fun_ptr_ty));
    }

    #[test]
    fn exposes_closure_with_many_free_vars_allocation_shape() {
        let func_ty = ty_arrow(ty_int(), ty_int());
        let fun_ptr_ty = convert_fun_ptr_type(&func_ty).to_type();
        let tuple_free_ty = Type::Tuple(vec![Type::Int, Type::Bool]);
        let closure_ty =
            Type::Tuple(vec![fun_ptr_ty.clone(), Type::Int, Type::Bool, tuple_free_ty.clone()]);
        let expr = closure_expr(
            "lambda$2",
            1,
            func_ty,
            vec![
                ("x", ClosureType::Int),
                ("b", ClosureType::Bool),
                ("t", ClosureType::Tuple(vec![ClosureType::Int, ClosureType::Bool])),
            ],
        );

        let result = run(expr);
        let (_, last, _) = expect_seq(&result);
        let (_, _, allocate, inits_seq, _) = expect_let(last);

        assert_eq!(allocate, &AllocExpr::AllocateClosure(48, 1, closure_ty.clone()));

        let (inits, _, _) = expect_seq(inits_seq);
        assert_eq!(inits.len(), 4);

        let (_, element, idx) = expect_tuple_set(&inits[0]);
        assert_eq!(idx, 0);
        assert_eq!(element, &AllocExpr::FunRef("lambda$2".to_string(), 1, fun_ptr_ty));

        let (_, element, idx) = expect_tuple_set(&inits[1]);
        assert_eq!(idx, 1);
        assert_eq!(element, &AllocExpr::Var("x".to_string(), Type::Int));

        let (_, element, idx) = expect_tuple_set(&inits[2]);
        assert_eq!(idx, 2);
        assert_eq!(element, &AllocExpr::Var("b".to_string(), Type::Bool));

        let (_, element, idx) = expect_tuple_set(&inits[3]);
        assert_eq!(idx, 3);
        assert_eq!(element, &AllocExpr::Var("t".to_string(), tuple_free_ty));
    }

    #[test]
    fn exposes_let_if_app_and_prim_io() {
        assert_eq!(
            run(ClosureExpr::Let(
                "x".to_string(),
                ClosureType::Int,
                Box::new(ClosureExpr::Int(1)),
                Box::new(ClosureExpr::Var("x".to_string(), ClosureType::Int)),
                ClosureType::Int,
            )),
            AllocExpr::Let(
                "x".to_string(),
                Type::Int,
                Box::new(AllocExpr::Int(1)),
                Box::new(AllocExpr::Var("x".to_string(), Type::Int)),
                Type::Int,
            )
        );

        assert_eq!(
            run(ClosureExpr::If(
                Box::new(ClosureExpr::Bool(true)),
                Box::new(ClosureExpr::Int(1)),
                Box::new(ClosureExpr::Int(2)),
                ClosureType::Int,
            )),
            AllocExpr::If(
                Box::new(AllocExpr::Bool(true)),
                Box::new(AllocExpr::Int(1)),
                Box::new(AllocExpr::Int(2)),
                Type::Int,
            )
        );

        let func_ty = ty_arrow(ty_int(), ty_int());
        assert_eq!(
            run(ClosureExpr::App(
                Box::new(ClosureExpr::Var(
                    "f".to_string(),
                    ClosureType::Closure(Box::new(func_ty.clone())),
                )),
                vec![ClosureExpr::Int(3)],
                ClosureType::Int,
            )),
            AllocExpr::App(
                Box::new(AllocExpr::Var("f".to_string(), func_ty)),
                vec![AllocExpr::Int(3)],
                Type::Int,
            )
        );

        assert_eq!(
            run(ClosureExpr::PrimIO(
                PrimIO::PrintInt,
                Some(Box::new(ClosureExpr::Int(7))),
                ClosureType::Unit,
            )),
            AllocExpr::PrimIO(PrimIO::PrintInt, Some(Box::new(AllocExpr::Int(7))), Type::Unit)
        );
    }

    #[test]
    fn exposes_val_def_with_annotation() {
        assert_eq!(
            run_def(ClosureDef::ValDef(
                "x".to_string(),
                Some(ClosureType::Int),
                ClosureExpr::Int(42),
            )),
            AllocDef::ValDef("x".to_string(), Some(Type::Int), AllocExpr::Int(42))
        );
    }

    #[test]
    fn exposes_val_def_without_annotation() {
        assert_eq!(
            run_def(ClosureDef::ValDef("b".to_string(), None, ClosureExpr::Bool(true))),
            AllocDef::ValDef("b".to_string(), None, AllocExpr::Bool(true))
        );
    }

    #[test]
    fn exposes_val_def_converts_annotations() {
        let tuple_ty = ClosureType::Tuple(vec![ClosureType::Int, ClosureType::Bool]);
        assert_eq!(
            run_def(ClosureDef::ValDef("t".to_string(), Some(tuple_ty), ClosureExpr::Int(0))),
            AllocDef::ValDef(
                "t".to_string(),
                Some(Type::Tuple(vec![Type::Int, Type::Bool])),
                AllocExpr::Int(0),
            )
        );

        // A closure-typed annotation is translated back to its source function type.
        let func_ty = ty_arrow(ty_int(), ty_int());
        assert_eq!(
            run_def(ClosureDef::ValDef(
                "f".to_string(),
                Some(ClosureType::Closure(Box::new(func_ty.clone()))),
                ClosureExpr::Int(0),
            )),
            AllocDef::ValDef("f".to_string(), Some(func_ty), AllocExpr::Int(0))
        );
    }

    #[test]
    fn exposes_fun_def_converts_params_and_return_type() {
        assert_eq!(
            run_def(ClosureDef::FunDef(
                "id".to_string(),
                vec![("x".to_string(), ClosureType::Int)],
                ClosureType::Int,
                ClosureExpr::Var("x".to_string(), ClosureType::Int),
            )),
            AllocDef::FunDef(
                "id".to_string(),
                vec![("x".to_string(), Type::Int)],
                Type::Int,
                AllocExpr::Var("x".to_string(), Type::Int),
            )
        );

        // A closure-typed parameter is translated back to its source function type.
        let func_ty = ty_arrow(ty_int(), ty_int());
        assert_eq!(
            run_def(ClosureDef::FunDef(
                "apply".to_string(),
                vec![("f".to_string(), ClosureType::Closure(Box::new(func_ty.clone())))],
                ClosureType::Int,
                ClosureExpr::Var("f".to_string(), ClosureType::Closure(Box::new(func_ty.clone()))),
            )),
            AllocDef::FunDef(
                "apply".to_string(),
                vec![("f".to_string(), func_ty.clone())],
                Type::Int,
                AllocExpr::Var("f".to_string(), func_ty),
            )
        );
    }

    #[test]
    fn exposes_def_expression_through_expose_allocation() {
        let tuple_ty = ClosureType::Tuple(vec![ClosureType::Int, ClosureType::Bool]);
        let expr = ClosureExpr::Tuple(
            vec![ClosureExpr::Int(1), ClosureExpr::Bool(true)],
            tuple_ty.clone(),
        );

        let result =
            run_def(ClosureDef::ValDef("t".to_string(), Some(tuple_ty.clone()), expr.clone()));
        let expected = expose_allocation(expr, &mut Gensym::new());

        assert_eq!(result, AllocDef::ValDef("t".to_string(), Some(tuple_ty.to_type()), expected));
    }

    #[test]
    fn exposes_program_maps_defs_and_main() {
        let prog = ClosureProgram {
            defs: vec![
                ClosureDef::ValDef("x".to_string(), Some(ClosureType::Int), ClosureExpr::Int(1)),
                ClosureDef::FunDef(
                    "inc".to_string(),
                    vec![("n".to_string(), ClosureType::Int)],
                    ClosureType::Int,
                    ClosureExpr::BinOp(
                        BinOp::Add,
                        Box::new(ClosureExpr::Var("n".to_string(), ClosureType::Int)),
                        Box::new(ClosureExpr::Int(1)),
                        ClosureType::Int,
                    ),
                ),
            ],
            main: ClosureExpr::Int(0),
        };

        assert_eq!(
            expose_allocation_prog(prog),
            AllocProgram {
                defs: vec![
                    AllocDef::ValDef("x".to_string(), Some(Type::Int), AllocExpr::Int(1)),
                    AllocDef::FunDef(
                        "inc".to_string(),
                        vec![("n".to_string(), Type::Int)],
                        Type::Int,
                        AllocExpr::BinOp(
                            BinOp::Add,
                            Box::new(AllocExpr::Var("n".to_string(), Type::Int)),
                            Box::new(AllocExpr::Int(1)),
                            Type::Int,
                        ),
                    ),
                ],
                main: AllocExpr::Int(0),
            }
        );
    }

    #[test]
    fn exposes_program_with_no_defs() {
        let alloc_prog =
            expose_allocation_prog(ClosureProgram { defs: vec![], main: ClosureExpr::Int(7) });
        assert_eq!(alloc_prog, AllocProgram { defs: vec![], main: AllocExpr::Int(7) });
    }

    #[test]
    fn exposes_program_shares_gensym_across_defs_and_main() {
        // Each tuple allocation draws exactly one fresh name, so the names must
        // keep increasing across defs and then the main expression.
        let tuple_ty = ClosureType::Tuple(vec![ClosureType::Int]);
        let tuple_expr = || ClosureExpr::Tuple(vec![ClosureExpr::Int(1)], tuple_ty.clone());

        let prog = ClosureProgram {
            defs: vec![
                ClosureDef::ValDef("a".to_string(), Some(tuple_ty.clone()), tuple_expr()),
                ClosureDef::ValDef("b".to_string(), Some(tuple_ty.clone()), tuple_expr()),
            ],
            main: tuple_expr(),
        };

        let alloc_prog = expose_allocation_prog(prog);

        fn bound_tuple_name(def: &AllocDef) -> Ident {
            match def {
                AllocDef::ValDef(_, _, AllocExpr::Seq(_, last, _)) => match &**last {
                    AllocExpr::Let(name, _, _, _, _) => name.clone(),
                    other => panic!("expected Let, got {:?}", other),
                },
                other => panic!("expected ValDef wrapping Seq, got {:?}", other),
            }
        }

        let main_name = match &alloc_prog.main {
            AllocExpr::Seq(_, last, _) => match &**last {
                AllocExpr::Let(name, _, _, _, _) => name.clone(),
                other => panic!("expected Let, got {:?}", other),
            },
            other => panic!("expected Seq, got {:?}", other),
        };

        assert_eq!(bound_tuple_name(&alloc_prog.defs[0]), "tuple$0");
        assert_eq!(bound_tuple_name(&alloc_prog.defs[1]), "tuple$1");
        assert_eq!(main_name, "tuple$2");
    }
}
