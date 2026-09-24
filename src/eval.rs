use crate::env::{Env, Value};
use crate::syntax::*;
use lalrpop_util::lalrpop_mod;
use std::io;
use std::{fs::read_to_string, path::PathBuf};
lalrpop_mod!(pub parser);

#[derive(Debug, Clone)]
pub struct EvalError(pub String);

impl std::fmt::Display for EvalError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "EvalError: {}", self.0)
    }
}

macro_rules! err {
    ($($t:tt)*) => { Err(EvalError(format!($($t)*))) };
}

pub type EvalResult = Result<Value, EvalError>;

pub fn eval(env: &Env, expr: &Expr) -> EvalResult {
    match expr {
        Expr::Unit => Ok(Value::Unit),
        Expr::Bool(b) => Ok(Value::Bool(*b)),
        Expr::Int(n) => Ok(Value::Int(*n)),
        Expr::Float(f) => Ok(Value::Float(*f)),

        Expr::Var(name) => env
            .get(name)
            .cloned()
            .ok_or_else(|| EvalError(format!("unbound variable: {}", name))),

        Expr::Ann(e, _) => eval(env, e),

        Expr::Tuple(exprs) => {
            let vals = exprs
                .into_iter()
                .map(|expr| eval(env, expr))
                .collect::<Result<Vec<Value>, _>>()?;

            Ok(Value::Tuple(vals))
        },

        Expr::TupleProj(expr, index) => {
            let val = eval(env, expr)?;
            match val {
                Value::Tuple(vals) => vals
                    .get(*index)
                    .cloned()
                    .ok_or_else(|| EvalError(format!("tuple index {} out of bounds", index))),
                other => err!("tuple projection on non-tuple: {}", other),
            }
        },

        Expr::PrimIO(prim_io, None) => {
            match prim_io {
                PrimIO::ReadInt => {
                    // 怎么确定语义和编译器的runtime相符？
                    let mut input = String::new();
                    io::stdin().read_line(&mut input).expect("failed to read line");
                    let n: i64 = input.trim().parse().expect("input was not an integer");
                    Ok(Value::Int(n))
                },
                PrimIO::ReadFloat => {
                    // 怎么确定语义和编译器的runtime相符？
                    let mut input = String::new();
                    io::stdin().read_line(&mut input).expect("failed to read line");
                    let f: f64 = input.trim().parse().expect("input was not an integer");
                    Ok(Value::Float(f))
                },
                _ => unreachable!(),
            }
        },
        Expr::PrimIO(prim_io, Some(expr)) => {
            match prim_io {
                PrimIO::PrintInt => {
                    let val = eval(env, expr)?;
                    // 怎么确定语义和编译器的runtime相符？
                    println!("{}", val);
                    Ok(Value::Unit)
                },
                PrimIO::PrintFloat => {
                    let val = eval(env, expr)?;
                    // 怎么确定语义和编译器的runtime相符？
                    println!("{}", val);
                    Ok(Value::Unit)
                },
                PrimIO::PrintBool => {
                    let val = eval(env, expr)?;
                    // 怎么确定语义和编译器的runtime相符？
                    println!("{}", val);
                    Ok(Value::Unit)
                },
                _ => unreachable!(),
            }
        },

        Expr::UnaryOp(op, e) => {
            let v = eval(env, e)?;
            eval_unary(op, v)
        },

        Expr::BinOp(op, e1, e2) => {
            let v1 = eval(env, e1)?;
            let v2 = eval(env, e2)?;
            eval_binop(op, v1, v2)
        },

        Expr::If(cond, thn, els) => match eval(env, cond)? {
            Value::Bool(true) => eval(env, thn),
            Value::Bool(false) => eval(env, els),
            other => err!("condition must be Bool, got {}", other),
        },

        Expr::Let(name, _ty, e1, e2) => {
            let v = eval(env, e1)?;
            let env2 = env.extend(name, v);
            eval(&env2, e2)
        },

        Expr::LetRec(fname, fparams, _, fbody, body) => {
            let params: Vec<Ident> = fparams.iter().map(|(id, _)| id.clone()).collect();
            let rec_val = Value::RecClosure {
                env: env.clone(),
                fname: fname.clone(),
                params: params.clone(),
                body: (**fbody).clone(),
            };
            let env2 = env.extend(&fname, rec_val);
            eval(&env2, body)
        },

        Expr::Lambda(params, _, body) => {
            let param_names: Vec<Ident> = params.iter().map(|(id, _)| id.clone()).collect();
            Ok(Value::Closure(env.clone(), param_names, (**body).clone()))
        },

        Expr::App(func, args) => {
            let fval = eval(env, func)?;
            let mut argvs = vec![];
            for arg in args {
                let argv = eval(env, arg)?;
                argvs.push(argv);
            }
            apply(fval, argvs)
        },
    }
}

