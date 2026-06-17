use std::collections::HashMap;
use crate::syntax::{BinOp, Expr, Ident, Type, UnaryOp};


#[derive(Debug, Clone, PartialEq)]
pub enum TypeError {
    UnboundVariable(Ident),
    Mismatch { expected: Type, found: Type },
    NotAFunction(Type),
    ArityMismatch { expected: usize, found: usize },
    BranchMismatch { then_ty: Type, else_ty: Type },
    InvalidOperands { op: String, left: Type, right: Type },
    InvalidUnary { op: String, ty: Type },
    AnnotationMismatch { annotated: Type, inferred: Type },
}

impl std::fmt::Display for TypeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TypeError::UnboundVariable(name) => write!(f, "Unbound variable: {name}"),
            TypeError::Mismatch { expected, found } => {
                write!(f, "Type mismatch: expected {expected:?}, found {found:?}")
            }
            TypeError::NotAFunction(ty) => write!(f, "Not a function: {ty:?}"),
            TypeError::ArityMismatch { expected, found } => {
                write!(f, "Arity mismatch: expected {expected} args, got {found}")
            }
            TypeError::BranchMismatch { then_ty, else_ty } => write!(
                f,
                "If branches have different types: then={then_ty:?}, else={else_ty:?}"
            ),
            TypeError::InvalidOperands { op, left, right } => write!(
                f,
                "Operator `{op}` cannot be applied to {left:?} and {right:?}"
            ),
            TypeError::InvalidUnary { op, ty } => {
                write!(f, "Operator `{op}` cannot be applied to {ty:?}")
            }
            TypeError::AnnotationMismatch { annotated, inferred } => write!(
                f,
                "Annotation mismatch: declared {annotated:?}, inferred {inferred:?}"
            ),
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
}

pub fn typecheck(expr: &Expr) -> Result<Type, TypeError> {
    infer(&Context::new(), expr)
}

pub fn typecheck_with_ctx(ctx: &Context, expr: &Expr) -> Result<Type, TypeError> {
    infer(ctx, expr)
}


fn infer(ctx: &Context, expr: &Expr) -> Result<Type, TypeError> {
    match expr {
        Expr::Unit => Ok(Type::Unit),
        Expr::Bool(_) => Ok(Type::Bool),
        Expr::Int(_) => Ok(Type::Int),
        Expr::Float(_) => Ok(Type::Float),

        Expr::Var(name) => ctx
            .lookup(name)
            .cloned()
            .ok_or_else(|| TypeError::UnboundVariable(name.clone())),

        Expr::Ann(inner, ann_ty) => {
            let inferred = infer(ctx, inner)?;
            if &inferred != ann_ty.as_ref() {
                return Err(TypeError::AnnotationMismatch {
                    annotated: *ann_ty.clone(),
                    inferred,
                });
            }
            Ok(inferred)
        }

        Expr::UnaryOp(op, operand) => {
            let ty = infer(ctx, operand)?;
            match op {
                UnaryOp::Neg => match ty {
                    Type::Int | Type::Float => Ok(ty),
                    _ => Err(TypeError::InvalidUnary {
                        op: "-".to_string(),
                        ty,
                    }),
                },
                UnaryOp::Not => match ty {
                    Type::Bool => Ok(Type::Bool),
                    _ => Err(TypeError::InvalidUnary {
                        op: "!".to_string(),
                        ty,
                    }),
                },
            }
        }

        Expr::BinOp(op, lhs, rhs) => {
            let lt = infer(ctx, lhs)?;
            let rt = infer(ctx, rhs)?;
            infer_binop(op, lt, rt)
        }

        Expr::If(cond, then_expr, else_expr) => {
            let cond_ty = infer(ctx, cond)?;
            check(ctx, cond, &Type::Bool, cond_ty)?;

            let then_ty = infer(ctx, then_expr)?;
            let else_ty = infer(ctx, else_expr)?;

            if then_ty != else_ty {
                return Err(TypeError::BranchMismatch { then_ty, else_ty });
            }
            Ok(then_ty)
        }

        Expr::Let(name, ann, val, body) => {
            let val_ty = infer(ctx, val)?;

            if let Some(ann_ty) = ann {
                if &val_ty != ann_ty {
                    return Err(TypeError::AnnotationMismatch {
                        annotated: ann_ty.clone(),
                        inferred: val_ty,
                    });
                }
            }

            let ctx2 = ctx.extend(name.clone(), val_ty);
            infer(&ctx2, body)
        }

        Expr::LetRec(fname, args, ret_ty, body, rest) => {
            let fn_ty = build_arrow(args.iter().map(|(_, t)| t.clone()).collect(), ret_ty.clone());

            let mut body_ctx = ctx.extend(fname.clone(), fn_ty.clone());
            for (arg_name, arg_ty) in args {
                body_ctx = body_ctx.extend(arg_name.clone(), arg_ty.clone());
            }

            let body_ty = infer(&body_ctx, body)?;
            if &body_ty != ret_ty {
                return Err(TypeError::AnnotationMismatch {
                    annotated: ret_ty.clone(),
                    inferred: body_ty,
                });
            }

            let rest_ctx = ctx.extend(fname.clone(), fn_ty);
            infer(&rest_ctx, rest)
        }

        Expr::Lambda(params, body) => {
            let mut lam_ctx = ctx.clone();
            let mut param_tys = Vec::new();
            for (pname, pty) in params {
                lam_ctx = lam_ctx.extend(pname.clone(), pty.clone());
                param_tys.push(pty.clone());
            }
            let body_ty = infer(&lam_ctx, body)?;
            Ok(build_arrow(param_tys, body_ty))
        }

        Expr::App(func, args) => {
            let mut fn_ty = infer(ctx, func)?;
            for arg in args {
                match fn_ty {
                    Type::Arrow(param_ty, ret_ty) => {
                        let arg_ty = infer(ctx, arg)?;
                        if arg_ty != *param_ty {
                            return Err(TypeError::Mismatch {
                                expected: *param_ty,
                                found: arg_ty,
                            });
                        }
                        fn_ty = *ret_ty;
                    }
                    other => return Err(TypeError::NotAFunction(other)),
                }
            }
            Ok(fn_ty)
        }
    }
}

