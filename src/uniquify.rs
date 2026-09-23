use crate::{gensym::Gensym, syntax::Ident, typechecker::TypedExpr};
use std::collections::HashMap;

type NameEnv = HashMap<Ident, Ident>;

fn bind(gensym: &mut Gensym, env: &mut NameEnv, name: Ident) -> (Ident, Option<Ident>) {
    let new_name = gensym.inc_fresh(&name);
    let old = env.insert(name, new_name.clone());
    (new_name, old)
}

fn unbind(env: &mut NameEnv, name: Ident, old: Option<Ident>) {
    match old {
        Some(prev) => {
            env.insert(name, prev);
        },
        None => {
            env.remove(&name);
        },
    }
}

pub fn rename(gensym: &mut Gensym, env: &mut NameEnv, expr: TypedExpr) -> TypedExpr {
    match expr {
        TypedExpr::Unit => expr,
        TypedExpr::Bool(_) => expr,
        TypedExpr::Int(_) => expr,
        TypedExpr::Float(_) => expr,
        TypedExpr::Tuple(typed_exprs, ty) => {
            let typed_exprs = typed_exprs.into_iter().map(|e| rename(gensym, env, e)).collect();
            TypedExpr::Tuple(typed_exprs, ty)
        },
        TypedExpr::TupleProjection(expr, index, ty) => {
            let expr = rename(gensym, env, *expr);
            TypedExpr::TupleProjection(Box::new(expr), index, ty)
        },
        TypedExpr::PrimIO(prim_io, typed_expr, ty) => match typed_expr {
            Some(typed_expr) => {
                let renamed = rename(gensym, env, *typed_expr);
                TypedExpr::PrimIO(prim_io, Some(Box::new(renamed)), ty)
            },
            None => TypedExpr::PrimIO(prim_io, None, ty),
        },
        TypedExpr::BinOp(op, left, right, ty) => {
            let left = rename(gensym, env, *left);
            let right = rename(gensym, env, *right);
            TypedExpr::BinOp(op, Box::new(left), Box::new(right), ty)
        },
        TypedExpr::UnaryOp(op, expr, ty) => {
            let expr = rename(gensym, env, *expr);
            TypedExpr::UnaryOp(op, Box::new(expr), ty)
        },
        TypedExpr::Ann(expr, ty) => {
            let expr = rename(gensym, env, *expr);
            TypedExpr::Ann(Box::new(expr), ty)
        },
        TypedExpr::If(cond, thn, els, ty) => {
            let cond = rename(gensym, env, *cond);
            let thn = rename(gensym, env, *thn);
            let els = rename(gensym, env, *els);
            TypedExpr::If(Box::new(cond), Box::new(thn), Box::new(els), ty)
        },
        TypedExpr::Let(name, ty, rhs, body, let_ty) => {
            let rhs = rename(gensym, env, *rhs);
            let (new_name, old) = bind(gensym, env, name.clone());
            let body = rename(gensym, env, *body);
            unbind(env, name, old);
            TypedExpr::Let(new_name, ty, Box::new(rhs), Box::new(body), let_ty)
        },
        TypedExpr::Var(name, ty) => {
            let new_name = env.get(&name).expect("unbound variable");
            TypedExpr::Var(new_name.to_string(), ty)
        },
        TypedExpr::LetRec(fname, fparams, fty, fbody, body, letrec_ty) => {
            let (new_fname, old_fname) = bind(gensym, env, fname.clone());

            let mut new_fparams = vec![];
            let mut old_fparams = vec![];
            for (name, param_ty) in fparams {
                let (new_name, old) = bind(gensym, env, name.clone());
                old_fparams.push((name, old));
                new_fparams.push((new_name, param_ty));
            }

            let fbody = rename(gensym, env, *fbody);

            for (name, old) in old_fparams.into_iter().rev() {
                unbind(env, name, old);
            }

            let body = rename(gensym, env, *body);
            unbind(env, fname, old_fname);

            TypedExpr::LetRec(
                new_fname,
                new_fparams,
                fty,
                Box::new(fbody),
                Box::new(body),
                letrec_ty,
            )
        },
        TypedExpr::App(func, args, ty) => {
            let func = rename(gensym, env, *func);
            let args = args.into_iter().map(|e| rename(gensym, env, e)).collect();
            TypedExpr::App(Box::new(func), args, ty)
        },
        TypedExpr::Lambda(param, ty, body, lambda_ty) => {
            let mut new_params = vec![];
            let mut old_params = vec![];
            for (name, param_ty) in param {
                let (new_name, old) = bind(gensym, env, name.clone());
                old_params.push((name, old));
                new_params.push((new_name, param_ty));
            }

            let body = rename(gensym, env, *body);

            for (name, old) in old_params.into_iter().rev() {
                unbind(env, name, old);
            }

            TypedExpr::Lambda(new_params, ty, Box::new(body), lambda_ty)
        },
    }
}

