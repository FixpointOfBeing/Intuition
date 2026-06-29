use std::collections::HashMap;
use crate::syntax::{Expr, Ident};

pub struct Uniquify {
    next_id: u64,
    env: HashMap<Ident, Ident>,
}

impl Uniquify {
    pub fn new() -> Uniquify {
        Uniquify {
            next_id: 0,
            env: HashMap::new(),
        }
    }

    fn refresh(&mut self, name: &Ident) -> Ident {
        let id = self.next_id;
        self.next_id += 1;
        format!("{name}.{id}")
    }

    fn bind(&mut self, name: &Ident) -> (Ident, Option<Ident>) {
        let new_name = self.refresh(name);
        let old = self.env.insert(name.clone(), new_name.clone());
        (new_name, old)
    }

    fn unbind(&mut self, name: &Ident, old: Option<Ident>) {
        match old {
            Some(prev) => {
                self.env.insert(name.clone(), prev);
            }
            None => {
                self.env.remove(name);
            }
        }
    }

    pub fn rename(&mut self, expr: &Expr) -> Expr {
        match expr {
            Expr::Unit => Expr::Unit,
            Expr::Bool(b) => Expr::Bool(*b),
            Expr::Int(i) => Expr::Int(*i),
            Expr::Float(f) => Expr::Float(*f),
            Expr::BinOp(op, left, right) => {
                let left = self.rename(left);
                let right = self.rename(right);
                Expr::BinOp(op.clone(), Box::new(left), Box::new(right))
            }
            Expr::UnaryOp(op, expr) => {
                let expr = self.rename(expr);
                Expr::UnaryOp(op.clone(), Box::new(expr))
            }
            Expr::Ann(expr, ty) => {
                let expr = self.rename(expr);
                Expr::Ann(Box::new(expr), ty.clone())
            }
            Expr::If(cond, then_e, else_e) => {
                let cond = self.rename(cond);
                let then_e = self.rename(then_e);
                let else_e = self.rename(else_e);
                Expr::If(Box::new(cond), Box::new(then_e), Box::new(else_e))
            }
            Expr::Let(name, ty, rhs, body) => {
                let rhs = self.rename(rhs);
                let (new_name, old) = self.bind(name);
                let body = self.rename(body);
                self.unbind(name, old);
                Expr::Let(new_name, ty.clone(), Box::new(rhs), Box::new(body))
            }
            Expr::Var(name) => {
                let new_name = self.env.get(name).expect("unbound variable");
                Expr::Var(new_name.clone())
            }
            Expr::LetRec(fname, fargs, fty, fbody, body) => {
                let (new_fname, old_fname) = self.bind(fname);

                let mut new_fargs = vec![];
                let mut old_fargs = vec![];
                for (name, ty) in fargs {
                    let (new_name, old) = self.bind(name);
                    old_fargs.push((name, old));
                    new_fargs.push((new_name, ty.clone()));
                }

                let fbody = self.rename(fbody);

                for (name, old) in old_fargs.into_iter().rev() {
                    self.unbind(name, old);
                }

                let body = self.rename(body);
                self.unbind(fname, old_fname);

                Expr::LetRec(new_fname, new_fargs, fty.clone(), Box::new(fbody), Box::new(body))
            }
            Expr::App(func, args) => {
                let func = self.rename(func);
                let args = args.iter().map(|e| self.rename(e)).collect();
                Expr::App(Box::new(func), args)
            }
            Expr::Lambda(args, ty, body) => {
                let mut new_args = vec![];
                let mut old_args = vec![];
                for (name, arg_ty) in args {
                    let (new_name, old) = self.bind(name);
                    old_args.push((name, old));
                    new_args.push((new_name, arg_ty.clone()));
                }

                let body = self.rename(body);

                for (name, old) in old_args.into_iter().rev() {
                    self.unbind(name, old);
                }

                Expr::Lambda(new_args, ty.clone(), Box::new(body))
            }
        }
    }
}

