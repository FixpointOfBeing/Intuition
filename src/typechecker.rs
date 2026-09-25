// use llvm_ir::types::Typed;

use crate::syntax::{BinOp, Def, Expr, Ident, PrimIO, Program, Type, UnaryOp};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum TypedExpr {
    Unit,
    Bool(bool),
    Int(i64),
    Float(f64),
    Var(Ident, Type),
    BinOp(BinOp, Box<TypedExpr>, Box<TypedExpr>, Type),
    Tuple(Vec<TypedExpr>, Type),
    TupleProj(Box<TypedExpr>, usize, Type),
    PrimIO(PrimIO, Option<Box<TypedExpr>>, Type),
    UnaryOp(UnaryOp, Box<TypedExpr>, Type),
    Ann(Box<TypedExpr>, Type),
    If(Box<TypedExpr>, Box<TypedExpr>, Box<TypedExpr>, Type),
    Let(Ident, Type, Box<TypedExpr>, Box<TypedExpr>, Type),
    LetRec(
        Ident,              // function name
        Vec<(Ident, Type)>, // function parameters with their types
        Type,               // function return type
        Box<TypedExpr>,     // function body
        Box<TypedExpr>,     // expression after the let rec
        Type,
    ),
    App(Box<TypedExpr>, Vec<TypedExpr>, Type),
    Lambda(Vec<(Ident, Type)>, Type, Box<TypedExpr>, Type),
}

#[derive(Debug, Clone, PartialEq)]
pub enum TypedDef {
    ValDef(
        Ident,        // name
        Option<Type>, // optional type annotation
        TypedExpr,    // expresion
    ),
    FunDef(
        Ident,              // function name
        Vec<(Ident, Type)>, // function arguments with their types
        Type,               // function return type
        TypedExpr,          // function body
    ),
}

