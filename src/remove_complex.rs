use crate::syntax::{BinOp, Expr, Ident, Type, UnaryOp};

// monadic normal form
#[derive(Debug, Clone, PartialEq)]
pub enum MonAtom {
    Unit,
    Bool(bool),
    Int(i64),
    Float(f64),
    Var(Ident),
}

#[derive(Debug, Clone, PartialEq)]
pub enum MonExpr {
    Atom(MonAtom),
    BinOp(BinOp, MonAtom, MonAtom),
    UnaryOp(UnaryOp, MonAtom),
    If(MonAtom, Box<MonExpr>, Box<MonExpr>),
    Let(Ident, Option<Type>, Box<MonExpr>, Box<MonExpr>),
    LetRec(Ident, Vec<(Ident, Type)>, Type, Box<MonExpr>, Box<MonExpr>),
    App(MonAtom, Vec<MonAtom>),
    Lambda(Vec<(Ident, Type)>, Option<Type>, Box<MonExpr>),
}

struct RemoveComplex {
    next_id: u64,
}

impl RemoveComplex {
    fn new() -> RemoveComplex {
        RemoveComplex { next_id: 0 }
    }

    fn fresh(&mut self) -> Ident {
        let id = self.next_id;
        self.next_id += 1;
        format!("${id}")
    }

    fn is_atom(&self, expr: &Expr) -> bool {
        matches!(
            expr,
            Expr::Unit | Expr::Bool(_) | Expr::Int(_) | Expr::Float(_) | Expr::Var(_)
        )
    }
    fn remove_complex_atom(&self, expr: &Expr) -> MonAtom {
        match expr {
            Expr::Unit => MonAtom::Unit,
            Expr::Bool(b) => MonAtom::Bool(*b),
            Expr::Int(i) => MonAtom::Int(*i),
            Expr::Float(f) => MonAtom::Float(*f),
            Expr::Var(name) => MonAtom::Var(name.to_string()),
            _ => unreachable!(),
        }
    }
    fn remove_complex(&mut self, expr: &Expr) -> MonExpr {
        match expr {
            Expr::BinOp(op, left, right) => match (self.is_atom(left), self.is_atom(right)) {
                (true, true) => {
                    let left = self.remove_complex_atom(left);
                    let right = self.remove_complex_atom(right);
                    MonExpr::BinOp((*op).clone(), left, right)
                }
                (true, false) => {
                    let left = self.remove_complex_atom(left);

                    let right = self.remove_complex(right);
                    let name = self.fresh();
                    let right_atom = MonAtom::Var(name.clone());

                    let body = MonExpr::BinOp((*op).clone(), left, right_atom);
                    MonExpr::Let(name, None, Box::new(right), Box::new(body))
                }
                (false, true) => {
                    let left = self.remove_complex(left);
                    let name = self.fresh();
                    let left_atom = MonAtom::Var(name.clone());

                    let right = self.remove_complex_atom(right);

                    let body = MonExpr::BinOp((*op).clone(), left_atom, right);
                    MonExpr::Let(name, None, Box::new(left), Box::new(body))
                }
                (false, false) => {
                    let left = self.remove_complex(left);
                    let name_left = self.fresh();
                    let left_atom = MonAtom::Var(name_left.clone());

                    let right = self.remove_complex(right);
                    let name_right = self.fresh();
                    let right_atom = MonAtom::Var(name_right.clone());

                    let body_inner = MonExpr::BinOp((*op).clone(), left_atom, right_atom);
                    let body_outer =
                        MonExpr::Let(name_right, None, Box::new(right), Box::new(body_inner));
                    MonExpr::Let(name_left, None, Box::new(left), Box::new(body_outer))
                }
            },
            Expr::UnaryOp(op, expr) => {
                if self.is_atom(expr) {
                    let expr = self.remove_complex_atom(expr);
                    MonExpr::UnaryOp((*op).clone(), expr)
                } else {
                    let expr = self.remove_complex(expr);
                    let name = self.fresh();
                    let expr_atom = MonAtom::Var(name.clone());

                    let body = MonExpr::UnaryOp((*op).clone(), expr_atom);
                    MonExpr::Let(name, None, Box::new(expr), Box::new(body))
                }
            }
            Expr::If(cond, then_e, else_e) => {
                if !self.is_atom(cond) {
                    let cond_mon = self.remove_complex(cond);
                    let name = self.fresh();
                    let then_mon = self.remove_complex(then_e);
                    let else_mon = self.remove_complex(else_e);
                    let body = MonExpr::If(
                        MonAtom::Var(name.clone()),
                        Box::new(then_mon),
                        Box::new(else_mon),
                    );
                    MonExpr::Let(name, None, Box::new(cond_mon), Box::new(body))
                } else {
                    let cond = self.remove_complex_atom(cond);
                    let then_e = self.remove_complex(then_e);
                    let else_e = self.remove_complex(else_e);
                    MonExpr::If(cond, Box::new(then_e), Box::new(else_e))
                }
            }
            Expr::Let(name, ty, rhs, body) => {
                let rhs = self.remove_complex(rhs);
                let body = self.remove_complex(body);

                MonExpr::Let(
                    name.to_string(),
                    (*ty).clone(),
                    Box::new(rhs),
                    Box::new(body),
                )
            }
            Expr::LetRec(fname, fargs, ty, fbody, body) => {
                let fbody = self.remove_complex(fbody);
                let body = self.remove_complex(body);

                MonExpr::LetRec(
                    fname.to_string(),
                    (*fargs).clone(),
                    (*ty).clone(),
                    Box::new(fbody),
                    Box::new(body),
                )
            }
            Expr::App(func, args) => {
                if args.len() == 0 {
                    panic!("Application with no arguments is not allowed");
                }
                if self.is_atom(func) {
                    self.remove_complex_app(self.remove_complex_atom(func), args)
                } else {
                    let func = self.remove_complex(func);
                    let name = self.fresh();
                    let body = self.remove_complex_app(MonAtom::Var(name.clone()), args);
                    MonExpr::Let(name, None, Box::new(func), Box::new(body))
                }
            }
            Expr::Lambda(args, ty, body) => {
                let body = self.remove_complex(body);
                MonExpr::Lambda((*args).clone(), (*ty).clone(), Box::new(body))
            }
            Expr::Ann(expr, _) => self.remove_complex(expr),
            _ => MonExpr::Atom(self.remove_complex_atom(expr)),
        }
    }

