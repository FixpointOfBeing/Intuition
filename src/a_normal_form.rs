use crate::syntax::{BinOp, Expr, Ident, Type, UnaryOp};
use crate::gensym::Gensym;

#[derive(Debug, Clone, PartialEq)]
pub enum AExpr {
    Unit,
    Bool(bool),
    Int(i64),
    Float(f64),
    Var(Ident),
}

#[derive(Debug, Clone, PartialEq)]
pub enum CompExpr {
    Atom(AExpr),
    BinOp(BinOp, AExpr, AExpr),
    UnaryOp(UnaryOp, AExpr),
    App(AExpr, Vec<AExpr>),
    If(AExpr, Box<AnfExpr>, Box<AnfExpr>),
    Lambda(Vec<(Ident, Type)>, Option<Type>, Box<AnfExpr>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum AnfExpr {
    Complex(CompExpr),
    Let(Ident, CompExpr, Box<AnfExpr>),
    LetRec(Ident, Vec<(Ident, Type)>, Type, Box<AnfExpr>, Box<AnfExpr>),
}

enum Binding {
    Let(Ident, CompExpr),
    LetRec(Ident, Vec<(Ident, Type)>, Type, AnfExpr),
}
type Bindings = Vec<Binding>;


fn to_atom(expr: &Expr, gs: &mut Gensym, bindings: &mut Bindings) -> AExpr {
    match expr {
        Expr::Unit => AExpr::Unit,
        Expr::Bool(b) => AExpr::Bool(*b),
        Expr::Int(i) => AExpr::Int(*i),
        Expr::Float(f) => AExpr::Float(*f),
        Expr::Var(name) => AExpr::Var(name.clone()),
        _ => {
            let c = collect_bindings(expr, gs, bindings);
            if let CompExpr::Atom(a) = c {
                return a;
            }
            let name = gs.fresh();
            bindings.push(Binding::Let(name.clone(), c));
            AExpr::Var(name)
        }
    }
}
fn collect_bindings(expr: &Expr, gs: &mut Gensym, bindings: &mut Bindings) -> CompExpr {
    match expr {
        Expr::Unit => CompExpr::Atom(AExpr::Unit),
        Expr::Bool(b) => CompExpr::Atom(AExpr::Bool(*b)),
        Expr::Int(i) => CompExpr::Atom(AExpr::Int(*i)),
        Expr::Float(f) => CompExpr::Atom(AExpr::Float(*f)),
        Expr::Var(name) => CompExpr::Atom(AExpr::Var(name.to_string())),
        Expr::BinOp(op, left, right) => {
            let left_atom = to_atom(left, gs, bindings);
            let right_atom = to_atom(right, gs, bindings);
            CompExpr::BinOp((*op).clone(), left_atom, right_atom)
        }
        Expr::UnaryOp(op, operand) => {
            let operand_atom = to_atom(operand, gs, bindings);
            CompExpr::UnaryOp((*op).clone(), operand_atom)
        }
        Expr::Ann(inner, _) => collect_bindings(inner, gs, bindings),
        Expr::If(cond, thn, els) => {
            let cond_atom = to_atom(cond, gs, bindings);
            let then_anf = normalize(thn, gs);
            let else_anf = normalize(els, gs);
            CompExpr::If(cond_atom, Box::new(then_anf), Box::new(else_anf))
        }
        Expr::App(func, args) => {
            let func_atom = to_atom(func, gs, bindings);
            let mut args_atom = Vec::with_capacity(args.len());
            for arg in args {
                let arg_atom = to_atom(arg, gs, bindings);
                args_atom.push(arg_atom);
            }
            CompExpr::App(func_atom, args_atom)
        }
        Expr::Lambda(args, ty, body) => {
            let body_anf = normalize(body, gs);
            CompExpr::Lambda((*args).clone(), (*ty).clone(), Box::new(body_anf))
        }
        Expr::Let(name, _, rhs, body) => {
            let rhs_comp = collect_bindings(rhs, gs, bindings);
            bindings.push(Binding::Let(name.clone(), rhs_comp));
            collect_bindings(body, gs, bindings)
        }
        Expr::LetRec(fname, fargs, fty, fbody, body) => {
            let fbody_anf = normalize(fbody, gs);
            bindings.push(Binding::LetRec(
                fname.clone(),
                fargs.clone(),
                fty.clone(),
                fbody_anf,
            ));
            collect_bindings(body, gs, bindings)
        }
    }
}

fn bindings_to_lets(bindings: Bindings, tail: AnfExpr) -> AnfExpr {
    bindings.into_iter().rev().fold(tail, |acc, b| match b {
        Binding::Let(name, c) => AnfExpr::Let(name, c, Box::new(acc)),
        Binding::LetRec(fname, fargs, fty, fbody) => {
            AnfExpr::LetRec(fname, fargs, fty, Box::new(fbody), Box::new(acc))
        }
    })
}

fn normalize(expr: &Expr, gs: &mut Gensym) -> AnfExpr {
    let mut bindings = Bindings::new();
    let c_expr = collect_bindings(expr, gs, &mut bindings);
    let tail = AnfExpr::Complex(c_expr);
    bindings_to_lets(bindings, tail)
}

pub fn convert(expr: &Expr) -> AnfExpr {
    let mut gs = Gensym::new();
    normalize(expr, &mut gs)
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::syntax::*;

    #[test]
    fn test_nested_binop() {
        // (1 + 2) * (3 + 4)
        // --->
        // let $0 = 1 + 2 in let $1 = 3 + 4 in $0 * $1
        let e = Expr::BinOp(
            BinOp::Mul,
            Box::new(Expr::BinOp(
                BinOp::Add,
                Box::new(Expr::Int(1)),
                Box::new(Expr::Int(2)),
            )),
            Box::new(Expr::BinOp(
                BinOp::Add,
                Box::new(Expr::Int(3)),
                Box::new(Expr::Int(4)),
            )),
        );
        let anf = convert(&e);

        assert_eq!(
            anf,
            AnfExpr::Let(
                "$0".to_string(),
                CompExpr::BinOp(BinOp::Add, AExpr::Int(1), AExpr::Int(2)),
                Box::new(AnfExpr::Let(
                    "$1".to_string(),
                    CompExpr::BinOp(BinOp::Add, AExpr::Int(3), AExpr::Int(4)),
                    Box::new(AnfExpr::Complex(CompExpr::BinOp(
                        BinOp::Mul,
                        AExpr::Var("$0".to_string()),
                        AExpr::Var("$1".to_string())
                    )))
                ))
            )
        );
    }

    #[test]
    fn test_atom_alone() {
        // 5 
        // ---> 
        // Complex(Atom(5))
        let e = Expr::Int(5);
        let anf = convert(&e);

        assert_eq!(anf, AnfExpr::Complex(CompExpr::Atom(AExpr::Int(5))));
    }

    #[test]
    fn test_let_simple_no_extra_binding() {
        // let x = 1 + 2 in x
        // --->
        // let x = 1 + 2 in Complex(Atom(x))
        let e = Expr::Let(
            "x".to_string(),
            None,
            Box::new(Expr::BinOp(
                BinOp::Add,
                Box::new(Expr::Int(1)),
                Box::new(Expr::Int(2)),
            )),
            Box::new(Expr::Var("x".to_string())),
        );

        let anf = convert(&e);
        assert_eq!(
            anf,
            AnfExpr::Let(
                "x".to_string(),
                CompExpr::BinOp(BinOp::Add, AExpr::Int(1), AExpr::Int(2)),
                Box::new(AnfExpr::Complex(CompExpr::Atom(AExpr::Var("x".to_string()))))
            )
        );
    }

    #[test]
    fn test_let_nested_inside_binop() {
        // 1 + (let x = 2 in x + 1)
        // --->
        // let x = 2 in let $0 = x + 1 in 1 + $0
        let e = Expr::BinOp(
            BinOp::Add,
            Box::new(Expr::Int(1)),
            Box::new(Expr::Let(
                "x".to_string(),
                None,
                Box::new(Expr::Int(2)),
                Box::new(Expr::BinOp(
                    BinOp::Add,
                    Box::new(Expr::Var("x".to_string())),
                    Box::new(Expr::Int(1)),
                )),
            )),
        );

        let anf = convert(&e);
        assert_eq!(
            anf,
            AnfExpr::Let(
                "x".to_string(),
                CompExpr::Atom(AExpr::Int(2)),
                Box::new(AnfExpr::Let(
                    "$0".to_string(),
                    CompExpr::BinOp(BinOp::Add, AExpr::Var("x".to_string()), AExpr::Int(1)),
                    Box::new(AnfExpr::Complex(CompExpr::BinOp(
                        BinOp::Add,
                        AExpr::Int(1),
                        AExpr::Var("$0".to_string())
                    )))
                ))
            )
        );
    }

    #[test]
    fn test_if_condition_gets_bound() {
        // if (1 < 2) then 1 else 2
        // --->
        // let $0 = 1 < 2 in if $0 then 1 else 2
        let e = Expr::If(
            Box::new(Expr::BinOp(
                BinOp::Lt,
                Box::new(Expr::Int(1)),
                Box::new(Expr::Int(2)),
            )),
            Box::new(Expr::Int(1)),
            Box::new(Expr::Int(2)),
        );

        let anf = convert(&e);
        assert_eq!(
            anf,
            AnfExpr::Let(
                "$0".to_string(),
                CompExpr::BinOp(BinOp::Lt, AExpr::Int(1), AExpr::Int(2)),
                Box::new(AnfExpr::Complex(CompExpr::If(
                    AExpr::Var("$0".to_string()),
                    Box::new(AnfExpr::Complex(CompExpr::Atom(AExpr::Int(1)))),
                    Box::new(AnfExpr::Complex(CompExpr::Atom(AExpr::Int(2))))
                )))
            )
        );
    }

    #[test]
    fn test_if_branches_have_independent_bindings() {
        // if true then (1 + (let x = 3 in x * x)) else ((let x = 5 in x * 3) + 4)
        // --->
        // if true then let x = 3 in 
        //              let $0 = x * x 
        //              in 1 + $0 
        //          else let x = 5 in 
        //               let $1 = x * 3 
        //               in $1 + 4
        let e = Expr::If(
            Box::new(Expr::Bool(true)),
            Box::new(Expr::BinOp(
                BinOp::Add, 
                Box::new(Expr::Int(1)),
                Box::new(Expr::Let(
                    "x".to_string(),
                    None,
                    Box::new(Expr::Int(3)),
                    Box::new(Expr::BinOp(
                        BinOp::Mul,
                        Box::new(Expr::Var("x".to_string())),
                        Box::new(Expr::Var("x".to_string())),
                    )),
                )),
            )),
            Box::new(Expr::BinOp(
                BinOp::Add,
                Box::new(Expr::Let(
                    "x".to_string(),
                    None,
                    Box::new(Expr::Int(5)),
                    Box::new(Expr::BinOp(
                        BinOp::Mul,
                        Box::new(Expr::Var("x".to_string())),
                        Box::new(Expr::Int(3)),
                    )),
                )),
                Box::new(Expr::Int(4)),
            )),
        );

        let anf = convert(&e);
        
        assert_eq!(
            anf,
            AnfExpr::Complex(CompExpr::If(
                AExpr::Bool(true),
                Box::new(AnfExpr::Let(
                    "x".to_string(),
                    CompExpr::Atom(AExpr::Int(3)),
                    Box::new(AnfExpr::Let(
                        "$0".to_string(),
                        CompExpr::BinOp(
                            BinOp::Mul,
                            AExpr::Var("x".to_string()),
                            AExpr::Var("x".to_string())
                        ),
                        Box::new(AnfExpr::Complex(CompExpr::BinOp(
                            BinOp::Add,
                            AExpr::Int(1),
                            AExpr::Var("$0".to_string())
                        )))
                    ))
                )),
                Box::new(AnfExpr::Let(
                    "x".to_string(),
                    CompExpr::Atom(AExpr::Int(5)),
                    Box::new(AnfExpr::Let(
                        "$1".to_string(),
                        CompExpr::BinOp(
                            BinOp::Mul,
                            AExpr::Var("x".to_string()),
                            AExpr::Int(3)
                        ),
                        Box::new(AnfExpr::Complex(CompExpr::BinOp(
                            BinOp::Add,
                            AExpr::Var("$1".to_string()),
                            AExpr::Int(4)
                        )))
                    ))
                ))
            ))
        );
    }

    #[test]
    fn test_app_with_complex_arg() {
        // f(1 + 2, 3)
        // --->
        // let $0 = 1 + 2 in f($0, 3)
        let e = Expr::App(
            Box::new(Expr::Var("f".to_string())),
            vec![
                Expr::BinOp(BinOp::Add, Box::new(Expr::Int(1)), Box::new(Expr::Int(2))),
                Expr::Int(3),
            ],
        );

        let anf = convert(&e);

        assert_eq!(
            anf,
            AnfExpr::Let(
                "$0".to_string(),
                CompExpr::BinOp(BinOp::Add, AExpr::Int(1), AExpr::Int(2)),
                Box::new(AnfExpr::Complex(CompExpr::App(
                    AExpr::Var("f".to_string()),
                    vec![AExpr::Var("$0".to_string()), AExpr::Int(3)]
                )))
            )
        );
    }

    #[test]
    fn test_lambda_body_normalized() {
        // fun (x: Int) => x + 1
        // --->
        // Complex(Lambda([("x", Int)], None, Complex(BinOp(Add, Var("x"), Int(1)))))
        let e = Expr::Lambda(
            vec![("x".to_string(), Type::Int)],
            None,
            Box::new(Expr::BinOp(
                BinOp::Add,
                Box::new(Expr::Var("x".to_string())),
                Box::new(Expr::Int(1)),
            )),
        );

        let anf = convert(&e);

        assert_eq!(
            anf,
            AnfExpr::Complex(CompExpr::Lambda(
                vec![("x".to_string(), Type::Int)],
                None,
                Box::new(AnfExpr::Complex(CompExpr::BinOp(
                    BinOp::Add,
                    AExpr::Var("x".to_string()),
                    AExpr::Int(1)
                )))
            ))
        );
    }

    #[test]
    fn test_letrec_and_call() {
        // let rec f (x: Int) : Int = x in f(1)
        // --->
        // let rec f (x: Int) : Int = Complex(Atom(Var("x"))) in Complex(App(Var("f"), [Int(1)]))
        let e = Expr::LetRec(
            "f".to_string(),
            vec![("x".to_string(), Type::Int)],
            Type::Int,
            Box::new(Expr::Var("x".to_string())),
            Box::new(Expr::App(
                Box::new(Expr::Var("f".to_string())),
                vec![Expr::Int(1)],
            )),
        );

        let anf = convert(&e);

        assert_eq!(
            anf,
            AnfExpr::LetRec(
                "f".to_string(),
                vec![("x".to_string(), Type::Int)],
                Type::Int,
                Box::new(AnfExpr::Complex(CompExpr::Atom(AExpr::Var("x".to_string())))),
                Box::new(AnfExpr::Complex(CompExpr::App(
                    AExpr::Var("f".to_string()),
                    vec![AExpr::Int(1)]
                )))
            )
        );
    }

    #[test]
    fn test_ann_wrapping_atom_no_extra_binding() {
        // 1 + (x : Int)
        // --->
        // Complex(BinOp(Add, Int(1), Var("x")))
        let e = Expr::BinOp(
            BinOp::Add,
            Box::new(Expr::Int(1)),
            Box::new(Expr::Ann(Box::new(Expr::Var("x".to_string())), Type::Int)),
        );

        let anf = convert(&e);

        assert_eq!(
            anf,
            AnfExpr::Complex(CompExpr::BinOp(
                BinOp::Add,
                AExpr::Int(1),
                AExpr::Var("x".to_string())
            ))
        );
    }

    #[test]
    fn test_let_body_is_var_no_extra_binding() {
        // 1 + (let y = 2 in y)
        // --->
        // let y = 2 in Complex(BinOp(Add, Int(1), Var("y")))
        let e = Expr::BinOp(
            BinOp::Add,
            Box::new(Expr::Int(1)),
            Box::new(Expr::Let(
                "y".to_string(),
                None,
                Box::new(Expr::Int(2)),
                Box::new(Expr::Var("y".to_string())),
            )),
        );
        let anf = convert(&e);

        assert_eq!(
            anf,
            AnfExpr::Let(
                "y".to_string(),
                CompExpr::Atom(AExpr::Int(2)),
                Box::new(AnfExpr::Complex(CompExpr::BinOp(
                    BinOp::Add,
                    AExpr::Int(1),
                    AExpr::Var("y".to_string())
                )))
            )
        );
    }
}

