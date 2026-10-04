use std::collections::{HashMap, HashSet};

use crate::{
    gensym::Gensym,
    reveal_functions::{RevealDef, RevealExpr, RevealProgram},
    syntax::{BinOp, HasType, Ident, PrimIO, Type, UnaryOp},
    typechecker::build_arrow,
};

#[derive(Debug, Clone, PartialEq)]
pub struct Closure {
    func_arity: usize,
    func_name: Ident,
    func_ty: Type,
    free_vars: Vec<(Ident, ClosureType)>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ClosureType {
    Unit,
    Bool,
    Float,
    Int,
    // A closure value. The precise free-variable types are erased here; they
    // live in `Closure::free_vars` and in the `ClosureFreeVar` node's type.
    Closure,
    Tuple(Vec<ClosureType>),
    Arrow(Box<ClosureType>, Box<ClosureType>),
}
impl std::fmt::Display for ClosureType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ClosureType::Unit => write!(f, "Unit"),
            ClosureType::Bool => write!(f, "Bool"),
            ClosureType::Float => write!(f, "Float"),
            ClosureType::Int => write!(f, "Int"),
            ClosureType::Closure => write!(f, "Closure"),
            ClosureType::Tuple(elems) => {
                let elems_str = elems
                    .iter()
                    .map(|ty| format!("{}", ty))
                    .collect::<Vec<_>>()
                    .join(", ");
                write!(f, "({})", elems_str)
            },
            ClosureType::Arrow(from, to) => {
                if matches!(**from, ClosureType::Arrow(_, _) | ClosureType::Tuple(_)) {
                    write!(f, "({}) -> {}", from, to)
                } else {
                    write!(f, "{} -> {}", from, to)
                }
            },
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ClosureExpr {
    Unit,
    Bool(bool),
    Int(i64),
    Float(f64),
    Var(Ident, ClosureType),
    BinOp(BinOp, Box<ClosureExpr>, Box<ClosureExpr>, ClosureType),
    Closure(Closure, ClosureType),
    ClosureFunPtr(Box<ClosureExpr>, ClosureType), // get function pointer of closure (index 0)
    ClosureFreeVar(Box<ClosureExpr>, usize, ClosureType), // get a free variable of closure (index 1..)
    Tuple(Vec<ClosureExpr>, ClosureType),
    TupleProj(Box<ClosureExpr>, usize, ClosureType),
    PrimIO(PrimIO, Option<Box<ClosureExpr>>, ClosureType),
    UnaryOp(UnaryOp, Box<ClosureExpr>, ClosureType),
    If(Box<ClosureExpr>, Box<ClosureExpr>, Box<ClosureExpr>, ClosureType),
    Let(
        Ident,
        ClosureType, // rhs's type
        Box<ClosureExpr>,
        Box<ClosureExpr>,
        ClosureType,
    ),
    // LetRec(
    //     Ident,                     // function name
    //     Vec<(Ident, ClosureType)>, // function parameters with their types
    //     ClosureType,               // function return type
    //     Box<ClosureExpr>,          // function body
    //     Box<ClosureExpr>,          // expression after the let rec
    //     ClosureType,
    // ),
    App(Box<ClosureExpr>, Vec<ClosureExpr>, ClosureType),
}
impl std::fmt::Display for ClosureExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ClosureExpr::Unit => write!(f, "()"),
            ClosureExpr::Bool(b) => write!(f, "{}", b),
            ClosureExpr::Int(n) => write!(f, "{}", n),
            ClosureExpr::Float(fl) => write!(f, "{}", fl),
            ClosureExpr::Var(name, ty) => write!(f, "({} : {})", name, ty),
            ClosureExpr::BinOp(op, left, right, _) => {
                write!(f, "{} {} {}", left, op, right)
            },
            ClosureExpr::Closure(clos, _) => {
                write!(
                    f,
                    "Closure (name: {}, arity: {}, free_vars: {:?})",
                    clos.func_name, clos.func_arity, clos.free_vars,
                )
            },
            ClosureExpr::ClosureFunPtr(clos, ty) => {
                write!(f, "((ClosureFunPtr {}) : {})", clos, ty)
            },
            ClosureExpr::ClosureFreeVar(clos, index, _) => {
                write!(f, "(ClosureFreeVar {} {})", clos, index)
            },
            ClosureExpr::Tuple(elems, _) => {
                let elems_str =
                    elems.iter().map(|e| format!("{}", e)).collect::<Vec<_>>().join(", ");
                write!(f, "{}", elems_str)
            },
            ClosureExpr::TupleProj(expr, idx, _) => {
                write!(f, "({}).{}", expr, idx)
            },
            ClosureExpr::PrimIO(prim, expr, _) => {
                if let Some(expr) = expr {
                    write!(f, "{} {}", prim, expr)
                } else {
                    write!(f, "{}", prim)
                }
            },
            ClosureExpr::UnaryOp(op, expr, _) => {
                write!(f, "{} {}", op, expr)
            },
            ClosureExpr::If(cond, thn, els, _) => {
                write!(f, "if {} then {} else {}", cond, thn, els)
            },
            ClosureExpr::Let(name, _, rhs, body, _) => {
                write!(f, "let {} = {} in {}", name, rhs, body)
            },
            ClosureExpr::App(func, args, _) => {
                let args_str = args
                    .iter()
                    .map(|arg| format!("{}", arg))
                    .collect::<Vec<_>>()
                    .join(" ");
                write!(f, "({} {})", func, args_str)
            },
        }
    }
}

