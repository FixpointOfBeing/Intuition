use crate::{
    gensym::Gensym,
    syntax::Ident,
    typechecker::{TypedDef, TypedExpr, TypedProgram},
};
use std::collections::HashMap;

type NameEnv = HashMap<Ident, Ident>;

// fn bind(gensym: &mut Gensym, env: &mut NameEnv, name: Ident) -> (Ident, Option<Ident>) {
//     let new_name = gensym.inc_fresh(&name);
//     let old = env.insert(name, new_name.clone());
//     (new_name, old)
// }
//
// fn unbind(env: &mut NameEnv, name: Ident, old: Option<Ident>) {
//     match old {
//         Some(prev) => {
//             env.insert(name, prev);
//         },
//         None => {
//             env.remove(&name);
//         },
//     }
// }
//
pub fn rename(gensym: &mut Gensym, env: &mut NameEnv, expr: TypedExpr) -> TypedExpr {
    match expr {
        TypedExpr::Unit => expr,
        TypedExpr::Bool(_) => expr,
        TypedExpr::Int(_) => expr,
        TypedExpr::Float(_) => expr,
        TypedExpr::Tuple(typed_exprs, ty) => {
            let typed_exprs = typed_exprs
                .into_iter()
                .map(|e| rename(gensym, &mut env.clone(), e))
                .collect();
            TypedExpr::Tuple(typed_exprs, ty)
        },
        TypedExpr::TupleProj(expr, index, ty) => {
            let expr = rename(gensym, env, *expr);
            TypedExpr::TupleProj(Box::new(expr), index, ty)
        },
        TypedExpr::PrimIO(prim_io, typed_expr, ty) => match typed_expr {
            Some(typed_expr) => {
                let renamed = rename(gensym, env, *typed_expr);
                TypedExpr::PrimIO(prim_io, Some(Box::new(renamed)), ty)
            },
            None => TypedExpr::PrimIO(prim_io, None, ty),
        },
        TypedExpr::BinOp(op, left, right, ty) => {
            let left = rename(gensym, &mut env.clone(), *left);
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
            let cond = rename(gensym, &mut env.clone(), *cond);
            let thn = rename(gensym, &mut env.clone(), *thn);
            let els = rename(gensym, &mut env.clone(), *els);
            TypedExpr::If(Box::new(cond), Box::new(thn), Box::new(els), ty)
        },
        TypedExpr::Let(name, ty, rhs, body, let_ty) => {
            let rhs = rename(gensym, &mut env.clone(), *rhs);

            let new_name = gensym.inc_fresh(&name);
            env.insert(name, new_name.clone());
            let body = rename(gensym, env, *body);
            TypedExpr::Let(new_name, ty, Box::new(rhs), Box::new(body), let_ty)
        },
        TypedExpr::Var(name, ty) => {
            let new_name = env.get(&name).expect(&format!("unbound variable: {}", name));
            TypedExpr::Var(new_name.to_string(), ty)
        },
        TypedExpr::LetRec(fname, fparams, fty, fbody, body, letrec_ty) => {
            let mut new_fparams = Vec::with_capacity(fparams.len());
            let new_fname = gensym.inc_fresh(&fname);
            let mut fbody_env = env.clone();
            fbody_env.insert(fname.clone(), new_fname.clone());
            for (name, param_ty) in fparams {
                let new_name = gensym.inc_fresh(&name);
                fbody_env.insert(name, new_name.clone());
                new_fparams.push((new_name, param_ty));
            }

            let fbody = rename(gensym, &mut fbody_env, *fbody);

            env.insert(fname, new_fname.clone());
            let body = rename(gensym, env, *body);

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
            let mut new_params = Vec::with_capacity(param.len());

            for (name, param_ty) in param {
                let new_name = gensym.inc_fresh(&name);
                env.insert(name, new_name.clone());
                new_params.push((new_name, param_ty));
            }

            let body = rename(gensym, env, *body);

            TypedExpr::Lambda(new_params, ty, Box::new(body), lambda_ty)
        },
    }
}

pub fn uniquify_expr(expr: TypedExpr) -> TypedExpr {
    let mut gensym = Gensym::new();
    let mut env = NameEnv::new();
    rename(&mut gensym, &mut env, expr)
}