fn apply(func: Value, argvs: Vec<Value>) -> EvalResult {
    match func {
        Value::Closure(mut env, params, body) => {
            if argvs.len() > params.len() {
                return err!("too many arguments: expected {}, got {}", params.len(), argvs.len());
            }

            for (name, arg) in params.iter().zip(argvs.iter()) {
                env.insert(name.clone(), arg.clone());
            }

            if argvs.len() < params.len() {
                Ok(Value::Closure(env, params[argvs.len()..].to_vec(), body))
            } else {
                eval(&env, &body)
            }
        },

        Value::RecClosure { mut env, fname, params, body } => {
            if argvs.len() > params.len() {
                return err!("too many arguments: expected {}, got {}", params.len(), argvs.len());
            }
            env.insert(
                fname.clone(),
                Value::RecClosure {
                    env: env.clone(),
                    fname: fname.clone(),
                    params: params.clone(),
                    body: body.clone(),
                },
            );

            for (name, arg) in params.iter().zip(argvs.iter()) {
                env.insert(name.clone(), arg.clone());
            }

            if argvs.len() < params.len() {
                Ok(Value::RecClosure { env, fname, params: params[argvs.len()..].to_vec(), body })
            } else {
                eval(&env, &body)
            }
        },
        other => err!("tried to apply a non-function: {}", other),
    }
}

fn eval_unary(op: &UnaryOp, v: Value) -> EvalResult {
    match (op, v) {
        (UnaryOp::Neg, Value::Int(n)) => Ok(Value::Int(-n)),
        (UnaryOp::Neg, Value::Float(f)) => Ok(Value::Float(-f)),
        (UnaryOp::Not, Value::Bool(b)) => Ok(Value::Bool(!b)),
        (op, v) => err!("type error in unary {:?}: got {}", op, v),
    }
}