fn check(
    _ctx: &Context,
    _expr: &Expr,
    expected: &Type,
    inferred: Type,
) -> Result<(), TypeError> {
    if &inferred != expected {
        Err(TypeError::Mismatch {
            expected: expected.clone(),
            found: inferred,
        })
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

fn infer_binop(op: &BinOp, lt: Type, rt: Type) -> Result<Type, TypeError> {
    let op_str = format!("{op:?}");

    match op {
        BinOp::Add | BinOp::Sub | BinOp::Mul | BinOp::Div => match (&lt, &rt) {
            (Type::Int, Type::Int) => Ok(Type::Int),
            (Type::Float, Type::Float) => Ok(Type::Float),
            _ => Err(TypeError::InvalidOperands {
                op: op_str,
                left: lt,
                right: rt,
            }),
        },

        BinOp::Lt | BinOp::Gt | BinOp::Leq | BinOp::Geq => match (&lt, &rt) {
            (Type::Int, Type::Int) | (Type::Float, Type::Float) => Ok(Type::Bool),
            _ => Err(TypeError::InvalidOperands {
                op: op_str,
                left: lt,
                right: rt,
            }),
        },

        BinOp::Eq | BinOp::Neq => {
            if lt == rt {
                Ok(Type::Bool)
            } else {
                Err(TypeError::InvalidOperands {
                    op: op_str,
                    left: lt,
                    right: rt,
                })
            }
        }

        BinOp::And | BinOp::Or => match (&lt, &rt) {
            (Type::Bool, Type::Bool) => Ok(Type::Bool),
            _ => Err(TypeError::InvalidOperands {
                op: op_str,
                left: lt,
                right: rt,
            }),
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::syntax::{BinOp, Expr, Type, UnaryOp};

    fn tc(e: Expr) -> Result<Type, TypeError> {
        typecheck(&e)
    }

    #[test]
    fn test_unit() {
        assert_eq!(tc(Expr::Unit), Ok(Type::Unit));
    }

    #[test]
    fn test_bool() {
        assert_eq!(tc(Expr::Bool(true)), Ok(Type::Bool));
    }

    #[test]
    fn test_int() {
        assert_eq!(tc(Expr::Int(42)), Ok(Type::Int));
    }

    #[test]
    fn test_float() {
        assert_eq!(tc(Expr::Float(3.14)), Ok(Type::Float));
    }

    #[test]
    fn test_neg_int() {
        assert_eq!(
            tc(Expr::UnaryOp(UnaryOp::Neg, Box::new(Expr::Int(1)))),
            Ok(Type::Int)
        );
    }

    #[test]
    fn test_neg_float() {
        assert_eq!(
            tc(Expr::UnaryOp(UnaryOp::Neg, Box::new(Expr::Float(1.0)))),
            Ok(Type::Float)
        );
    }

    #[test]
    fn test_neg_bool_err() {
        assert!(tc(Expr::UnaryOp(UnaryOp::Neg, Box::new(Expr::Bool(true)))).is_err());
    }

    #[test]
    fn test_not_bool() {
        assert_eq!(
            tc(Expr::UnaryOp(UnaryOp::Not, Box::new(Expr::Bool(false)))),
            Ok(Type::Bool)
        );
    }

    #[test]
    fn test_not_int_err() {
        assert!(tc(Expr::UnaryOp(UnaryOp::Not, Box::new(Expr::Int(0)))).is_err());
    }

    #[test]
    fn test_add_int() {
        assert_eq!(
            tc(Expr::BinOp(
                BinOp::Add,
                Box::new(Expr::Int(1)),
                Box::new(Expr::Int(2))
            )),
            Ok(Type::Int)
        );
    }

    #[test]
    fn test_add_float() {
        assert_eq!(
            tc(Expr::BinOp(
                BinOp::Add,
                Box::new(Expr::Float(1.0)),
                Box::new(Expr::Float(2.0))
            )),
            Ok(Type::Float)
        );
    }

    #[test]
    fn test_add_int_float_err() {
        assert!(tc(Expr::BinOp(
            BinOp::Add,
            Box::new(Expr::Int(1)),
            Box::new(Expr::Float(2.0))
        ))
        .is_err());
    }

    #[test]
    fn test_lt_int() {
        assert_eq!(
            tc(Expr::BinOp(
                BinOp::Lt,
                Box::new(Expr::Int(1)),
                Box::new(Expr::Int(2))
            )),
            Ok(Type::Bool)
        );
    }

    #[test]
    fn test_eq_any_type() {
        assert_eq!(
            tc(Expr::BinOp(
                BinOp::Eq,
                Box::new(Expr::Bool(true)),
                Box::new(Expr::Bool(false))
            )),
            Ok(Type::Bool)
        );
    }

    #[test]
    fn test_eq_type_mismatch_err() {
        assert!(tc(Expr::BinOp(
            BinOp::Eq,
            Box::new(Expr::Int(1)),
            Box::new(Expr::Bool(true))
        ))
        .is_err());
    }

    #[test]
    fn test_and_bool() {
        assert_eq!(
            tc(Expr::BinOp(
                BinOp::And,
                Box::new(Expr::Bool(true)),
                Box::new(Expr::Bool(false))
            )),
            Ok(Type::Bool)
        );
    }

    #[test]
    fn test_if_ok() {
        assert_eq!(
            tc(Expr::If(
                Box::new(Expr::Bool(true)),
                Box::new(Expr::Int(1)),
                Box::new(Expr::Int(0))
            )),
            Ok(Type::Int)
        );
    }

    #[test]
    fn test_if_cond_not_bool_err() {
        assert!(tc(Expr::If(
            Box::new(Expr::Int(1)),
            Box::new(Expr::Int(2)),
            Box::new(Expr::Int(3))
        ))
        .is_err());
    }

    #[test]
    fn test_if_branch_mismatch_err() {
        assert!(tc(Expr::If(
            Box::new(Expr::Bool(true)),
            Box::new(Expr::Int(1)),
            Box::new(Expr::Bool(false))
        ))
        .is_err());
    }

    #[test]
    fn test_let_no_ann() {
        assert_eq!(
            tc(Expr::Let(
                "x".to_string(),
                None,
                Box::new(Expr::Int(1)),
                Box::new(Expr::Var("x".to_string()))
            )),
            Ok(Type::Int)
        );
    }

    #[test]
    fn test_let_with_correct_ann() {
        assert_eq!(
            tc(Expr::Let(
                "x".to_string(),
                Some(Type::Int),
                Box::new(Expr::Int(1)),
                Box::new(Expr::Var("x".to_string()))
            )),
            Ok(Type::Int)
        );
    }

    #[test]
    fn test_let_wrong_ann_err() {
        assert!(tc(Expr::Let(
            "x".to_string(),
            Some(Type::Bool),
            Box::new(Expr::Int(1)),
            Box::new(Expr::Var("x".to_string()))
        ))
        .is_err());
    }

    #[test]
    fn test_lambda_identity_int() {
        assert_eq!(
            tc(Expr::Lambda(
                vec![("x".to_string(), Type::Int)],
                Box::new(Expr::Var("x".to_string()))
            )),
            Ok(Type::Arrow(Box::new(Type::Int), Box::new(Type::Int)))
        );
    }

    #[test]
    fn test_lambda_multi_param() {
        assert_eq!(
            tc(Expr::Lambda(
                vec![
                    ("x".to_string(), Type::Int),
                    ("y".to_string(), Type::Int)
                ],
                Box::new(Expr::BinOp(
                    BinOp::Add,
                    Box::new(Expr::Var("x".to_string())),
                    Box::new(Expr::Var("y".to_string()))
                ))
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
            tc(Expr::App(
                Box::new(Expr::Lambda(
                    vec![("x".to_string(), Type::Int)],
                    Box::new(Expr::Var("x".to_string()))
                )),
                vec![Expr::Int(42)]
            )),
            Ok(Type::Int)
        );
    }

    #[test]
    fn test_app_wrong_arg_type_err() {
        assert!(tc(Expr::App(
            Box::new(Expr::Lambda(
                vec![("x".to_string(), Type::Int)],
                Box::new(Expr::Var("x".to_string()))
            )),
            vec![Expr::Bool(true)]
        ))
        .is_err());
    }

    #[test]
    fn test_app_not_function_err() {
        assert!(tc(Expr::App(
            Box::new(Expr::Int(42)),
            vec![Expr::Int(1)]
        ))
        .is_err());
    }

    #[test]
    fn test_letrec_factorial() {
        let body = Expr::If(
            Box::new(Expr::BinOp(
                BinOp::Eq,
                Box::new(Expr::Var("n".to_string())),
                Box::new(Expr::Int(0)),
            )),
            Box::new(Expr::Int(1)),
            Box::new(Expr::BinOp(
                BinOp::Mul,
                Box::new(Expr::Var("n".to_string())),
                Box::new(Expr::App(
                    Box::new(Expr::Var("fact".to_string())),
                    vec![Expr::BinOp(
                        BinOp::Sub,
                        Box::new(Expr::Var("n".to_string())),
                        Box::new(Expr::Int(1)),
                    )],
                )),
            )),
        );

        let rest = Expr::App(
            Box::new(Expr::Var("fact".to_string())),
            vec![Expr::Int(5)],
        );

        assert_eq!(
            tc(Expr::LetRec(
                "fact".to_string(),
                vec![("n".to_string(), Type::Int)],
                Type::Int,
                Box::new(body),
                Box::new(rest)
            )),
            Ok(Type::Int)
        );
    }

    #[test]
    fn test_letrec_wrong_body_type_err() {
        assert!(tc(Expr::LetRec(
            "f".to_string(),
            vec![("n".to_string(), Type::Int)],
            Type::Int,
            Box::new(Expr::Bool(true)),
            Box::new(Expr::App(
                Box::new(Expr::Var("f".to_string())),
                vec![Expr::Int(0)]
            ))
        ))
        .is_err());
    }

    #[test]
    fn test_ann_ok() {
        assert_eq!(
            tc(Expr::Ann(Box::new(Expr::Int(1)), Box::new(Type::Int))),
            Ok(Type::Int)
        );
    }

    #[test]
    fn test_ann_mismatch_err() {
        assert!(tc(Expr::Ann(Box::new(Expr::Int(1)), Box::new(Type::Bool))).is_err());
    }

    #[test]
    fn test_unbound_variable_err() {
        assert!(tc(Expr::Var("foo".to_string())).is_err());
    }
}