pub fn uniquify_def(gensym: &mut Gensym, env: &NameEnv, def: TypedDef) -> TypedDef {
    match def {
        TypedDef::ValDef(name, ty, typed_expr) => {
            let mut val_env = env.clone();

            let new_name = gensym.inc_fresh(&name);
            let renamed = rename(gensym, &mut val_env, typed_expr);

            TypedDef::ValDef(new_name, ty, renamed)
        },
        TypedDef::FunDef(name, param_tys, rt_ty, typed_expr) => {
            let mut fun_env = env.clone();

            let new_name = gensym.inc_fresh(&name);
            fun_env.insert(name, new_name.clone());

            let mut new_param_tys = Vec::with_capacity(param_tys.len());
            for (param, ty) in param_tys {
                let new_param = gensym.inc_fresh(&param);
                fun_env.insert(param, new_param.clone());
                new_param_tys.push((new_param, ty));
            }
            let renamed = rename(gensym, &mut fun_env, typed_expr);

            TypedDef::FunDef(new_name, new_param_tys, rt_ty, renamed)
        },
    }
}

pub fn uniquify_program(prog: TypedProgram) -> TypedProgram {
    let mut gensym = Gensym::new();
    let mut env = NameEnv::new();
    let mut renamed_defs = Vec::with_capacity(prog.defs.len());
    for def in prog.defs {
        let name = def.name();
        let renamed_def = uniquify_def(&mut gensym, &env, def);
        let new_name = renamed_def.name();
        env.insert(name, new_name);
        renamed_defs.push(renamed_def);
    }
    let renamed_main = rename(&mut gensym, &mut env, prog.main);
    TypedProgram { defs: renamed_defs, main: renamed_main }
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

    fn expect_val_def(def: &TypedDef) -> (&Ident, &Option<Type>, &TypedExpr) {
        match def {
            TypedDef::ValDef(name, ty, expr) => (name, ty, expr),
            other => panic!("expected TypedDef::ValDef, got {:?}", other),
        }
    }

    fn expect_fun_def(def: &TypedDef) -> (&Ident, &Vec<(Ident, Type)>, &Type, &TypedExpr) {
        match def {
            TypedDef::FunDef(name, params, rt_ty, body) => (name, params, rt_ty, body),
            other => panic!("expected TypedDef::FunDef, got {:?}", other),
        }
    }

    fn run_def(def: TypedDef) -> TypedDef {
        let mut gensym = Gensym::new();
        let mut env = NameEnv::new();
        uniquify_def(&mut gensym, &mut env, def)
    }

    #[test]
    fn unit_is_unchanged() {
        assert_eq!(uniquify_expr(t_unit()), t_unit());
    }

    #[test]
    fn bool_is_unchanged() {
        assert_eq!(uniquify_expr(t_bool(true)), t_bool(true));
        assert_eq!(uniquify_expr(t_bool(false)), t_bool(false));
    }

    #[test]
    fn int_is_unchanged() {
        assert_eq!(uniquify_expr(t_int(42)), t_int(42));
    }

    #[test]
    fn float_is_unchanged() {
        assert_eq!(uniquify_expr(t_float(3.14)), t_float(3.14));
    }

    #[test]
    fn let_renames_var_in_body() {
        let expr = t_let("x", Type::Int, t_int(1), t_var("x", Type::Int), Type::Int);
        let renamed = uniquify_expr(expr);
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
        let renamed = uniquify_expr(expr);
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

        let renamed = uniquify_expr(expr);
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

        let renamed = uniquify_expr(expr);
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
        let renamed = uniquify_expr(expr);
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
        let renamed = uniquify_expr(expr);
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
        let renamed = uniquify_expr(expr);
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
        let renamed = uniquify_expr(expr);
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
        let renamed = uniquify_expr(expr);
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
        let renamed = uniquify_expr(expr);
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
        let expr =
            t_let("x", Type::Int, t_int(1), t_ann(t_var("x", Type::Int), Type::Int), Type::Int);
        let renamed = uniquify_expr(expr);
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
        let renamed = uniquify_expr(expr);
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

        let renamed = uniquify_expr(expr);

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
            t_tuple(vec![t_var("x", Type::Int), t_int(2)], Type::Tuple(vec![Type::Int, Type::Int])),
            Type::Tuple(vec![Type::Int, Type::Int]),
        );
        let renamed = uniquify_expr(expr);
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
        let renamed = uniquify_expr(expr);
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
        assert_eq!(uniquify_expr(expr.clone()), expr);
    }

    // ---- uniquify_def ----

    #[test]
    fn def_val_renames_name_and_literal_body() {
        let def = TypedDef::ValDef("x".to_string(), Some(Type::Int), t_int(42));
        let renamed = run_def(def);
        let (name, ty, body) = expect_val_def(&renamed);
        assert!(name.starts_with("x."));
        assert_eq!(*ty, Some(Type::Int));
        assert_eq!(body, &t_int(42));
    }

    #[test]
    fn def_val_renames_outer_reference() {
        let mut gensym = Gensym::new();
        let mut env = NameEnv::new();
        env.insert("y".to_string(), "y.0".to_string());
        let def = TypedDef::ValDef("x".to_string(), Some(Type::Int), t_var("y", Type::Int));
        let renamed = uniquify_def(&mut gensym, &mut env, def);
        let (name, _, body) = expect_val_def(&renamed);
        assert!(name.starts_with("x."));
        assert_eq!(expect_var(body), "y.0");
    }

    #[test]
    fn def_fun_renames_name_params_and_body() {
        let def = TypedDef::FunDef(
            "add".to_string(),
            vec![("a".to_string(), Type::Int), ("b".to_string(), Type::Int)],
            Type::Int,
            t_bin_op(BinOp::Add, t_var("a", Type::Int), t_var("b", Type::Int), Type::Int),
        );
        let renamed = run_def(def);
        let (name, params, _, body) = expect_fun_def(&renamed);
        assert!(name.starts_with("add."));
        assert!(params[0].0.starts_with("a."));
        assert!(params[1].0.starts_with("b."));
        assert_ne!(params[0].0, params[1].0);
        match body {
            TypedExpr::BinOp(BinOp::Add, l, r, _) => {
                assert_eq!(expect_var(l), &params[0].0);
                assert_eq!(expect_var(r), &params[1].0);
            },
            other => panic!("expected BinOp, got {:?}", other),
        }
    }

    #[test]
    fn def_fun_self_recursion() {
        let def = TypedDef::FunDef(
            "fact".to_string(),
            vec![("n".to_string(), Type::Int)],
            Type::Int,
            t_if(
                t_bin_op(BinOp::Eq, t_var("n", Type::Int), t_int(0), Type::Bool),
                t_int(1),
                t_app(
                    t_var("fact", Type::Int),
                    vec![t_bin_op(BinOp::Sub, t_var("n", Type::Int), t_int(1), Type::Int)],
                    Type::Int,
                ),
                Type::Int,
            ),
        );
        let renamed = run_def(def);
        let (name, params, _, body) = expect_fun_def(&renamed);
        assert!(name.starts_with("fact."));
        assert!(params[0].0.starts_with("n."));
        match body {
            TypedExpr::If(cond, _, els, _) => {
                match &**cond {
                    TypedExpr::BinOp(BinOp::Eq, l, _, _) => {
                        assert_eq!(expect_var(l), &params[0].0)
                    },
                    other => panic!("expected Eq, got {:?}", other),
                }
                match &**els {
                    TypedExpr::App(func, args, _) => {
                        assert_eq!(expect_var(func), name);
                        match &args[0] {
                            TypedExpr::BinOp(BinOp::Sub, l, _, _) => {
                                assert_eq!(expect_var(l), &params[0].0)
                            },
                            other => panic!("expected Sub, got {:?}", other),
                        }
                    },
                    other => panic!("expected App, got {:?}", other),
                }
            },
            other => panic!("expected If, got {:?}", other),
        }
    }

    #[test]
    fn def_fun_param_shadows_fname() {
        let def = TypedDef::FunDef(
            "f".to_string(),
            vec![("f".to_string(), Type::Int)],
            Type::Int,
            t_var("f", Type::Int),
        );
        let renamed = run_def(def);
        let (name, params, _, body) = expect_fun_def(&renamed);
        assert!(name.starts_with("f."));
        assert!(params[0].0.starts_with("f."));
        assert_ne!(name, &params[0].0, "param shadowing fname must get a distinct fresh name");
        assert_eq!(expect_var(body), &params[0].0);
    }

    // ---- uniquify_program ----

    #[test]
    fn prog_empty() {
        let prog = TypedProgram { defs: vec![], main: t_int(42) };
        let renamed = uniquify_program(prog);
        assert_eq!(renamed.defs.len(), 0);
        assert_eq!(renamed.main, t_int(42));
    }

    #[test]
    fn prog_val_def_main_uses_it() {
        let prog = TypedProgram {
            defs: vec![TypedDef::ValDef("x".to_string(), Some(Type::Int), t_int(42))],
            main: t_var("x", Type::Int),
        };
        let renamed = uniquify_program(prog);
        assert_eq!(renamed.defs.len(), 1);
        let (name, _, _) = expect_val_def(&renamed.defs[0]);
        assert!(name.starts_with("x."));
        assert_eq!(expect_var(&renamed.main), name);
    }

    #[test]
    fn prog_fun_def_main_calls_it() {
        let prog = TypedProgram {
            defs: vec![TypedDef::FunDef(
                "add".to_string(),
                vec![("a".to_string(), Type::Int), ("b".to_string(), Type::Int)],
                Type::Int,
                t_bin_op(BinOp::Add, t_var("a", Type::Int), t_var("b", Type::Int), Type::Int),
            )],
            main: t_app(t_var("add", Type::Int), vec![t_int(1), t_int(2)], Type::Int),
        };
        let renamed = uniquify_program(prog);
        let (name, _, _, _) = expect_fun_def(&renamed.defs[0]);
        match &renamed.main {
            TypedExpr::App(func, args, _) => {
                assert_eq!(expect_var(func), name);
                assert_eq!(args, &vec![t_int(1), t_int(2)]);
            },
            other => panic!("expected App, got {:?}", other),
        }
    }

    #[test]
    fn prog_recursive_fun() {
        let prog = TypedProgram {
            defs: vec![TypedDef::FunDef(
                "fact".to_string(),
                vec![("n".to_string(), Type::Int)],
                Type::Int,
                t_if(
                    t_bin_op(BinOp::Eq, t_var("n", Type::Int), t_int(0), Type::Bool),
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
            )],
            main: t_app(t_var("fact", Type::Int), vec![t_int(5)], Type::Int),
        };
        let renamed = uniquify_program(prog);
        let (name, _, _, _) = expect_fun_def(&renamed.defs[0]);
        match &renamed.main {
            TypedExpr::App(func, args, _) => {
                assert_eq!(expect_var(func), name);
                assert_eq!(args, &vec![t_int(5)]);
            },
            other => panic!("expected App, got {:?}", other),
        }
    }

    #[test]
    fn prog_chained_defs() {
        let prog = TypedProgram {
            defs: vec![
                TypedDef::ValDef("x".to_string(), Some(Type::Int), t_int(42)),
                TypedDef::ValDef(
                    "y".to_string(),
                    Some(Type::Int),
                    t_bin_op(BinOp::Add, t_var("x", Type::Int), t_int(1), Type::Int),
                ),
            ],
            main: t_var("y", Type::Int),
        };
        let renamed = uniquify_program(prog);
        assert_eq!(renamed.defs.len(), 2);
        let (x_name, _, _) = expect_val_def(&renamed.defs[0]);
        let (y_name, _, body) = expect_val_def(&renamed.defs[1]);
        match body {
            TypedExpr::BinOp(BinOp::Add, l, _, _) => assert_eq!(expect_var(l), x_name),
            other => panic!("expected BinOp, got {:?}", other),
        }
        assert_eq!(expect_var(&renamed.main), y_name);
    }

    #[test]
    fn prog_shadowing_val_defs() {
        let prog = TypedProgram {
            defs: vec![
                TypedDef::ValDef("x".to_string(), Some(Type::Int), t_int(1)),
                TypedDef::ValDef(
                    "x".to_string(),
                    Some(Type::Int),
                    t_bin_op(BinOp::Add, t_var("x", Type::Int), t_int(1), Type::Int),
                ),
            ],
            main: t_var("x", Type::Int),
        };
        let renamed = uniquify_program(prog);
        let (outer_name, _, _) = expect_val_def(&renamed.defs[0]);
        let (inner_name, _, body) = expect_val_def(&renamed.defs[1]);
        match body {
            TypedExpr::BinOp(BinOp::Add, l, _, _) => {
                assert_eq!(expect_var(l), outer_name, "inner x must refer to outer x")
            },
            other => panic!("expected BinOp, got {:?}", other),
        }
        assert_eq!(expect_var(&renamed.main), inner_name);
    }

    #[test]
    fn prog_fun_calls_earlier_fun() {
        let prog = TypedProgram {
            defs: vec![
                TypedDef::FunDef(
                    "add".to_string(),
                    vec![("a".to_string(), Type::Int), ("b".to_string(), Type::Int)],
                    Type::Int,
                    t_bin_op(BinOp::Add, t_var("a", Type::Int), t_var("b", Type::Int), Type::Int),
                ),
                TypedDef::FunDef(
                    "inc".to_string(),
                    vec![("x".to_string(), Type::Int)],
                    Type::Int,
                    t_app(
                        t_var("add", Type::Int),
                        vec![t_var("x", Type::Int), t_int(1)],
                        Type::Int,
                    ),
                ),
            ],
            main: t_app(t_var("inc", Type::Int), vec![t_int(41)], Type::Int),
        };
        let renamed = uniquify_program(prog);
        assert_eq!(renamed.defs.len(), 2);
        let (add_name, _, _, _) = expect_fun_def(&renamed.defs[0]);
        let (inc_name, _, _, body) = expect_fun_def(&renamed.defs[1]);
        match body {
            TypedExpr::App(func, _, _) => {
                assert_eq!(expect_var(func), add_name, "inc body must call the fresh add name")
            },
            other => panic!("expected App, got {:?}", other),
        }
        match &renamed.main {
            TypedExpr::App(func, _, _) => assert_eq!(expect_var(func), inc_name),
            other => panic!("expected App, got {:?}", other),
        }
    }
}