impl ClosureExpr {
    fn type_of(&self) -> ClosureType {
        match self {
            ClosureExpr::Unit => ClosureType::Unit,
            ClosureExpr::Bool(_) => ClosureType::Bool,
            ClosureExpr::Int(_) => ClosureType::Int,
            ClosureExpr::Float(_) => ClosureType::Float,
            ClosureExpr::Var(_, ty) => (*ty).clone(),
            ClosureExpr::BinOp(_, _, _, ty) => (*ty).clone(),
            ClosureExpr::Closure(_, ty) => (*ty).clone(),
            ClosureExpr::ClosureFunPtr(_, ty) => (*ty).clone(),
            ClosureExpr::ClosureFreeVar(_, _, ty) => (*ty).clone(),
            ClosureExpr::Tuple(_, ty) => (*ty).clone(),
            ClosureExpr::TupleProj(_, _, ty) => (*ty).clone(),
            ClosureExpr::PrimIO(_, _, ty) => (*ty).clone(),
            ClosureExpr::UnaryOp(_, _, ty) => (*ty).clone(),
            ClosureExpr::If(_, _, _, ty) => (*ty).clone(),
            ClosureExpr::Let(_, _, _, _, ty) => (*ty).clone(),
            // ClosureExpr::LetRec(_, _, _, _, _, ty) => (*ty).clone(),
            ClosureExpr::App(_, _, ty) => (*ty).clone(),
        }
    }
}

pub fn clos_unit() -> ClosureExpr {
    ClosureExpr::Unit
}

pub fn clos_bool(b: bool) -> ClosureExpr {
    ClosureExpr::Bool(b)
}

pub fn clos_int(n: i64) -> ClosureExpr {
    ClosureExpr::Int(n)
}

pub fn clos_float(f: f64) -> ClosureExpr {
    ClosureExpr::Float(f)
}

pub fn clos_clos(clos: Closure, ty: ClosureType) -> ClosureExpr {
    ClosureExpr::Closure(clos, ty)
}

pub fn clos_clos_fun_ptr(clos: ClosureExpr, ty: ClosureType) -> ClosureExpr {
    ClosureExpr::ClosureFunPtr(Box::new(clos), ty)
}

pub fn clos_clos_free_var(clos: ClosureExpr, index: usize, ty: ClosureType) -> ClosureExpr {
    ClosureExpr::ClosureFreeVar(Box::new(clos), index, ty)
}

pub fn clos_var(name: impl Into<Ident>, ty: ClosureType) -> ClosureExpr {
    ClosureExpr::Var(name.into(), ty)
}

pub fn clos_tuple(elems: Vec<ClosureExpr>, ty: ClosureType) -> ClosureExpr {
    ClosureExpr::Tuple(elems, ty)
}

pub fn clos_tuple_projection(expr: ClosureExpr, index: usize, ty: ClosureType) -> ClosureExpr {
    ClosureExpr::TupleProj(Box::new(expr), index, ty)
}

pub fn clos_bin_op(
    op: BinOp,
    left: ClosureExpr,
    right: ClosureExpr,
    ty: ClosureType,
) -> ClosureExpr {
    ClosureExpr::BinOp(op, Box::new(left), Box::new(right), ty)
}

pub fn clos_prim_io(prim: PrimIO, expr: Option<ClosureExpr>, ty: ClosureType) -> ClosureExpr {
    ClosureExpr::PrimIO(prim, expr.map(Box::new), ty)
}

pub fn clos_unary(op: UnaryOp, expr: ClosureExpr, ty: ClosureType) -> ClosureExpr {
    ClosureExpr::UnaryOp(op, Box::new(expr), ty)
}

pub fn clos_if(
    cond: ClosureExpr,
    thn: ClosureExpr,
    els: ClosureExpr,
    ty: ClosureType,
) -> ClosureExpr {
    ClosureExpr::If(Box::new(cond), Box::new(thn), Box::new(els), ty)
}

pub fn clos_let(
    name: impl Into<Ident>,
    ty: ClosureType,
    rhs: ClosureExpr,
    body: ClosureExpr,
    let_ty: ClosureType,
) -> ClosureExpr {
    ClosureExpr::Let(name.into(), ty, Box::new(rhs), Box::new(body), let_ty)
}

// pub fn clos_let_rec(
//     name: impl Into<Ident>,
//     params: Vec<(Ident, ClosureType)>,
//     ret_ty: ClosureType,
//     fbody: ClosureExpr,
//     body: ClosureExpr,
//     ty: ClosureType,
// ) -> ClosureExpr {
//     ClosureExpr::LetRec(name.into(), params, ret_ty, Box::new(fbody), Box::new(body), ty)
// }

pub fn clos_app(func: ClosureExpr, args: Vec<ClosureExpr>, ty: ClosureType) -> ClosureExpr {
    ClosureExpr::App(Box::new(func), args, ty)
}

#[derive(Debug, Clone, PartialEq)]
pub enum ClosureDef {
    ValDef(
        Ident,               // name
        Option<ClosureType>, // optional type annotation
        ClosureExpr,         // expresion
    ),
    FunDef(
        Ident,                     // function name
        Vec<(Ident, ClosureType)>, // function arguments with their types
        ClosureType,               // function return type
        ClosureExpr,               // function body
    ),
}

