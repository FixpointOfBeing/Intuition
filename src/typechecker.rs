use std::collections::HashMap;
use crate::syntax::{BinOp, Expr, Ident, Type, UnaryOp};


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
}

impl std::fmt::Display for TypeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TypeError::UnboundVariable(name) => write!(f, "Unbound variable: {name}"),
            TypeError::Mismatch { expected, found } => {
                write!(f, "Type mismatch: expected {expected:?}, found {found:?}")
            }
            TypeError::ReturnTypeMismatch { expected, found } => {
                write!(f, "Return type mismatch: expected {expected:?}, found {found:?}")
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

        Expr::Lambda(params, opty, body) => {
            let mut lam_ctx = ctx.clone();
            let mut param_tys = Vec::new();
            for (pname, pty) in params {
                lam_ctx = lam_ctx.extend(pname.clone(), pty.clone());
                param_tys.push(pty.clone());
            }
            let body_ty = infer(&lam_ctx, body)?;
            if let Some(rt_ty) = opty {
                if body_ty != *rt_ty {
                    return Err(TypeError::ReturnTypeMismatch {
                        expected: (*rt_ty).clone(),
                        found: body_ty,
                    });
                }
            }
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
    use crate::syntax::Type;
    use lalrpop_util::lalrpop_mod;
    lalrpop_mod!(pub parser);

    fn tc(src: &str) -> Result<Type, TypeError> {
        let expr = parser::ExprParser::new()
            .parse(src)
            .unwrap_or_else(|e| panic!("failed to parse `{src}`: {e:?}"));
        typecheck(&expr)
    }

    #[test]
    fn test_unit() {
        assert_eq!(tc("()"), Ok(Type::Unit));
    }

    #[test]
    fn test_bool() {
        assert_eq!(tc("true"), Ok(Type::Bool));
    }

    #[test]
    fn test_int() {
        assert_eq!(tc("42"), Ok(Type::Int));
    }

    #[test]
    fn test_float() {
        assert_eq!(tc("3.14"), Ok(Type::Float));
    }

    #[test]
    fn test_neg_int() {
        assert_eq!(tc("-1"), Ok(Type::Int));
    }

    #[test]
    fn test_neg_float() {
        assert_eq!(tc("-1.0"), Ok(Type::Float));
    }

    #[test]
    fn test_neg_bool_err() {
        assert!(tc("-true").is_err());
    }

    #[test]
    fn test_not_bool() {
        assert_eq!(tc("!false"), Ok(Type::Bool));
    }

    #[test]
    fn test_not_int_err() {
        assert!(tc("!0").is_err());
    }

    #[test]
    fn test_add_int() {
        assert_eq!(tc("1 + 2"), Ok(Type::Int));
    }

    #[test]
    fn test_add_float() {
        assert_eq!(tc("1.0 + 2.0"), Ok(Type::Float));
    }

    #[test]
    fn test_add_int_float_err() {
        assert!(tc("1 + 2.0").is_err());
    }

    #[test]
    fn test_lt_int() {
        assert_eq!(tc("1 < 2"), Ok(Type::Bool));
    }

    #[test]
    fn test_eq_any_type() {
        assert_eq!(tc("true == false"), Ok(Type::Bool));
    }

    #[test]
    fn test_eq_type_mismatch_err() {
        assert!(tc("1 == true").is_err());
    }

    #[test]
    fn test_and_bool() {
        assert_eq!(tc("true && false"), Ok(Type::Bool));
    }

    #[test]
    fn test_if_ok() {
        assert_eq!(tc("if true then 1 else 0"), Ok(Type::Int));
    }

    #[test]
    fn test_if_cond_not_bool_err() {
        assert!(tc("if 1 then 2 else 3").is_err());
    }

    #[test]
    fn test_if_branch_mismatch_err() {
        assert!(tc("if true then 1 else false").is_err());
    }

    #[test]
    fn test_let_no_ann() {
        assert_eq!(tc("let x = 1 in x"), Ok(Type::Int));
    }

    #[test]
    fn test_let_with_correct_ann() {
        assert_eq!(tc("let x: Int = 1 in x"), Ok(Type::Int));
    }

    #[test]
    fn test_let_wrong_ann_err() {
        assert!(tc("let x: Bool = 1 in x").is_err());
    }

    #[test]
    fn test_lambda_identity_int() {
        assert_eq!(
            tc("fun (x: Int) => x"),
            Ok(Type::Arrow(Box::new(Type::Int), Box::new(Type::Int)))
        );
    }

    #[test]
    fn test_lambda_multi_param() {
        assert_eq!(
            tc("fun (x: Int) (y: Int) => x + y"),
            Ok(Type::Arrow(
                Box::new(Type::Int),
                Box::new(Type::Arrow(Box::new(Type::Int), Box::new(Type::Int)))
            ))
        );
    }

    #[test]
    fn test_app_lambda() {
        assert_eq!(tc("(fun (x: Int) : Int => x) 42"), Ok(Type::Int));
    }

    #[test]
    fn test_app_wrong_arg_type_err() {
        assert!(tc("(fun (x: Int) => x) true").is_err());
    }

    #[test]
    fn test_app_not_function_err() {
        assert!(tc("42 1").is_err());
    }
    #[test]
    fn test_function_ret_mismatch() {
        assert!(tc("fun (x : Int) : Int => x <= 37").is_err())
    }

    #[test]
    fn test_letrec_factorial() {
        assert_eq!(
            tc("let rec fact (n: Int) : Int = \
                    if n == 0 then 1 else n * fact(n - 1) \
                in fact(5)"),
            Ok(Type::Int)
        );
    }

    #[test]
    fn test_letrec_wrong_body_type_err() {
        assert!(tc("let rec f (n: Int) : Int = true in f(0)").is_err());
    }

    #[test]
    fn test_ann_ok() {
        assert_eq!(tc("(1 : Int)"), Ok(Type::Int));
    }

    #[test]
    fn test_ann_mismatch_err() {
        assert!(tc("(1 : Bool)").is_err());
    }

    #[test]
    fn test_unbound_variable_err() {
        assert!(tc("foo").is_err());
    }
}