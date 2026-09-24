use crate::gensym::Gensym;
use crate::syntax::BinOp;
use crate::syntax::Ident;
use crate::syntax::PrimIO;
use crate::syntax::Type;
use crate::syntax::UnaryOp;
use crate::typechecker::TypedExpr;

#[derive(Debug, Clone, PartialEq)]
pub enum AllocExpr {
    Unit,
    Bool(bool),
    Int(i64),
    Float(f64),
    Var(Ident, Type),
    BinOp(BinOp, Box<AllocExpr>, Box<AllocExpr>, Type),
    Collect(usize), // collect bytes
    Allocate(usize),
    GlobalValue(GlobalValue),
    TupleProj(Box<AllocExpr>, usize, Type),
    TupleElemInit(
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
    LetRec(
        Ident,              // function name
        Vec<(Ident, Type)>, // function parameters with their types
        Type,               // function return type
        Box<AllocExpr>,     // function body
        Box<AllocExpr>,     // expression after the let rec
        Type,
    ),
    App(Box<AllocExpr>, Vec<AllocExpr>, Type),
    Lambda(Vec<(Ident, Type)>, Type, Box<AllocExpr>, Type),
}

#[derive(Debug, Clone, PartialEq)]
pub enum GlobalValue {
    FreePtr,
    FromspaceEnd,
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

pub fn alloc_bin_op(op: BinOp, left: AllocExpr, right: AllocExpr, ty: Type) -> AllocExpr {
    AllocExpr::BinOp(op, Box::new(left), Box::new(right), ty)
}

pub fn collect(bytes: usize) -> AllocExpr {
    AllocExpr::Collect(bytes)
}

pub fn allocate(bytes: usize) -> AllocExpr {
    AllocExpr::Allocate(bytes)
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

pub fn alloc_tuple_elem_init(tuple: AllocExpr, element: AllocExpr, idx: usize) -> AllocExpr {
    AllocExpr::TupleElemInit(Box::new(tuple), Box::new(element), idx)
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

pub fn alloc_let_rec(
    name: impl Into<Ident>,
    params: Vec<(Ident, Type)>,
    ret_ty: Type,
    body: AllocExpr,
    next: AllocExpr,
    next_ty: Type,
) -> AllocExpr {
    AllocExpr::LetRec(name.into(), params, ret_ty, Box::new(body), Box::new(next), next_ty)
}

pub fn alloc_app(func: AllocExpr, args: Vec<AllocExpr>, ty: Type) -> AllocExpr {
    AllocExpr::App(Box::new(func), args, ty)
}

pub fn alloc_lambda(
    params: Vec<(Ident, Type)>,
    ret_ty: Type,
    body: AllocExpr,
    ty: Type,
) -> AllocExpr {
    AllocExpr::Lambda(params, ret_ty, Box::new(body), ty)
}

pub fn expose_allocation(typed_expr: TypedExpr, gs: &mut Gensym) -> AllocExpr {
    match typed_expr {
        TypedExpr::Unit => alloc_unit(),
        TypedExpr::Bool(b) => alloc_bool(b),
        TypedExpr::Int(n) => alloc_int(n),
        TypedExpr::Float(f) => alloc_float(f),
        TypedExpr::Var(name, ty) => alloc_var(name, ty),
        TypedExpr::BinOp(bin_op, typed_expr, typed_expr1, ty) => {
            let left = expose_allocation(*typed_expr, gs);
            let right = expose_allocation(*typed_expr1, gs);
            alloc_bin_op(bin_op, left, right, ty)
        },
        TypedExpr::Tuple(typed_exprs, tuple_ty) => {
            let mut init_tuple = Vec::with_capacity(typed_exprs.len() + 2);
            let bytes_needed = tuple_ty.bytes_of();

            let ptr_after_alloc =
                alloc_bin_op(BinOp::Add, global_freeptr(), alloc_int(bytes_needed as i64), Type::Int);
            let cond = alloc_bin_op(BinOp::Lt, ptr_after_alloc, global_fromspace_end(), Type::Bool);
            let collect = collect(bytes_needed);
            let try_collect = alloc_if_else(cond, alloc_unit(), collect, Type::Unit);
            init_tuple.push(try_collect);

            let tuple_var = alloc_var(gs.fresh_with_prefix("$tuple"), tuple_ty.clone());
            let alloc_exprs = typed_exprs
                .into_iter()
                .map(|e| expose_allocation(e, gs))
                .collect::<Vec<AllocExpr>>();

            init_tuple.push(allocate(bytes_needed));
            for (idx, expr) in alloc_exprs.into_iter().enumerate() {
                let init_value = alloc_tuple_elem_init(tuple_var.clone(), expr, idx);
                init_tuple.push(init_value);
            }

            alloc_seq(init_tuple, tuple_var, tuple_ty)
        },
        TypedExpr::TupleProj(typed_expr, idx, ty) => {
            let alloc_expr = expose_allocation(*typed_expr, gs);
            alloc_tuple_proj(alloc_expr, idx, ty)
        },
        TypedExpr::PrimIO(prim_io, typed_expr, ty) => {
            let expr = typed_expr.map(|e| expose_allocation(*e, gs));
            alloc_prim_io(prim_io, expr, ty)
        },
        TypedExpr::UnaryOp(unary_op, typed_expr, ty) => {
            alloc_unary(unary_op, expose_allocation(*typed_expr, gs), ty)
        },
        TypedExpr::Ann(typed_expr, _) => expose_allocation(*typed_expr, gs),
        TypedExpr::If(typed_expr, typed_expr1, typed_expr2, ty) => {
            let cond = expose_allocation(*typed_expr, gs);
            let then_branch = expose_allocation(*typed_expr1, gs);
            let else_branch = expose_allocation(*typed_expr2, gs);
            alloc_if_else(cond, then_branch, else_branch, ty)
        },
        TypedExpr::Let(name, ty, typed_expr, typed_expr1, body_ty) => {
            let rhs = expose_allocation(*typed_expr, gs);
            let body = expose_allocation(*typed_expr1, gs);
            alloc_let(name, ty, rhs, body, body_ty)
        },
        TypedExpr::LetRec(name, items, ret_ty, typed_expr, typed_expr1, body_ty) => {
            let fbody = expose_allocation(*typed_expr, gs);
            let body = expose_allocation(*typed_expr1, gs);
            alloc_let_rec(name, items, ret_ty, fbody, body, body_ty)
        },
        TypedExpr::App(typed_expr, typed_exprs, ty) => {
            let func = expose_allocation(*typed_expr, gs);
            let args = typed_exprs.into_iter().map(|e| expose_allocation(e, gs)).collect();
            alloc_app(func, args, ty)
        },
        TypedExpr::Lambda(items, ret_ty, typed_expr, ty) => {
            alloc_lambda(items, ret_ty, expose_allocation(*typed_expr, gs), ty)
        },
    }
}

// todo: tests