impl std::fmt::Display for ClosureDef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ClosureDef::ValDef(name, ty, expr) => {
                if let Some(ty) = ty {
                    write!(f, "let {} : {} = {}", name, ty, expr)
                } else {
                    write!(f, "let {} = {}", name, expr)
                }
            },
            ClosureDef::FunDef(name, params, ret_ty, body) => {
                let params_str = params
                    .iter()
                    .map(|(name, ty)| format!("{} : {}", name, ty))
                    .collect::<Vec<_>>()
                    .join(", ");
                write!(f, "let {} ({}) : {} = {}", name, params_str, ret_ty, body)
            },
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ClosureProgram {
    defs: Vec<ClosureDef>,
    main: ClosureExpr,
}

impl std::fmt::Display for ClosureProgram {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let defs_str = self
            .defs
            .iter()
            .map(|def| format!("{}", def))
            .collect::<Vec<_>>()
            .join("\n");
        write!(f, "{}\n{}", defs_str, self.main)
    }
}

/// Translate a source type into its runtime (closure-converted) type.
///
/// A source function type denotes a *closure value*: a tuple
/// `(fun_ptr, free_var_1, ..., free_var_k)` whose function pointer takes the
/// whole closure as its first argument. So the type translation adds exactly
/// one environment slot at the front of every function type:
///
///     A -> B        ==>   Closure -> (A' -> B')
///     A -> (B -> C)  ==>   Closure -> (A' -> (B' -> C'))   (arity 2)
///
/// Multi-argument types are right-nested in the source AST
/// (`A -> B -> C` is `A -> (B -> C)`), so we flatten the leading arrows into a
/// parameter list and rebuild them after adding the environment slot.
fn convert_type(ty: Type) -> ClosureType {
    match ty {
        Type::Unit => ClosureType::Unit,
        Type::Bool => ClosureType::Bool,
        Type::Float => ClosureType::Float,
        Type::Int => ClosureType::Int,
        Type::Tuple(tys) => ClosureType::Tuple(tys.into_iter().map(convert_type).collect()),
        Type::Arrow(from, to) => {
            let mut params = vec![*from];
            let mut ret = *to;
            while let Type::Arrow(param, rest) = ret {
                params.push(*param);
                ret = *rest;
            }
            convert_fun_type(params, ret)
        },
        Type::Var(_) => unreachable!(),
        Type::Dummy => unreachable!(),
    }
}

/// Translate an n-ary function type from its parameter types and return type,
/// adding exactly one environment slot at the front.
fn convert_fun_type(params: Vec<Type>, ret: Type) -> ClosureType {
    let mut acc = convert_type(ret);
    for param in params.into_iter().rev() {
        acc = ClosureType::Arrow(Box::new(convert_type(param)), Box::new(acc));
    }
    ClosureType::Arrow(Box::new(ClosureType::Closure), Box::new(acc))
}

pub fn convert_to_closure(
    reveal: RevealExpr,
    gensym: &mut Gensym,
    lifted: &mut Vec<ClosureDef>,
) -> ClosureExpr {
    convert_expr(reveal, gensym, lifted, &HashMap::new())
}

