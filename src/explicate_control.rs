use std::collections::HashMap;

use crate::{
    a_normal_form::{AExpr, AnfExpr, CompExpr},
    syntax::{BinOp, Ident, UnaryOp},
};

#[derive(Debug, Clone, PartialEq)]
pub enum CAtom {
    Unit,
    Bool(bool),
    Int(i64),
    Float(f64),
    Var(Ident),
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
    Assign(Ident, CExpr),
}

#[derive(Debug, Clone, PartialEq)]
pub enum CTail {
    Return(CExpr),
    TailCall(CAtom, Vec<CAtom>),
    Seq(CStmt, Box<CTail>),
    If(CAtom, Box<CTail>, Box<CTail>),
}

pub struct CProgram {
    blocks: HashMap<Ident, CTail>,
}

fn explicate_assign(expr: AnfExpr, name: &str, cont: CTail) -> CTail {
    match expr {
        AnfExpr::Complex(cexpr) => match cexpr {
            CompExpr::Atom(aexpr) => {
                let cexpr = CExpr::Atom(aexpr_to_catom(aexpr));
                let stmt = CStmt::Assign(name.to_string(), cexpr);
                CTail::Seq(stmt, Box::new(cont))
            }
            CompExpr::BinOp(op, left, right) => {
                let cexpr = CExpr::BinOp(op, aexpr_to_catom(left), aexpr_to_catom(right));
                let stmt = CStmt::Assign(name.to_string(), cexpr);
                CTail::Seq(stmt, Box::new(cont))
            }
            CompExpr::UnaryOp(op, operand) => {
                let cexpr = CExpr::UnaryOp(op, aexpr_to_catom(operand));
                let stmt = CStmt::Assign(name.to_string(), cexpr);
                CTail::Seq(stmt, Box::new(cont))
            }
            CompExpr::App(func, args) => {
                let cexpr = CExpr::Call(
                    aexpr_to_catom(func),
                    args.into_iter().map(aexpr_to_catom).collect(),
                );
                let stmt = CStmt::Assign(name.to_string(), cexpr);

                CTail::Seq(stmt, Box::new(cont))
            }
            CompExpr::If(cond, thn, els) => {
                let cond_catom = aexpr_to_catom(cond);
                let then_tail = explicate_assign(*thn, name, cont.clone());
                let else_tail = explicate_assign(*els, name, cont);
                CTail::If(cond_catom, Box::new(then_tail), Box::new(else_tail))
            }
            CompExpr::Lambda(args, _, body) => todo!(),
        },
        AnfExpr::Let(name1, cexpr, body) => {
            let inner_cont = explicate_assign(*body, name, cont);
            explicate_assign(AnfExpr::Complex(cexpr), &name1, inner_cont)
        }
        AnfExpr::LetRec(fname, fargs, _, fbody, body) => todo!(),
    }
}
fn aexpr_to_catom(aexpr: AExpr) -> CAtom {
    match aexpr {
        AExpr::Unit => CAtom::Unit,
        AExpr::Bool(b) => CAtom::Bool(b),
        AExpr::Int(i) => CAtom::Int(i),
        AExpr::Float(f) => CAtom::Float(f),
        AExpr::Var(name) => CAtom::Var(name),
    }
}
fn explicate_tail(anf: AnfExpr) -> CTail {
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

    fn anf_var(name: &str) -> AExpr {
        AExpr::Var(name.to_string())
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

    fn c_var(name: &str) -> CAtom {
        CAtom::Var(name.to_string())
    }

    fn c_int(n: i64) -> CAtom {
        CAtom::Int(n)
    }

    fn c_bool(b: bool) -> CAtom {
        CAtom::Bool(b)
    }

    fn c_assign(name: &str, e: CExpr, cont: CTail) -> CTail {
        CTail::Seq(CStmt::Assign(name.to_string(), e), Box::new(cont))
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

        let input = anf_atom_to_anf(anf_var("x"));
        let expected = CTail::Return(CExpr::Atom(c_var("x")));
        assert_eq!(explicate_tail(input), expected);
    }

    #[test]
    fn tail_binop() {
        // x + 1
        // --->
        // Return(BinOp(Add, x, 1))

        let input = AnfExpr::Complex(CompExpr::BinOp(BinOp::Add, anf_var("x"), anf_int(1)));
        let expected = CTail::Return(CExpr::BinOp(BinOp::Add, c_var("x"), c_int(1)));
        assert_eq!(explicate_tail(input), expected);
    }

    #[test]
    fn tail_unaryop() {
        // -x
        // -->
        // Return(UnaryOp(Neg, x))

        let input = AnfExpr::Complex(CompExpr::UnaryOp(UnaryOp::Neg, anf_var("x")));
        let expected = CTail::Return(CExpr::UnaryOp(UnaryOp::Neg, c_var("x")));
        assert_eq!(explicate_tail(input), expected);
    }

    #[test]
    fn tail_app_becomes_tailcall() {
        // f(x, y)  在 tail 位置
        // -->
        // TailCall(f, [x, y])

        let input = AnfExpr::Complex(CompExpr::App(
            anf_var("f"),
            vec![anf_var("x"), anf_var("y")],
        ));
        let expected = CTail::TailCall(c_var("f"), vec![c_var("x"), c_var("y")]);
        assert_eq!(explicate_tail(input), expected);
    }

    #[test]
    fn tail_if_both_branches_are_tail() {
        // if c then 1 else 2
        // -->
        // If(c, Return(1), Return(2))

        let input = AnfExpr::Complex(CompExpr::If(
            anf_var("c"),
            Box::new(anf_atom_to_anf(anf_int(1))),
            Box::new(anf_atom_to_anf(anf_int(2))),
        ));
        let expected = CTail::If(
            c_var("c"),
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
            anf_atom_to_anf(anf_var("x")),
        );
        let expected = c_assign(
            "x",
            CExpr::Atom(c_int(1)),
            CTail::Return(CExpr::Atom(c_var("x"))),
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
                CompExpr::BinOp(BinOp::Add, anf_var("x"), anf_int(1)),
                anf_atom_to_anf(anf_var("y")),
            ),
        );
        let expected = c_assign(
            "x",
            CExpr::Atom(c_int(1)),
            c_assign(
                "y",
                CExpr::BinOp(BinOp::Add, c_var("x"), c_int(1)),
                CTail::Return(CExpr::Atom(c_var("y"))),
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
                anf_var("c"),
                Box::new(anf_atom_to_anf(anf_int(1))),
                Box::new(anf_atom_to_anf(anf_int(2))),
            ),
            anf_atom_to_anf(anf_var("x")),
        );
        let cont = CTail::Return(CExpr::Atom(c_var("x")));
        let expected = CTail::If(
            c_var("c"),
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
        let cont = CTail::Return(CExpr::Atom(c_var("x")));
        let expected = c_assign("x", CExpr::Atom(c_int(5)), cont.clone());
        assert_eq!(explicate_assign(input, "x", cont), expected);
    }

    #[test]
    fn assign_binop() {
        // explicate_assign(a * b, "x", Return(x))
        // --->
        // Seq(Assign(x, a * b), Return(x))
        
        let input = AnfExpr::Complex(CompExpr::BinOp(BinOp::Mul, anf_var("a"), anf_var("b")));
        let cont = CTail::Return(CExpr::Atom(c_var("x")));
        let expected = c_assign(
            "x",
            CExpr::BinOp(BinOp::Mul, c_var("a"), c_var("b")),
            cont.clone(),
        );
        assert_eq!(explicate_assign(input, "x", cont), expected);
    }

    #[test]
    fn assign_unaryop() {
        // explicate_assign(!a, "x", Return(x))
        // --->
        // Seq(Assign(x, !a), Return(x))
        
        let input = AnfExpr::Complex(CompExpr::UnaryOp(UnaryOp::Not, anf_var("a")));
        let cont = CTail::Return(CExpr::Atom(c_var("x")));
        let expected = c_assign("x", CExpr::UnaryOp(UnaryOp::Not, c_var("a")), cont.clone());
        assert_eq!(explicate_assign(input, "x", cont), expected);
    }

    #[test]
    fn assign_app_is_non_tail_call() {
        // explicate_assign(f(a, b), "x", cont)
        // ---> 
        // Seq(Assign(x, Call(f, [a, b])), cont)

        let input = AnfExpr::Complex(CompExpr::App(
            anf_var("f"),
            vec![anf_var("a"), anf_var("b")],
        ));
        let cont = CTail::Return(CExpr::Atom(c_var("x")));
        let expected = c_assign(
            "x",
            CExpr::Call(c_var("f"), vec![c_var("a"), c_var("b")]),
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
            anf_var("c"),
            Box::new(anf_atom_to_anf(anf_int(1))),
            Box::new(anf_atom_to_anf(anf_int(2))),
        ));
        let cont = CTail::Return(CExpr::Atom(c_var("x")));
        let expected = CTail::If(
            c_var("c"),
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
                anf_var("c"),
                Box::new(anf_atom_to_anf(anf_int(1))),
                Box::new(anf_atom_to_anf(anf_int(2))),
            ),
            anf_atom_to_anf(anf_var("x")),
        );

        let final_cont = CTail::Return(CExpr::Atom(c_var("y")));
        let result = explicate_assign(inner_let, "y", final_cont.clone());

        let expected = CTail::If(
            c_var("c"),
            Box::new(c_assign(
                "x",
                CExpr::Atom(c_int(1)),
                c_assign("y", CExpr::Atom(c_var("x")), final_cont.clone()),
            )),
            Box::new(c_assign(
                "x",
                CExpr::Atom(c_int(2)),
                c_assign("y", CExpr::Atom(c_var("x")), final_cont),
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
            anf_atom_to_anf(anf_var("a")),
        );
        let cont = CTail::Return(CExpr::Atom(c_var("x")));
        let expected = c_assign(
            "a",
            CExpr::Atom(c_int(1)),
            c_assign("x", CExpr::Atom(c_var("a")), cont.clone()),
        );
        assert_eq!(explicate_assign(input, "x", cont), expected);
    }

    #[test]
    fn assign_true_false_atoms() {
        // explicate_assign(true, "x", Return(x))
        // --->
        // Seq(Assign(x, true), Return(x))
        
        let input = anf_atom_to_anf(anf_bool(true));
        let cont = CTail::Return(CExpr::Atom(c_var("x")));
        let expected = c_assign("x", CExpr::Atom(c_bool(true)), cont.clone());
        assert_eq!(explicate_assign(input, "x", cont), expected);
    }
}
