use crate::expose_allocation::{AllocDef, AllocExpr, AllocProgram, GlobalValue};
use crate::gensym::Gensym;
use crate::syntax::{
    BinOp, HasType, Ident, PrimIO, Type, UnaryOp, ty_bool, ty_float, ty_int, ty_unit,
};

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

impl HasType for AExpr {
    fn type_of(&self) -> Type {
        match self {
            AExpr::Unit => Type::Unit,
            AExpr::Bool(_) => Type::Bool,
            AExpr::Int(_) => Type::Int,
            AExpr::Float(_) => Type::Float,
            AExpr::Var(_, ty) => ty.clone(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum CompExpr {
    Atom(AExpr, Type),
    BinOp(BinOp, AExpr, AExpr, Type),
    UnaryOp(UnaryOp, AExpr, Type),
    TupleProj(AExpr, usize, Type),
    Allocate(
        usize, // bytes
        Type,
    ),
    AllocateClosure(
        usize, // bytes
        usize, // arity
        Type,
    ),
    PrimIO(PrimIO, Option<AExpr>, Type),
    App(AExpr, Vec<AExpr>, Type),
    If(AExpr, Box<AnfExpr>, Box<AnfExpr>, Type), // if <cond>
                                                 // then let <name> = if <cond> 
                                                 //                   then let <name> = <rhs> in <body>
                                                 //                   else <expr>
                                                 // else <expr>
    Collect(usize),
    TupleSet(AExpr, AExpr, usize),
    // Seq(Vec<CompExpr>, Box<AExpr>, Type),
    GlobalValue(GlobalValue),
    FunRef(Ident, usize, Type), // closure convention去掉了FunRef, expose allocation加了回来
}

impl HasType for CompExpr {
    fn type_of(&self) -> Type {
        match self {
            CompExpr::Atom(_, ty) => ty.clone(),
            CompExpr::BinOp(_, _, _, ty) => ty.clone(),
            CompExpr::UnaryOp(_, _, ty) => ty.clone(),
            CompExpr::TupleProj(_, _, ty) => ty.clone(),
            CompExpr::Allocate(_, ty) => ty.clone(),
            CompExpr::AllocateClosure(_, _, ty) => ty.clone(),
            CompExpr::PrimIO(_, _, ty) => ty.clone(),
            CompExpr::App(_, _, ty) => ty.clone(),
            CompExpr::If(_, _, _, ty) => ty.clone(),
            CompExpr::Collect(_) => Type::Unit,
            CompExpr::TupleSet(_, _, _) => Type::Int,
            // CompExpr::Seq(_, _, ty) => ty.clone(),
            CompExpr::GlobalValue(gv) => gv.type_of(),
            CompExpr::FunRef(_, _, ty) => ty.clone(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum AnfExpr {
    Complex(CompExpr, Type),
    Let(Ident, CompExpr, Box<AnfExpr>, Type),
}

impl HasType for AnfExpr {
    fn type_of(&self) -> Type {
        match self {
            AnfExpr::Complex(_, ty) => ty.clone(),
            AnfExpr::Let(_, _, _, ty) => ty.clone(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum AnfDef {
    ValDef(
        Ident,        // name
        Option<Type>, // optional type annotation
        AnfExpr,      // expresion
    ),
    FunDef(
        Ident,              // function name
        Vec<(Ident, Type)>, // function arguments with their types
        Type,               // function return type
        AnfExpr,            // function body
    ),
}

#[derive(Debug, Clone, PartialEq)]
pub struct AnfProgram {
    pub defs: Vec<AnfDef>,
    pub main: AnfExpr,
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

pub fn c_atom(atom: AExpr, ty: Type) -> CompExpr {
    CompExpr::Atom(atom, ty)
}

pub fn c_bin_op(op: BinOp, left: AExpr, right: AExpr, ty: Type) -> CompExpr {
    CompExpr::BinOp(op, left, right, ty)
}

pub fn c_unary(op: UnaryOp, expr: AExpr, ty: Type) -> CompExpr {
    CompExpr::UnaryOp(op, expr, ty)
}

pub fn c_tuple_proj(tuple: AExpr, index: usize, ty: Type) -> CompExpr {
    CompExpr::TupleProj(tuple, index, ty)
}

pub fn c_allocate(bytes: usize, ty: Type) -> CompExpr {
    CompExpr::Allocate(bytes, ty)
}

pub fn c_allocate_closure(bytes: usize, arity: usize, ty: Type) -> CompExpr {
    CompExpr::AllocateClosure(bytes, arity, ty)
}

pub fn c_prim_io(prim: PrimIO, expr: Option<AExpr>, ty: Type) -> CompExpr {
    CompExpr::PrimIO(prim, expr, ty)
}

pub fn c_app(func: AExpr, args: Vec<AExpr>, ty: Type) -> CompExpr {
    CompExpr::App(func, args, ty)
}

pub fn c_if(cond: AExpr, thn: AnfExpr, els: AnfExpr, ty: Type) -> CompExpr {
    CompExpr::If(cond, Box::new(thn), Box::new(els), ty)
}

pub fn c_collect(bytes: usize) -> CompExpr {
    CompExpr::Collect(bytes)
}

pub fn c_tuple_set(tuple: AExpr, element: AExpr, index: usize) -> CompExpr {
    CompExpr::TupleSet(tuple, element, index)
}

// pub fn c_seq(exprs: Vec<CompExpr>, last: AExpr, ty: Type) -> CompExpr {
//     CompExpr::Seq(exprs, Box::new(last), ty)
// }

pub fn c_global_value(value: GlobalValue) -> CompExpr {
    CompExpr::GlobalValue(value)
}

pub fn c_free_ptr() -> CompExpr {
    CompExpr::GlobalValue(GlobalValue::FreePtr)
}

pub fn c_fromspace_end() -> CompExpr {
    CompExpr::GlobalValue(GlobalValue::FromspaceEnd)
}

pub fn c_fun_ref(name: impl Into<Ident>, arity: usize, ty: Type) -> CompExpr {
    CompExpr::FunRef(name.into(), arity, ty)
}

pub fn anf_complex(c: CompExpr, ty: Type) -> AnfExpr {
    AnfExpr::Complex(c, ty)
}

pub fn anf_let(name: impl Into<Ident>, c: CompExpr, body: AnfExpr, ty: Type) -> AnfExpr {
    AnfExpr::Let(name.into(), c, Box::new(body), ty)
}
enum Binding {
    Let(Ident, CompExpr),
    // LetRec(Ident, Vec<(Ident, Type)>, Type, AnfExpr),
}
//
type Bindings = Vec<Binding>;

fn to_atom(expr: AllocExpr, gs: &mut Gensym, bindings: &mut Bindings) -> AExpr {
    match expr {
        AllocExpr::Unit => AExpr::Unit,
        AllocExpr::Bool(b) => AExpr::Bool(b),
        AllocExpr::Int(i) => AExpr::Int(i),
        AllocExpr::Float(f) => AExpr::Float(f),
        AllocExpr::Var(name, ty) => AExpr::Var(name, ty),
        _ => {
            let c = to_complex(expr, gs, bindings);
            let ty = c.type_of();
            if let CompExpr::Atom(a, _) = c {
                a
            } else {
                let name = gs.fresh();
                bindings.push(Binding::Let(name.clone(), c));
                AExpr::Var(name, ty)
            }
        },
    }
}

fn to_complex(expr: AllocExpr, gs: &mut Gensym, bindings: &mut Bindings) -> CompExpr {
    match expr {
        AllocExpr::Unit => c_atom(a_unit(), ty_unit()),
        AllocExpr::Bool(b) => c_atom(a_bool(b), ty_bool()),
        AllocExpr::Int(i) => c_atom(a_int(i), ty_int()),
        AllocExpr::Float(f) => c_atom(a_float(f), ty_float()),
        AllocExpr::Var(name, ty) => c_atom(a_var(name, ty.clone()), ty),
        AllocExpr::TupleProj(expr, idx, ty) => {
            let atom = to_atom(*expr, gs, bindings);
            c_tuple_proj(atom, idx, ty)
        },
        AllocExpr::PrimIO(prim_io, alloc_expr, ty) => {
            let a_expr = match alloc_expr {
                Some(e) => {
                    let atom = to_atom(*e, gs, bindings);
                    Some(atom)
                },
                None => None,
            };
            c_prim_io(prim_io, a_expr, ty)
        },
        AllocExpr::BinOp(op, left, right, ty) => {
            let left_atom = to_atom(*left, gs, bindings);
            let right_atom = to_atom(*right, gs, bindings);
            c_bin_op(op, left_atom, right_atom, ty)
        },
        AllocExpr::UnaryOp(op, operand, ty) => {
            let operand_atom = to_atom(*operand, gs, bindings);
            c_unary(op, operand_atom, ty)
        },
        AllocExpr::If(cond, thn, els, ty) => {
            let cond_atom = to_atom(*cond, gs, bindings);
            let thn_anf = to_anf(*thn, gs);
            let els_anf = to_anf(*els, gs);
            c_if(cond_atom, thn_anf, els_anf, ty)
        },
        AllocExpr::App(func, args, ty) => {
            let func_atom = to_atom(*func, gs, bindings);
            let mut args_atom = Vec::with_capacity(args.len());
            for arg in args {
                let arg_atom = to_atom(arg, gs, bindings);
                args_atom.push(arg_atom);
            }
            c_app(func_atom, args_atom, ty)
        },
        AllocExpr::Let(name, _, rhs, body, _) => {
            let rhs_comp = to_complex(*rhs, gs, bindings);
            bindings.push(Binding::Let(name, rhs_comp));
            to_complex(*body, gs, bindings)
        },
        AllocExpr::Collect(bytes) => c_collect(bytes),
        AllocExpr::Allocate(bytes, ty) => c_allocate(bytes, ty),
        AllocExpr::AllocateClosure(bytes, arity, ty) => c_allocate_closure(bytes, arity, ty),
        AllocExpr::FunRef(name, arity, ty) => c_fun_ref(name, arity, ty),
        AllocExpr::GlobalValue(value) => c_global_value(value),
        AllocExpr::TupleSet(tuple, elem, idx) => {
            let tuple_atom = to_atom(*tuple, gs, bindings);
            let elem_atom = to_atom(*elem, gs, bindings);
            c_tuple_set(tuple_atom, elem_atom, idx)
        },
        // AllocExpr::Seq(exprs, last, ty) => {
        //     let mut c_exprs = Vec::with_capacity(exprs.len());
        //     for expr in exprs {
        //         let c_expr = to_complex(expr, gs, bindings);
        //         c_exprs.push(c_expr);
        //     }
        //     let last_atom = to_atom(*last, gs, bindings);
        //     c_seq(c_exprs, last_atom, ty)
        // },
    }
}

fn bindings_to_lets(bindings: Bindings, tail: AnfExpr) -> AnfExpr {
    let ty = tail.type_of();
    bindings.into_iter().rev().fold(tail, |acc, b| match b {
        Binding::Let(name, c) => anf_let(name, c, acc, ty.clone()),
        // Binding::LetRec(fname, fparams, fty, fbody) => {
        //     AnfExpr::LetRec(fname, fparams, fty, Box::new(fbody), Box::new(acc))
        // },
    })
}

fn to_anf(expr: AllocExpr, gs: &mut Gensym) -> AnfExpr {
    let mut bindings = Bindings::new();
    let c_expr = to_complex(expr, gs, &mut bindings);
    let ty = c_expr.type_of();
    let tail = anf_complex(c_expr, ty);
    bindings_to_lets(bindings, tail)
}

pub fn anf_def(def: AllocDef, gs: &mut Gensym) -> AnfDef {
    match def {
        AllocDef::ValDef(name, opty, alloc_expr) => {
            let anf_expr = to_anf(alloc_expr, gs);
            AnfDef::ValDef(name, opty, anf_expr)
        },
        AllocDef::FunDef(name, params, ret_ty, alloc_expr) => {
            let anf_expr = to_anf(alloc_expr, gs);
            AnfDef::FunDef(name, params, ret_ty, anf_expr)
        },
    }
}

pub fn anf_program(prog: AllocProgram) -> AnfProgram {
    let mut defs = Vec::with_capacity(prog.defs.len());
    let mut gs = Gensym::new();
    for def in prog.defs {
        defs.push(anf_def(def, &mut gs));
    }
    let main = to_anf(prog.main, &mut gs);
    AnfProgram { defs, main }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::expose_allocation::{
        AllocDef, AllocExpr, AllocProgram, GlobalValue, alloc_app, alloc_bin_op, alloc_bool,
        alloc_float, alloc_fun_ref, alloc_if_else, alloc_int, alloc_let, alloc_prim_io, 
        alloc_tuple_proj, alloc_tuple_set, alloc_unary, alloc_unit, alloc_var, allocate,
        allocate_closure, collect, global_freeptr, global_fromspace_end, global_value,
    };
    use crate::gensym::Gensym;
    use crate::syntax::{
        BinOp, HasType, PrimIO, Type, UnaryOp, ty_arrow, ty_bool, ty_int, ty_unit,
    };

    fn run(expr: AllocExpr) -> AnfExpr {
        to_anf(expr, &mut Gensym::new())
    }

    fn complex(c: CompExpr) -> AnfExpr {
        let ty = c.type_of();
        anf_complex(c, ty)
    }

    fn atom(a: AExpr) -> AnfExpr {
        let ty = a.type_of();
        anf_complex(c_atom(a, ty.clone()), ty)
    }

    #[test]
    fn atoms_pass_through() {
        assert_eq!(run(alloc_unit()), atom(a_unit()));
        assert_eq!(run(alloc_bool(true)), atom(a_bool(true)));
        assert_eq!(run(alloc_int(42)), atom(a_int(42)));
        assert_eq!(run(alloc_float(1.5)), atom(a_float(1.5)));
        assert_eq!(run(alloc_var("x", ty_int())), atom(a_var("x", ty_int())));
    }

    #[test]
    fn nested_binop_flattens_operands() {
        // (1 + 2) * (3 + 4)
        // --->
        // let $0 = 1 + 2 in
        // let $1 = 3 + 4 in
        // $0 * $1
        let e = alloc_bin_op(
            BinOp::Mul,
            alloc_bin_op(BinOp::Add, alloc_int(1), alloc_int(2), ty_int()),
            alloc_bin_op(BinOp::Add, alloc_int(3), alloc_int(4), ty_int()),
            ty_int(),
        );

        assert_eq!(
            run(e),
            anf_let(
                "$0",
                c_bin_op(BinOp::Add, a_int(1), a_int(2), ty_int()),
                anf_let(
                    "$1",
                    c_bin_op(BinOp::Add, a_int(3), a_int(4), ty_int()),
                    complex(c_bin_op(
                        BinOp::Mul,
                        a_var("$0", ty_int()),
                        a_var("$1", ty_int()),
                        ty_int(),
                    )),
                    ty_int(),
                ),
                ty_int(),
            )
        );
    }

    #[test]
    fn let_simple_no_extra_binding() {
        // let x = 1 + 2 in x
        // --->
        // let x = 1 + 2 in x
        let e = alloc_let(
            "x",
            ty_int(),
            alloc_bin_op(BinOp::Add, alloc_int(1), alloc_int(2), ty_int()),
            alloc_var("x", ty_int()),
            ty_int(),
        );

        assert_eq!(
            run(e),
            anf_let(
                "x",
                c_bin_op(BinOp::Add, a_int(1), a_int(2), ty_int()),
                atom(a_var("x", ty_int())),
                ty_int(),
            )
        );
    }

    #[test]
    fn let_nested_inside_binop() {
        // 1 + (let x = 2 in x + 1)
        // --->
        // let x = 2 in
        // let $0 = x + 1 in
        // 1 + $0
        let e = alloc_bin_op(
            BinOp::Add,
            alloc_int(1),
            alloc_let(
                "x",
                ty_int(),
                alloc_int(2),
                alloc_bin_op(BinOp::Add, alloc_var("x", ty_int()), alloc_int(1), ty_int()),
                ty_int(),
            ),
            ty_int(),
        );

        assert_eq!(
            run(e),
            anf_let(
                "x",
                c_atom(a_int(2), ty_int()),
                anf_let(
                    "$0",
                    c_bin_op(BinOp::Add, a_var("x", ty_int()), a_int(1), ty_int()),
                    complex(c_bin_op(BinOp::Add, a_int(1), a_var("$0", ty_int()), ty_int())),
                    ty_int(),
                ),
                ty_int(),
            )
        );
    }

    #[test]
    fn if_condition_gets_bound() {
        // if 1 < 2 then 1 else 2
        // --->
        // let $0 = 1 < 2 in
        // if $0 then 1 else 2
        let e = alloc_if_else(
            alloc_bin_op(BinOp::Lt, alloc_int(1), alloc_int(2), ty_bool()),
            alloc_int(1),
            alloc_int(2),
            ty_int(),
        );

        assert_eq!(
            run(e),
            anf_let(
                "$0",
                c_bin_op(BinOp::Lt, a_int(1), a_int(2), ty_bool()),
                complex(c_if(a_var("$0", ty_bool()), atom(a_int(1)), atom(a_int(2)), ty_int(),)),
                ty_int(),
            )
        );
    }

    #[test]
    fn if_branches_have_independent_bindings() {
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
        //         in $1 + 4
        let e = alloc_if_else(
            alloc_bool(true),
            alloc_bin_op(
                BinOp::Add,
                alloc_int(1),
                alloc_let(
                    "x",
                    ty_int(),
                    alloc_int(3),
                    alloc_bin_op(
                        BinOp::Mul,
                        alloc_var("x", ty_int()),
                        alloc_var("x", ty_int()),
                        ty_int(),
                    ),
                    ty_int(),
                ),
                ty_int(),
            ),
            alloc_bin_op(
                BinOp::Add,
                alloc_let(
                    "x",
                    ty_int(),
                    alloc_int(5),
                    alloc_bin_op(BinOp::Mul, alloc_var("x", ty_int()), alloc_int(3), ty_int()),
                    ty_int(),
                ),
                alloc_int(4),
                ty_int(),
            ),
            ty_int(),
        );

        assert_eq!(
            run(e),
            complex(c_if(
                a_bool(true),
                anf_let(
                    "x",
                    c_atom(a_int(3), ty_int()),
                    anf_let(
                        "$0",
                        c_bin_op(BinOp::Mul, a_var("x", ty_int()), a_var("x", ty_int()), ty_int()),
                        complex(c_bin_op(BinOp::Add, a_int(1), a_var("$0", ty_int()), ty_int())),
                        ty_int(),
                    ),
                    ty_int(),
                ),
                anf_let(
                    "x",
                    c_atom(a_int(5), ty_int()),
                    anf_let(
                        "$1",
                        c_bin_op(BinOp::Mul, a_var("x", ty_int()), a_int(3), ty_int()),
                        complex(c_bin_op(BinOp::Add, a_var("$1", ty_int()), a_int(4), ty_int())),
                        ty_int(),
                    ),
                    ty_int(),
                ),
                ty_int(),
            ))
        );
    }

    #[test]
    fn app_with_complex_arg() {
        // f (1 + 2) 3
        // --->
        // let $0 = 1 + 2 in
        // f $0 3
        let fn_ty = ty_arrow(ty_int(), ty_arrow(ty_int(), ty_int()));
        let e = alloc_app(
            alloc_var("f", fn_ty.clone()),
            vec![
                alloc_bin_op(BinOp::Add, alloc_int(1), alloc_int(2), ty_int()),
                alloc_int(3),
            ],
            ty_int(),
        );

        assert_eq!(
            run(e),
            anf_let(
                "$0",
                c_bin_op(BinOp::Add, a_int(1), a_int(2), ty_int()),
                complex(c_app(a_var("f", fn_ty), vec![a_var("$0", ty_int()), a_int(3)], ty_int(),)),
                ty_int(),
            )
        );
    }

    #[test]
    fn let_body_is_var_no_extra_binding() {
        // 1 + (let y = 2 in y)
        // --->
        // let y = 2 in
        // 1 + y
        let e = alloc_bin_op(
            BinOp::Add,
            alloc_int(1),
            alloc_let("y", ty_int(), alloc_int(2), alloc_var("y", ty_int()), ty_int()),
            ty_int(),
        );

        assert_eq!(
            run(e),
            anf_let(
                "y",
                c_atom(a_int(2), ty_int()),
                complex(c_bin_op(BinOp::Add, a_int(1), a_var("y", ty_int()), ty_int())),
                ty_int(),
            )
        );
    }

    #[test]
    fn nested_if_predicate() {
        // let x = 10 in
        // let y = 12 in
        // if if x < 1 then x == 0 else x == 2
        // then y + 2
        // else y + 10
        //
        // --->
        // let x = 10 in
        // let y = 12 in
        // let $0 = x < 1 in
        // let $1 = if $0 then x == 0 else x == 2 in
        // if $1 then y + 2 else y + 10
        let e = alloc_let(
            "x",
            ty_int(),
            alloc_int(10),
            alloc_let(
                "y",
                ty_int(),
                alloc_int(12),
                alloc_if_else(
                    alloc_if_else(
                        alloc_bin_op(BinOp::Lt, alloc_var("x", ty_int()), alloc_int(1), ty_bool()),
                        alloc_bin_op(BinOp::Eq, alloc_var("x", ty_int()), alloc_int(0), ty_bool()),
                        alloc_bin_op(BinOp::Eq, alloc_var("x", ty_int()), alloc_int(2), ty_bool()),
                        ty_bool(),
                    ),
                    alloc_bin_op(BinOp::Add, alloc_var("y", ty_int()), alloc_int(2), ty_int()),
                    alloc_bin_op(BinOp::Add, alloc_var("y", ty_int()), alloc_int(10), ty_int()),
                    ty_int(),
                ),
                ty_int(),
            ),
            ty_int(),
        );

        assert_eq!(
            run(e),
            anf_let(
                "x",
                c_atom(a_int(10), ty_int()),
                anf_let(
                    "y",
                    c_atom(a_int(12), ty_int()),
                    anf_let(
                        "$0",
                        c_bin_op(BinOp::Lt, a_var("x", ty_int()), a_int(1), ty_bool()),
                        anf_let(
                            "$1",
                            c_if(
                                a_var("$0", ty_bool()),
                                complex(c_bin_op(
                                    BinOp::Eq,
                                    a_var("x", ty_int()),
                                    a_int(0),
                                    ty_bool(),
                                )),
                                complex(c_bin_op(
                                    BinOp::Eq,
                                    a_var("x", ty_int()),
                                    a_int(2),
                                    ty_bool(),
                                )),
                                ty_bool(),
                            ),
                            complex(c_if(
                                a_var("$1", ty_bool()),
                                complex(c_bin_op(
                                    BinOp::Add,
                                    a_var("y", ty_int()),
                                    a_int(2),
                                    ty_int(),
                                )),
                                complex(c_bin_op(
                                    BinOp::Add,
                                    a_var("y", ty_int()),
                                    a_int(10),
                                    ty_int(),
                                )),
                                ty_int(),
                            )),
                            ty_int(),
                        ),
                        ty_int(),
                    ),
                    ty_int(),
                ),
                ty_int(),
            )
        );
    }

    #[test]
    fn nested_if_branch_then() {
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
        let e = alloc_let(
            "x",
            ty_int(),
            alloc_prim_io(PrimIO::ReadInt, None, ty_int()),
            alloc_let(
                "y",
                ty_int(),
                alloc_prim_io(PrimIO::ReadInt, None, ty_int()),
                alloc_if_else(
                    alloc_bin_op(
                        BinOp::Lt,
                        alloc_var("x", ty_int()),
                        alloc_var("y", ty_int()),
                        ty_bool(),
                    ),
                    alloc_if_else(
                        alloc_bin_op(
                            BinOp::Lt,
                            alloc_var("y", ty_int()),
                            alloc_int(100),
                            ty_bool(),
                        ),
                        alloc_bin_op(
                            BinOp::Add,
                            alloc_var("y", ty_int()),
                            alloc_int(100),
                            ty_int(),
                        ),
                        alloc_bin_op(
                            BinOp::Add,
                            alloc_var("y", ty_int()),
                            alloc_var("x", ty_int()),
                            ty_int(),
                        ),
                        ty_int(),
                    ),
                    alloc_bin_op(BinOp::Add, alloc_var("y", ty_int()), alloc_int(10), ty_int()),
                    ty_int(),
                ),
                ty_int(),
            ),
            ty_int(),
        );

        assert_eq!(
            run(e),
            anf_let(
                "x",
                c_prim_io(PrimIO::ReadInt, None, ty_int()),
                anf_let(
                    "y",
                    c_prim_io(PrimIO::ReadInt, None, ty_int()),
                    anf_let(
                        "$0",
                        c_bin_op(BinOp::Lt, a_var("x", ty_int()), a_var("y", ty_int()), ty_bool(),),
                        complex(c_if(
                            a_var("$0", ty_bool()),
                            anf_let(
                                "$1",
                                c_bin_op(BinOp::Lt, a_var("y", ty_int()), a_int(100), ty_bool(),),
                                complex(c_if(
                                    a_var("$1", ty_bool()),
                                    complex(c_bin_op(
                                        BinOp::Add,
                                        a_var("y", ty_int()),
                                        a_int(100),
                                        ty_int(),
                                    )),
                                    complex(c_bin_op(
                                        BinOp::Add,
                                        a_var("y", ty_int()),
                                        a_var("x", ty_int()),
                                        ty_int(),
                                    )),
                                    ty_int(),
                                )),
                                ty_int(),
                            ),
                            complex(c_bin_op(
                                BinOp::Add,
                                a_var("y", ty_int()),
                                a_int(10),
                                ty_int(),
                            )),
                            ty_int(),
                        )),
                        ty_int(),
                    ),
                    ty_int(),
                ),
                ty_int(),
            )
        );
    }

    #[test]
    fn nested_if_branch_else() {
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
        let e = alloc_let(
            "x",
            ty_int(),
            alloc_prim_io(PrimIO::ReadInt, None, ty_int()),
            alloc_let(
                "y",
                ty_int(),
                alloc_prim_io(PrimIO::ReadInt, None, ty_int()),
                alloc_if_else(
                    alloc_bin_op(
                        BinOp::Eq,
                        alloc_var("x", ty_int()),
                        alloc_var("y", ty_int()),
                        ty_bool(),
                    ),
                    alloc_bin_op(
                        BinOp::Add,
                        alloc_var("x", ty_int()),
                        alloc_var("y", ty_int()),
                        ty_int(),
                    ),
                    alloc_if_else(
                        alloc_bin_op(BinOp::Lt, alloc_var("x", ty_int()), alloc_int(42), ty_bool()),
                        alloc_if_else(
                            alloc_bin_op(
                                BinOp::Gt,
                                alloc_var("y", ty_int()),
                                alloc_int(10),
                                ty_bool(),
                            ),
                            alloc_bin_op(
                                BinOp::Add,
                                alloc_bin_op(
                                    BinOp::Add,
                                    alloc_var("x", ty_int()),
                                    alloc_int(10),
                                    ty_int(),
                                ),
                                alloc_var("y", ty_int()),
                                ty_int(),
                            ),
                            alloc_bin_op(
                                BinOp::Sub,
                                alloc_var("x", ty_int()),
                                alloc_int(100),
                                ty_int(),
                            ),
                            ty_int(),
                        ),
                        alloc_bin_op(BinOp::Add, alloc_var("x", ty_int()), alloc_int(42), ty_int()),
                        ty_int(),
                    ),
                    ty_int(),
                ),
                ty_int(),
            ),
            ty_int(),
        );

        assert_eq!(
            run(e),
            anf_let(
                "x",
                c_prim_io(PrimIO::ReadInt, None, ty_int()),
                anf_let(
                    "y",
                    c_prim_io(PrimIO::ReadInt, None, ty_int()),
                    anf_let(
                        "$0",
                        c_bin_op(BinOp::Eq, a_var("x", ty_int()), a_var("y", ty_int()), ty_bool(),),
                        complex(c_if(
                            a_var("$0", ty_bool()),
                            complex(c_bin_op(
                                BinOp::Add,
                                a_var("x", ty_int()),
                                a_var("y", ty_int()),
                                ty_int(),
                            )),
                            anf_let(
                                "$1",
                                c_bin_op(BinOp::Lt, a_var("x", ty_int()), a_int(42), ty_bool(),),
                                complex(c_if(
                                    a_var("$1", ty_bool()),
                                    anf_let(
                                        "$2",
                                        c_bin_op(
                                            BinOp::Gt,
                                            a_var("y", ty_int()),
                                            a_int(10),
                                            ty_bool(),
                                        ),
                                        complex(c_if(
                                            a_var("$2", ty_bool()),
                                            anf_let(
                                                "$3",
                                                c_bin_op(
                                                    BinOp::Add,
                                                    a_var("x", ty_int()),
                                                    a_int(10),
                                                    ty_int(),
                                                ),
                                                complex(c_bin_op(
                                                    BinOp::Add,
                                                    a_var("$3", ty_int()),
                                                    a_var("y", ty_int()),
                                                    ty_int(),
                                                )),
                                                ty_int(),
                                            ),
                                            complex(c_bin_op(
                                                BinOp::Sub,
                                                a_var("x", ty_int()),
                                                a_int(100),
                                                ty_int(),
                                            )),
                                            ty_int(),
                                        )),
                                        ty_int(),
                                    ),
                                    complex(c_bin_op(
                                        BinOp::Add,
                                        a_var("x", ty_int()),
                                        a_int(42),
                                        ty_int(),
                                    )),
                                    ty_int(),
                                )),
                                ty_int(),
                            ),
                            ty_int(),
                        )),
                        ty_int(),
                    ),
                    ty_int(),
                ),
                ty_int(),
            )
        );
    }

    #[test]
    fn prim_io_print_complex_arg() {
        // print_int (1 + 2)
        // --->
        // let $0 = 1 + 2 in
        // print_int $0
        let e = alloc_prim_io(
            PrimIO::PrintInt,
            Some(alloc_bin_op(BinOp::Add, alloc_int(1), alloc_int(2), ty_int())),
            ty_unit(),
        );

        assert_eq!(
            run(e),
            anf_let(
                "$0",
                c_bin_op(BinOp::Add, a_int(1), a_int(2), ty_int()),
                complex(c_prim_io(PrimIO::PrintInt, Some(a_var("$0", ty_int())), ty_unit(),)),
                ty_unit(),
            )
        );
    }

    #[test]
    fn prim_io_read_none() {
        // read_int ()
        // --->
        // read_int ()
        let e = alloc_prim_io(PrimIO::ReadInt, None, ty_int());
        assert_eq!(run(e), complex(c_prim_io(PrimIO::ReadInt, None, ty_int())));
    }

    #[test]
    fn unary_op_normalizes_operand() {
        assert_eq!(
            run(alloc_unary(UnaryOp::Neg, alloc_int(3), ty_int())),
            complex(c_unary(UnaryOp::Neg, a_int(3), ty_int()))
        );
        assert_eq!(
            run(alloc_unary(UnaryOp::Not, alloc_bool(true), ty_bool())),
            complex(c_unary(UnaryOp::Not, a_bool(true), ty_bool()))
        );

        let e = alloc_unary(
            UnaryOp::Neg,
            alloc_bin_op(BinOp::Add, alloc_int(1), alloc_int(2), ty_int()),
            ty_int(),
        );
        assert_eq!(
            run(e),
            anf_let(
                "$0",
                c_bin_op(BinOp::Add, a_int(1), a_int(2), ty_int()),
                complex(c_unary(UnaryOp::Neg, a_var("$0", ty_int()), ty_int())),
                ty_int(),
            )
        );
    }

    #[test]
    fn tuple_proj_normalizes_tuple_operand() {
        let tuple_ty = Type::Tuple(vec![Type::Int, Type::Bool]);
        let e = alloc_tuple_proj(alloc_var("t", tuple_ty.clone()), 1, ty_bool());
        assert_eq!(run(e), complex(c_tuple_proj(a_var("t", tuple_ty), 1, ty_bool())));

        let e = alloc_tuple_proj(
            alloc_bin_op(BinOp::Add, alloc_int(1), alloc_int(2), ty_int()),
            0,
            ty_bool(),
        );
        assert_eq!(
            run(e),
            anf_let(
                "$0",
                c_bin_op(BinOp::Add, a_int(1), a_int(2), ty_int()),
                complex(c_tuple_proj(a_var("$0", ty_int()), 0, ty_bool())),
                ty_bool(),
            )
        );
    }

    #[test]
    fn tuple_set_binds_complex_element() {
        let tuple_ty = Type::Tuple(vec![Type::Int, Type::Int]);
        let e = alloc_tuple_set(alloc_var("t", tuple_ty.clone()), alloc_int(7), 0);
        assert_eq!(run(e), complex(c_tuple_set(a_var("t", tuple_ty), a_int(7), 0)));

        let tuple_ty = Type::Tuple(vec![Type::Int, Type::Int]);
        let e = alloc_tuple_set(
            alloc_var("t", tuple_ty.clone()),
            alloc_bin_op(BinOp::Add, alloc_int(1), alloc_int(2), ty_int()),
            1,
        );
        assert_eq!(
            run(e),
            anf_let(
                "$0",
                c_bin_op(BinOp::Add, a_int(1), a_int(2), ty_int()),
                complex(c_tuple_set(a_var("t", tuple_ty), a_var("$0", ty_int()), 1)),
                ty_int(),
            )
        );
    }

    #[test]
    fn allocation_constructs_pass_through() {
        assert_eq!(run(allocate(8, ty_int())), complex(c_allocate(8, ty_int())));
        assert_eq!(
            run(allocate_closure(16, 2, ty_int())),
            complex(c_allocate_closure(16, 2, ty_int()))
        );
        assert_eq!(run(alloc_fun_ref("f", 2, ty_int())), complex(c_fun_ref("f", 2, ty_int())));
        assert_eq!(
            run(global_value(GlobalValue::FreePtr)),
            complex(c_global_value(GlobalValue::FreePtr))
        );
        assert_eq!(run(global_freeptr()), complex(c_free_ptr()));
        assert_eq!(run(global_fromspace_end()), complex(c_fromspace_end()));
        assert_eq!(run(collect(32)), complex(c_collect(32)));
    }

    #[test]
    fn anf_def_normalizes_val_and_fun() {
        let val = AllocDef::ValDef(
            "answer".to_string(),
            Some(ty_int()),
            alloc_bin_op(BinOp::Add, alloc_int(20), alloc_int(22), ty_int()),
        );
        assert_eq!(
            anf_def(val, &mut Gensym::new()),
            AnfDef::ValDef(
                "answer".to_string(),
                Some(ty_int()),
                complex(c_bin_op(BinOp::Add, a_int(20), a_int(22), ty_int())),
            )
        );

        let fun = AllocDef::FunDef(
            "add".to_string(),
            vec![("a".to_string(), ty_int()), ("b".to_string(), ty_int())],
            ty_int(),
            alloc_bin_op(BinOp::Add, alloc_var("a", ty_int()), alloc_var("b", ty_int()), ty_int()),
        );
        assert_eq!(
            anf_def(fun, &mut Gensym::new()),
            AnfDef::FunDef(
                "add".to_string(),
                vec![("a".to_string(), ty_int()), ("b".to_string(), ty_int())],
                ty_int(),
                complex(
                    c_bin_op(BinOp::Add, a_var("a", ty_int()), a_var("b", ty_int()), ty_int(),)
                ),
            )
        );
    }

    #[test]
    fn anf_program_normalizes_defs_then_main() {
        let fn_ty = ty_arrow(ty_int(), ty_int());
        let prog = AllocProgram {
            defs: vec![AllocDef::FunDef(
                "id".to_string(),
                vec![("x".to_string(), ty_int())],
                ty_int(),
                alloc_bin_op(
                    BinOp::Add,
                    alloc_var("x", ty_int()),
                    alloc_bin_op(BinOp::Add, alloc_int(1), alloc_int(2), ty_int()),
                    ty_int(),
                ),
            )],
            main: alloc_app(
                alloc_var("id", fn_ty.clone()),
                vec![alloc_bin_op(BinOp::Add, alloc_int(2), alloc_int(3), ty_int())],
                ty_int(),
            ),
        };

        assert_eq!(
            anf_program(prog),
            AnfProgram {
                defs: vec![AnfDef::FunDef(
                    "id".to_string(),
                    vec![("x".to_string(), ty_int())],
                    ty_int(),
                    anf_let(
                        "$0",
                        c_bin_op(BinOp::Add, a_int(1), a_int(2), ty_int()),
                        complex(c_bin_op(
                            BinOp::Add,
                            a_var("x", ty_int()),
                            a_var("$0", ty_int()),
                            ty_int(),
                        )),
                        ty_int(),
                    ),
                )],
                main: anf_let(
                    "$1",
                    c_bin_op(BinOp::Add, a_int(2), a_int(3), ty_int()),
                    complex(c_app(a_var("id", fn_ty), vec![a_var("$1", ty_int())], ty_int(),)),
                    ty_int(),
                ),
            }
        );
    }
}