fn convert_expr(
    reveal: RevealExpr,
    gensym: &mut Gensym,
    lifted: &mut Vec<ClosureDef>,
    closure_env: &HashMap<Ident, (Closure, ClosureType)>, // for LetRec
) -> ClosureExpr {
    match reveal {
        RevealExpr::Unit => clos_unit(),
        RevealExpr::Bool(b) => clos_bool(b),
        RevealExpr::Int(i) => clos_int(i),
        RevealExpr::Float(f) => clos_float(f),
        RevealExpr::Var(name, ty) => {
            // LetRec
            if let Some((clos, ty)) = closure_env.get(&name) {
                clos_clos((*clos).clone(), (*ty).clone())
            } else {
                clos_var(name, convert_type(ty))
            }
        },
        RevealExpr::FunRef(name, arity, ty) => {
            // A recursive function's self-reference resolves to the closure
            // value created for it, not to a fresh empty closure.
            if let Some((clos, ty)) = closure_env.get(&name) {
                clos_clos((*clos).clone(), (*ty).clone())
            } else {
                let clos_ty = convert_type(ty.clone());
                let clos =
                    Closure { func_arity: arity, func_name: name, func_ty: ty, free_vars: vec![] };
                clos_clos(clos, clos_ty)
            }
        },
        RevealExpr::BinOp(op, left, right, ty) => {
            let clos_left = convert_expr(*left, gensym, lifted, closure_env);
            let clos_right = convert_expr(*right, gensym, lifted, closure_env);
            clos_bin_op(op, clos_left, clos_right, convert_type(ty))
        },
        RevealExpr::Tuple(reveal_exprs, ty) => {
            let elems = reveal_exprs
                .into_iter()
                .map(|e| convert_expr(e, gensym, lifted, closure_env))
                .collect::<Vec<_>>();
            clos_tuple(elems, convert_type(ty))
        },
        RevealExpr::TupleProj(reveal_expr, idx, ty) => {
            let clos_expr = convert_expr(*reveal_expr, gensym, lifted, closure_env);
            clos_tuple_projection(clos_expr, idx, convert_type(ty))
        },
        RevealExpr::PrimIO(prim, expr, ty) => {
            if let Some(reveal_expr) = expr {
                let clos_expr = convert_expr(*reveal_expr, gensym, lifted, closure_env);
                clos_prim_io(prim, Some(clos_expr), convert_type(ty))
            } else {
                clos_prim_io(prim, None, convert_type(ty))
            }
        },
        RevealExpr::UnaryOp(op, reveal_expr, ty) => {
            let clos_expr = convert_expr(*reveal_expr, gensym, lifted, closure_env);
            clos_unary(op, clos_expr, convert_type(ty))
        },
        RevealExpr::If(cond, then_e, else_e, ty) => {
            let clos_cond = convert_expr(*cond, gensym, lifted, closure_env);
            let clos_then = convert_expr(*then_e, gensym, lifted, closure_env);
            let clos_else = convert_expr(*else_e, gensym, lifted, closure_env);
            clos_if(clos_cond, clos_then, clos_else, convert_type(ty))
        },
        RevealExpr::Let(name, rhs_ty, rhs, body, ty) => {
            let clos_rhs = convert_expr(*rhs, gensym, lifted, closure_env);
            let clos_body = convert_expr(*body, gensym, lifted, closure_env);
            clos_let(name, convert_type(rhs_ty), clos_rhs, clos_body, convert_type(ty))
        },
        RevealExpr::LetRec(fname, fparams, f_ret_ty, fbody, body, _) => {
            //   let y = 5 in
            //   let rec f (x : Int) : Int = if x == 0 then y else f (x - 1) in
            //   f 3
            //
            //   After conversion:
            //     1) Lift the recursive function; the self-reference `f`
            //        resolves to the closure itself:
            //          def lambda$N (clos : Closure) (x : Int) : Int =
            //            let y = ClosureFreeVar clos 1 in
            //            if x == 0 then y else ClosureFunPtr clos (x - 1)
            //     2) Bind `f` to a closure, then convert the body:
            //          let y = 5 in
            //          let f = Closure { func_name = lambda$N, free_vars = [y] } in
            //          (ClosureFunPtr f 3)
            let fbody_frees = {
                let mut fbody_bound = fparams
                    .iter()
                    .map(|(name, _)| name.to_string())
                    .collect::<HashSet<Ident>>();
                fbody_bound.insert(fname.clone());

                free_vars(&*fbody, &fbody_bound)
            };

            let f_func_ty = build_arrow(
                fparams.iter().map(|(_, ty)| (*ty).clone()).collect::<Vec<Type>>(),
                f_ret_ty,
            );
            // The runtime type of the closure value bound to `f` (and of the
            // lifted function's first parameter, which receives that closure).
            let clos_f_func_ty = convert_type(f_func_ty.clone());
            let clos_f_func = Closure {
                func_arity: fparams.len(),
                func_ty: f_func_ty.clone(),
                func_name: fname.clone(),
                free_vars: fbody_frees
                    .iter()
                    .map(|(name, ty)| (name.to_string(), convert_type((*ty).clone())))
                    .collect::<Vec<_>>(),
            };
            let mut fbody_closure_env = closure_env.clone();
            fbody_closure_env.insert(fname.clone(), (clos_f_func.clone(), clos_f_func_ty.clone()));
            let mut new_fbody = convert_expr(*fbody, gensym, lifted, &fbody_closure_env);
            let new_fbody_ty = new_fbody.type_of();

            let def_name = gensym.fresh_with_prefix("lambda");
            let clos_name = gensym.fresh_with_prefix("clos");
            let clos_ty = clos_f_func_ty.clone();
            let clos = clos_var(clos_name.clone(), clos_ty.clone());

            let mut def_params = Vec::with_capacity(fparams.len() + 1);
            def_params.push((clos_name, clos_ty));
            fparams.iter().for_each(|(name, ty)| {
                def_params.push((name.to_string(), convert_type((*ty).clone())));
            });

            for (idx, (name, ty)) in fbody_frees.clone().into_iter().enumerate() {
                let ty = convert_type(ty);
                let rhs = clos_clos_free_var(clos.clone(), idx + 1, ty.clone());
                new_fbody = clos_let(name, ty, rhs, new_fbody.clone(), new_fbody_ty.clone());
            }

            lifted.push(ClosureDef::FunDef(def_name.clone(), def_params, new_fbody_ty, new_fbody));

            let new_body = convert_expr(*body, gensym, lifted, &fbody_closure_env);
            let new_body_ty = new_body.type_of();
            clos_let(
                fname,
                clos_f_func_ty.clone(),
                clos_clos(clos_f_func, clos_f_func_ty),
                new_body,
                new_body_ty,
            )
        },
        RevealExpr::App(func, args, ty) => {
            // Apply func args
            // =>
            // Let clos_name = func′ in
            // Apply (ClosureFunPtr (Var clos_name) 0) ((Var clos_name) args')

            let clos_func = convert_expr(*func, gensym, lifted, closure_env);
            let func_ty = clos_func.type_of();

            let clos_name = gensym.fresh_with_prefix("tmp");
            let clos_var = clos_var(clos_name.clone(), func_ty.clone());

            // `func_ty` is `Closure -> arg1' -> ... -> ret'`. The function
            // pointer stored inside the closure takes the closure value itself
            // as its first argument, so its type is `func_ty -> arg1' -> ...`.
            let rest_ty = match &func_ty {
                ClosureType::Arrow(_, rest) => (**rest).clone(),
                _ => unreachable!(),
            };
            let func_ptr_ty = ClosureType::Arrow(Box::new(func_ty.clone()), Box::new(rest_ty));
            let new_func = clos_clos_fun_ptr(clos_var.clone(), func_ptr_ty);

            let mut new_args = Vec::with_capacity(args.len() + 1);
            new_args.push(clos_var.clone());
            args.into_iter().for_each(|arg| {
                let new_arg = convert_expr(arg, gensym, lifted, closure_env);
                new_args.push(new_arg);
            });

            let let_ty = convert_type(ty);
            let let_body = clos_app(new_func, new_args, let_ty.clone());
            clos_let(clos_name, func_ty, clos_func, let_body, let_ty)
        },

        RevealExpr::Lambda(params, body, lambda_ty) => {
            // fun (x : Int) : Int => x + y      // y is a free variable
            // =>
            // 1) Lift a top-level function (first parameter is `clos`, followed
            //    by the original parameters):
            //          def lambda$N (clos : Closure) (x : Int) : Int =
            //            let y = ClosureFreeVar clos 1 in   // free variable loaded from the closure
            //            x + y
            // 2) Replace the lambda with a closure value:
            //          Closure { arity = 1, func_name = lambda$N, free_vars = [y] }
            //    i.e. the runtime tuple (func_ptr(lambda$N), y)
            //

            let bound = params
                .iter()
                .map(|(name, _)| name.to_string())
                .collect::<HashSet<Ident>>();
            let frees = free_vars(&body, &bound);

            let clos_param_name = gensym.fresh_with_prefix("clos");
            // The lifted function receives the *whole closure value* as its
            // first argument, so this type matches the closure's own type.
            let clos_param_ty = convert_type(lambda_ty.clone());
            let clos_param = clos_var(clos_param_name.clone(), clos_param_ty.clone());

            let ret_ty = convert_type(body.type_of());
            let mut new_body = convert_expr(*body, gensym, lifted, closure_env);

            for (idx, (name, ty)) in frees.iter().enumerate() {
                let fv_ty = convert_type((*ty).clone());
                let rhs = clos_clos_free_var(clos_param.clone(), idx + 1, fv_ty.clone());
                new_body = clos_let(name.to_string(), fv_ty, rhs, new_body, ret_ty.clone())
            }

            let def_name = gensym.fresh_with_prefix("lambda");
            let mut def_params = Vec::with_capacity(params.len() + 1);
            def_params.push((clos_param_name, clos_param_ty));
            for (param_name, param_ty) in &params {
                def_params.push((param_name.to_string(), convert_type((*param_ty).clone())));
            }
            lifted.push(ClosureDef::FunDef(def_name.clone(), def_params, ret_ty, new_body));

            let clos_ty = convert_type(lambda_ty.clone());
            ClosureExpr::Closure(
                Closure {
                    func_arity: params.len(),
                    func_name: def_name,
                    func_ty: lambda_ty,
                    free_vars: frees
                        .into_iter()
                        .map(|(name, ty)| (name, convert_type(ty)))
                        .collect(),
                },
                clos_ty,
            )
        },
    }
}

