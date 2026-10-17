use crate::{
    a_normal_form::{AExpr, AnfDef, AnfExpr, AnfProgram, CompExpr},
    expose_allocation::GlobalValue,
    gensym::Gensym,
    syntax::{
        BinOp, HasType, Ident, PrimInput, PrimOutput, Type, UnaryOp, ty_bool, ty_float, ty_int,
        ty_unit,
    },
};

/*
 * For the integers and variables, we needed assignment and tail positions. The if expressions introduced predicate positions. For While , the begin expression introduces yet another kind of position: effect position.
 */
#[derive(Debug, Clone, PartialEq)]
pub enum CAtom {
    Unit,
    Bool(bool),
    Int(i64),
    Float(f64),
    Var(Ident, Type),
}

impl std::fmt::Display for CAtom {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CAtom::Unit => write!(f, "()"),
            CAtom::Bool(b) => write!(f, "{}", b),
            CAtom::Int(n) => write!(f, "{}", n),
            CAtom::Float(fl) => write!(f, "{}", fl),
            CAtom::Var(name, _) => write!(f, "{}", name),
        }
    }
}

impl HasType for CAtom {
    fn type_of(&self) -> Type {
        match self {
            CAtom::Unit => ty_unit(),
            CAtom::Bool(_) => ty_bool(),
            CAtom::Int(_) => ty_int(),
            CAtom::Float(_) => ty_float(),
            CAtom::Var(_, ty) => ty.clone(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum CExpr {
    Atom(CAtom, Type),
    BinOp(BinOp, CAtom, CAtom, Type),
    UnaryOp(UnaryOp, CAtom, Type),
    PrimInput(PrimInput, Type),
    TupleProj(CAtom, usize, Type),
    Allocate(
        usize, // bytes
        Type,
    ),
    AllocateClosure(
        usize, // bytes
        usize, //arity
        Type,
    ),
    Call(CAtom, Vec<CAtom>, Type),
    FunRef(Ident, usize, Type),
    GlobalValue(GlobalValue),
    Global(Ident, Type),
}

impl std::fmt::Display for CExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CExpr::Atom(atom, _) => write!(f, "{}", atom),
            CExpr::BinOp(op, left, right, _) => write!(f, "({} {} {})", left, op, right),
            CExpr::UnaryOp(op, atom, _) => write!(f, "({}{})", op, atom),
            CExpr::PrimInput(prim, _) => write!(f, "{} ()", prim),
            CExpr::TupleProj(atom, index, _) => write!(f, "{}.{}", atom, index),
            CExpr::Allocate(bytes, ty) => write!(f, "allocate({}, {})", bytes, ty),
            CExpr::AllocateClosure(bytes, arity, ty) => {
                write!(f, "allocate_closure({}, {}, {})", bytes, arity, ty)
            },
            CExpr::Call(func, args, _) => {
                let args_str = args
                    .iter()
                    .map(|arg| format!("{}", arg))
                    .collect::<Vec<_>>()
                    .join(" ");
                write!(f, "({} {})", func, args_str)
            },
            CExpr::FunRef(name, arity, _) => write!(f, "fun_ref({}, {})", name, arity),
            CExpr::GlobalValue(value) => write!(f, "{}", value),
            CExpr::Global(name, _) => write!(f, "{}", name),
        }
    }
}

impl HasType for CExpr {
    fn type_of(&self) -> Type {
        match self {
            CExpr::Atom(_, ty) => ty.clone(),
            CExpr::BinOp(_, _, _, ty) => ty.clone(),
            CExpr::UnaryOp(_, _, ty) => ty.clone(),
            CExpr::PrimInput(_, ty) => ty.clone(),
            CExpr::TupleProj(_, _, ty) => ty.clone(),
            CExpr::Allocate(_, ty) => ty.clone(),
            CExpr::AllocateClosure(_, _, ty) => ty.clone(),
            CExpr::Call(_, _, ty) => ty.clone(),
            CExpr::FunRef(_, _, ty) => ty.clone(),
            CExpr::GlobalValue(value) => value.type_of(),
            CExpr::Global(_, ty) => ty.clone(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum CStmt {
    Assign(Ident, CExpr),
    TupleSet(CAtom, CAtom, usize),
    PrimOutput(PrimOutput, CAtom),
    Collect(usize),
    // Effect(CExpr),
    // If(CAtom, Vec<CStmt>, Vec<CStmt>),
}

impl std::fmt::Display for CStmt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CStmt::Assign(name, expr) => writeln!(f, "{} = {};", name, expr),
            CStmt::TupleSet(tuple, element, index) => {
                write!(f, "{}[{}] = {};", tuple, index, element)
            },
            CStmt::PrimOutput(prim, atom) => write!(f, "{} {};", prim, atom),
            CStmt::Collect(bytes) => write!(f, "collect({});", bytes),
            // CStmt::Effect(expr) => write!(f, "{};", expr),
            // CStmt::If(cond, thn, els) => {
            //     let thn_str = thn.iter().map(|s| format!("{}", s)).collect::<Vec<_>>().join(" ");
            //     let els_str = els.iter().map(|s| format!("{}", s)).collect::<Vec<_>>().join(" ");
            //     write!(f, "if {} {{ {} }} else {{ {} }}", cond, thn_str, els_str)
            // },
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum CTail {
    Return(CExpr),
    Goto(Ident),
    Seq(Vec<CStmt>, Box<CTail>),
    TailCall(CAtom, Vec<CAtom>),
    TailIf(CAtom, Box<CTail>, Box<CTail>),
}

impl std::fmt::Display for CTail {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CTail::Return(expr) => write!(f, "return {};", expr),
            CTail::Goto(label) => write!(f, "goto {};", label),
            CTail::Seq(stmts, tail) => {
                for stmt in stmts {
                    write!(f, "{} ", stmt)?;
                }
                write!(f, "{}", tail)
            },
            CTail::TailCall(func, args) => {
                let args_str = args
                    .iter()
                    .map(|arg| format!("{}", arg))
                    .collect::<Vec<_>>()
                    .join(" ");
                write!(f, "tailcall {} {}", func, args_str)
            },
            CTail::TailIf(cond, thn, els) => {
                write!(f, "if {} {{\n {} }}\n else {{\n {} }}", cond, thn, els)
            },
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct CBlock {
    pub label: Ident,
    pub tail: CTail,
}

impl std::fmt::Display for CBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:\n{}", self.label, self.tail)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum CDef {
    ValDef(
        Ident,        // name
        Option<Type>, // optional type annotation
        CTail,        // expresion
        Vec<CBlock>,   // blocks
    ),
    FunDef(
        Ident,              // function name
        Vec<(Ident, Type)>, // function arguments with their types
        Type,               // function return type
        CTail,              // function body
        Vec<CBlock>,         // blocks
    ),
}

#[derive(Debug, Clone, PartialEq)]
pub struct CProgram {
    pub defs: Vec<CDef>,
    pub main: CTail,
    pub main_blocks: Vec<CBlock>,
}

pub fn c_unit() -> CAtom {
    CAtom::Unit
}

pub fn c_bool(b: bool) -> CAtom {
    CAtom::Bool(b)
}

pub fn c_int(n: i64) -> CAtom {
    CAtom::Int(n)
}

pub fn c_float(f: f64) -> CAtom {
    CAtom::Float(f)
}

pub fn c_var(name: impl Into<Ident>, ty: Type) -> CAtom {
    CAtom::Var(name.into(), ty)
}

pub fn c_global(name: impl Into<Ident>, ty: Type) -> CExpr {
    CExpr::Global(name.into(), ty)
}

pub fn c_atom(atom: CAtom, ty: Type) -> CExpr {
    CExpr::Atom(atom, ty)
}

pub fn c_bin_op(op: BinOp, left: CAtom, right: CAtom, ty: Type) -> CExpr {
    CExpr::BinOp(op, left, right, ty)
}

pub fn c_unary(op: UnaryOp, catom: CAtom, ty: Type) -> CExpr {
    CExpr::UnaryOp(op, catom, ty)
}

pub fn c_tuple_proj(tuple: CAtom, index: usize, ty: Type) -> CExpr {
    CExpr::TupleProj(tuple, index, ty)
}

pub fn c_allocate(bytes: usize, ty: Type) -> CExpr {
    CExpr::Allocate(bytes, ty)
}

pub fn c_allocate_closure(bytes: usize, arity: usize, ty: Type) -> CExpr {
    CExpr::AllocateClosure(bytes, arity, ty)
}

pub fn c_call(func: CAtom, args: Vec<CAtom>, ty: Type) -> CExpr {
    CExpr::Call(func, args, ty)
}

pub fn c_fun_ref(name: impl Into<Ident>, arity: usize, ty: Type) -> CExpr {
    CExpr::FunRef(name.into(), arity, ty)
}

pub fn c_global_value(value: GlobalValue) -> CExpr {
    CExpr::GlobalValue(value)
}

pub fn c_assign(name: impl Into<Ident>, cexpr: CExpr) -> CStmt {
    CStmt::Assign(name.into(), cexpr)
}

pub fn c_tuple_set(tuple: CAtom, element: CAtom, index: usize) -> CStmt {
    CStmt::TupleSet(tuple, element, index)
}

pub fn c_prim_input(prim: PrimInput) -> CExpr {
    CExpr::PrimInput(prim, ty_unit())
}

pub fn c_prim_output(prim: PrimOutput, expr: CAtom) -> CStmt {
    CStmt::PrimOutput(prim, expr)
}

pub fn c_collect(bytes: usize) -> CStmt {
    CStmt::Collect(bytes)
}

// pub fn c_effect(expr: CExpr) -> CStmt {
//     CStmt::Effect(expr)
// }

pub fn c_return(expr: CExpr) -> CTail {
    CTail::Return(expr)
}

pub fn c_goto(label: Ident) -> CTail {
    CTail::Goto(label)
}

pub fn c_tail_call(func: CAtom, args: Vec<CAtom>) -> CTail {
    CTail::TailCall(func, args)
}

pub fn c_seq(stmts: Vec<CStmt>, tail: CTail) -> CTail {
    CTail::Seq(stmts, Box::new(tail))
}

pub fn c_tail_if(cond: CAtom, thn: CTail, els: CTail) -> CTail {
    CTail::TailIf(cond, Box::new(thn), Box::new(els))
}

pub fn explicate_assign_anf(
    name: String,
    anf: AnfExpr,
    cont: CTail,
    gensym: &mut Gensym,
    blocks: &mut Vec<CBlock>,
) -> CTail {
    match anf {
        AnfExpr::Complex(cexpr, _) => explicate_assign_complex(name, cexpr, cont, gensym, blocks),
        AnfExpr::Let(let_name, rhs, body, _) => {
            let inner_cont = explicate_assign_anf(name, *body, cont, gensym, blocks);
            explicate_assign_complex(let_name, rhs, inner_cont, gensym, blocks)
        },
    }
}

pub fn explicate_assign_complex(
    name: String,
    cexpr: CompExpr,
    cont: CTail,
    gensym: &mut Gensym,
    blocks: &mut Vec<CBlock>,
) -> CTail {
    let dummy = "_".to_string();
    match cexpr {
        CompExpr::Atom(aexpr, ty) => {
            let catom = aexpr_to_catom(aexpr);
            let cexpr = c_atom(catom, ty);
            let stmt = c_assign(name, cexpr);
            c_seq(vec![stmt], cont)
        },
        CompExpr::BinOp(op, left, right, ty) => {
            let left_catom = aexpr_to_catom(left);
            let right_catom = aexpr_to_catom(right);
            let cexpr = c_bin_op(op, left_catom, right_catom, ty);
            let stmt = c_assign(name, cexpr);
            c_seq(vec![stmt], cont)
        },
        CompExpr::UnaryOp(op, aexpr, ty) => {
            let catom = aexpr_to_catom(aexpr);
            let cexpr = c_unary(op, catom, ty);
            let stmt = c_assign(name, cexpr);
            c_seq(vec![stmt], cont)
        },
        CompExpr::TupleProj(tuple, idx, ty) => {
            let catom = aexpr_to_catom(tuple);
            let cexpr = c_tuple_proj(catom, idx, ty);
            let stmt = c_assign(name, cexpr);
            c_seq(vec![stmt], cont)
        },
        CompExpr::Allocate(bytes, ty) => {
            let cexpr = c_allocate(bytes, ty);
            let stmt = c_assign(name, cexpr);
            c_seq(vec![stmt], cont)
        },
        CompExpr::AllocateClosure(bytes, arity, ty) => {
            let cexpr = c_allocate_closure(bytes, arity, ty);
            let stmt = c_assign(name, cexpr);
            c_seq(vec![stmt], cont)
        },
        CompExpr::PrimInput(prim, _) => {
            let cexpr = c_prim_input(prim);
            let stmt = c_assign(name, cexpr);
            c_seq(vec![stmt], cont)
        },
        CompExpr::PrimOutput(prim, aexpr, _) => {
            debug_assert!(name == dummy);
            let catom = aexpr_to_catom(aexpr);
            let stmt = c_prim_output(prim, catom);
            c_seq(vec![stmt], cont)
        },

        CompExpr::App(func, args, ty) => {
            let func_catom = aexpr_to_catom(func);
            let args_catom = args.into_iter().map(aexpr_to_catom).collect();
            let cexpr = c_call(func_catom, args_catom, ty);
            let stmt = c_assign(name, cexpr);
            c_seq(vec![stmt], cont)
        },
        CompExpr::If(cond, thn, els, _) => {
            // name = if <cond> then <thn> else <els>
            // cont
            let block = create_block(cont, gensym);
            let cont_goto = c_goto(block.label.clone());
            blocks.push(block);

            let thn_cont =
                explicate_assign_anf(name.clone(), *thn, cont_goto.clone(), gensym, blocks);
            // let thn_block = create_block(thn_cont, gensym);
            // let thn_goto = c_goto(thn_block.label.clone());
            // blocks.push(thn_block);

            let els_cont = explicate_assign_anf(name, *els, cont_goto, gensym, blocks);
            // let els_block = create_block(els_cont, gensym);
            // let els_goto = c_goto(els_block.label.clone());
            // blocks.push(els_block);

            let cond_catom = aexpr_to_catom(cond);

            c_tail_if(cond_catom, thn_cont, els_cont)
        },
        CompExpr::Collect(bytes) => {
            debug_assert!(name == dummy);
            let stmt = c_collect(bytes);
            c_seq(vec![stmt], cont)
        },
        CompExpr::TupleSet(tuple, elem, idx) => {
            debug_assert!(name == dummy);
            let tuple_catom = aexpr_to_catom(tuple);
            let elem_catom = aexpr_to_catom(elem);
            let stmt = c_tuple_set(tuple_catom, elem_catom, idx);
            c_seq(vec![stmt], cont)
        },
        CompExpr::GlobalValue(value) => {
            let cexpr = c_global_value(value);
            let stmt = c_assign(name, cexpr);
            c_seq(vec![stmt], cont)
        },
        CompExpr::Global(global_name, ty) => {
            let cexpr = c_global(global_name, ty);
            let stmt = c_assign(name, cexpr);
            c_seq(vec![stmt], cont)
        },
        CompExpr::FunRef(func_name, arity, ty) => {
            let cexpr = c_fun_ref(func_name, arity, ty);
            let stmt = c_assign(name, cexpr);
            c_seq(vec![stmt], cont)
        },
    }
}

fn create_block(cont: CTail, gensym: &mut Gensym) -> CBlock {
    let name = gensym.fresh_with_prefix("block");
    CBlock { label: name, tail: cont }
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

fn explicate_tail(anf: AnfExpr, gensym: &mut Gensym, blocks: &mut Vec<CBlock>) -> CTail {
    match anf {
        AnfExpr::Complex(comp_expr, _) => match comp_expr {
            CompExpr::Atom(aexpr, ty) => {
                let catom = aexpr_to_catom(aexpr);
                c_return(c_atom(catom, ty))
            },
            CompExpr::BinOp(op, left, right, ty) => {
                let left_atom = aexpr_to_catom(left);
                let right_atom = aexpr_to_catom(right);
                let cexpr = c_bin_op(op, left_atom, right_atom, ty);
                c_return(cexpr)
            },
            CompExpr::UnaryOp(op, aexpr, ty) => {
                let atom = aexpr_to_catom(aexpr);
                let cexpr = c_unary(op, atom, ty);
                c_return(cexpr)
            },
            CompExpr::TupleProj(aexpr, idx, ty) => {
                let atom = aexpr_to_catom(aexpr);
                let cexpr = c_tuple_proj(atom, idx, ty);
                c_return(cexpr)
            },
            CompExpr::Allocate(bytes, ty) => {
                let cexpr = c_allocate(bytes, ty);
                c_return(cexpr)
            },
            CompExpr::AllocateClosure(bytes, arity, ty) => {
                let cexpr = c_allocate_closure(bytes, arity, ty);
                c_return(cexpr)
            },
            CompExpr::PrimInput(prim, _) => {
                let cexpr = c_prim_input(prim);
                c_return(cexpr)
            },
            CompExpr::PrimOutput(prim, aexpr, _) => {
                let catom = aexpr_to_catom(aexpr);
                let stmt = c_prim_output(prim, catom);
                let last = c_return(c_atom(c_unit(), ty_unit()));
                c_seq(vec![stmt], last)
            },
            CompExpr::App(func, args, _) => {
                let func_atom = aexpr_to_catom(func);
                let args_atom = args.into_iter().map(|e| aexpr_to_catom(e)).collect::<Vec<_>>();
                c_tail_call(func_atom, args_atom)
            },
            CompExpr::If(cond, thn, els, _) => {
                let cond_atom = aexpr_to_catom(cond);
                let thn_atom = explicate_tail(*thn, gensym, blocks);
                let els_atom = explicate_tail(*els, gensym, blocks);
                c_tail_if(cond_atom, thn_atom, els_atom)
            },
            CompExpr::Collect(bytes) => {
                let stmt = c_collect(bytes);
                let last = c_return(c_atom(c_unit(), ty_unit()));
                c_seq(vec![stmt], last)
            },
            CompExpr::TupleSet(tuple, elem, idx) => {
                let tuple_atom = aexpr_to_catom(tuple);
                let elem_atom = aexpr_to_catom(elem);
                let stmt = c_tuple_set(tuple_atom, elem_atom, idx);
                let last = c_return(c_atom(c_unit(), ty_unit()));
                c_seq(vec![stmt], last)
            },
            CompExpr::GlobalValue(value) => c_return(c_global_value(value)),
            CompExpr::Global(name, ty) => c_return(c_global(name, ty)),
            CompExpr::FunRef(name, arity, ty) => c_return(c_fun_ref(name, arity, ty)),
        },
        AnfExpr::Let(name, rhs, body, _) => {
            let tail = explicate_tail(*body, gensym, blocks);
            explicate_assign_complex(name, rhs, tail, gensym, blocks)
        },
    }
}

pub fn explicate_control_def(def: AnfDef, gs: &mut Gensym) -> CDef {
    match def {
        AnfDef::ValDef(name, opty, anf) => {
            let mut blocks = vec![];
            let tail = explicate_tail(anf, gs, &mut blocks);

            CDef::ValDef(name, opty, tail, blocks)
        },
        AnfDef::FunDef(name, params, ret_ty, anf) => {
            let mut blocks = vec![];
            let tail = explicate_tail(anf, gs, &mut blocks);
            CDef::FunDef(name, params, ret_ty, tail, blocks)
        },
    }
}

pub fn explicate_control_prog(prog: AnfProgram) -> CProgram {
    let mut gs = Gensym::new();
    let mut defs = Vec::with_capacity(prog.defs.len());
    for def in prog.defs {
        let cdef = explicate_control_def(def, &mut gs);
        defs.push(cdef);
    }
    let mut main_blocks = vec![];
    let main = explicate_tail(prog.main, &mut gs, &mut main_blocks);
    CProgram { defs, main, main_blocks }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        a_normal_form::*,
        syntax::{BinOp, PrimInput, UnaryOp, ty_arrow, ty_bool, ty_float, ty_int, ty_unit},
    };
    fn run_tail(anf: AnfExpr) -> (CTail, Vec<CBlock>) {
        let mut blocks = Vec::new();
        let tail = explicate_tail(anf, &mut Gensym::new(), &mut blocks);
        (tail, blocks)
    }

    fn run_assign(name: &str, anf: AnfExpr, cont: CTail) -> (CTail, Vec<CBlock>) {
        let mut blocks = Vec::new();
        let tail =
            explicate_assign_anf(name.to_string(), anf, cont, &mut Gensym::new(), &mut blocks);
        (tail, blocks)
    }

    #[test]
    fn explicate_tail_nested_let_and_if_creates_block() {
        // let x = read_int () in
        // let y = read_int () in
        // let $0 = x < 1 in
        // let $1 = if $0
        //          then x == 0
        //          else x == 2
        // if $1
        // then y + 2
        // else y + 10
        //
        let anf = anf_let(
            "x",
            co_prim_input(PrimInput::ReadInt, ty_int()),
            anf_let(
                "y",
                co_prim_input(PrimInput::ReadInt, ty_int()),
                anf_let(
                    "$0",
                    co_bin_op(BinOp::Lt, a_var("x", ty_int()), a_int(1), ty_bool()),
                    anf_let(
                        "$1",
                        co_if(
                            a_var("$0", ty_bool()),
                            anf_complex(
                                co_bin_op(BinOp::Eq, a_var("x", ty_int()), a_int(0), ty_bool()),
                                ty_bool(),
                            ),
                            anf_complex(
                                co_bin_op(BinOp::Eq, a_var("x", ty_int()), a_int(2), ty_bool()),
                                ty_bool(),
                            ),
                            ty_bool(),
                        ),
                        anf_complex(
                            co_if(
                                a_var("$1", ty_bool()),
                                anf_complex(
                                    co_bin_op(BinOp::Add, a_var("y", ty_int()), a_int(2), ty_int()),
                                    ty_int(),
                                ),
                                anf_complex(
                                    co_bin_op(
                                        BinOp::Add,
                                        a_var("y", ty_int()),
                                        a_int(10),
                                        ty_int(),
                                    ),
                                    ty_int(),
                                ),
                                ty_int(),
                            ),
                            ty_int(),
                        ),
                        ty_int(),
                    ),
                    ty_int(),
                ),
                ty_int(),
            ),
            ty_int(),
        );

        let expected_tail = c_seq(
            vec![c_assign("x", c_prim_input(PrimInput::ReadInt))],
            c_seq(
                vec![c_assign("y", c_prim_input(PrimInput::ReadInt))],
                c_seq(
                    vec![c_assign(
                        "$0",
                        c_bin_op(BinOp::Lt, c_var("x", ty_int()), c_int(1), ty_bool()),
                    )],
                    c_tail_if(
                        c_var("$0", ty_bool()),
                        c_seq(
                            vec![c_assign(
                                "$1",
                                c_bin_op(BinOp::Eq, c_var("x", ty_int()), c_int(0), ty_bool()),
                            )],
                            c_goto("block$0".to_string()),
                        ),
                        c_seq(
                            vec![c_assign(
                                "$1",
                                c_bin_op(BinOp::Eq, c_var("x", ty_int()), c_int(2), ty_bool()),
                            )],
                            c_goto("block$0".to_string()),
                        ),
                    ),
                ),
            ),
        );
        let expected_blocks = vec![CBlock {
            label: "block$0".to_string(),
            tail: c_tail_if(
                c_var("$1", ty_bool()),
                c_return(c_bin_op(BinOp::Add, c_var("y", ty_int()), c_int(2), ty_int())),
                c_return(c_bin_op(BinOp::Add, c_var("y", ty_int()), c_int(10), ty_int())),
            ),
        }];

        let (tail, blocks) = run_tail(anf);
        assert_eq!(tail, expected_tail);
        assert_eq!(blocks, expected_blocks);
    }

    #[test]
    fn tail_atom_int() {
        let input = anf_complex(co_atom(a_int(5), ty_int()), ty_int());
        let expected = c_return(c_atom(c_int(5), ty_int()));
        let (tail, blocks) = run_tail(input);
        assert_eq!(tail, expected);
        assert!(blocks.is_empty());
    }

    #[test]
    fn tail_atom_var() {
        let input = anf_complex(co_atom(a_var("x", ty_int()), ty_int()), ty_int());
        let expected = c_return(c_atom(c_var("x", ty_int()), ty_int()));
        let (tail, blocks) = run_tail(input);
        assert_eq!(tail, expected);
        assert!(blocks.is_empty());
    }

    #[test]
    fn tail_binop() {
        let input =
            anf_complex(co_bin_op(BinOp::Add, a_var("x", ty_int()), a_int(1), ty_int()), ty_int());
        let expected = c_return(c_bin_op(BinOp::Add, c_var("x", ty_int()), c_int(1), ty_int()));
        let (tail, blocks) = run_tail(input);
        assert_eq!(tail, expected);
        assert!(blocks.is_empty());
    }

    #[test]
    fn tail_unaryop() {
        let input =
            anf_complex(co_unary(UnaryOp::Neg, a_var("x", ty_float()), ty_float()), ty_float());
        let expected = c_return(c_unary(UnaryOp::Neg, c_var("x", ty_float()), ty_float()));
        let (tail, blocks) = run_tail(input);
        assert_eq!(tail, expected);
        assert!(blocks.is_empty());
    }

    #[test]
    fn tail_app_becomes_tailcall() {
        let x_ty = ty_int();
        let y_ty = ty_int();
        let fn_ty = ty_arrow(x_ty.clone(), ty_arrow(y_ty.clone(), ty_unit()));
        let input = anf_complex(
            co_app(
                a_var("f", fn_ty.clone()),
                vec![a_var("x", x_ty.clone()), a_var("y", y_ty.clone())],
                ty_unit(),
            ),
            ty_unit(),
        );
        let expected = c_tail_call(c_var("f", fn_ty), vec![c_var("x", x_ty), c_var("y", y_ty)]);
        let (tail, blocks) = run_tail(input);
        assert_eq!(tail, expected);
        assert!(blocks.is_empty());
    }

    #[test]
    fn tail_if_both_branches_are_tail() {
        let input = anf_complex(
            co_if(
                a_var("c", ty_bool()),
                anf_complex(co_atom(a_int(1), ty_int()), ty_int()),
                anf_complex(co_atom(a_int(2), ty_int()), ty_int()),
                ty_int(),
            ),
            ty_int(),
        );
        let expected = c_tail_if(
            c_var("c", ty_bool()),
            c_return(c_atom(c_int(1), ty_int())),
            c_return(c_atom(c_int(2), ty_int())),
        );
        let (tail, blocks) = run_tail(input);
        assert_eq!(tail, expected);
        assert!(blocks.is_empty());
    }

    #[test]
    fn tail_let_single() {
        let input = anf_let(
            "x",
            co_atom(a_int(1), ty_int()),
            anf_complex(co_atom(a_var("x", ty_int()), ty_int()), ty_int()),
            ty_int(),
        );
        let expected = c_seq(
            vec![c_assign("x", c_atom(c_int(1), ty_int()))],
            c_return(c_atom(c_var("x", ty_int()), ty_int())),
        );
        let (tail, blocks) = run_tail(input);
        assert_eq!(tail, expected);
        assert!(blocks.is_empty());
    }

    #[test]
    fn tail_let_nested_order_is_preserved() {
        let input = anf_let(
            "x",
            co_atom(a_int(1), ty_int()),
            anf_let(
                "y",
                co_bin_op(BinOp::Add, a_var("x", ty_int()), a_int(1), ty_int()),
                anf_complex(co_atom(a_var("y", ty_int()), ty_int()), ty_int()),
                ty_int(),
            ),
            ty_int(),
        );
        let expected = c_seq(
            vec![c_assign("x", c_atom(c_int(1), ty_int()))],
            c_seq(
                vec![c_assign(
                    "y",
                    c_bin_op(BinOp::Add, c_var("x", ty_int()), c_int(1), ty_int()),
                )],
                c_return(c_atom(c_var("y", ty_int()), ty_int())),
            ),
        );
        let (tail, blocks) = run_tail(input);
        assert_eq!(tail, expected);
        assert!(blocks.is_empty());
    }

    #[test]
    fn tail_let_with_if_rhs() {
        let input = anf_let(
            "x",
            co_if(
                a_var("c", ty_bool()),
                anf_complex(co_atom(a_int(1), ty_int()), ty_int()),
                anf_complex(co_atom(a_int(2), ty_int()), ty_int()),
                ty_int(),
            ),
            anf_complex(co_atom(a_var("x", ty_int()), ty_int()), ty_int()),
            ty_int(),
        );
        let expected = c_tail_if(
            c_var("c", ty_bool()),
            c_seq(vec![c_assign("x", c_atom(c_int(1), ty_int()))], c_goto("block$0".to_string())),
            c_seq(vec![c_assign("x", c_atom(c_int(2), ty_int()))], c_goto("block$0".to_string())),
        );
        let expected_blocks = vec![CBlock {
            label: "block$0".to_string(),
            tail: c_return(c_atom(c_var("x", ty_int()), ty_int())),
        }];
        let (tail, blocks) = run_tail(input);
        assert_eq!(tail, expected);
        assert_eq!(blocks, expected_blocks);
    }

    #[test]
    fn assign_atom() {
        let input = anf_complex(co_atom(a_int(5), ty_int()), ty_int());
        let cont = c_return(c_atom(c_var("x", ty_int()), ty_int()));
        let expected = c_seq(vec![c_assign("x", c_atom(c_int(5), ty_int()))], cont.clone());
        let (tail, blocks) = run_assign("x", input, cont);
        assert_eq!(tail, expected);
        assert!(blocks.is_empty());
    }

    #[test]
    fn assign_binop() {
        let input = anf_complex(
            co_bin_op(BinOp::Mul, a_var("a", ty_float()), a_var("b", ty_float()), ty_float()),
            ty_float(),
        );
        let cont = c_return(c_atom(c_var("x", ty_float()), ty_float()));
        let expected = c_seq(
            vec![c_assign(
                "x",
                c_bin_op(BinOp::Mul, c_var("a", ty_float()), c_var("b", ty_float()), ty_float()),
            )],
            cont.clone(),
        );
        let (tail, blocks) = run_assign("x", input, cont);
        assert_eq!(tail, expected);
        assert!(blocks.is_empty());
    }

    #[test]
    fn assign_unaryop() {
        let input =
            anf_complex(co_unary(UnaryOp::Not, a_var("a", ty_bool()), ty_bool()), ty_bool());
        let cont = c_return(c_atom(c_var("x", ty_bool()), ty_bool()));
        let expected = c_seq(
            vec![c_assign("x", c_unary(UnaryOp::Not, c_var("a", ty_bool()), ty_bool()))],
            cont.clone(),
        );
        let (tail, blocks) = run_assign("x", input, cont);
        assert_eq!(tail, expected);
        assert!(blocks.is_empty());
    }

    #[test]
    fn assign_app_is_non_tail_call() {
        let a_ty = ty_int();
        let b_ty = ty_int();
        let result_ty = ty_int();
        let f_ty = ty_arrow(a_ty.clone(), ty_arrow(b_ty.clone(), result_ty.clone()));
        let input = anf_complex(
            co_app(
                a_var("f", f_ty.clone()),
                vec![a_var("a", a_ty.clone()), a_var("b", b_ty.clone())],
                result_ty.clone(),
            ),
            result_ty.clone(),
        );
        let cont = c_return(c_atom(c_var("x", result_ty.clone()), result_ty.clone()));
        let expected = c_seq(
            vec![c_assign(
                "x",
                c_call(c_var("f", f_ty), vec![c_var("a", a_ty), c_var("b", b_ty)], result_ty),
            )],
            cont.clone(),
        );
        let (tail, blocks) = run_assign("x", input, cont);
        assert_eq!(tail, expected);
        assert!(blocks.is_empty());
    }

    #[test]
    fn assign_if_both_branches_assign_and_share_cont() {
        let input = anf_complex(
            co_if(
                a_var("c", ty_bool()),
                anf_complex(co_atom(a_int(1), ty_int()), ty_int()),
                anf_complex(co_atom(a_int(2), ty_int()), ty_int()),
                ty_int(),
            ),
            ty_int(),
        );
        let cont = c_return(c_atom(c_var("x", ty_int()), ty_int()));
        let expected = c_tail_if(
            c_var("c", ty_bool()),
            c_seq(vec![c_assign("x", c_atom(c_int(1), ty_int()))], c_goto("block$0".to_string())),
            c_seq(vec![c_assign("x", c_atom(c_int(2), ty_int()))], c_goto("block$0".to_string())),
        );
        let expected_blocks = vec![CBlock { label: "block$0".to_string(), tail: cont.clone() }];
        let (tail, blocks) = run_assign("x", input, cont);
        assert_eq!(tail, expected);
        assert_eq!(blocks, expected_blocks);
    }

    #[test]
    fn assign_let_forwards_correctly() {
        let input = anf_let(
            "a",
            co_atom(a_int(1), ty_int()),
            anf_complex(co_atom(a_var("a", ty_int()), ty_int()), ty_int()),
            ty_int(),
        );
        let cont = c_return(c_atom(c_var("x", ty_int()), ty_int()));
        let expected = c_seq(
            vec![c_assign("a", c_atom(c_int(1), ty_int()))],
            c_seq(vec![c_assign("x", c_atom(c_var("a", ty_int()), ty_int()))], cont.clone()),
        );
        let (tail, blocks) = run_assign("x", input, cont);
        assert_eq!(tail, expected);
        assert!(blocks.is_empty());
    }

    #[test]
    fn assign_true_false_atoms() {
        let input = anf_complex(co_atom(a_bool(true), ty_bool()), ty_bool());
        let cont = c_return(c_atom(c_var("x", ty_bool()), ty_bool()));
        let expected = c_seq(vec![c_assign("x", c_atom(c_bool(true), ty_bool()))], cont.clone());
        let (tail, blocks) = run_assign("x", input, cont);
        assert_eq!(tail, expected);
        assert!(blocks.is_empty());
    }
}