pub fn uniquify_convert(expr: TypedExpr) -> TypedExpr {
    let mut gensym = Gensym::new();
    let mut env = NameEnv::new();
    rename(&mut gensym, &mut env, expr)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::syntax::{BinOp, PrimIO, Type, UnaryOp};
    use crate::typechecker::{
        t_ann, t_app, t_bin_op, t_bool, t_float, t_if, t_int, t_lambda, t_let, t_let_rec,
        t_prim_io, t_tuple, t_unary, t_unit, t_var,
    };

    fn expect_let(expr: &TypedExpr) -> (&Ident, &TypedExpr, &TypedExpr) {
        match expr {
            TypedExpr::Let(name, _, rhs, body, _) => (name, rhs, body),
            other => {
                panic!("expected TypedExpr::Let, got {:?}", other)
            },
        }
    }

    fn expect_var(expr: &TypedExpr) -> &Ident {
        match expr {
            TypedExpr::Var(name, _) => name,
            other => {
                panic!("expected TypedExpr::Var, got {:?}", other)
            },
        }
    }

    #[test]
    fn unit_is_unchanged() {
        assert_eq!(uniquify_convert(t_unit()), t_unit());
    }

    #[test]
    fn bool_is_unchanged() {
        assert_eq!(uniquify_convert(t_bool(true)), t_bool(true));
        assert_eq!(uniquify_convert(t_bool(false)), t_bool(false));
    }

    #[test]
    fn int_is_unchanged() {
        assert_eq!(uniquify_convert(t_int(42)), t_int(42));
    }

    #[test]
    fn float_is_unchanged() {
        assert_eq!(uniquify_convert(t_float(3.14)), t_float(3.14));
    }

    #[test]
    fn let_renames_var_in_body() {
        let expr = t_let("x", Type::Int, t_int(1), t_var("x", Type::Int), Type::Int);
        let renamed = uniquify_convert(expr);
        let (bound_name, _, body) = expect_let(&renamed);
        let used_name = expect_var(body);
        assert_eq!(bound_name, used_name);
        assert!(bound_name.starts_with("x."));
    }

    #[test]
    fn fresh_names_are_distinct_across_lets() {
        let expr = t_let(
            "x",
            Type::Int,
            t_int(1),
            t_let("x", Type::Int, t_int(2), t_var("x", Type::Int), Type::Int),
            Type::Int,
        );
        let renamed = uniquify_convert(expr);
        let (outer_name, _, outer_body) = expect_let(&renamed);
        let (inner_name, _, inner_body) = expect_let(outer_body);
        let used_name = expect_var(inner_body);
        assert_ne!(outer_name, inner_name, "shadowing let must get a new fresh name");
        assert_eq!(inner_name, used_name, "body must refer to the innermost x");
    }

    #[test]
    fn outer_let_survives_lambda_param_shadowing() {
        let expr = t_let(
            "x",
            Type::Int,
            t_int(1),
            t_let(
                "f",
                Type::Arrow(Box::new(Type::Int), Box::new(Type::Int)),
                t_lambda(
                    vec![("x".to_string(), Type::Int)],
                    Type::Int,
                    t_var("x", Type::Int),
                    Type::Arrow(Box::new(Type::Int), Box::new(Type::Int)),
                ),
                t_var("x", Type::Int),
                Type::Int,
            ),
            Type::Int,
        );

        let renamed = uniquify_convert(expr);
        let (outer_name, _, outer_body) = expect_let(&renamed);
        let (_, lambda_rhs, final_body) = expect_let(outer_body);

        let lambda_param_name = match lambda_rhs {
            TypedExpr::Lambda(params, _, lambda_body, _) => {
                let param_name = &params[0].0;
                let used_in_lambda = expect_var(lambda_body);
                assert_eq!(param_name, used_in_lambda, "lambda body must refer to its own param");
                param_name.clone()
            },
            other => panic!("expected Lambda, got {:?}", other),
        };

        let final_name = expect_var(final_body);
        assert_eq!(
            final_name, outer_name,
            "outer `x` must not be hijacked by the lambda's shadowing `x`"
        );
        assert_ne!(
            final_name, &lambda_param_name,
            "outer x and lambda's x must end up as different fresh names"
        );
    }

    #[test]
    fn letrec_params_dont_leak_into_continuation() {
        let expr = t_let(
            "x",
            Type::Int,
            t_int(0),
            t_let_rec(
                "f",
                vec![("x".to_string(), Type::Int)],
                Type::Int,
                t_var("x", Type::Int),
                t_var("x", Type::Int),
                Type::Int,
            ),
            Type::Int,
        );

        let renamed = uniquify_convert(expr);
        let (outer_x, _, letrec) = expect_let(&renamed);

        match letrec {
            TypedExpr::LetRec(_, fparams, _, fbody, cont, _) => {
                let param_name = &fparams[0].0;
                let body_var = expect_var(fbody);
                assert_eq!(param_name, body_var, "function body refers to its own param");

                let cont_var = expect_var(cont);
                assert_eq!(
                    cont_var, outer_x,
                    "continuation after LetRec must see the outer x, not f's argument"
                );
            },
            other => panic!("expected LetRec, got {:?}", other),
        }
    }

    #[test]
    fn letrec_continuation_can_still_call_fname() {
        let expr = t_let_rec(
            "fact",
            vec![("n".to_string(), Type::Int)],
            Type::Int,
            t_var("n", Type::Int),
            t_var("fact", Type::Int),
            Type::Int,
        );
        let renamed = uniquify_convert(expr);
        match renamed {
            TypedExpr::LetRec(new_fname, _, _, _, cont, _) => {
                let cont_name = expect_var(&cont);
                assert_eq!(
                    &new_fname, cont_name,
                    "continuation must resolve fact to its fresh name"
                );
            },
            other => panic!("expected LetRec, got {:?}", other),
        }
    }

    #[test]
    fn lambda_multi_param_each_gets_fresh_name() {
        let expr = t_lambda(
            vec![("x".to_string(), Type::Int), ("y".to_string(), Type::Int)],
            Type::Int,
            t_bin_op(BinOp::Add, t_var("x", Type::Int), t_var("y", Type::Int), Type::Int),
            Type::Arrow(Box::new(Type::Int), Box::new(Type::Int)),
        );
        let renamed = uniquify_convert(expr);
        match renamed {
            TypedExpr::Lambda(params, _, body, _) => {
                assert_ne!(params[0].0, params[1].0);
                match *body {
                    TypedExpr::BinOp(BinOp::Add, l, r, _) => {
                        assert_eq!(expect_var(&l), &params[0].0);
                        assert_eq!(expect_var(&r), &params[1].0);
                    },
                    other => {
                        panic!("expected BinOp, got {:?}", other)
                    },
                }
            },
            other => panic!("expected Lambda, got {:?}", other),
        }
    }

    #[test]
    fn binop_renames_both_sides() {
        let expr = t_let(
            "x",
            Type::Int,
            t_int(1),
            t_let(
                "y",
                Type::Int,
                t_int(2),
                t_bin_op(BinOp::Add, t_var("x", Type::Int), t_var("y", Type::Int), Type::Int),
                Type::Int,
            ),
            Type::Int,
        );
        let renamed = uniquify_convert(expr);
        let (x_name, _, body1) = expect_let(&renamed);
        let (y_name, _, body2) = expect_let(body1);
        match body2 {
            TypedExpr::BinOp(BinOp::Add, l, r, _) => {
                assert_eq!(expect_var(l), x_name);
                assert_eq!(expect_var(r), y_name);
            },
            other => panic!("expected BinOp, got {:?}", other),
        }
    }

    #[test]
    fn unaryop_renames_inner() {
        let expr = t_let(
            "x",
            Type::Int,
            t_int(1),
            t_unary(UnaryOp::Neg, t_var("x", Type::Int), Type::Int),
            Type::Int,
        );
        let renamed = uniquify_convert(expr);
        let (x_name, _, body) = expect_let(&renamed);
        match body {
            TypedExpr::UnaryOp(UnaryOp::Neg, inner, _) => {
                assert_eq!(expect_var(inner), x_name)
            },
            other => panic!("expected UnaryOp, got {:?}", other),
        }
    }

    #[test]
    fn if_renames_all_three_branches() {
        let expr = t_let(
            "x",
            Type::Int,
            t_int(1),
            t_if(t_var("x", Type::Int), t_var("x", Type::Int), t_var("x", Type::Int), Type::Int),
            Type::Int,
        );
        let renamed = uniquify_convert(expr);
        let (x_name, _, body) = expect_let(&renamed);
        match body {
            TypedExpr::If(c, t, e, _) => {
                assert_eq!(expect_var(c), x_name);
                assert_eq!(expect_var(t), x_name);
                assert_eq!(expect_var(e), x_name);
            },
            other => panic!("expected If, got {:?}", other),
        }
    }

    #[test]
    fn app_renames_func_and_all_args() {
        let expr = t_let(
            "f",
            Type::Int,
            t_int(1),
            t_let(
                "x",
                Type::Int,
                t_int(2),
                t_app(
                    t_var("f", Type::Int),
                    vec![t_var("x", Type::Int), t_var("x", Type::Int)],
                    Type::Int,
                ),
                Type::Int,
            ),
            Type::Int,
        );
        let renamed = uniquify_convert(expr);
        let (f_name, _, body1) = expect_let(&renamed);
        let (x_name, _, body2) = expect_let(body1);
        match body2 {
            TypedExpr::App(func, args, _) => {
                assert_eq!(expect_var(func), f_name);
                assert_eq!(args.len(), 2);
                assert_eq!(expect_var(&args[0]), x_name);
                assert_eq!(expect_var(&args[1]), x_name);
            },
            other => panic!("expected App, got {:?}", other),
        }
    }

    #[test]
    fn ann_inner_expr_is_actually_renamed() {
        let expr = t_let(
            "x",
            Type::Int,
            t_int(1),
            t_ann(t_var("x", Type::Int), Type::Int),
            Type::Int,
        );
        let renamed = uniquify_convert(expr);
        let (x_name, _, body) = expect_let(&renamed);
        match body {
            TypedExpr::Ann(inner, ty) => {
                assert_eq!(
                    expect_var(inner),
                    x_name,
                    "Ann must rename its inner expr, not clone it verbatim"
                );
                assert_eq!(*ty, Type::Int);
            },
            other => panic!("expected Ann, got {:?}", other),
        }
    }

    #[test]
    fn ann_nested_inside_lambda_still_renamed() {
        let expr = t_lambda(
            vec![("x".to_string(), Type::Int)],
            Type::Int,
            t_ann(t_var("x", Type::Int), Type::Int),
            Type::Arrow(Box::new(Type::Int), Box::new(Type::Int)),
        );
        let renamed = uniquify_convert(expr);
        match renamed {
            TypedExpr::Lambda(params, _, body, _) => match *body {
                TypedExpr::Ann(inner, _) => {
                    assert_eq!(expect_var(&inner), &params[0].0)
                },
                other => panic!("expected Ann, got {:?}", other),
            },
            other => panic!("expected Lambda, got {:?}", other),
        }
    }

    #[test]
    fn letrec_factorial_shape_renames_consistently() {
        let expr = t_let_rec(
            "fact",
            vec![("n".to_string(), Type::Int)],
            Type::Int,
            t_if(
                t_bin_op(BinOp::Eq, t_var("n", Type::Int), t_int(0), Type::Int),
                t_int(1),
                t_bin_op(
                    BinOp::Mul,
                    t_var("n", Type::Int),
                    t_app(
                        t_var("fact", Type::Int),
                        vec![t_bin_op(BinOp::Sub, t_var("n", Type::Int), t_int(1), Type::Int)],
                        Type::Int,
                    ),
                    Type::Int,
                ),
                Type::Int,
            ),
            t_app(t_var("fact", Type::Int), vec![t_int(5)], Type::Int),
            Type::Int,
        );

        let renamed = uniquify_convert(expr);
        match renamed {
            TypedExpr::LetRec(new_fname, fparams, _, fbody, cont, _) => {
                let n_name = &fparams[0].0;

                match *fbody {
                    TypedExpr::If(cond, _, else_branch, _) => {
                        match *cond {
                            TypedExpr::BinOp(BinOp::Eq, l, _, _) => {
                                assert_eq!(expect_var(&l), n_name)
                            },
                            other => {
                                panic!("expected Eq, got {:?}", other)
                            },
                        }
                        match *else_branch {
                            TypedExpr::BinOp(BinOp::Mul, l, r, _) => {
                                assert_eq!(expect_var(&l), n_name);
                                match *r {
                                    TypedExpr::App(func, args, _) => {
                                        assert_eq!(
                                            expect_var(&func),
                                            &new_fname,
                                            "recursive call must use fact's fresh name"
                                        );
                                        match &args[0] {
                                            TypedExpr::BinOp(BinOp::Sub, l, _, _) => {
                                                assert_eq!(expect_var(l), n_name)
                                            },
                                            other => panic!("expected Sub, got {:?}", other),
                                        }
                                    },
                                    other => panic!("expected App, got {:?}", other),
                                }
                            },
                            other => panic!("expected Mul, got {:?}", other),
                        }
                    },
                    other => panic!("expected If, got {:?}", other),
                }

                match *cont {
                    TypedExpr::App(func, args, _) => {
                        assert_eq!(
                            expect_var(&func),
                            &new_fname,
                            "top-level call site must use fact's fresh name"
                        );
                        assert_eq!(args, vec![t_int(5)]);
                    },
                    other => panic!("expected App, got {:?}", other),
                }
            },
            other => panic!("expected LetRec, got {:?}", other),
        }
    }

    #[test]
    fn tuple_renames_elements() {
        let expr = t_let(
            "x",
            Type::Int,
            t_int(1),
            t_tuple(
                vec![t_var("x", Type::Int), t_int(2)],
                Type::Tuple(vec![Type::Int, Type::Int]),
            ),
            Type::Tuple(vec![Type::Int, Type::Int]),
        );
        let renamed = uniquify_convert(expr);
        let (x_name, _, body) = expect_let(&renamed);
        match body {
            TypedExpr::Tuple(elements, _) => {
                assert_eq!(elements.len(), 2);
                assert_eq!(expect_var(&elements[0]), x_name);
                assert_eq!(elements[1], t_int(2));
            },
            other => panic!("expected Tuple, got {:?}", other),
        }
    }

    #[test]
    fn prim_io_renames_inner() {
        let expr = t_let(
            "x",
            Type::Int,
            t_int(1),
            t_prim_io(PrimIO::PrintInt, Some(t_var("x", Type::Int)), Type::Unit),
            Type::Unit,
        );
        let renamed = uniquify_convert(expr);
        let (x_name, _, body) = expect_let(&renamed);
        match body {
            TypedExpr::PrimIO(PrimIO::PrintInt, Some(inner), _) => {
                assert_eq!(expect_var(inner), x_name);
            },
            other => panic!("expected PrimIO, got {:?}", other),
        }
    }

    #[test]
    fn prim_io_read_none_unchanged() {
        let expr = t_prim_io(PrimIO::ReadInt, None, Type::Unit);
        assert_eq!(uniquify_convert(expr.clone()), expr);
    }
}