pub fn convert_to_closure_def(
    def: RevealDef,
    gensym: &mut Gensym,
    lifted: &mut Vec<ClosureDef>,
) -> ClosureDef {
    match def {
        RevealDef::ValDef(name, ty, reveal_expr) => {
            let clos_expr = convert_to_closure(reveal_expr, gensym, lifted);
            ClosureDef::ValDef(name, ty.map(|ty| convert_type(ty)), clos_expr)
        },
        RevealDef::FunDef(name, params, ret_ty, reveal_expr) => {
            let clos_name = gensym.fresh_with_prefix("clos");
            // Top-level functions are the function pointers of their own
            // (empty) closures, so the environment parameter receives the
            // closure value of this function's type.
            let param_tys = params.iter().map(|(_, ty)| (*ty).clone()).collect::<Vec<_>>();
            let clos_ty = convert_type(build_arrow(param_tys, ret_ty.clone()));
            let clos_body = convert_to_closure(reveal_expr, gensym, lifted);
            let mut def_params = vec![(clos_name, clos_ty)];
            def_params.extend(
                params
                    .into_iter()
                    .map(|(name, ty)| (name, convert_type(ty)))
                    .collect::<Vec<_>>(),
            );
            ClosureDef::FunDef(name, def_params, convert_type(ret_ty), clos_body)
        },
    }
}

pub fn convert_to_closure_prog(prog: RevealProgram) -> ClosureProgram {
    let mut gensym = Gensym::new();
    let mut lifted = vec![];
    let mut clos_defs = vec![];
    for def in prog.defs {
        let clos_def = convert_to_closure_def(def, &mut gensym, &mut lifted);
        clos_defs.push(clos_def);
    }

    let clos_main = convert_to_closure(prog.main, &mut gensym, &mut lifted);
    clos_defs.extend(lifted);
    ClosureProgram { defs: clos_defs, main: clos_main }
}