fn eval_binop(op: &BinOp, v1: Value, v2: Value) -> EvalResult {
    match (op, v1, v2) {
        (BinOp::Add, Value::Int(a), Value::Int(b)) => Ok(Value::Int(a + b)),
        (BinOp::Sub, Value::Int(a), Value::Int(b)) => Ok(Value::Int(a - b)),
        (BinOp::Mul, Value::Int(a), Value::Int(b)) => Ok(Value::Int(a * b)),
        (BinOp::Div, Value::Int(a), Value::Int(b)) => {
            if b == 0 {
                err!("division by zero")
            } else {
                Ok(Value::Int(a / b))
            }
        },

        (BinOp::Add, Value::Float(a), Value::Float(b)) => Ok(Value::Float(a + b)),
        (BinOp::Sub, Value::Float(a), Value::Float(b)) => Ok(Value::Float(a - b)),
        (BinOp::Mul, Value::Float(a), Value::Float(b)) => Ok(Value::Float(a * b)),
        (BinOp::Div, Value::Float(a), Value::Float(b)) => {
            if b == 0.0 {
                err!("division by zero (float)")
            } else {
                Ok(Value::Float(a / b))
            }
        },

        (BinOp::Eq, Value::Int(a), Value::Int(b)) => Ok(Value::Bool(a == b)),
        (BinOp::Neq, Value::Int(a), Value::Int(b)) => Ok(Value::Bool(a != b)),
        (BinOp::Lt, Value::Int(a), Value::Int(b)) => Ok(Value::Bool(a < b)),
        (BinOp::Leq, Value::Int(a), Value::Int(b)) => Ok(Value::Bool(a <= b)),
        (BinOp::Gt, Value::Int(a), Value::Int(b)) => Ok(Value::Bool(a > b)),
        (BinOp::Geq, Value::Int(a), Value::Int(b)) => Ok(Value::Bool(a >= b)),

        (BinOp::Eq, Value::Float(a), Value::Float(b)) => Ok(Value::Bool(a == b)),
        (BinOp::Neq, Value::Float(a), Value::Float(b)) => Ok(Value::Bool(a != b)),
        (BinOp::Lt, Value::Float(a), Value::Float(b)) => Ok(Value::Bool(a < b)),
        (BinOp::Leq, Value::Float(a), Value::Float(b)) => Ok(Value::Bool(a <= b)),
        (BinOp::Gt, Value::Float(a), Value::Float(b)) => Ok(Value::Bool(a > b)),
        (BinOp::Geq, Value::Float(a), Value::Float(b)) => Ok(Value::Bool(a >= b)),

        (BinOp::And, Value::Bool(a), Value::Bool(b)) => Ok(Value::Bool(a && b)),
        (BinOp::Or, Value::Bool(a), Value::Bool(b)) => Ok(Value::Bool(a || b)),
        (BinOp::Eq, Value::Bool(a), Value::Bool(b)) => Ok(Value::Bool(a == b)),
        (BinOp::Neq, Value::Bool(a), Value::Bool(b)) => Ok(Value::Bool(a != b)),

        (BinOp::Eq, Value::Unit, Value::Unit) => Ok(Value::Bool(true)),
        (BinOp::Neq, Value::Unit, Value::Unit) => Ok(Value::Bool(false)),

        (op, v1, v2) => {
            err!("type error in {:?}: got {} and {}", op, v1, v2)
        },
    }
}

pub fn eval_top(expr: &Expr) -> EvalResult {
    eval(&Env::new(), expr)
}

