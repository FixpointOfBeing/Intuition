use std::collections::HashMap;

use crate::syntax::BinOp;
use crate::syntax::HasType;
use crate::syntax::Ident;
use crate::syntax::PrimIO;
use crate::syntax::Type;
use crate::syntax::TypedDef;
use crate::syntax::TypedExpr;
use crate::syntax::TypedProgram;
use crate::syntax::UnaryOp;

type Fnames = HashMap<Ident, usize>;

#[derive(Debug, Clone, PartialEq)]
pub enum RevealExpr {
    Unit,
    Bool(bool),
    Int(i64),
    Float(f64),
    Var(Ident, Type),
    FunRef(Ident, usize, Type), // 新增：函数名 → label
    BinOp(BinOp, Box<RevealExpr>, Box<RevealExpr>, Type),
    Tuple(Vec<RevealExpr>, Type),
    TupleProj(Box<RevealExpr>, usize, Type),
    PrimIO(PrimIO, Option<Box<RevealExpr>>, Type),
    UnaryOp(UnaryOp, Box<RevealExpr>, Type),
    Ann(Box<RevealExpr>, Type),
    If(Box<RevealExpr>, Box<RevealExpr>, Box<RevealExpr>, Type),
    Let(Ident, Type, Box<RevealExpr>, Box<RevealExpr>, Type),
    LetRec(Ident, Vec<(Ident, Type)>, Type, Box<RevealExpr>, Box<RevealExpr>, Type),
    App(Box<RevealExpr>, Vec<RevealExpr>, Type),
    Lambda(Vec<(Ident, Type)>, Box<RevealExpr>, Type),
}

impl HasType for RevealExpr {
    fn type_of(&self) -> Type {
        todo!()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum RevealDef {
    ValDef(Ident, Option<Type>, RevealExpr),
    FunDef(Ident, Vec<(Ident, Type)>, Type, RevealExpr),
}

#[derive(Debug, Clone, PartialEq)]
pub struct RevealProgram {
    defs: Vec<RevealDef>,
    main: RevealExpr,
}

pub fn reveal_expr(typed_expr: TypedExpr, fnames: &mut Fnames) -> RevealExpr {
    todo!()
}

pub fn reveal_def(def: TypedDef, fnames: &mut Fnames) -> RevealDef {
    todo!()
}

pub fn reveal_program(prog: TypedProgram) -> RevealProgram {
    todo!()
}