fn free_vars(expr: &RevealExpr, bound: &HashSet<Ident>) -> Vec<(Ident, Type)> {
    let mut frees_map = HashMap::new();
    free_vars_rec(expr, bound, &mut frees_map);
    let mut frees = frees_map
        .into_iter()
        .map(|(name, ty)| (name, ty))
        .collect::<Vec<(Ident, Type)>>();
    frees.sort_by(|(name0, _), (name1, _)| name0.cmp(name1));
    frees
}

fn free_vars_rec(expr: &RevealExpr, bound: &HashSet<Ident>, frees_map: &mut HashMap<Ident, Type>) {
    match expr {
        RevealExpr::Unit => {},
        RevealExpr::Bool(_) => {},
        RevealExpr::Int(_) => {},
        RevealExpr::Float(_) => {},
        RevealExpr::Var(name, ty) => {
            if !bound.contains(name) {
                frees_map.insert(name.to_string(), (*ty).clone());
            }
        },
        RevealExpr::FunRef(_, _, _) => {},
        RevealExpr::BinOp(_, left, right, _) => {
            free_vars_rec(left, bound, frees_map);
            free_vars_rec(right, bound, frees_map);
        },
        RevealExpr::Tuple(reveal_exprs, _) => {
            reveal_exprs.iter().for_each(|e| {
                free_vars_rec(e, bound, frees_map);
            });
        },
        RevealExpr::TupleProj(reveal_expr, _, _) => {
            free_vars_rec(reveal_expr, bound, frees_map);
        },
        RevealExpr::PrimIO(_, reveal_expr, _) => {
            if let Some(reveal_expr) = reveal_expr {
                free_vars_rec(reveal_expr, bound, frees_map);
            }
        },
        RevealExpr::UnaryOp(_, reveal_expr, _) => {
            free_vars_rec(reveal_expr, bound, frees_map);
        },
        RevealExpr::If(cond, thn, els, _) => {
            free_vars_rec(cond, bound, frees_map);
            free_vars_rec(thn, bound, frees_map);
            free_vars_rec(els, bound, frees_map);
        },
        RevealExpr::Let(name, _, rhs, body, _) => {
            free_vars_rec(rhs, bound, frees_map);

            let mut body_bound = bound.clone();
            body_bound.insert(name.clone());
            free_vars_rec(body, &body_bound, frees_map);
        },
        RevealExpr::LetRec(fname, fparams, _, fbody, body, _) => {
            let mut fbody_bound = bound.clone();
            fbody_bound.insert(fname.to_string());
            fparams.iter().for_each(|(name, _)| {
                fbody_bound.insert(name.to_string());
            });
            free_vars_rec(fbody, &fbody_bound, frees_map);

            let mut body_bound = bound.clone();
            body_bound.insert(fname.to_string());
            free_vars_rec(body, &body_bound, frees_map);
        },
        RevealExpr::App(func, args, _) => {
            free_vars_rec(func, bound, frees_map);
            args.iter().for_each(|e| {
                free_vars_rec(e, bound, frees_map);
            });
        },
        RevealExpr::Lambda(params, body, _) => {
            let mut body_bound = bound.clone();
            params.iter().for_each(|(name, _)| {
                body_bound.insert(name.to_string());
            });
            free_vars_rec(body, &body_bound, frees_map);
        },
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::reveal_functions::{
        r_app, r_bin_op, r_fun_ref, r_if, r_int, r_lambda, r_let, r_let_rec, r_var,
    };
    use crate::syntax::{BinOp, ty_arrow, ty_bool, ty_int};

    #[test]
    fn converts_simple_lambda_application() {
        // let f = fun x => 1 + x
        // in f 37
        let lambda_body = r_bin_op(BinOp::Add, r_int(1), r_var("x", ty_int()), ty_int());
        let lambda_ty = ty_arrow(ty_int(), ty_int());
        let lambda = r_lambda(vec![("x".into(), ty_int())], lambda_body, lambda_ty.clone());

        let let_body = r_app(r_var("f", lambda_ty.clone()), vec![r_int(37)], ty_int());
        let let_expr = r_let("f", lambda_ty, lambda, let_body, ty_int());

        let mut lifted = Vec::new();
        let clos = convert_expr(let_expr, &mut Gensym::new(), &mut lifted, &HashMap::new());

        assert_eq!(lifted.len(), 1, "exactly one lambda should be lifted");
        match &lifted[0] {
            ClosureDef::FunDef(name, params, ret_ty, _) => {
                assert!(name.starts_with("lambda"));
                assert_eq!(params.len(), 2, "clos param + x");
                assert_eq!(*ret_ty, ClosureType::Int);
            },
            other => panic!("expected FunDef, got {:?}", other),
        }

        match clos {
            ClosureExpr::Let(name, _, rhs, body, _) => {
                assert_eq!(name, "f");
                assert!(matches!(*rhs, ClosureExpr::Closure(_, _)), "f binds a closure value");
                match *body {
                    ClosureExpr::Let(_, _, _, app, _) => {
                        assert!(
                            matches!(*app, ClosureExpr::App(_, _, _)),
                            "f 37 is an application"
                        );
                    },
                    other => panic!("expected Let wrapping an App, got {:?}", other),
                }
            },
            other => panic!("expected Let, got {:?}", other),
        }
    }
    #[test]
    fn converts_function_typed_argument() {
        // let inc = fun x => x + 1 in
        // let double (f: Int -> Int) (x: Int) = f (f x) in
        // in double inc 3
        let inc_body = r_bin_op(BinOp::Add, r_var("x", ty_int()), r_int(1), ty_int());
        let inc_ty = ty_arrow(ty_int(), ty_int());
        let inc_lambda = r_lambda(vec![("x".into(), ty_int())], inc_body, inc_ty.clone());
        let double_ty = ty_arrow(ty_arrow(ty_int(), ty_int()), ty_arrow(ty_int(), ty_int()));
        let inc_let_body = r_let(
            "double",
            double_ty.clone(),
            r_lambda(
                vec![("f".into(), ty_arrow(ty_int(), ty_int())), ("x".into(), ty_int())],
                r_app(
                    r_var("f", ty_arrow(ty_int(), ty_int())),
                    vec![r_app(
                        r_var("f", ty_arrow(ty_int(), ty_int())),
                        vec![r_var("x", ty_int())],
                        ty_int(),
                    )],
                    ty_int(),
                ),
                double_ty.clone(),
            ),
            r_app(
                r_var("double", double_ty),
                vec![r_var("inc", inc_ty.clone()), r_int(3)],
                ty_int(),
            ),
            ty_int(),
        );
        let let_expr = r_let("inc", inc_ty, inc_lambda, inc_let_body, ty_int());
        let mut lifted = Vec::new();
        let _clos = convert_expr(let_expr, &mut Gensym::new(), &mut lifted, &HashMap::new());

        assert_eq!(lifted.len(), 2, "`inc` and `double` are both lifted");
        let double_params = lifted
            .iter()
            .find_map(|def| match def {
                ClosureDef::FunDef(_, params, _, _) if params.len() == 3 => Some(params),
                _ => None,
            })
            .expect("expected the `double` lambda to have 3 params");

        // (clos, f, x) -- the function-typed parameter `f` becomes a closure type.
        let expected_f_ty = ClosureType::Arrow(
            Box::new(ClosureType::Closure),
            Box::new(ClosureType::Arrow(Box::new(ClosureType::Int), Box::new(ClosureType::Int))),
        );
        assert_eq!(&double_params[1].1, &expected_f_ty);
    }
    #[test]
    fn converts_captured_variable_closure() {
        // let triple (f: Int -> Int) (x: Int) = f (f (f x)) in
        // let x = 10 in
        // let f = fun y => x + y in
        // triple f 3
        let triple_body = r_app(
            r_var("f", ty_arrow(ty_int(), ty_int())),
            vec![r_app(
                r_var("f", ty_arrow(ty_int(), ty_int())),
                vec![r_app(
                    r_var("f", ty_arrow(ty_int(), ty_int())),
                    vec![r_var("x", ty_int())],
                    ty_int(),
                )],
                ty_int(),
            )],
            ty_int(),
        );
        let triple_ty = ty_arrow(ty_arrow(ty_int(), ty_int()), ty_arrow(ty_int(), ty_int()));
        let triple_lambda = r_lambda(
            vec![("f".into(), ty_arrow(ty_int(), ty_int())), ("x".into(), ty_int())],
            triple_body,
            triple_ty.clone(),
        );
        let triple_let_body = r_let(
            "x",
            ty_int(),
            r_int(10),
            r_let(
                "f",
                ty_arrow(ty_int(), ty_int()),
                r_lambda(
                    vec![("y".into(), ty_int())],
                    r_bin_op(BinOp::Add, r_var("x", ty_int()), r_var("y", ty_int()), ty_int()),
                    ty_arrow(ty_int(), ty_int()),
                ),
                r_app(
                    r_var(
                        "triple",
                        ty_arrow(ty_arrow(ty_int(), ty_int()), ty_arrow(ty_int(), ty_int())),
                    ),
                    vec![r_var("f", ty_arrow(ty_int(), ty_int())), r_int(3)],
                    ty_int(),
                ),
                ty_int(),
            ),
            ty_int(),
        );
        let let_expr = r_let("triple", triple_ty, triple_lambda, triple_let_body, ty_int());
        let mut lifted = Vec::new();
        let clos = convert_expr(let_expr, &mut Gensym::new(), &mut lifted, &HashMap::new());

        assert_eq!(lifted.len(), 2, "`triple` and the capturing lambda are lifted");

        // The capturing lambda's body projects the free variable `x` at index 1
        // (index 0 is the function pointer).
        let mut indices = Vec::new();
        for def in &lifted {
            if let ClosureDef::FunDef(_, _, _, body) = def {
                collect_free_var_indices(body, &mut indices);
            }
        }
        assert_eq!(indices, vec![1]);

        // The main expression contains a closure that captures `x : Int`.
        let mut closures = Vec::new();
        collect_closures(&clos, &mut closures);
        assert!(
            closures
                .iter()
                .any(|clos| clos.free_vars == vec![("x".to_string(), ClosureType::Int)])
        );
    }
    #[test]
    fn converts_let_rec() {
        let int_to_int = ty_arrow(ty_int(), ty_int());
        // let rec f (n: Int) : Int = if n == 0 then y else f (n - 1) in f 3
        let fbody = r_if(
            r_bin_op(BinOp::Eq, r_var("n", ty_int()), r_int(0), ty_bool()),
            r_var("y", ty_int()),
            r_app(
                r_fun_ref("f", 1, int_to_int.clone()),
                vec![r_bin_op(BinOp::Sub, r_var("n", ty_int()), r_int(1), ty_int())],
                ty_int(),
            ),
            ty_int(),
        );
        let body = r_app(r_fun_ref("f", 1, int_to_int.clone()), vec![r_int(3)], ty_int());
        let let_rec =
            r_let_rec("f", vec![("n".to_string(), ty_int())], ty_int(), fbody, body, ty_int());

        let mut gensym = Gensym::new();
        let mut defs = vec![];
        let result = convert_to_closure(let_rec, &mut gensym, &mut defs);

        assert_eq!(defs.len(), 1);
        match &defs[0] {
            ClosureDef::FunDef(name, params, ret_ty, _) => {
                assert!(name.starts_with("lambda"));
                assert_eq!(params.len(), 2, "clos param + n");
                assert_eq!(*ret_ty, ClosureType::Int);
            },
            other => panic!("expected FunDef, got {:?}", other),
        }

        match result {
            ClosureExpr::Let(name, _, rhs, _, _) => {
                assert_eq!(name, "f");
                assert!(matches!(*rhs, ClosureExpr::Closure(_, _)));
            },
            other => panic!("expected Let binding for f, got {:?}", other),
        }
    }

    #[test]
    fn fun_ref_inside_lambda_is_not_captured() {
        let int_to_int = ty_arrow(ty_int(), ty_int());
        // fun (x: Int) : Int => g x   where g is a top-level function
        let lambda = crate::reveal_functions::r_lambda(
            vec![("x".to_string(), ty_int())],
            r_app(r_fun_ref("g", 1, int_to_int.clone()), vec![r_var("x", ty_int())], ty_int()),
            int_to_int.clone(),
        );

        let mut gensym = Gensym::new();
        let mut defs = vec![];
        let result = convert_to_closure(lambda, &mut gensym, &mut defs);

        match &result {
            ClosureExpr::Closure(closure, _) => {
                assert!(closure.free_vars.is_empty(), "top-level FunRef must not be captured");
            },
            other => panic!("expected Closure, got {:?}", other),
        }
    }

    fn collect_free_var_indices(expr: &ClosureExpr, out: &mut Vec<usize>) {
        match expr {
            ClosureExpr::Unit
            | ClosureExpr::Bool(_)
            | ClosureExpr::Int(_)
            | ClosureExpr::Float(_)
            | ClosureExpr::Var(_, _)
            | ClosureExpr::Closure(_, _) => {},
            ClosureExpr::BinOp(_, left, right, _) => {
                collect_free_var_indices(left, out);
                collect_free_var_indices(right, out);
            },
            ClosureExpr::ClosureFunPtr(inner, _) => collect_free_var_indices(inner, out),
            ClosureExpr::ClosureFreeVar(inner, index, _) => {
                out.push(*index);
                collect_free_var_indices(inner, out);
            },
            ClosureExpr::Tuple(elems, _) => {
                elems.iter().for_each(|elem| collect_free_var_indices(elem, out));
            },
            ClosureExpr::TupleProj(inner, _, _) => collect_free_var_indices(inner, out),
            ClosureExpr::PrimIO(_, expr, _) => {
                if let Some(expr) = expr {
                    collect_free_var_indices(expr, out);
                }
            },
            ClosureExpr::UnaryOp(_, inner, _) => collect_free_var_indices(inner, out),
            ClosureExpr::If(cond, thn, els, _) => {
                collect_free_var_indices(cond, out);
                collect_free_var_indices(thn, out);
                collect_free_var_indices(els, out);
            },
            ClosureExpr::Let(_, _, rhs, body, _) => {
                collect_free_var_indices(rhs, out);
                collect_free_var_indices(body, out);
            },
            ClosureExpr::App(func, args, _) => {
                collect_free_var_indices(func, out);
                args.iter().for_each(|arg| collect_free_var_indices(arg, out));
            },
        }
    }

    fn collect_closures(expr: &ClosureExpr, out: &mut Vec<Closure>) {
        match expr {
            ClosureExpr::Closure(clos, _) => out.push(clos.clone()),
            ClosureExpr::BinOp(_, left, right, _) => {
                collect_closures(left, out);
                collect_closures(right, out);
            },
            ClosureExpr::ClosureFunPtr(inner, _) => collect_closures(inner, out),
            ClosureExpr::ClosureFreeVar(inner, _, _) => collect_closures(inner, out),
            ClosureExpr::Tuple(elems, _) => {
                elems.iter().for_each(|elem| collect_closures(elem, out));
            },
            ClosureExpr::TupleProj(inner, _, _) => collect_closures(inner, out),
            ClosureExpr::PrimIO(_, expr, _) => {
                if let Some(expr) = expr {
                    collect_closures(expr, out);
                }
            },
            ClosureExpr::UnaryOp(_, inner, _) => collect_closures(inner, out),
            ClosureExpr::If(cond, thn, els, _) => {
                collect_closures(cond, out);
                collect_closures(thn, out);
                collect_closures(els, out);
            },
            ClosureExpr::Let(_, _, rhs, body, _) => {
                collect_closures(rhs, out);
                collect_closures(body, out);
            },
            ClosureExpr::App(func, args, _) => {
                collect_closures(func, out);
                args.iter().for_each(|arg| collect_closures(arg, out));
            },
            _ => {},
        }
    }
}
