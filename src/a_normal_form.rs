use crate::gensym::Gensym;
use crate::syntax::{BinOp, Ident, PrimIO, Type, UnaryOp};
use crate::typechecker::TypedExpr;

/*
 * an atomic expression ends up as an immediate argument of an assembly instruction
 */
#[derive(Debug, Clone, PartialEq)]
pub enum AExpr {
    Unit,
    Bool(bool),
    Int(i64),
    Float(f64),
    Var(Ident, Type),
}

#[derive(Debug, Clone, PartialEq)]
pub enum CompExpr {
    Atom(AExpr),
    BinOp(BinOp, AExpr, AExpr),
    Tuple(Vec<AExpr>),
    PrimIO(PrimIO, Option<AExpr>),
    UnaryOp(UnaryOp, AExpr),
    App(AExpr, Vec<AExpr>),
    If(AExpr, Box<AnfExpr>, Box<AnfExpr>),
    Lambda(Vec<(Ident, Type)>, Type, Box<AnfExpr>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum AnfExpr {
    Complex(CompExpr),
    Let(Ident, CompExpr, Box<AnfExpr>),
    LetRec(Ident, Vec<(Ident, Type)>, Type, Box<AnfExpr>, Box<AnfExpr>),
}

pub fn a_unit() -> AExpr {
    AExpr::Unit
}

pub fn a_bool(b: bool) -> AExpr {
    AExpr::Bool(b)
}

pub fn a_int(n: i64) -> AExpr {
    AExpr::Int(n)
}

pub fn a_float(f: f64) -> AExpr {
    AExpr::Float(f)
}

pub fn a_var(name: impl Into<Ident>, ty: Type) -> AExpr {
    AExpr::Var(name.into(), ty)
}

pub fn atom(a: AExpr) -> CompExpr {
    CompExpr::Atom(a)
}

pub fn c_bin_op(op: BinOp, left: AExpr, right: AExpr) -> CompExpr {
    CompExpr::BinOp(op, left, right)
}

pub fn c_tuple(elems: Vec<AExpr>) -> CompExpr {
    CompExpr::Tuple(elems)
}

pub fn c_prim_io(prim: PrimIO, expr: Option<AExpr>) -> CompExpr {
    CompExpr::PrimIO(prim, expr)
}

pub fn c_unary(op: UnaryOp, expr: AExpr) -> CompExpr {
    CompExpr::UnaryOp(op, expr)
}

pub fn c_app(func: AExpr, args: Vec<AExpr>) -> CompExpr {
    CompExpr::App(func, args)
}

pub fn c_if(cond: AExpr, thn: AnfExpr, els: AnfExpr) -> CompExpr {
    CompExpr::If(cond, Box::new(thn), Box::new(els))
}

pub fn c_lambda(params: Vec<(Ident, Type)>, ret_ty: Type, body: AnfExpr) -> CompExpr {
    CompExpr::Lambda(params, ret_ty, Box::new(body))
}

pub fn complex(c: CompExpr) -> AnfExpr {
    AnfExpr::Complex(c)
}

pub fn anf_let(name: impl Into<Ident>, c: CompExpr, body: AnfExpr) -> AnfExpr {
    AnfExpr::Let(name.into(), c, Box::new(body))
}

pub fn anf_let_rec(
    name: impl Into<Ident>,
    params: Vec<(Ident, Type)>,
    ret_ty: Type,
    fbody: AnfExpr,
    body: AnfExpr,
) -> AnfExpr {
    AnfExpr::LetRec(name.into(), params, ret_ty, Box::new(fbody), Box::new(body))
}

enum Binding {
    Let(Ident, CompExpr),
    LetRec(Ident, Vec<(Ident, Type)>, Type, AnfExpr),
}

type Bindings = Vec<Binding>;

fn to_atom(expr: TypedExpr, gs: &mut Gensym, bindings: &mut Bindings) -> AExpr {
    match expr {
        TypedExpr::Unit => AExpr::Unit,
        TypedExpr::Bool(b) => AExpr::Bool(b),
        TypedExpr::Int(i) => AExpr::Int(i),
        TypedExpr::Float(f) => AExpr::Float(f),
        TypedExpr::Var(name, ty) => AExpr::Var(name, ty),
        _ => {
            let ty = expr.type_of();

            let c = collect_bindings(expr, gs, bindings);
            if let CompExpr::Atom(a) = c {
                return a;
            }
            let name = gs.fresh();
            bindings.push(Binding::Let(name.clone(), c));
            AExpr::Var(name, ty)
        },
    }
}

fn collect_bindings(expr: TypedExpr, gs: &mut Gensym, bindings: &mut Bindings) -> CompExpr {
    match expr {
        TypedExpr::Unit => CompExpr::Atom(AExpr::Unit),
        TypedExpr::Bool(b) => CompExpr::Atom(AExpr::Bool(b)),
        TypedExpr::Int(i) => CompExpr::Atom(AExpr::Int(i)),
        TypedExpr::Float(f) => CompExpr::Atom(AExpr::Float(f)),
        TypedExpr::Var(name, ty) => CompExpr::Atom(AExpr::Var(name, ty)),
        TypedExpr::Tuple(typed_exprs, _) => {
            let mut elements_atom = Vec::with_capacity(typed_exprs.len());
            for expr in typed_exprs {
                let element_atom = to_atom(expr, gs, bindings);
                elements_atom.push(element_atom);
            }
            CompExpr::Tuple(elements_atom)
        },
        TypedExpr::PrimIO(prim_io, typed_expr, _) => match typed_expr {
            Some(typed_expr) => {
                let atom = to_atom(*typed_expr, gs, bindings);
                CompExpr::PrimIO(prim_io, Some(atom))
            },
            None => CompExpr::PrimIO(prim_io, None),
        },
        TypedExpr::BinOp(op, left, right, _) => {
            let left_atom = to_atom(*left, gs, bindings);
            let right_atom = to_atom(*right, gs, bindings);
            CompExpr::BinOp(op, left_atom, right_atom)
        },
        TypedExpr::UnaryOp(op, operand, _) => {
            let operand_atom = to_atom(*operand, gs, bindings);
            CompExpr::UnaryOp(op, operand_atom)
        },
        TypedExpr::Ann(inner, _) => collect_bindings(*inner, gs, bindings),
        TypedExpr::If(cond, thn, els, _) => {
            let cond_atom = to_atom(*cond, gs, bindings);
            let then_anf = normalize(*thn, gs);
            let else_anf = normalize(*els, gs);
            CompExpr::If(cond_atom, Box::new(then_anf), Box::new(else_anf))
        },
        TypedExpr::App(func, args, _) => {
            let func_atom = to_atom(*func, gs, bindings);
            let mut args_atom = Vec::with_capacity(args.len());
            for arg in args {
                let arg_atom = to_atom(arg, gs, bindings);
                args_atom.push(arg_atom);
            }
            CompExpr::App(func_atom, args_atom)
        },
        TypedExpr::Lambda(params, ret_ty, body, _) => {
            let body_anf = normalize(*body, gs);
            CompExpr::Lambda(params, ret_ty, Box::new(body_anf))
        },
        TypedExpr::Let(name, _, rhs, body, _) => {
            let rhs_comp = collect_bindings(*rhs, gs, bindings);
            bindings.push(Binding::Let(name, rhs_comp));
            collect_bindings(*body, gs, bindings)
        },
        TypedExpr::LetRec(fname, fparams, fty, fbody, body, _) => {
            let fbody_anf = normalize(*fbody, gs);
            bindings.push(Binding::LetRec(fname, fparams, fty, fbody_anf));
            collect_bindings(*body, gs, bindings)
        },
    }
}

fn bindings_to_lets(bindings: Bindings, tail: AnfExpr) -> AnfExpr {
    bindings.into_iter().rev().fold(tail, |acc, b| match b {
        Binding::Let(name, c) => AnfExpr::Let(name, c, Box::new(acc)),
        Binding::LetRec(fname, fparams, fty, fbody) => {
            AnfExpr::LetRec(fname, fparams, fty, Box::new(fbody), Box::new(acc))
        },
    })
}

fn normalize(expr: TypedExpr, gs: &mut Gensym) -> AnfExpr {
    let mut bindings = Bindings::new();
    let c_expr = collect_bindings(expr, gs, &mut bindings);
    let tail = AnfExpr::Complex(c_expr);
    bindings_to_lets(bindings, tail)
}

pub fn anf_convert(expr: TypedExpr) -> AnfExpr {
    let mut gs = Gensym::new();
    normalize(expr, &mut gs)
}

// todo
#[cfg(test)]
mod tests {
    use super::*;
    use crate::syntax::BinOp;
    use crate::syntax::PrimIO;
    use crate::syntax::Type;
    use crate::typechecker::{
        t_app, t_bin_op, t_bool, t_if, t_int, t_lambda, t_let, t_let_rec, t_prim_io, t_tuple,
        t_var,
    };

    #[test]
    fn test_nested_binop() {
        // (1 + 2) * (3 + 4)
        // --->
        // let $0 = 1 + 2 in
        // let $1 = 3 + 4 in
        // $0 * $1
        let e = t_bin_op(
            BinOp::Mul,
            t_bin_op(BinOp::Add, t_int(1), t_int(2), Type::Int),
            t_bin_op(BinOp::Add, t_int(3), t_int(4), Type::Int),
            Type::Int,
        );
        let anf = anf_convert(e);

        assert_eq!(
            anf,
            anf_let(
                "$0",
                c_bin_op(BinOp::Add, a_int(1), a_int(2)),
                anf_let(
                    "$1",
                    c_bin_op(BinOp::Add, a_int(3), a_int(4)),
                    complex(c_bin_op(
                        BinOp::Mul,
                        a_var("$0", Type::Int),
                        a_var("$1", Type::Int)
                    ))
                )
            )
        );
    }

    #[test]
    fn test_atom_alone() {
        let e = t_int(5);
        let anf = anf_convert(e);
        assert_eq!(anf, complex(atom(a_int(5))));
    }

    #[test]
    fn test_let_simple_no_extra_binding() {
        // let x = 1 + 2 in x
        // --->
        // let x = 1 + 2 in x
        let e = t_let(
            "x",
            Type::Int,
            t_bin_op(BinOp::Add, t_int(1), t_int(2), Type::Int),
            t_var("x", Type::Int),
            Type::Int,
        );

        let anf = anf_convert(e);
        assert_eq!(
            anf,
            anf_let(
                "x",
                c_bin_op(BinOp::Add, a_int(1), a_int(2)),
                complex(atom(a_var("x", Type::Int)))
            )
        );
    }

    #[test]
    fn test_let_nested_inside_binop() {
        // 1 + (let x = 2 in x + 1)
        // --->
        // let x = 2 in
        // let $0 = x + 1 in
        // 1 + $0
        let e = t_bin_op(
            BinOp::Add,
            t_int(1),
            t_let(
                "x",
                Type::Int,
                t_int(2),
                t_bin_op(BinOp::Add, t_var("x", Type::Int), t_int(1), Type::Int),
                Type::Int,
            ),
            Type::Int,
        );

        let anf = anf_convert(e);
        assert_eq!(
            anf,
            anf_let(
                "x",
                atom(a_int(2)),
                anf_let(
                    "$0",
                    c_bin_op(BinOp::Add, a_var("x", Type::Int), a_int(1)),
                    complex(c_bin_op(BinOp::Add, a_int(1), a_var("$0", Type::Int)))
                )
            )
        );
    }

    #[test]
    fn test_if_condition_gets_bound() {
        // if 1 < 2 then 1 else 2
        // --->
        // let $0 = 1 < 2
        // in if $0 then 1 else 2
        let e = t_if(
            t_bin_op(BinOp::Lt, t_int(1), t_int(2), Type::Bool),
            t_int(1),
            t_int(2),
            Type::Int,
        );

        let anf = anf_convert(e);
        assert_eq!(
            anf,
            anf_let(
                "$0",
                c_bin_op(BinOp::Lt, a_int(1), a_int(2)),
                complex(c_if(
                    a_var("$0", Type::Bool),
                    complex(atom(a_int(1))),
                    complex(atom(a_int(2)))
                ))
            )
        );
    }

    #[test]
    fn test_if_branches_have_independent_bindings() {
        // if true
        // then 1 + (let x = 3 in x * x)
        // else (let x = 5 in x * 3) + 4
        // --->
        // if true
        // then let x = 3
        //      in let $0 = x * x
        //         in 1 + $0
        // else let x = 5
        //      in let $1 = x * 3
        //      in $1 + 4
        let e = t_if(
            t_bool(true),
            t_bin_op(
                BinOp::Add,
                t_int(1),
                t_let(
                    "x",
                    Type::Int,
                    t_int(3),
                    t_bin_op(BinOp::Mul, t_var("x", Type::Int), t_var("x", Type::Int), Type::Int),
                    Type::Int,
                ),
                Type::Int,
            ),
            t_bin_op(
                BinOp::Add,
                t_let(
                    "x",
                    Type::Int,
                    t_int(5),
                    t_bin_op(BinOp::Mul, t_var("x", Type::Int), t_int(3), Type::Int),
                    Type::Int,
                ),
                t_int(4),
                Type::Int,
            ),
            Type::Int,
        );

        let anf = anf_convert(e);

        assert_eq!(
            anf,
            complex(c_if(
                a_bool(true),
                anf_let(
                    "x",
                    atom(a_int(3)),
                    anf_let(
                        "$0",
                        c_bin_op(BinOp::Mul, a_var("x", Type::Int), a_var("x", Type::Int)),
                        complex(c_bin_op(BinOp::Add, a_int(1), a_var("$0", Type::Int)))
                    )
                ),
                anf_let(
                    "x",
                    atom(a_int(5)),
                    anf_let(
                        "$1",
                        c_bin_op(BinOp::Mul, a_var("x", Type::Int), a_int(3)),
                        complex(c_bin_op(BinOp::Add, a_var("$1", Type::Int), a_int(4)))
                    )
                )
            ))
        );
    }

    #[test]
    fn test_app_with_complex_arg() {
        // f (1 + 2) 3
        // --->
        // let $0 = 1 + 2
        // in f $0 3
        let fn_ty = Type::Arrow(
            Box::new(Type::Int),
            Box::new(Type::Arrow(Box::new(Type::Int), Box::new(Type::Int))),
        );
        let e = t_app(
            t_var("f", fn_ty.clone()),
            vec![
                t_bin_op(BinOp::Add, t_int(1), t_int(2), Type::Int),
                t_int(3),
            ],
            Type::Int,
        );

        let anf = anf_convert(e);

        assert_eq!(
            anf,
            anf_let(
                "$0",
                c_bin_op(BinOp::Add, a_int(1), a_int(2)),
                complex(c_app(a_var("f", fn_ty), vec![a_var("$0", Type::Int), a_int(3)]))
            )
        );
    }

    #[test]
    fn test_lambda_body_normalized() {
        // fun (x : Int) : Int => x + 1
        // --->
        // fun (x : Int) : Int => x + 1
        let e = t_lambda(
            vec![("x".to_string(), Type::Int)],
            Type::Int,
            t_bin_op(BinOp::Add, t_var("x", Type::Int), t_int(1), Type::Int),
            Type::Arrow(Box::new(Type::Int), Box::new(Type::Int)),
        );

        let anf = anf_convert(e);

        assert_eq!(
            anf,
            complex(c_lambda(
                vec![("x".to_string(), Type::Int)],
                Type::Int,
                complex(c_bin_op(BinOp::Add, a_var("x", Type::Int), a_int(1)))
            ))
        );
    }

    #[test]
    fn test_letrec_and_call() {
        // let rec f (x : Int) : Int = x in f 1
        // --->
        // let rec f (x : Int) : Int = x in f 1
        let fn_ty = Type::Arrow(Box::new(Type::Int), Box::new(Type::Int));
        let e = t_let_rec(
            "f",
            vec![("x".to_string(), Type::Int)],
            Type::Int,
            t_var("x", Type::Int),
            t_app(t_var("f", fn_ty.clone()), vec![t_int(1)], Type::Int),
            Type::Int,
        );

        let anf = anf_convert(e);

        assert_eq!(
            anf,
            anf_let_rec(
                "f",
                vec![("x".to_string(), Type::Int)],
                Type::Int,
                complex(atom(a_var("x", Type::Int))),
                complex(c_app(a_var("f", fn_ty), vec![a_int(1)]))
            )
        );
    }

    #[test]
    fn test_ann_wrapping_atom_no_extra_binding() {
        let e = t_bin_op(BinOp::Add, t_int(1), t_var("x", Type::Int), Type::Int);

        let anf = anf_convert(e);

        assert_eq!(
            anf,
            complex(c_bin_op(BinOp::Add, a_int(1), a_var("x", Type::Int)))
        );
    }

    #[test]
    fn test_let_body_is_var_no_extra_binding() {
        // 1 + (let y = 2 in y)
        // --->
        // let y = 2 in 1 + y
        let e = t_bin_op(
            BinOp::Add,
            t_int(1),
            t_let("y", Type::Int, t_int(2), t_var("y", Type::Int), Type::Int),
            Type::Int,
        );

        let anf = anf_convert(e);

        assert_eq!(
            anf,
            anf_let(
                "y",
                atom(a_int(2)),
                complex(c_bin_op(BinOp::Add, a_int(1), a_var("y", Type::Int)))
            )
        );
    }
    #[test]
    fn test_nested_if_predicate() {
        // let x = 10 in
        // let y = 12 in
        // if if x < 1 then x == 0 else x == 2
        // then y + 2
        // else y + 10
        //
        // --->
        // let x = 10 in
        // let y = 12 in
        // let $0 = x < 1
        // let $1 = if $0 then x == 0 else x == 2 in
        // if $1 then y + 2 else y + 10

        let e = t_let(
            "x",
            Type::Int,
            t_int(10),
            t_let(
                "y",
                Type::Int,
                t_int(12),
                t_if(
                    t_if(
                        t_bin_op(BinOp::Lt, t_var("x", Type::Int), t_int(1), Type::Bool),
                        t_bin_op(BinOp::Eq, t_var("x", Type::Int), t_int(0), Type::Bool),
                        t_bin_op(BinOp::Eq, t_var("x", Type::Int), t_int(2), Type::Bool),
                        Type::Bool,
                    ),
                    t_bin_op(BinOp::Add, t_var("y", Type::Int), t_int(2), Type::Int),
                    t_bin_op(BinOp::Add, t_var("y", Type::Int), t_int(10), Type::Int),
                    Type::Int,
                ),
                Type::Int,
            ),
            Type::Int,
        );
        let anf = anf_convert(e);
        let expected = anf_let(
            "x",
            atom(a_int(10)),
            anf_let(
                "y",
                atom(a_int(12)),
                anf_let(
                    "$0",
                    c_bin_op(BinOp::Lt, a_var("x", Type::Int), a_int(1)),
                    anf_let(
                        "$1",
                        c_if(
                            a_var("$0", Type::Bool),
                            complex(c_bin_op(BinOp::Eq, a_var("x", Type::Int), a_int(0))),
                            complex(c_bin_op(BinOp::Eq, a_var("x", Type::Int), a_int(2))),
                        ),
                        complex(c_if(
                            a_var("$1", Type::Bool),
                            complex(c_bin_op(BinOp::Add, a_var("y", Type::Int), a_int(2))),
                            complex(c_bin_op(BinOp::Add, a_var("y", Type::Int), a_int(10))),
                        )),
                    ),
                ),
            ),
        );
        assert_eq!(anf, expected);
    }

    #[test]
    fn test_nested_if_branch_then() {
        // let x = read_int () in
        // let y = read_int () in
        // if x < y
        // then if y < 100 then y + 100 else y + x
        // else y + 10
        //
        // --->
        // let x = read_int () in
        // let y = read_int () in
        // let $0 = x < y in
        // if $0
        // then let $1 = y < 100 in
        //      if $1 then y + 100 else y + x
        // else y + 10

        let e = t_let(
            "x",
            Type::Int,
            t_prim_io(PrimIO::ReadInt, None, Type::Int),
            t_let(
                "y",
                Type::Int,
                t_prim_io(PrimIO::ReadInt, None, Type::Int),
                t_if(
                    t_bin_op(BinOp::Lt, t_var("x", Type::Int), t_var("y", Type::Int), Type::Bool),
                    t_if(
                        t_bin_op(BinOp::Lt, t_var("y", Type::Int), t_int(100), Type::Bool),
                        t_bin_op(BinOp::Add, t_var("y", Type::Int), t_int(100), Type::Int),
                        t_bin_op(BinOp::Add, t_var("y", Type::Int), t_var("x", Type::Int), Type::Int),
                        Type::Int,
                    ),
                    t_bin_op(BinOp::Add, t_var("y", Type::Int), t_int(10), Type::Int),
                    Type::Int,
                ),
                Type::Int,
            ),
            Type::Int,
        );

        let anf = anf_convert(e);

        let expected = anf_let(
            "x",
            c_prim_io(PrimIO::ReadInt, None),
            anf_let(
                "y",
                c_prim_io(PrimIO::ReadInt, None),
                anf_let(
                    "$0",
                    c_bin_op(BinOp::Lt, a_var("x", Type::Int), a_var("y", Type::Int)),
                    complex(c_if(
                        a_var("$0", Type::Bool),
                        anf_let(
                            "$1",
                            c_bin_op(BinOp::Lt, a_var("y", Type::Int), a_int(100)),
                            complex(c_if(
                                a_var("$1", Type::Bool),
                                complex(c_bin_op(BinOp::Add, a_var("y", Type::Int), a_int(100))),
                                complex(c_bin_op(
                                    BinOp::Add,
                                    a_var("y", Type::Int),
                                    a_var("x", Type::Int)
                                )),
                            )),
                        ),
                        complex(c_bin_op(BinOp::Add, a_var("y", Type::Int), a_int(10))),
                    )),
                ),
            ),
        );
        assert_eq!(anf, expected);
    }

    #[test]
    fn test_nested_if_branch_else() {
        // let x = read_int () in
        // let y = read_int () in
        // if x == y
        // then x + y
        // else if x < 42
        //      then if y > 10
        //           then x + 10 + y
        //           else x - 100
        //      else x + 42
        //
        // --->
        // let x = read_int () in
        // let y = read_int () in
        // let $0 = x == y in
        // if $0
        // then x + y
        // else let $1 = x < 42 in
        //      if $1
        //      then let $2 = y > 10 in
        //           if $2
        //           then let $3 = x + 10 in $3 + y
        //           else x - 100
        //      else x + 42

        let e = t_let(
            "x",
            Type::Int,
            t_prim_io(PrimIO::ReadInt, None, Type::Int),
            t_let(
                "y",
                Type::Int,
                t_prim_io(PrimIO::ReadInt, None, Type::Int),
                t_if(
                    t_bin_op(BinOp::Eq, t_var("x", Type::Int), t_var("y", Type::Int), Type::Bool),
                    t_bin_op(BinOp::Add, t_var("x", Type::Int), t_var("y", Type::Int), Type::Int),
                    t_if(
                        t_bin_op(BinOp::Lt, t_var("x", Type::Int), t_int(42), Type::Bool),
                        t_if(
                            t_bin_op(BinOp::Gt, t_var("y", Type::Int), t_int(10), Type::Bool),
                            t_bin_op(
                                BinOp::Add,
                                t_bin_op(BinOp::Add, t_var("x", Type::Int), t_int(10), Type::Int),
                                t_var("y", Type::Int),
                                Type::Int,
                            ),
                            t_bin_op(BinOp::Sub, t_var("x", Type::Int), t_int(100), Type::Int),
                            Type::Int,
                        ),
                        t_bin_op(BinOp::Add, t_var("x", Type::Int), t_int(42), Type::Int),
                        Type::Int,
                    ),
                    Type::Int,
                ),
                Type::Int,
            ),
            Type::Int,
        );

        let anf = anf_convert(e);

        let expected = anf_let(
            "x",
            c_prim_io(PrimIO::ReadInt, None),
            anf_let(
                "y",
                c_prim_io(PrimIO::ReadInt, None),
                anf_let(
                    "$0",
                    c_bin_op(BinOp::Eq, a_var("x", Type::Int), a_var("y", Type::Int)),
                    complex(c_if(
                        a_var("$0", Type::Bool),
                        complex(c_bin_op(BinOp::Add, a_var("x", Type::Int), a_var("y", Type::Int))),
                        anf_let(
                            "$1",
                            c_bin_op(BinOp::Lt, a_var("x", Type::Int), a_int(42)),
                            complex(c_if(
                                a_var("$1", Type::Bool),
                                anf_let(
                                    "$2",
                                    c_bin_op(BinOp::Gt, a_var("y", Type::Int), a_int(10)),
                                    complex(c_if(
                                        a_var("$2", Type::Bool),
                                        anf_let(
                                            "$3",
                                            c_bin_op(
                                                BinOp::Add,
                                                a_var("x", Type::Int),
                                                a_int(10)
                                            ),
                                            complex(c_bin_op(
                                                BinOp::Add,
                                                a_var("$3", Type::Int),
                                                a_var("y", Type::Int)
                                            )),
                                        ),
                                        complex(c_bin_op(
                                            BinOp::Sub,
                                            a_var("x", Type::Int),
                                            a_int(100)
                                        )),
                                    )),
                                ),
                                complex(c_bin_op(
                                    BinOp::Add,
                                    a_var("x", Type::Int),
                                    a_int(42)
                                )),
                            )),
                        ),
                    )),
                ),
            ),
        );
        assert_eq!(anf, expected);
    }

    #[test]
    fn test_tuple_with_complex_element() {
        // (1, x, 2 + 3)
        // --->
        // let $0 = 2 + 3 in (1, x, $0)
        let e = t_tuple(
            vec![
                t_int(1),
                t_var("x", Type::Int),
                t_bin_op(BinOp::Add, t_int(2), t_int(3), Type::Int),
            ],
            Type::Tuple(vec![Type::Int, Type::Int, Type::Int]),
        );

        let anf = anf_convert(e);

        let expected = anf_let(
            "$0",
            c_bin_op(BinOp::Add, a_int(2), a_int(3)),
            complex(c_tuple(vec![
                a_int(1),
                a_var("x", Type::Int),
                a_var("$0", Type::Int),
            ])),
        );
        assert_eq!(anf, expected);
    }

    #[test]
    fn test_prim_io_print_complex_arg() {
        // print_int (1 + 2)
        // --->
        // let $0 = 1 + 2 in print_int $0
        let e = t_prim_io(
            PrimIO::PrintInt,
            Some(t_bin_op(BinOp::Add, t_int(1), t_int(2), Type::Int)),
            Type::Unit,
        );

        let anf = anf_convert(e);

        let expected = anf_let(
            "$0",
            c_bin_op(BinOp::Add, a_int(1), a_int(2)),
            complex(c_prim_io(PrimIO::PrintInt, Some(a_var("$0", Type::Int)))),
        );
        assert_eq!(anf, expected);
    }

    #[test]
    fn test_prim_io_read_none() {
        // read_int ()
        // --->
        // read_int ()
        let e = t_prim_io(PrimIO::ReadInt, None, Type::Int);

        let anf = anf_convert(e);

        assert_eq!(anf, complex(c_prim_io(PrimIO::ReadInt, None)));
    }
}
