use std::collections::HashMap;

use crate::{
    a_normal_form::{AExpr, AnfExpr, CompExpr},
    syntax::{BinOp, Ident, Type, UnaryOp},
};

#[derive(Debug, Clone, PartialEq)]
pub enum CAtom {
    Unit,
    Bool(bool),
    Int(i64),
    Float(f64),
    Var(Ident, Type),
}

#[derive(Debug, Clone, PartialEq)]
pub enum CExpr {
    Atom(CAtom),
    BinOp(BinOp, CAtom, CAtom),
    UnaryOp(UnaryOp, CAtom),
    Call(CAtom, Vec<CAtom>), // 非尾调用
}

#[derive(Debug, Clone, PartialEq)]
pub enum CStmt {
    Assign(Ident, CExpr, Type),
}

#[derive(Debug, Clone, PartialEq)]
pub enum CTail {
    Return(CExpr),
    TailCall(CAtom, Vec<CAtom>),
    Seq(CStmt, Box<CTail>),
    If(CAtom, Box<CTail>, Box<CTail>),
}

pub fn explicate_assign(expr: AnfExpr, name: &str, cont: CTail) -> CTail {
    match expr {
        AnfExpr::Complex(cexpr) => match cexpr {
            CompExpr::Atom(aexpr) => {
                let cexpr = CExpr::Atom(aexpr_to_catom(aexpr));
                let ty = type_of_cexpr(&cexpr);
                let stmt = CStmt::Assign(name.to_string(), cexpr, ty);
                CTail::Seq(stmt, Box::new(cont))
            }
            CompExpr::BinOp(op, left, right) => {
                let cexpr = CExpr::BinOp(op, aexpr_to_catom(left), aexpr_to_catom(right));
                let ty = type_of_cexpr(&cexpr);
                let stmt = CStmt::Assign(name.to_string(), cexpr, ty);
                CTail::Seq(stmt, Box::new(cont))
            }
            CompExpr::UnaryOp(op, operand) => {
                let cexpr = CExpr::UnaryOp(op, aexpr_to_catom(operand));
                let ty = type_of_cexpr(&cexpr);
                let stmt = CStmt::Assign(name.to_string(), cexpr, ty);
                CTail::Seq(stmt, Box::new(cont))
            }
            CompExpr::App(func, args) => {
                let cexpr = CExpr::Call(
                    aexpr_to_catom(func),
                    args.into_iter().map(aexpr_to_catom).collect(),
                );
                let ty = type_of_cexpr(&cexpr);
                let stmt = CStmt::Assign(name.to_string(), cexpr, ty);
                CTail::Seq(stmt, Box::new(cont))
            }
            CompExpr::If(cond, thn, els) => {
                let cond_catom = aexpr_to_catom(cond);
                let then_tail = explicate_assign(*thn, name, cont.clone());
                let else_tail = explicate_assign(*els, name, cont);
                CTail::If(cond_catom, Box::new(then_tail), Box::new(else_tail))
            }
            CompExpr::Lambda(params, _, body) => todo!(),
        },
        AnfExpr::Let(name1, cexpr, body) => {
            let inner_cont = explicate_assign(*body, name, cont);
            explicate_assign(AnfExpr::Complex(cexpr), &name1, inner_cont)
        }
        AnfExpr::LetRec(fname, fparams, _, fbody, body) => todo!(),
    }
}

fn type_of_catom(atom: &CAtom) -> Type {
    match atom {
        CAtom::Unit => Type::Unit,
        CAtom::Bool(_) => Type::Bool,
        CAtom::Int(_) => Type::Int,
        CAtom::Float(_) => Type::Float,
        CAtom::Var(_, ty) => (*ty).clone(),
    }
}