impl TypedDef {
    pub fn name(&self) -> Ident {
        match self {
            TypedDef::ValDef(name, _, _) => name.to_string(),
            TypedDef::FunDef(name, _, _, _) => name.to_string(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct TypedProgram {
    pub defs: Vec<TypedDef>,
    pub main: TypedExpr,
}

impl TypedExpr {
    pub fn type_of(&self) -> Type {
        match self {
            TypedExpr::Unit => Type::Unit,
            TypedExpr::Bool(_) => Type::Bool,
            TypedExpr::Int(_) => Type::Int,
            TypedExpr::Float(_) => Type::Float,
            TypedExpr::Var(_, ty) => ty.clone(),
            TypedExpr::BinOp(_, _, _, ty) => ty.clone(),
            TypedExpr::UnaryOp(_, _, ty) => ty.clone(),
            TypedExpr::Tuple(_, ty) => ty.clone(),
            TypedExpr::TupleProj(_, _, ty) => ty.clone(),
            TypedExpr::PrimIO(_, _, ty) => ty.clone(),
            TypedExpr::Ann(_, ty) => ty.clone(),
            TypedExpr::If(_, _, _, ty) => ty.clone(),
            TypedExpr::Let(_, _, _, _, ty) => ty.clone(),
            TypedExpr::LetRec(_, _, _, _, _, ty) => ty.clone(),
            TypedExpr::App(_, _, ty) => ty.clone(),
            TypedExpr::Lambda(_, _, _, ty) => ty.clone(),
        }
    }
}

pub fn t_unit() -> TypedExpr {
    TypedExpr::Unit
}

pub fn t_bool(b: bool) -> TypedExpr {
    TypedExpr::Bool(b)
}

pub fn t_int(n: i64) -> TypedExpr {
    TypedExpr::Int(n)
}

pub fn t_float(f: f64) -> TypedExpr {
    TypedExpr::Float(f)
}

pub fn t_var(name: impl Into<Ident>, ty: Type) -> TypedExpr {
    TypedExpr::Var(name.into(), ty)
}

pub fn t_tuple(elems: Vec<TypedExpr>, ty: Type) -> TypedExpr {
    TypedExpr::Tuple(elems, ty)
}

pub fn t_tuple_projection(expr: TypedExpr, index: usize, ty: Type) -> TypedExpr {
    TypedExpr::TupleProj(Box::new(expr), index, ty)
}

pub fn t_bin_op(op: BinOp, left: TypedExpr, right: TypedExpr, ty: Type) -> TypedExpr {
    TypedExpr::BinOp(op, Box::new(left), Box::new(right), ty)
}

pub fn t_prim_io(prim: PrimIO, expr: Option<TypedExpr>, ty: Type) -> TypedExpr {
    TypedExpr::PrimIO(prim, expr.map(Box::new), ty)
}

pub fn t_unary(op: UnaryOp, expr: TypedExpr, ty: Type) -> TypedExpr {
    TypedExpr::UnaryOp(op, Box::new(expr), ty)
}

pub fn t_ann(expr: TypedExpr, ty: Type) -> TypedExpr {
    TypedExpr::Ann(Box::new(expr), ty)
}

pub fn t_if(cond: TypedExpr, thn: TypedExpr, els: TypedExpr, ty: Type) -> TypedExpr {
    TypedExpr::If(Box::new(cond), Box::new(thn), Box::new(els), ty)
}

pub fn t_let(
    name: impl Into<Ident>,
    ty: Type,
    rhs: TypedExpr,
    body: TypedExpr,
    let_ty: Type,
) -> TypedExpr {
    TypedExpr::Let(name.into(), ty, Box::new(rhs), Box::new(body), let_ty)
}

pub fn t_let_rec(
    name: impl Into<Ident>,
    params: Vec<(Ident, Type)>,
    ret_ty: Type,
    fbody: TypedExpr,
    body: TypedExpr,
    ty: Type,
) -> TypedExpr {
    TypedExpr::LetRec(name.into(), params, ret_ty, Box::new(fbody), Box::new(body), ty)
}

pub fn t_app(func: TypedExpr, args: Vec<TypedExpr>, ty: Type) -> TypedExpr {
    TypedExpr::App(Box::new(func), args, ty)
}

pub fn t_lambda(
    params: Vec<(Ident, Type)>,
    ret_ty: Type,
    body: TypedExpr,
    lambda_ty: Type,
) -> TypedExpr {
    TypedExpr::Lambda(params, ret_ty, Box::new(body), lambda_ty)
}

#[derive(Debug, Clone, PartialEq)]
pub enum TypeError {
    UnboundVariable(Ident),
    Mismatch { expected: Type, found: Type },
    ReturnTypeMismatch { expected: Type, found: Type },
    NotAFunction(Type),
    ArityMismatch { expected: usize, found: usize },
    BranchMismatch { then_ty: Type, else_ty: Type },
    InvalidOperands { op: String, left: Type, right: Type },
    InvalidUnary { op: String, ty: Type },
    AnnotationMismatch { annotated: Type, inferred: Type },
    NotATuple(Type),
    TupleIndexOutOfBounds { len: usize, index: usize },
}

impl std::fmt::Display for TypeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TypeError::UnboundVariable(name) => {
                write!(f, "Unbound variable: {}", name)
            },
            TypeError::Mismatch { expected, found } => {
                write!(f, "Type mismatch: expected {:?}, found {:?}", expected, found)
            },
            TypeError::ReturnTypeMismatch { expected, found } => {
                write!(f, "Return type mismatch: expected {:?}, found {:?}", expected, found)
            },
            TypeError::NotAFunction(ty) => {
                write!(f, "Not a function: {:?}", ty)
            },
            TypeError::ArityMismatch { expected, found } => {
                write!(f, "Arity mismatch: expected {} args, got {}", expected, found)
            },
            TypeError::BranchMismatch { then_ty, else_ty } => {
                write!(
                    f,
                    "If branches have different types: then={:?}, else={:?}",
                    then_ty, else_ty
                )
            },
            TypeError::InvalidOperands { op, left, right } => {
                write!(f, "Operator `{}` cannot be applied to {:?} and {:?}", op, left, right)
            },
            TypeError::InvalidUnary { op, ty } => {
                write!(f, "Operator `{}` cannot be applied to {:?}", op, ty)
            },
            TypeError::AnnotationMismatch { annotated, inferred } => {
                write!(f, "Annotation mismatch: declared {:?}, inferred {:?}", annotated, inferred)
            },
            TypeError::NotATuple(ty) => {
                write!(f, "Tuple projection on a non-tuple: {:?}", ty)
            },
            TypeError::TupleIndexOutOfBounds { len, index } => {
                write!(f, "Tuple index {} out of bounds (tuple length {})", index, len)
            },
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct Context(HashMap<Ident, Type>);

impl Context {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    pub fn lookup(&self, name: &str) -> Option<&Type> {
        self.0.get(name)
    }

    pub fn extend(&self, name: Ident, ty: Type) -> Self {
        let mut inner = self.0.clone();
        inner.insert(name, ty);
        Self(inner)
    }

    pub fn insert(&mut self, name: Ident, ty: Type) {
        let inner = &mut self.0;
        inner.insert(name, ty);
    }
}

fn infer(ctx: &mut Context, expr: Expr) -> Result<TypedExpr, TypeError> {
    match expr {
        Expr::Unit => Ok(TypedExpr::Unit),
        Expr::Bool(b) => Ok(TypedExpr::Bool(b)),
        Expr::Int(i) => Ok(TypedExpr::Int(i)),
        Expr::Float(f) => Ok(TypedExpr::Float(f)),

        Expr::Var(name) => match ctx.lookup(&name) {
            Some(ty) => Ok(TypedExpr::Var(name.to_string(), (*ty).clone())),
            None => Err(TypeError::UnboundVariable(name.clone())),
        },

        Expr::Ann(inner, ann_ty) => {
            let typed_expr = infer(ctx, *inner)?;
            let inferred = typed_expr.type_of();
            if inferred != ann_ty {
                return Err(TypeError::AnnotationMismatch { annotated: ann_ty, inferred });
            }
            Ok(typed_expr)
        },

        Expr::Tuple(exprs) => {
            let mut typed_exprs = Vec::with_capacity(exprs.len());
            let mut types = Vec::with_capacity(exprs.len());
            for expr in exprs {
                let typed_expr = infer(ctx, expr)?;
                types.push(typed_expr.type_of());
                typed_exprs.push(typed_expr);
            }
            let tuple_ty = Type::Tuple(types);
            Ok(TypedExpr::Tuple(typed_exprs, tuple_ty))
        },

        Expr::TupleProj(inner, index) => {
            let typed_inner = infer(ctx, *inner)?;
            let inner_ty = typed_inner.type_of();
            match inner_ty {
                Type::Tuple(types) => {
                    if index >= types.len() {
                        return Err(TypeError::TupleIndexOutOfBounds { len: types.len(), index });
                    }
                    let elem_ty = types[index].clone();
                    Ok(TypedExpr::TupleProj(Box::new(typed_inner), index, elem_ty))
                },
                other => Err(TypeError::NotATuple(other)),
            }
        },

        Expr::PrimIO(prim_io, Some(expr)) => match prim_io {
            PrimIO::PrintInt => {
                let typed_expr = infer(ctx, *expr)?;
                let ty = typed_expr.type_of();
                Ok(TypedExpr::PrimIO(PrimIO::PrintInt, Some(Box::new(typed_expr)), ty))
            },
            PrimIO::PrintFloat => {
                let typed_expr = infer(ctx, *expr)?;
                let ty = typed_expr.type_of();
                Ok(TypedExpr::PrimIO(PrimIO::PrintFloat, Some(Box::new(typed_expr)), ty))
            },
            PrimIO::PrintBool => {
                let typed_expr = infer(ctx, *expr)?;
                let ty = typed_expr.type_of();
                Ok(TypedExpr::PrimIO(PrimIO::PrintBool, Some(Box::new(typed_expr)), ty))
            },
            _ => unreachable!(),
        },

        Expr::PrimIO(prim_io, None) => match prim_io {
            PrimIO::ReadInt => {
                let ty = Type::Unit;
                Ok(TypedExpr::PrimIO(PrimIO::ReadInt, None, ty))
            },
            PrimIO::ReadFloat => {
                let ty = Type::Unit;
                Ok(TypedExpr::PrimIO(PrimIO::ReadFloat, None, ty))
            },
            _ => unreachable!(),
        },
        Expr::UnaryOp(op, operand) => {
            let typed_operand = infer(ctx, *operand)?;
            let ty = typed_operand.type_of();
            match op {
                UnaryOp::Neg => match &ty {
                    Type::Int | Type::Float => {
                        Ok(TypedExpr::UnaryOp(op, Box::new(typed_operand), ty))
                    },
                    _ => Err(TypeError::InvalidUnary { op: "-".to_string(), ty }),
                },
                UnaryOp::Not => match &ty {
                    Type::Bool => Ok(TypedExpr::UnaryOp(op, Box::new(typed_operand), ty)),
                    _ => Err(TypeError::InvalidUnary { op: "!".to_string(), ty }),
                },
            }
        },

        Expr::BinOp(op, lhs, rhs) => {
            let left_typed_expr = infer(ctx, *lhs)?;
            let left_ty = left_typed_expr.type_of();
            let right_typed_expr = infer(ctx, *rhs)?;
            let right_ty = right_typed_expr.type_of();
            infer_binop(op, left_ty, left_typed_expr, right_ty, right_typed_expr)
        },

        Expr::If(cond, thn, els) => {
            let typed_cond_expr = infer(ctx, *cond)?;
            let cond_ty = typed_cond_expr.type_of();
            check(&Type::Bool, &cond_ty)?;

            let typed_then_expr = infer(ctx, *thn)?;
            let then_ty = typed_then_expr.type_of();
            let typed_else_expr = infer(ctx, *els)?;

            let else_ty = typed_else_expr.type_of();
            if then_ty != else_ty {
                return Err(TypeError::BranchMismatch { then_ty, else_ty });
            }
            Ok(TypedExpr::If(
                Box::new(typed_cond_expr),
                Box::new(typed_then_expr),
                Box::new(typed_else_expr),
                then_ty,
            ))
        },

        Expr::Let(name, ann, rhs, body) => {
            let typed_rhs = infer(ctx, *rhs)?;

            let rhs_ty = typed_rhs.type_of();
            if let Some(ann_ty) = ann {
                if rhs_ty != ann_ty {
                    return Err(TypeError::AnnotationMismatch {
                        annotated: ann_ty,
                        inferred: rhs_ty,
                    });
                }
            }

            let mut body_ctx = ctx.extend(name.clone(), rhs_ty.clone());
            let typed_body = infer(&mut body_ctx, *body)?;
            let body_ty = typed_body.type_of();
            Ok(TypedExpr::Let(name, rhs_ty, Box::new(typed_rhs), Box::new(typed_body), body_ty))
        },

        Expr::LetRec(fname, fparams, fret_ty, body, rest) => {
            let fn_ty =
                build_arrow(fparams.iter().map(|(_, t)| t.clone()).collect(), fret_ty.clone());

            let mut body_ctx = ctx.extend(fname.clone(), fn_ty.clone());
            for (param_name, param_ty) in fparams.clone() {
                body_ctx = body_ctx.extend(param_name, param_ty);
            }

            let typed_fbody = infer(&mut body_ctx, *body)?;
            let fbody_ty = typed_fbody.type_of();
            if fbody_ty != fret_ty {
                return Err(TypeError::AnnotationMismatch {
                    annotated: fret_ty,
                    inferred: fbody_ty,
                });
            }

            let mut rest_ctx = ctx.extend(fname.clone(), fn_ty);
            let typed_body = infer(&mut rest_ctx, *rest)?;
            let body_ty = typed_body.type_of();
            let typed_letrec = TypedExpr::LetRec(
                fname,
                fparams,
                fret_ty,
                Box::new(typed_fbody),
                Box::new(typed_body),
                body_ty.clone(),
            );
            Ok(typed_letrec)
        },

        Expr::Lambda(params, opty, body) => {
            let mut lam_ctx = ctx.clone();
            let mut param_tys = Vec::with_capacity(params.len());
            for (pname, pty) in &params {
                lam_ctx.insert(pname.clone(), pty.clone());
                param_tys.push(pty.clone());
            }
            let typed_body = infer(&mut lam_ctx, *body)?;
            let body_ty = typed_body.type_of();
            if let Some(rt_ty) = opty {
                if body_ty != rt_ty {
                    return Err(TypeError::ReturnTypeMismatch { expected: rt_ty, found: body_ty });
                }
            }
            let lambda_ty = build_arrow(param_tys, body_ty.clone());
            let typed_lambda = TypedExpr::Lambda(params, body_ty, Box::new(typed_body), lambda_ty);
            Ok(typed_lambda)
        },

        Expr::App(func, args) => {
            let typed_fn = infer(ctx, *func)?;
            let fn_ty = typed_fn.type_of();
            let mut typed_args = vec![];
            let mut curr_ty = fn_ty;
            for arg in args {
                match curr_ty {
                    Type::Arrow(param_ty, ret_ty) => {
                        let typed_arg = infer(ctx, arg)?;
                        let arg_ty = typed_arg.type_of();
                        if arg_ty != *param_ty {
                            return Err(TypeError::Mismatch { expected: *param_ty, found: arg_ty });
                        }
                        typed_args.push(typed_arg);
                        curr_ty = *ret_ty;
                    },
                    other => {
                        return Err(TypeError::NotAFunction(other));
                    },
                }
            }
            Ok(TypedExpr::App(Box::new(typed_fn), typed_args, curr_ty))
        },
    }
}

fn check(expected: &Type, inferred: &Type) -> Result<(), TypeError> {
    if inferred != expected {
        Err(TypeError::Mismatch { expected: expected.clone(), found: inferred.clone() })
    } else {
        Ok(())
    }
}

fn build_arrow(params: Vec<Type>, ret: Type) -> Type {
    params
        .into_iter()
        .rev()
        .fold(ret, |acc, p| Type::Arrow(Box::new(p), Box::new(acc)))
}

fn infer_binop(
    op: BinOp,
    left_ty: Type,
    left_typed_expr: TypedExpr,
    right_ty: Type,
    right_typed_expr: TypedExpr,
) -> Result<TypedExpr, TypeError> {
    let op_str = format!("{:?}", op);
    let make_ty_expr = |ty: Type| {
        TypedExpr::BinOp(op.clone(), Box::new(left_typed_expr), Box::new(right_typed_expr), ty)
    };
    match op {
        BinOp::Add | BinOp::Sub | BinOp::Mul | BinOp::Div => match (&left_ty, &right_ty) {
            (Type::Int, Type::Int) => Ok(make_ty_expr(Type::Int)),
            (Type::Float, Type::Float) => Ok(make_ty_expr(Type::Float)),
            _ => Err(TypeError::InvalidOperands { op: op_str, left: left_ty, right: right_ty }),
        },

        BinOp::Lt | BinOp::Gt | BinOp::Leq | BinOp::Geq => match (&left_ty, &right_ty) {
            (Type::Int, Type::Int) | (Type::Float, Type::Float) => Ok(make_ty_expr(Type::Bool)),
            _ => Err(TypeError::InvalidOperands { op: op_str, left: left_ty, right: right_ty }),
        },

        BinOp::Eq | BinOp::Neq => {
            if left_ty == right_ty {
                Ok(make_ty_expr(Type::Bool))
            } else {
                Err(TypeError::InvalidOperands { op: op_str, left: left_ty, right: right_ty })
            }
        },

        BinOp::And | BinOp::Or => match (&left_ty, &right_ty) {
            (Type::Bool, Type::Bool) => Ok(make_ty_expr(Type::Bool)),
            _ => Err(TypeError::InvalidOperands { op: op_str, left: left_ty, right: right_ty }),
        },
    }
}

pub fn typecheck_expr(expr: Expr) -> Result<(Type, TypedExpr), TypeError> {
    let typed_expr = infer(&mut Context::new(), expr)?;
    Ok((typed_expr.type_of(), typed_expr))
}

pub fn typecheck_expr_with_ctx(
    ctx: &mut Context,
    expr: Expr,
) -> Result<(Type, TypedExpr), TypeError> {
    let typed_expr = infer(ctx, expr)?;
    Ok((typed_expr.type_of(), typed_expr))
}

pub fn typecheck_def(def: Def, ctx: &mut Context) -> Result<(Type, TypedDef), TypeError> {
    match def {
        Def::ValDef(name, ty, expr) => {
            let typed_expr = infer(ctx, expr)?;
            let found = typed_expr.type_of();
            match ty {
                Some(val_ty) => {
                    if found != val_ty {
                        Err(TypeError::Mismatch { expected: val_ty, found })
                    } else {
                        Ok((found.clone(), TypedDef::ValDef(name, Some(found), typed_expr)))
                    }
                },
                None => Ok((found.clone(), TypedDef::ValDef(name, Some(found), typed_expr))),
            }
        },

        Def::FunDef(name, param_tys, rt_ty, expr) => {
            let fn_ty =
                build_arrow(param_tys.iter().map(|(_, t)| t.clone()).collect(), rt_ty.clone());

            let mut body_ctx = ctx.clone();
            body_ctx.insert(name.clone(), fn_ty.clone());
            for (param, ty) in param_tys.iter() {
                body_ctx.insert(param.clone(), ty.clone());
            }

            let typed_expr = infer(&mut body_ctx, expr)?;
            let found = typed_expr.type_of();
            if rt_ty != found {
                return Err(TypeError::Mismatch { expected: rt_ty, found });
            }
            Ok((fn_ty, TypedDef::FunDef(name, param_tys, rt_ty, typed_expr)))
        },
    }
}

pub fn typecheck_program(prog: Program) -> Result<(Type, TypedProgram), TypeError> {
    let mut ctx = Context::new();
    let mut typed_defs = Vec::with_capacity(prog.defs.len());
    for def in prog.defs {
        let name = def.name();
        let (ty, typed_def) = typecheck_def(def, &mut ctx)?;
        typed_defs.push(typed_def);
        ctx.insert(name, ty);
    }
    let (main_ty, typed_main) = typecheck_expr_with_ctx(&mut ctx, prog.main)?;
    let typed_prog = TypedProgram { defs: typed_defs, main: typed_main };
    Ok((main_ty, typed_prog))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::syntax::{
        BinOp, Type, UnaryOp, a_let, ann, app, bin_op, bool, float, fun_def, if_else, int, lambda,
        let_rec, print_bool, print_float, print_int, read_float, read_int, tuple, tuple_projection,
        ty_bool, ty_int, unary, unit, val_def, var,
    };

    fn infer_type(expr: Expr) -> Result<Type, TypeError> {
        typecheck_expr(expr).map(|(ty, _)| ty)
    }

    #[test]
    fn test_unit() {
        assert_eq!(infer_type(unit()), Ok(Type::Unit));
    }

    #[test]
    fn test_bool() {
        assert_eq!(infer_type(bool(true)), Ok(Type::Bool));
    }

    #[test]
    fn test_int() {
        assert_eq!(infer_type(int(42)), Ok(Type::Int));
    }

    #[test]
    fn test_float() {
        assert_eq!(infer_type(float(3.14)), Ok(Type::Float));
    }

    #[test]
    fn test_neg_int() {
        assert_eq!(infer_type(unary(UnaryOp::Neg, int(1))), Ok(Type::Int));
    }

    #[test]
    fn test_neg_float() {
        assert_eq!(infer_type(unary(UnaryOp::Neg, float(1.0))), Ok(Type::Float));
    }

    #[test]
    fn test_neg_bool_err() {
        assert!(infer_type(unary(UnaryOp::Neg, bool(true))).is_err());
    }

    #[test]
    fn test_not_bool() {
        assert_eq!(infer_type(unary(UnaryOp::Not, bool(false))), Ok(Type::Bool));
    }

    #[test]
    fn test_not_int_err() {
        assert!(infer_type(unary(UnaryOp::Not, int(0))).is_err());
    }

    #[test]
    fn test_add_int() {
        assert_eq!(infer_type(bin_op(BinOp::Add, int(1), int(2))), Ok(Type::Int));
    }

    #[test]
    fn test_add_float() {
        assert_eq!(infer_type(bin_op(BinOp::Add, float(1.0), float(2.0))), Ok(Type::Float));
    }

    #[test]
    fn test_add_int_float_err() {
        assert!(infer_type(bin_op(BinOp::Add, int(1), float(2.0))).is_err());
    }

    #[test]
    fn test_lt_int() {
        assert_eq!(infer_type(bin_op(BinOp::Lt, int(1), int(2))), Ok(Type::Bool));
    }

    #[test]
    fn test_eq_any_type() {
        assert_eq!(infer_type(bin_op(BinOp::Eq, bool(true), bool(false))), Ok(Type::Bool));
    }

    #[test]
    fn test_eq_type_mismatch_err() {
        assert!(infer_type(bin_op(BinOp::Eq, int(1), bool(true))).is_err());
    }

    #[test]
    fn test_and_bool() {
        assert_eq!(infer_type(bin_op(BinOp::And, bool(true), bool(false))), Ok(Type::Bool));
    }

    #[test]
    fn test_if_ok() {
        assert_eq!(infer_type(if_else(bool(true), int(1), int(0))), Ok(Type::Int));
    }

    #[test]
    fn test_if_cond_not_bool_err() {
        assert!(infer_type(if_else(int(1), int(2), int(3))).is_err());
    }

    #[test]
    fn test_if_branch_mismatch_err() {
        assert!(infer_type(if_else(bool(true), int(1), bool(false))).is_err());
    }

    #[test]
    fn test_let_no_ann() {
        assert_eq!(infer_type(a_let("x", None, int(1), var("x"))), Ok(Type::Int));
    }

    #[test]
    fn test_let_with_correct_ann() {
        assert_eq!(infer_type(a_let("x", Some(ty_int()), int(1), var("x"))), Ok(Type::Int));
    }

    #[test]
    fn test_let_wrong_ann_err() {
        assert!(infer_type(a_let("x", Some(ty_bool()), int(1), var("x"))).is_err());
    }

    #[test]
    fn test_lambda_identity_int() {
        assert_eq!(
            infer_type(lambda(vec![("x".to_string(), ty_int())], None, var("x"))),
            Ok(Type::Arrow(Box::new(Type::Int), Box::new(Type::Int)))
        );
    }

    #[test]
    fn test_lambda_multi_param() {
        assert_eq!(
            infer_type(lambda(
                vec![("x".to_string(), ty_int()), ("y".to_string(), ty_int())],
                None,
                bin_op(BinOp::Add, var("x"), var("y"))
            )),
            Ok(Type::Arrow(
                Box::new(Type::Int),
                Box::new(Type::Arrow(Box::new(Type::Int), Box::new(Type::Int)))
            ))
        );
    }

    #[test]
    fn test_let_lambda() {
        assert_eq!(
            infer_type(a_let(
                "add",
                None,
                lambda(
                    vec![("x".to_string(), ty_int()), ("y".to_string(), ty_int())],
                    None,
                    bin_op(BinOp::Add, var("x"), var("y"))
                ),
                var("add")
            )),
            Ok(Type::Arrow(
                Box::new(Type::Int),
                Box::new(Type::Arrow(Box::new(Type::Int), Box::new(Type::Int)))
            ))
        );
    }

    #[test]
    fn test_app_lambda() {
        assert_eq!(
            infer_type(app(
                lambda(vec![("x".to_string(), ty_int())], Some(ty_int()), var("x")),
                vec![int(42)]
            )),
            Ok(Type::Int)
        );
    }

    #[test]
    fn test_app_wrong_arg_type_err() {
        assert!(
            infer_type(app(
                lambda(vec![("x".to_string(), ty_int())], None, var("x")),
                vec![bool(true)]
            ))
            .is_err()
        );
    }

    #[test]
    fn test_app_not_function_err() {
        assert!(infer_type(app(int(42), vec![int(1)])).is_err());
    }

    #[test]
    fn test_function_ret_mismatch() {
        assert!(
            infer_type(lambda(
                vec![("x".to_string(), ty_int())],
                Some(ty_int()),
                bin_op(BinOp::Leq, var("x"), int(37))
            ))
            .is_err()
        );
    }

    #[test]
    fn test_letrec_factorial() {
        assert_eq!(
            infer_type(let_rec(
                "fact",
                vec![("n".to_string(), ty_int())],
                ty_int(),
                if_else(
                    bin_op(BinOp::Eq, var("n"), int(0)),
                    int(1),
                    bin_op(
                        BinOp::Mul,
                        var("n"),
                        app(var("fact"), vec![bin_op(BinOp::Sub, var("n"), int(1))])
                    )
                ),
                app(var("fact"), vec![int(5)])
            )),
            Ok(Type::Int)
        );
    }

    #[test]
    fn test_letrec_wrong_body_type_err() {
        assert!(
            infer_type(let_rec(
                "f",
                vec![("n".to_string(), ty_int())],
                ty_int(),
                bool(true),
                app(var("f"), vec![int(0)])
            ))
            .is_err()
        );
    }

    #[test]
    fn test_ann_ok() {
        assert_eq!(infer_type(ann(int(1), ty_int())), Ok(Type::Int));
    }

    #[test]
    fn test_ann_mismatch_err() {
        assert!(infer_type(ann(int(1), ty_bool())).is_err());
    }

    #[test]
    fn test_unbound_variable_err() {
        assert!(infer_type(var("foo")).is_err());
    }

    #[test]
    fn test_tuple_type() {
        assert_eq!(
            infer_type(tuple(vec![int(1), bool(true), float(3.14)])),
            Ok(Type::Tuple(vec![Type::Int, Type::Bool, Type::Float]))
        );
    }

    #[test]
    fn test_nested_tuple_type() {
        assert_eq!(
            infer_type(tuple(vec![int(1), tuple(vec![bool(true), float(2.0)])])),
            Ok(Type::Tuple(vec![Type::Int, Type::Tuple(vec![Type::Bool, Type::Float])]))
        );
    }

    #[test]
    fn test_tuple_projection() {
        let expr = tuple_projection(tuple(vec![int(1), bool(true), float(3.14)]), 1);
        assert_eq!(infer_type(expr), Ok(Type::Bool));
    }

    #[test]
    fn test_tuple_projection_nested() {
        let expr =
            tuple_projection(tuple_projection(tuple(vec![int(1), tuple(vec![bool(true)])]), 1), 0);
        assert_eq!(infer_type(expr), Ok(Type::Bool));
    }

    #[test]
    fn test_tuple_projection_out_of_bounds_err() {
        let expr = tuple_projection(tuple(vec![int(1), bool(true)]), 5);
        assert!(infer_type(expr).is_err());
    }

    #[test]
    fn test_tuple_projection_non_tuple_err() {
        let expr = tuple_projection(int(1), 0);
        assert!(infer_type(expr).is_err());
    }

    #[test]
    fn test_print_int() {
        assert_eq!(infer_type(print_int(int(42))), Ok(Type::Int));
    }

    #[test]
    fn test_print_bool() {
        assert_eq!(infer_type(print_bool(bool(true))), Ok(Type::Bool));
    }

    #[test]
    fn test_print_float() {
        assert_eq!(infer_type(print_float(float(3.14))), Ok(Type::Float));
    }

    #[test]
    fn test_read_int() {
        assert_eq!(infer_type(read_int()), Ok(Type::Unit));
    }

    #[test]
    fn test_read_float() {
        assert_eq!(infer_type(read_float()), Ok(Type::Unit));
    }

    // ---- typecheck_def ----

    fn check_def(def: Def) -> Result<(Type, TypedDef), TypeError> {
        typecheck_def(def, &mut Context::new())
    }

    #[test]
    fn test_def_val_no_ann() {
        let (ty, typed_def) = check_def(val_def("x", None, int(42))).unwrap();
        assert_eq!(ty, Type::Int);
        assert_eq!(
            typed_def,
            TypedDef::ValDef("x".to_string(), Some(Type::Int), TypedExpr::Int(42))
        );
    }

    #[test]
    fn test_def_val_matching_ann() {
        let (ty, typed_def) = check_def(val_def("x", Some(ty_int()), int(42))).unwrap();
        assert_eq!(ty, Type::Int);
        assert_eq!(
            typed_def,
            TypedDef::ValDef("x".to_string(), Some(Type::Int), TypedExpr::Int(42))
        );
    }

    #[test]
    fn test_def_val_mismatched_ann() {
        assert_eq!(
            check_def(val_def("x", Some(ty_bool()), int(42))),
            Err(TypeError::Mismatch { expected: Type::Bool, found: Type::Int })
        );
    }

    #[test]
    fn test_def_val_unbound() {
        assert_eq!(
            check_def(val_def("x", None, var("u"))),
            Err(TypeError::UnboundVariable("u".to_string()))
        );
    }

    #[test]
    fn test_def_fun_no_params() {
        let (ty, typed_def) = check_def(fun_def("f", vec![], ty_int(), int(1))).unwrap();
        assert_eq!(ty, Type::Int);
        assert_eq!(
            typed_def,
            TypedDef::FunDef("f".to_string(), vec![], Type::Int, TypedExpr::Int(1))
        );
    }

    #[test]
    fn test_def_fun_body_type_mismatch() {
        assert_eq!(
            check_def(fun_def("f", vec![], ty_int(), bool(true))),
            Err(TypeError::Mismatch { expected: Type::Int, found: Type::Bool })
        );
    }

    #[test]
    fn test_def_fun_params_in_scope() {
        let (ty, typed_def) = check_def(fun_def(
            "add",
            vec![("a".to_string(), ty_int()), ("b".to_string(), ty_int())],
            ty_int(),
            bin_op(BinOp::Add, var("a"), var("b")),
        ))
        .unwrap();
        assert_eq!(
            ty,
            Type::Arrow(
                Box::new(Type::Int),
                Box::new(Type::Arrow(Box::new(Type::Int), Box::new(Type::Int)))
            )
        );
        match typed_def {
            TypedDef::FunDef(name, params, ret_ty, body) => {
                assert_eq!(name, "add");
                assert_eq!(params, vec![("a".to_string(), ty_int()), ("b".to_string(), ty_int())]);
                assert_eq!(ret_ty, ty_int());
                assert_eq!(
                    body,
                    TypedExpr::BinOp(
                        BinOp::Add,
                        Box::new(TypedExpr::Var("a".to_string(), Type::Int)),
                        Box::new(TypedExpr::Var("b".to_string(), Type::Int)),
                        Type::Int,
                    )
                );
            },
            _ => panic!("expected TypedDef::FunDef"),
        }
    }

    #[test]
    fn test_def_fun_self_recursion() {
        let fact = fun_def(
            "fact",
            vec![("n".to_string(), ty_int())],
            ty_int(),
            if_else(
                bin_op(BinOp::Eq, var("n"), int(0)),
                int(1),
                bin_op(
                    BinOp::Mul,
                    var("n"),
                    app(var("fact"), vec![bin_op(BinOp::Sub, var("n"), int(1))]),
                ),
            ),
        );
        let (ty, _) = check_def(fact).unwrap();
        assert_eq!(ty, Type::Arrow(Box::new(Type::Int), Box::new(Type::Int)));
    }

    #[test]
    fn test_def_val_uses_prior_binding() {
        let mut ctx = Context::new();
        ctx.insert("x".to_string(), Type::Int);
        let (ty, _) = typecheck_def(val_def("y", None, var("x")), &mut ctx).unwrap();
        assert_eq!(ty, Type::Int);
    }

    // ---- typecheck_program ----

    #[test]
    fn test_program_empty() {
        let (ty, typed_prog) = typecheck_program(Program { defs: vec![], main: int(42) }).unwrap();
        assert_eq!(ty, Type::Int);
        assert_eq!(typed_prog, TypedProgram { defs: vec![], main: TypedExpr::Int(42) });
    }

    #[test]
    fn test_program_main_uses_def() {
        let (ty, _) =
            typecheck_program(Program { defs: vec![val_def("x", None, int(42))], main: var("x") })
                .unwrap();
        assert_eq!(ty, Type::Int);
    }

    #[test]
    fn test_program_chained_defs() {
        let (ty, typed_prog) = typecheck_program(Program {
            defs: vec![val_def("x", None, int(1)), val_def("y", None, var("x"))],
            main: int(0),
        })
        .unwrap();
        assert_eq!(ty, Type::Int);
        assert_eq!(typed_prog.defs.len(), 2);
    }

    #[test]
    fn test_program_def_error_propagates() {
        let result = typecheck_program(Program {
            defs: vec![val_def("x", Some(ty_bool()), int(1))],
            main: int(0),
        });
        assert_eq!(result, Err(TypeError::Mismatch { expected: Type::Bool, found: Type::Int }));
    }

    #[test]
    fn test_program_factorial() {
        let prog = Program {
            defs: vec![fun_def(
                "fact",
                vec![("n".to_string(), ty_int())],
                ty_int(),
                if_else(
                    bin_op(BinOp::Eq, var("n"), int(0)),
                    int(1),
                    bin_op(
                        BinOp::Mul,
                        var("n"),
                        app(var("fact"), vec![bin_op(BinOp::Sub, var("n"), int(1))]),
                    ),
                ),
            )],
            main: app(var("fact"), vec![int(5)]),
        };
        let (ty, typed_prog) = typecheck_program(prog).unwrap();
        assert_eq!(ty, Type::Int);
        assert_eq!(typed_prog.defs.len(), 1);
    }

    #[test]
    fn test_program_answer_add_max() {
        let prog = Program {
            defs: vec![
                val_def("answer", Some(ty_int()), int(42)),
                fun_def(
                    "add",
                    vec![("a".to_string(), ty_int()), ("b".to_string(), ty_int())],
                    ty_int(),
                    bin_op(BinOp::Add, var("a"), var("b")),
                ),
                fun_def(
                    "max",
                    vec![("a".to_string(), ty_int()), ("b".to_string(), ty_int())],
                    ty_int(),
                    if_else(bin_op(BinOp::Geq, var("a"), var("b")), var("a"), var("b")),
                ),
            ],
            main: app(var("add"), vec![var("answer"), app(var("max"), vec![int(1), int(2)])]),
        };
        let (ty, typed_prog) = typecheck_program(prog).unwrap();
        assert_eq!(ty, Type::Int);
        assert_eq!(typed_prog.defs.len(), 3);
    }
}