    fn remove_complex_app(&mut self, func: MonAtom, args: &[Expr]) -> MonExpr {
        let mut atom_args = Vec::with_capacity(args.len());
        let mut lets: Vec<(Ident, MonExpr)> = Vec::new();

        for arg in args {
            if self.is_atom(arg) {
                atom_args.push(self.remove_complex_atom(arg));
            } else {
                let mon = self.remove_complex(arg);
                let name = self.fresh();
                atom_args.push(MonAtom::Var(name.clone()));
                lets.push((name, mon));
            }
        }

        let mut body = MonExpr::App(func, atom_args);
        for (name, mon) in lets.into_iter().rev() {
            body = MonExpr::Let(name, None, Box::new(mon), Box::new(body));
        }
        body
    }
}
pub fn remove_complex(expr: &Expr) -> MonExpr {
    RemoveComplex::new().remove_complex(expr)
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::syntax::*;

    fn test(expr: Expr) -> MonExpr {
        RemoveComplex::new().remove_complex(&expr)
    }

    #[test]
    fn test_atom() {
        assert_eq!(test(Expr::Int(42)), MonExpr::Atom(MonAtom::Int(42)));
    }

    #[test]
    fn test_binop_atom() {
        assert_eq!(
            test(Expr::BinOp(
                BinOp::Add,
                Box::new(Expr::Int(1)),
                Box::new(Expr::Int(2)),
            )),
            MonExpr::BinOp(BinOp::Add, MonAtom::Int(1), MonAtom::Int(2),)
        );
    }

    #[test]
    fn test_binop_right_complex() {
        assert_eq!(
            test(Expr::BinOp(
                BinOp::Add,
                Box::new(Expr::Int(1)),
                Box::new(Expr::BinOp(
                    BinOp::Mul,
                    Box::new(Expr::Int(2)),
                    Box::new(Expr::Int(3)),
                )),
            )),
            MonExpr::Let(
                "$0".into(),
                None,
                Box::new(MonExpr::BinOp(BinOp::Mul, MonAtom::Int(2), MonAtom::Int(3),)),
                Box::new(MonExpr::BinOp(
                    BinOp::Add,
                    MonAtom::Int(1),
                    MonAtom::Var("$0".into()),
                ))
            )
        );
    }

    #[test]
    fn test_unary_complex() {
        assert_eq!(
            test(Expr::UnaryOp(
                UnaryOp::Neg,
                Box::new(Expr::BinOp(
                    BinOp::Add,
                    Box::new(Expr::Int(1)),
                    Box::new(Expr::Int(2)),
                )),
            )),
            MonExpr::Let(
                "$0".into(),
                None,
                Box::new(MonExpr::BinOp(BinOp::Add, MonAtom::Int(1), MonAtom::Int(2),)),
                Box::new(MonExpr::UnaryOp(UnaryOp::Neg, MonAtom::Var("$0".into())))
            )
        );
    }

    #[test]
    fn test_if_complex_condition() {
        assert_eq!(
            test(Expr::If(
                Box::new(Expr::BinOp(
                    BinOp::Gt,
                    Box::new(Expr::Var("x".into())),
                    Box::new(Expr::Int(0)),
                )),
                Box::new(Expr::Int(1)),
                Box::new(Expr::Int(2)),
            )),
            MonExpr::Let(
                "$0".into(),
                None,
                Box::new(MonExpr::BinOp(
                    BinOp::Gt,
                    MonAtom::Var("x".into()),
                    MonAtom::Int(0),
                )),
                Box::new(MonExpr::If(
                    MonAtom::Var("$0".into()),
                    Box::new(MonExpr::Atom(MonAtom::Int(1))),
                    Box::new(MonExpr::Atom(MonAtom::Int(2))),
                ))
            )
        );
    }

    #[test]
    fn test_app_complex_arg() {
        assert_eq!(
            test(Expr::App(
                Box::new(Expr::Var("f".into())),
                vec![Expr::BinOp(
                    BinOp::Add,
                    Box::new(Expr::Int(1)),
                    Box::new(Expr::Int(2)),
                )],
            )),
            MonExpr::Let(
                "$0".into(),
                None,
                Box::new(MonExpr::BinOp(BinOp::Add, MonAtom::Int(1), MonAtom::Int(2),)),
                Box::new(MonExpr::App(
                    MonAtom::Var("f".into()),
                    vec![MonAtom::Var("$0".into())],
                ))
            )
        );
    }
}