fn type_of_cexpr(cexpr: &CExpr) -> Type {
    match cexpr {
        CExpr::Atom(catom) => type_of_catom(catom),
        CExpr::BinOp(op, left, _) => match op {
            BinOp::Add => type_of_catom(left),
            BinOp::Sub => type_of_catom(left),
            BinOp::Mul => type_of_catom(left),
            BinOp::Div => type_of_catom(left),
            BinOp::And => Type::Bool,
            BinOp::Or => Type::Bool,
            BinOp::Eq => Type::Bool,
            BinOp::Neq => type_of_catom(left),
            BinOp::Lt => Type::Bool,
            BinOp::Gt => Type::Bool,
            BinOp::Leq => Type::Bool,
            BinOp::Geq => Type::Bool,
        },
        CExpr::UnaryOp(op, catom) => match op {
            UnaryOp::Neg => type_of_catom(catom),
            UnaryOp::Not => Type::Bool,
        },
        CExpr::Call(func, args) => {
            let mut curr_ty = type_of_catom(func);
            for arg in args {
                let arg_ty = type_of_catom(arg);
                match curr_ty {
                    Type::Arrow(input_ty, output_ty) => {
                        if *input_ty != arg_ty {
                            unreachable!()
                        }
                        curr_ty = *output_ty
                    }
                    _ => unreachable!(),
                }
            }
            curr_ty
        }
    }
}
fn aexpr_to_catom(aexpr: AExpr) -> CAtom {
    match aexpr {
        AExpr::Unit => CAtom::Unit,
        AExpr::Bool(b) => CAtom::Bool(b),
        AExpr::Int(i) => CAtom::Int(i),
        AExpr::Float(f) => CAtom::Float(f),
        AExpr::Var(name, ty) => CAtom::Var(name, ty),
    }
}
pub fn explicate_tail(anf: AnfExpr) -> CTail {
    match anf {
        AnfExpr::Complex(cexpr) => match cexpr {
            CompExpr::Atom(aexpr) => CTail::Return(CExpr::Atom(aexpr_to_catom(aexpr))),
            CompExpr::BinOp(op, left, right) => {
                let cleft = aexpr_to_catom(left);
                let cright = aexpr_to_catom(right);
                let cexpr = CExpr::BinOp(op, cleft, cright);
                CTail::Return(cexpr)
            }
            CompExpr::UnaryOp(op, aexpr) => {
                let catom = aexpr_to_catom(aexpr);
                let cexpr = CExpr::UnaryOp(op, catom);
                CTail::Return(cexpr)
            }
            CompExpr::App(func, args) => {
                let func_catom = aexpr_to_catom(func);
                let args_catom = args.into_iter().map(aexpr_to_catom).collect();
                CTail::TailCall(func_catom, args_catom)
            }
            CompExpr::If(cond, thn, els) => {
                let cond_catom = aexpr_to_catom(cond);
                let then_tail = explicate_tail(*thn);
                let else_tail = explicate_tail(*els);
                CTail::If(cond_catom, Box::new(then_tail), Box::new(else_tail))
            }
            CompExpr::Lambda(args, _, body) => todo!(),
        },
        AnfExpr::Let(name, rhs, body) => {
            let tail = explicate_tail(*body);
            explicate_assign(AnfExpr::Complex(rhs), &name, tail)
        }
        AnfExpr::LetRec(fname, fargs, _, fbody, body) => todo!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::a_normal_form::{AExpr, AnfExpr, CompExpr};
    use crate::syntax::BinOp;
    use crate::syntax::UnaryOp;
    use rand::seq::IndexedRandom;

    fn anf_var(name: &str, ty: Type) -> AExpr {
        AExpr::Var(name.to_string(), ty)
    }

    fn random_type_with_depth(depth: usize) -> Type {
        let mut rng = rand::rng();

        // 深度为 0 时，只生成叶子类型（不含 Arrow）
        if depth == 0 {
            let leaf_variants: &[fn() -> Type] = &[
                || Type::Unit,
                || Type::Bool,
                || Type::Float,
                || Type::Int,
                || Type::Var("$dummy".to_string()),
            ];
            return leaf_variants.choose(&mut rng).unwrap()();
        }

        let all_variants: &[fn(usize) -> Type] = &[
            |_| Type::Unit,
            |_| Type::Bool,
            |_| Type::Float,
            |_| Type::Int,
            |d| {
                Type::Arrow(
                    Box::new(random_type_with_depth(d - 1)),
                    Box::new(random_type_with_depth(d - 1)),
                )
            },
            |_| Type::Var("$dummy".to_string()),
        ];
        all_variants.choose(&mut rng).unwrap()(depth)
    }

    fn random_type() -> Type {
        random_type_with_depth(5)
    }

    fn anf_int(n: i64) -> AExpr {
        AExpr::Int(n)
    }

    fn anf_bool(b: bool) -> AExpr {
        AExpr::Bool(b)
    }

    fn anf_atom_to_comp(a: AExpr) -> CompExpr {
        CompExpr::Atom(a)
    }

    fn anf_atom_to_anf(a: AExpr) -> AnfExpr {
        AnfExpr::Complex(CompExpr::Atom(a))
    }

    fn anf_let(name: &str, rhs: CompExpr, body: AnfExpr) -> AnfExpr {
        AnfExpr::Let(name.to_string(), rhs, Box::new(body))
    }

    fn c_var(name: &str, ty: Type) -> CAtom {
        CAtom::Var(name.to_string(), ty)
    }

    fn c_int(n: i64) -> CAtom {
        CAtom::Int(n)
    }

    fn c_bool(b: bool) -> CAtom {
        CAtom::Bool(b)
    }

    fn c_assign(name: &str, e: CExpr, cont: CTail) -> CTail {
        let ty = type_of_cexpr(&e);
        CTail::Seq(CStmt::Assign(name.to_string(), e, ty), Box::new(cont))
    }

    #[test]
    fn tail_atom_int() {
        // 5
        // --->
        // Return(Atom(Int(5)))

        let input = anf_atom_to_anf(anf_int(5));
        let expected = CTail::Return(CExpr::Atom(CAtom::Int(5)));
        assert_eq!(explicate_tail(input), expected);
    }

    #[test]
    fn tail_atom_var() {
        // x
        // --->
        // Return(Atom(Var(x)))

        let ty = random_type();
        let input = anf_atom_to_anf(anf_var("x", ty.clone()));
        let expected = CTail::Return(CExpr::Atom(c_var("x", ty)));
        assert_eq!(explicate_tail(input), expected);
    }

    #[test]
    fn tail_binop() {
        // x + 1
        // --->
        // Return(BinOp(Add, x, 1))

        let ty = Type::Int;
        let input = AnfExpr::Complex(CompExpr::BinOp(
            BinOp::Add,
            anf_var("x", ty.clone()),
            anf_int(1),
        ));
        let expected = CTail::Return(CExpr::BinOp(BinOp::Add, c_var("x", ty), c_int(1)));
        assert_eq!(explicate_tail(input), expected);
    }

    #[test]
    fn tail_unaryop() {
        // -x
        // -->
        // Return(UnaryOp(Neg, x))

        let ty = Type::Float;
        let input = AnfExpr::Complex(CompExpr::UnaryOp(UnaryOp::Neg, anf_var("x", ty.clone())));
        let expected = CTail::Return(CExpr::UnaryOp(UnaryOp::Neg, c_var("x", ty)));
        assert_eq!(explicate_tail(input), expected);
    }

    #[test]
    fn tail_app_becomes_tailcall() {
        // f(x, y)  在 tail 位置
        // -->
        // TailCall(f, [x, y])

        let x_ty = random_type();
        let y_ty = random_type();
        let fn_ty = Type::Arrow(
            Box::new(x_ty.clone()),
            Box::new(Type::Arrow(Box::new(y_ty.clone()), Box::new(Type::Unit))),
        );
        let input = AnfExpr::Complex(CompExpr::App(
            anf_var("f", fn_ty.clone()),
            vec![anf_var("x", x_ty.clone()), anf_var("y", y_ty.clone())],
        ));
        let expected = CTail::TailCall(c_var("f", fn_ty), vec![c_var("x", x_ty), c_var("y", y_ty)]);
        assert_eq!(explicate_tail(input), expected);
    }

    #[test]
    fn tail_if_both_branches_are_tail() {
        // if c then 1 else 2
        // -->
        // If(c, Return(1), Return(2))

        let input = AnfExpr::Complex(CompExpr::If(
            anf_var("c", Type::Bool),
            Box::new(anf_atom_to_anf(anf_int(1))),
            Box::new(anf_atom_to_anf(anf_int(2))),
        ));
        let expected = CTail::If(
            c_var("c", Type::Bool),
            Box::new(CTail::Return(CExpr::Atom(c_int(1)))),
            Box::new(CTail::Return(CExpr::Atom(c_int(2)))),
        );
        assert_eq!(explicate_tail(input), expected);
    }

    #[test]
    fn tail_let_single() {
        // let x = 1 in x
        // -->
        // Seq(Assign(x, 1), Return(x))

        let input = anf_let(
            "x",
            anf_atom_to_comp(anf_int(1)),
            anf_atom_to_anf(anf_var("x", Type::Int)),
        );
        let expected = c_assign(
            "x",
            CExpr::Atom(c_int(1)),
            CTail::Return(CExpr::Atom(c_var("x", Type::Int))),
        );
        assert_eq!(explicate_tail(input), expected);
    }

    #[test]
    fn tail_let_nested_order_is_preserved() {
        // let x = 1 in let y = x + 1 in y
        // --->
        // Seq(Assign(x,1), Seq(Assign(y, x+1), Return(y)))

        let input = anf_let(
            "x",
            anf_atom_to_comp(anf_int(1)),
            anf_let(
                "y",
                CompExpr::BinOp(BinOp::Add, anf_var("x", Type::Int), anf_int(1)),
                anf_atom_to_anf(anf_var("y", Type::Int)),
            ),
        );
        let expected = c_assign(
            "x",
            CExpr::Atom(c_int(1)),
            c_assign(
                "y",
                CExpr::BinOp(BinOp::Add, c_var("x", Type::Int), c_int(1)),
                CTail::Return(CExpr::Atom(c_var("y", Type::Int))),
            ),
        );
        assert_eq!(explicate_tail(input), expected);
    }

    #[test]
    fn tail_let_with_if_rhs() {
        // let x = (if c then 1 else 2) in x
        // --->
        // If(c, Seq(Assign(x, 1), Return(x)), Seq(Assign(x, 2), Return(x)))

        let input = anf_let(
            "x",
            CompExpr::If(
                anf_var("c", Type::Bool),
                Box::new(anf_atom_to_anf(anf_int(1))),
                Box::new(anf_atom_to_anf(anf_int(2))),
            ),
            anf_atom_to_anf(anf_var("x", Type::Int)),
        );
        let cont = CTail::Return(CExpr::Atom(c_var("x", Type::Int)));
        let expected = CTail::If(
            c_var("c", Type::Bool),
            Box::new(c_assign("x", CExpr::Atom(c_int(1)), cont.clone())),
            Box::new(c_assign("x", CExpr::Atom(c_int(2)), cont)),
        );
        assert_eq!(explicate_tail(input), expected);
    }

    // ============================================================
    // explicate_assign
    // ============================================================

    #[test]
    fn assign_atom() {
        // explicate_assign(5, "x", Return(x))
        // --->
        // Seq(Assign(x, 5), Return(x))

        let input = anf_atom_to_anf(anf_int(5));
        let cont = CTail::Return(CExpr::Atom(c_var("x", Type::Int)));
        let expected = c_assign("x", CExpr::Atom(c_int(5)), cont.clone());
        assert_eq!(explicate_assign(input, "x", cont), expected);
    }

    #[test]
    fn assign_binop() {
        // explicate_assign(a * b, "x", Return(x))
        // --->
        // Seq(Assign(x, a * b), Return(x))

        let input = AnfExpr::Complex(CompExpr::BinOp(
            BinOp::Mul,
            anf_var("a", Type::Float),
            anf_var("b", Type::Float),
        ));
        let cont = CTail::Return(CExpr::Atom(c_var("x", Type::Float)));
        let expected = c_assign(
            "x",
            CExpr::BinOp(BinOp::Mul, c_var("a", Type::Float), c_var("b", Type::Float)),
            cont.clone(),
        );
        assert_eq!(explicate_assign(input, "x", cont), expected);
    }

    #[test]
    fn assign_unaryop() {
        // explicate_assign(!a, "x", Return(x))
        // --->
        // Seq(Assign(x, !a), Return(x))

        let input = AnfExpr::Complex(CompExpr::UnaryOp(UnaryOp::Not, anf_var("a", Type::Bool)));
        let cont = CTail::Return(CExpr::Atom(c_var("x", Type::Bool)));
        let expected = c_assign(
            "x",
            CExpr::UnaryOp(UnaryOp::Not, c_var("a", Type::Bool)),
            cont.clone(),
        );
        assert_eq!(explicate_assign(input, "x", cont), expected);
    }

    #[test]
    fn assign_app_is_non_tail_call() {
        // explicate_assign(f(a, b), "x", cont)
        // --->
        // Seq(Assign(x, Call(f, [a, b])), cont)

        let a_ty = Type::Int;
        let b_ty = Type::Int;
        let result_ty = Type::Int;
        let f_ty = Type::Arrow(
            Box::new(a_ty.clone()),
            Box::new(Type::Arrow(
                Box::new(b_ty.clone()),
                Box::new(result_ty.clone()),
            )),
        );
        let input = AnfExpr::Complex(CompExpr::App(
            anf_var("f", f_ty.clone()),
            vec![anf_var("a", a_ty.clone()), anf_var("b", b_ty.clone())],
        ));
        let cont = CTail::Return(CExpr::Atom(c_var("x", result_ty.clone())));
        let expected = c_assign(
            "x",
            CExpr::Call(c_var("f", f_ty), vec![c_var("a", a_ty), c_var("b", b_ty)]),
            cont.clone(),
        );
        assert_eq!(explicate_assign(input, "x", cont), expected);
    }

    #[test]
    fn assign_if_both_branches_assign_and_share_cont() {
        // explicate_assign(if c then 1 else 2, "x", Return(x))
        // --->
        // If(c, Seq(Assign(x, 1), Return(x)),
        //       Seq(Assign(x, 2), Return(x)))

        let input = AnfExpr::Complex(CompExpr::If(
            anf_var("c", Type::Bool),
            Box::new(anf_atom_to_anf(anf_int(1))),
            Box::new(anf_atom_to_anf(anf_int(2))),
        ));
        let cont = CTail::Return(CExpr::Atom(c_var("x", Type::Int)));
        let expected = CTail::If(
            c_var("c", Type::Bool),
            Box::new(c_assign("x", CExpr::Atom(c_int(1)), cont.clone())),
            Box::new(c_assign("x", CExpr::Atom(c_int(2)), cont.clone())),
        );
        assert_eq!(explicate_assign(input, "x", cont.clone()), expected);
    }

    #[test]
    fn assign_if_nested_inside_let_rhs_and_outer_let() {
        // let y = let x = (if c then 1 else 2) in x in y
        // --->
        // If(c, Seq(Assign(x, 1), Seq(Assign(y, x), Return(y))),
        //       Seq(Assign(x, 2), Seq(Assign(y, x), Return(y))))

        let inner_let = anf_let(
            "x",
            CompExpr::If(
                anf_var("c", Type::Bool),
                Box::new(anf_atom_to_anf(anf_int(1))),
                Box::new(anf_atom_to_anf(anf_int(2))),
            ),
            anf_atom_to_anf(anf_var("x", Type::Int)),
        );

        let final_cont = CTail::Return(CExpr::Atom(c_var("y", Type::Int)));
        let result = explicate_assign(inner_let, "y", final_cont.clone());

        let expected = CTail::If(
            c_var("c", Type::Bool),
            Box::new(c_assign(
                "x",
                CExpr::Atom(c_int(1)),
                c_assign("y", CExpr::Atom(c_var("x", Type::Int)), final_cont.clone()),
            )),
            Box::new(c_assign(
                "x",
                CExpr::Atom(c_int(2)),
                c_assign("y", CExpr::Atom(c_var("x", Type::Int)), final_cont),
            )),
        );
        assert_eq!(result, expected);
    }

    #[test]
    fn assign_let_forwards_correctly() {
        // explicate_assign(let a = 1 in a, "x", Return(x))
        // --->
        // Seq(Assign(a, 1), Seq(Assign(x, a), Return(x)))

        let input = anf_let(
            "a",
            anf_atom_to_comp(anf_int(1)),
            anf_atom_to_anf(anf_var("a", Type::Int)),
        );
        let cont = CTail::Return(CExpr::Atom(c_var("x", Type::Int)));
        let expected = c_assign(
            "a",
            CExpr::Atom(c_int(1)),
            c_assign("x", CExpr::Atom(c_var("a", Type::Int)), cont.clone()),
        );
        assert_eq!(explicate_assign(input, "x", cont), expected);
    }

    #[test]
    fn assign_true_false_atoms() {
        // explicate_assign(true, "x", Return(x))
        // --->
        // Seq(Assign(x, true), Return(x))

        let input = anf_atom_to_anf(anf_bool(true));
        let cont = CTail::Return(CExpr::Atom(c_var("x", Type::Bool)));
        let expected = c_assign("x", CExpr::Atom(c_bool(true)), cont.clone());
        assert_eq!(explicate_assign(input, "x", cont), expected);
    }
}