pub fn rename_top(expr: &Expr) -> Expr {
    let mut renamer = Uniquify::new();
    renamer.rename(expr)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::syntax::{BinOp, Type, UnaryOp};

    fn v(name: &str) -> Box<Expr> {
        Box::new(Expr::Var(name.to_string()))
    }

    fn int(n: i64) -> Box<Expr> {
        Box::new(Expr::Int(n))
    }

    fn expect_let(expr: &Expr) -> (&Ident, &Expr, &Expr) {
        match expr {
            Expr::Let(name, _, rhs, body) => (name, rhs, body),
            other => panic!("expected Expr::Let, got {other:?}"),
        }
    }

    fn expect_var(expr: &Expr) -> &Ident {
        match expr {
            Expr::Var(name) => name,
            other => panic!("expected Expr::Var, got {other:?}"),
        }
    }

    #[test]
    fn unit_is_unchanged() {
        assert_eq!(rename_top(&Expr::Unit), Expr::Unit);
    }

    #[test]
    fn bool_is_unchanged() {
        assert_eq!(rename_top(&Expr::Bool(true)), Expr::Bool(true));
        assert_eq!(rename_top(&Expr::Bool(false)), Expr::Bool(false));
    }

    #[test]
    fn int_is_unchanged() {
        assert_eq!(rename_top(&Expr::Int(42)), Expr::Int(42));
    }

    #[test]
    fn float_is_unchanged() {
        assert_eq!(rename_top(&Expr::Float(3.14)), Expr::Float(3.14));
    }

    #[test]
    fn let_renames_var_in_body() {
        let expr = Expr::Let("x".to_string(), None, int(1), v("x"));
        let renamed = rename_top(&expr);
        let (bound_name, _, body) = expect_let(&renamed);
        let used_name = expect_var(body);
        assert_eq!(bound_name, used_name);
        assert!(bound_name.starts_with("x."));
    }

    #[test]
    fn fresh_names_are_distinct_across_lets() {
        let expr = Expr::Let(
            "x".to_string(),
            None,
            int(1),
            Box::new(Expr::Let("x".to_string(), None, int(2), v("x"))),
        );
        let renamed = rename_top(&expr);
        let (outer_name, _, outer_body) = expect_let(&renamed);
        let (inner_name, _, inner_body) = expect_let(outer_body);
        let used_name = expect_var(inner_body);
        assert_ne!(outer_name, inner_name, "shadowing let must get a new fresh name");
        assert_eq!(inner_name, used_name, "body must refer to the innermost x");
    }

    #[test]
    fn outer_let_survives_lambda_param_shadowing() {
        let expr = Expr::Let(
            "x".to_string(),
            None,
            int(1),
            Box::new(Expr::Let(
                "f".to_string(),
                None,
                Box::new(Expr::Lambda(
                    vec![("x".to_string(), Type::Int)],
                    None,
                    v("x"),
                )),
                v("x"),
            )),
        );

        let renamed = rename_top(&expr);
        let (outer_name, _, outer_body) = expect_let(&renamed);
        let (_, lambda_rhs, final_body) = expect_let(outer_body);

        let lambda_param_name = match lambda_rhs {
            Expr::Lambda(params, _, lambda_body) => {
                let param_name = &params[0].0;
                let used_in_lambda = expect_var(lambda_body);
                assert_eq!(param_name, used_in_lambda, "lambda body must refer to its own param");
                param_name.clone()
            }
            other => panic!("expected Lambda, got {other:?}"),
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
    fn letrec_args_dont_leak_into_continuation() {
        let expr = Expr::Let(
            "x".to_string(),
            None,
            int(0),
            Box::new(Expr::LetRec(
                "f".to_string(),
                vec![("x".to_string(), Type::Int)],
                Type::Int,
                v("x"),
                v("x"),
            )),
        );

        let renamed = rename_top(&expr);
        let (outer_x, _, letrec) = expect_let(&renamed);

        match letrec {
            Expr::LetRec(_, fargs, _, fbody, cont) => {
                let arg_name = &fargs[0].0;
                let body_var = expect_var(fbody);
                assert_eq!(arg_name, body_var, "function body refers to its own param");

                let cont_var = expect_var(cont);
                assert_eq!(
                    cont_var, outer_x,
                    "continuation after LetRec must see the outer x, not f's argument"
                );
            }
            other => panic!("expected LetRec, got {other:?}"),
        }
    }

    #[test]
    fn letrec_continuation_can_still_call_fname() {
        let expr = Expr::LetRec(
            "fact".to_string(),
            vec![("n".to_string(), Type::Int)],
            Type::Int,
            v("n"),
            v("fact"),
        );
        let renamed = rename_top(&expr);
        match renamed {
            Expr::LetRec(new_fname, _, _, _, cont) => {
                let cont_name = expect_var(&cont);
                assert_eq!(&new_fname, cont_name, "continuation must resolve fact to its fresh name");
            }
            other => panic!("expected LetRec, got {other:?}"),
        }
    }

    #[test]
    fn lambda_multi_param_each_gets_fresh_name() {
        let expr = Expr::Lambda(
            vec![("x".to_string(), Type::Int), ("y".to_string(), Type::Int)],
            None,
            Box::new(Expr::BinOp(BinOp::Add, v("x"), v("y"))),
        );
        let renamed = rename_top(&expr);
        match renamed {
            Expr::Lambda(params, _, body) => {
                assert_ne!(params[0].0, params[1].0);
                match *body {
                    Expr::BinOp(BinOp::Add, l, r) => {
                        assert_eq!(expect_var(&l), &params[0].0);
                        assert_eq!(expect_var(&r), &params[1].0);
                    }
                    other => panic!("expected BinOp, got {other:?}"),
                }
            }
            other => panic!("expected Lambda, got {other:?}"),
        }
    }

    #[test]
    fn binop_renames_both_sides() {
        let expr = Expr::Let(
            "x".to_string(),
            None,
            int(1),
            Box::new(Expr::Let(
                "y".to_string(),
                None,
                int(2),
                Box::new(Expr::BinOp(BinOp::Add, v("x"), v("y"))),
            )),
        );
        let renamed = rename_top(&expr);
        let (x_name, _, body1) = expect_let(&renamed);
        let (y_name, _, body2) = expect_let(body1);
        match body2 {
            Expr::BinOp(BinOp::Add, l, r) => {
                assert_eq!(expect_var(l), x_name);
                assert_eq!(expect_var(r), y_name);
            }
            other => panic!("expected BinOp, got {other:?}"),
        }
    }

    #[test]
    fn unaryop_renames_inner() {
        let expr = Expr::Let(
            "x".to_string(),
            None,
            int(1),
            Box::new(Expr::UnaryOp(UnaryOp::Neg, v("x"))),
        );
        let renamed = rename_top(&expr);
        let (x_name, _, body) = expect_let(&renamed);
        match body {
            Expr::UnaryOp(UnaryOp::Neg, inner) => assert_eq!(expect_var(inner), x_name),
            other => panic!("expected UnaryOp, got {other:?}"),
        }
    }

    #[test]
    fn if_renames_all_three_branches() {
        let expr = Expr::Let(
            "x".to_string(),
            None,
            int(1),
            Box::new(Expr::If(v("x"), v("x"), v("x"))),
        );
        let renamed = rename_top(&expr);
        let (x_name, _, body) = expect_let(&renamed);
        match body {
            Expr::If(c, t, e) => {
                assert_eq!(expect_var(c), x_name);
                assert_eq!(expect_var(t), x_name);
                assert_eq!(expect_var(e), x_name);
            }
            other => panic!("expected If, got {other:?}"),
        }
    }

    #[test]
    fn app_renames_func_and_all_args() {
        let expr = Expr::Let(
            "f".to_string(),
            None,
            int(1),
            Box::new(Expr::Let(
                "x".to_string(),
                None,
                int(2),
                Box::new(Expr::App(v("f"), vec![Expr::Var("x".to_string()), Expr::Var("x".to_string())])),
            )),
        );
        let renamed = rename_top(&expr);
        let (f_name, _, body1) = expect_let(&renamed);
        let (x_name, _, body2) = expect_let(body1);
        match body2 {
            Expr::App(func, args) => {
                assert_eq!(expect_var(func), f_name);
                assert_eq!(args.len(), 2);
                assert_eq!(expect_var(&args[0]), x_name);
                assert_eq!(expect_var(&args[1]), x_name);
            }
            other => panic!("expected App, got {other:?}"),
        }
    }

    #[test]
    fn ann_inner_expr_is_actually_renamed() {
        let expr = Expr::Let(
            "x".to_string(),
            None,
            int(1),
            Box::new(Expr::Ann(v("x"), Type::Int)),
        );
        let renamed = rename_top(&expr);
        let (x_name, _, body) = expect_let(&renamed);
        match body {
            Expr::Ann(inner, ty) => {
                assert_eq!(expect_var(inner), x_name, "Ann must rename its inner expr, not clone it verbatim");
                assert_eq!(*ty, Type::Int);
            }
            other => panic!("expected Ann, got {other:?}"),
        }
    }

    #[test]
    fn ann_nested_inside_lambda_still_renamed() {
        let expr = Expr::Lambda(
            vec![("x".to_string(), Type::Int)],
            None,
            Box::new(Expr::Ann(v("x"), Type::Int)),
        );
        let renamed = rename_top(&expr);
        match renamed {
            Expr::Lambda(params, _, body) => match *body {
                Expr::Ann(inner, _) => assert_eq!(expect_var(&inner), &params[0].0),
                other => panic!("expected Ann, got {other:?}"),
            },
            other => panic!("expected Lambda, got {other:?}"),
        }
    }

    #[test]
    fn letrec_factorial_shape_renames_consistently() {
        let expr = Expr::LetRec(
            "fact".to_string(),
            vec![("n".to_string(), Type::Int)],
            Type::Int,
            Box::new(Expr::If(
                Box::new(Expr::BinOp(BinOp::Eq, v("n"), int(0))),
                int(1),
                Box::new(Expr::BinOp(
                    BinOp::Mul,
                    v("n"),
                    Box::new(Expr::App(
                        v("fact"),
                        vec![Expr::BinOp(BinOp::Sub, Box::new(Expr::Var("n".to_string())), int(1))],
                    )),
                )),
            )),
            Box::new(Expr::App(v("fact"), vec![Expr::Int(5)])),
        );

        let renamed = rename_top(&expr);
        match renamed {
            Expr::LetRec(new_fname, fargs, _, fbody, cont) => {
                let n_name = &fargs[0].0;

                match *fbody {
                    Expr::If(cond, _, else_branch) => {
                        match *cond {
                            Expr::BinOp(BinOp::Eq, l, _) => assert_eq!(expect_var(&l), n_name),
                            other => panic!("expected Eq, got {other:?}"),
                        }
                        match *else_branch {
                            Expr::BinOp(BinOp::Mul, l, r) => {
                                assert_eq!(expect_var(&l), n_name);
                                match *r {
                                    Expr::App(func, args) => {
                                        assert_eq!(expect_var(&func), &new_fname, "recursive call must use fact's fresh name");
                                        match &args[0] {
                                            Expr::BinOp(BinOp::Sub, l, _) => assert_eq!(expect_var(l), n_name),
                                            other => panic!("expected Sub, got {other:?}"),
                                        }
                                    }
                                    other => panic!("expected App, got {other:?}"),
                                }
                            }
                            other => panic!("expected Mul, got {other:?}"),
                        }
                    }
                    other => panic!("expected If, got {other:?}"),
                }

                match *cont {
                    Expr::App(func, args) => {
                        assert_eq!(expect_var(&func), &new_fname, "top-level call site must use fact's fresh name");
                        assert_eq!(args, vec![Expr::Int(5)]);
                    }
                    other => panic!("expected App, got {other:?}"),
                }
            }
            other => panic!("expected LetRec, got {other:?}"),
        }
    }
}