pub fn eval_file(file: &PathBuf) -> EvalResult {
    if let Ok(content) = read_to_string(file.clone()) {
        let expr = parser::ExprParser::new().parse(&content).unwrap();
        eval_top(&*expr)
    } else {
        err!("read file error: {}", file.to_str().unwrap())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::syntax::{
        a_let, ann, app, bin_op, bool, float, if_else, int, lambda, let_rec, print_bool,
        print_float, print_int, tuple, tuple_projection, ty_arrow, ty_bool, ty_int, unary, unit,
        var, BinOp, UnaryOp,
    };

    fn run(expr: Expr) -> Value {
        eval_top(&expr).unwrap()
    }

    fn run_err(expr: Expr) -> String {
        eval_top(&expr).unwrap_err().0
    }

    #[test]
    fn test_literals() {
        assert_eq!(run(unit()), Value::Unit);
        assert_eq!(run(bool(true)), Value::Bool(true));
        assert_eq!(run(bool(false)), Value::Bool(false));
        assert_eq!(run(int(42)), Value::Int(42));
        assert_eq!(run(float(3.14)), Value::Float(3.14));
    }

    #[test]
    fn test_int_arith() {
        assert_eq!(run(bin_op(BinOp::Add, int(1), int(2))), Value::Int(3));
        assert_eq!(run(bin_op(BinOp::Sub, int(10), int(3))), Value::Int(7));
        assert_eq!(run(bin_op(BinOp::Mul, int(6), int(7))), Value::Int(42));
        assert_eq!(run(bin_op(BinOp::Div, int(10), int(3))), Value::Int(3));
        assert_eq!(run(bin_op(BinOp::Mul, bin_op(BinOp::Add, int(1), int(2)), int(3))), Value::Int(9));
        assert_eq!(run(bin_op(BinOp::Sub, bin_op(BinOp::Sub, int(10), int(3)), int(2))), Value::Int(5));
    }

    #[test]
    fn test_float_arith() {
        assert_eq!(run(bin_op(BinOp::Add, float(1.0), float(2.0))), Value::Float(3.0));
        assert_eq!(run(bin_op(BinOp::Div, float(6.0), float(2.0))), Value::Float(3.0));
    }

    #[test]
    fn test_div_by_zero() {
        assert!(run_err(bin_op(BinOp::Div, int(1), int(0))).contains("division by zero"));
    }

    #[test]
    fn test_unary() {
        assert_eq!(run(unary(UnaryOp::Neg, int(5))), Value::Int(-5));
        assert_eq!(run(unary(UnaryOp::Neg, float(3.0))), Value::Float(-3.0));
        assert_eq!(run(unary(UnaryOp::Not, bool(true))), Value::Bool(false));
        assert_eq!(run(unary(UnaryOp::Not, unary(UnaryOp::Not, bool(false)))), Value::Bool(false));
    }

    #[test]
    fn test_cmp() {
        assert_eq!(run(bin_op(BinOp::Lt, int(1), int(2))), Value::Bool(true));
        assert_eq!(run(bin_op(BinOp::Gt, int(2), int(3))), Value::Bool(false));
        assert_eq!(run(bin_op(BinOp::Eq, int(3), int(3))), Value::Bool(true));
        assert_eq!(run(bin_op(BinOp::Neq, int(3), int(4))), Value::Bool(true));
        assert_eq!(run(bin_op(BinOp::Leq, int(2), int(2))), Value::Bool(true));
        assert_eq!(run(bin_op(BinOp::Geq, int(2), int(3))), Value::Bool(false));
    }

    #[test]
    fn test_logic() {
        assert_eq!(run(bin_op(BinOp::And, bool(true), bool(false))), Value::Bool(false));
        assert_eq!(run(bin_op(BinOp::Or, bool(true), bool(false))), Value::Bool(true));
    }

    #[test]
    fn test_if() {
        assert_eq!(run(if_else(bool(true), int(1), int(2))), Value::Int(1));
        assert_eq!(run(if_else(bool(false), int(1), int(2))), Value::Int(2));
        assert_eq!(run(if_else(bin_op(BinOp::Lt, int(1), int(2)), int(10), int(20))), Value::Int(10));
        assert_eq!(run(if_else(bool(true), if_else(bool(false), int(1), int(2)), int(3))), Value::Int(2));
    }

    #[test]
    fn test_let() {
        assert_eq!(run(a_let("x", None, int(1), var("x"))), Value::Int(1));
        assert_eq!(run(a_let("x", Some(ty_int()), int(5), bin_op(BinOp::Add, var("x"), int(1)))), Value::Int(6));
        assert_eq!(
            run(a_let(
                "x",
                None,
                int(1),
                a_let("y", None, int(2), bin_op(BinOp::Add, var("x"), var("y")))
            )),
            Value::Int(3)
        );
    }

    #[test]
    fn test_lambda_apply() {
        assert_eq!(
            run(app(lambda(vec![("x".to_string(), ty_int())], None, var("x")), vec![int(42)])),
            Value::Int(42)
        );
        assert_eq!(
            run(app(
                lambda(
                    vec![("x".to_string(), ty_int()), ("y".to_string(), ty_int())],
                    None,
                    bin_op(BinOp::Add, var("x"), var("y"))
                ),
                vec![int(3), int(4)]
            )),
            Value::Int(7)
        );
    }

    #[test]
    fn test_higher_order() {
        let expr = a_let(
            "apply",
            None,
            lambda(
                vec![("f".to_string(), ty_arrow(ty_int(), ty_int())), ("x".to_string(), ty_int())],
                None,
                app(var("f"), vec![var("x")])
            ),
            a_let(
                "double",
                None,
                lambda(vec![("x".to_string(), ty_int())], None, bin_op(BinOp::Mul, var("x"), int(2))),
                app(var("apply"), vec![var("double"), int(21)])
            )
        );
        assert_eq!(run(expr), Value::Int(42));
    }

    #[test]
    fn test_higher_order_closure() {
        let expr = a_let(
            "apply",
            None,
            lambda(
                vec![("f".to_string(), ty_arrow(ty_int(), ty_int())), ("x".to_string(), ty_int())],
                None,
                app(var("f"), vec![var("x")])
            ),
            a_let(
                "double",
                None,
                lambda(vec![("x".to_string(), ty_int())], None, bin_op(BinOp::Mul, var("x"), int(2))),
                app(var("apply"), vec![var("double")])
            )
        );
        let apply_closure = run(expr);
        assert!(matches!(apply_closure, Value::Closure(_, _, _)));
    }

    #[test]
    fn test_factorial() {
        let expr = let_rec(
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
            app(var("fact"), vec![int(10)])
        );
        assert_eq!(run(expr), Value::Int(3628800));
    }

    #[test]
    fn test_fib() {
        let expr = let_rec(
            "fib",
            vec![("n".to_string(), ty_int())],
            ty_int(),
            if_else(
                bin_op(BinOp::Leq, var("n"), int(1)),
                var("n"),
                bin_op(
                    BinOp::Add,
                    app(var("fib"), vec![bin_op(BinOp::Sub, var("n"), int(1))]),
                    app(var("fib"), vec![bin_op(BinOp::Sub, var("n"), int(2))])
                )
            ),
            app(var("fib"), vec![int(10)])
        );
        assert_eq!(run(expr), Value::Int(55));
    }

    #[test]
    fn test_letrec_multi_arg() {
        let expr = let_rec(
            "add",
            vec![("x".to_string(), ty_int()), ("y".to_string(), ty_int())],
            ty_int(),
            bin_op(BinOp::Add, var("x"), var("y")),
            app(var("add"), vec![int(19), int(23)])
        );
        assert_eq!(run(expr), Value::Int(42));
    }

    #[test]
    fn test_ann() {
        assert_eq!(run(ann(int(42), ty_int())), Value::Int(42));
        assert_eq!(run(ann(bool(true), ty_bool())), Value::Bool(true));
    }

    #[test]
    fn test_tuple() {
        assert_eq!(
            run(tuple(vec![int(1), bool(true), float(3.14)])),
            Value::Tuple(vec![Value::Int(1), Value::Bool(true), Value::Float(3.14)])
        );
    }

    #[test]
    fn test_nested_tuple() {
        assert_eq!(
            run(tuple(vec![int(1), tuple(vec![bool(true), float(2.0)])])),
            Value::Tuple(vec![
                Value::Int(1),
                Value::Tuple(vec![Value::Bool(true), Value::Float(2.0)])
            ])
        );
    }

    #[test]
    fn test_tuple_projection() {
        assert_eq!(
            run(tuple_projection(tuple(vec![int(1), bool(true), float(3.14)]), 0)),
            Value::Int(1)
        );
        assert_eq!(
            run(tuple_projection(tuple(vec![int(1), bool(true), float(3.14)]), 2)),
            Value::Float(3.14)
        );
    }

    #[test]
    fn test_tuple_projection_nested() {
        assert_eq!(
            run(tuple_projection(
                tuple_projection(tuple(vec![int(1), tuple(vec![bool(true)])]), 1),
                0
            )),
            Value::Bool(true)
        );
    }

    #[test]
    fn test_tuple_projection_out_of_bounds() {
        assert!(eval_top(&tuple_projection(tuple(vec![int(1)]), 3)).is_err());
    }

    #[test]
    fn test_tuple_projection_non_tuple() {
        assert!(eval_top(&tuple_projection(int(1), 0)).is_err());
    }

    #[test]
    fn test_print_int() {
        assert_eq!(run(print_int(int(42))), Value::Unit);
    }

    #[test]
    fn test_print_bool() {
        assert_eq!(run(print_bool(bool(true))), Value::Unit);
    }

    #[test]
    fn test_print_float() {
        assert_eq!(run(print_float(float(3.14))), Value::Unit);
    }

    #[test]
    fn test_unbound_var() {
        assert!(eval_top(&var("foo")).is_err());
    }
}
