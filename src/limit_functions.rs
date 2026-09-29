use std::collections::HashMap;

use crate::{
    gensym::Gensym,
    syntax::{Ident, Type},
    typechecker::{TypedDef, TypedExpr, TypedProgram, t_app, t_tuple, t_tuple_projection},
};

pub fn limit_funcs_expr(
    typed_expr: TypedExpr,
    params_in_tuple: &HashMap<Ident, (Ident, Type, usize)>,
    limit: usize,
) -> TypedExpr {
    match typed_expr {
        TypedExpr::Unit => todo!(),
        TypedExpr::Bool(_) => todo!(),
        TypedExpr::Int(_) => todo!(),
        TypedExpr::Float(_) => todo!(),
        TypedExpr::Var(name, ty) => {
            if let Some((tuple_name, tuple_ty, idx)) = params_in_tuple.get(&name) {
                t_tuple_projection(
                    TypedExpr::Var(tuple_name.to_string(), (*tuple_ty).clone()),
                    *idx,
                    ty,
                )
            } else {
                TypedExpr::Var(name, ty)
            }
        },
        TypedExpr::BinOp(bin_op, typed_expr, typed_expr1, _) => todo!(),
        TypedExpr::Tuple(typed_exprs, _) => todo!(),
        TypedExpr::TupleProj(typed_expr, _, _) => todo!(),
        TypedExpr::PrimIO(prim_io, typed_expr, _) => todo!(),
        TypedExpr::UnaryOp(unary_op, typed_expr, _) => todo!(),
        TypedExpr::Ann(typed_expr, _) => todo!(),
        TypedExpr::If(typed_expr, typed_expr1, typed_expr2, _) => todo!(),
        TypedExpr::Let(_, _, typed_expr, typed_expr1, _) => todo!(),
        TypedExpr::LetRec(_, items, _, typed_expr, typed_expr1, _) => todo!(),
        TypedExpr::App(func, args, ty) => {
            if args.len() < limit {
                TypedExpr::App(func, args, ty)
            } else {
                let mut new_args = Vec::with_capacity(limit + 1);
                let mut args_tuple = Vec::with_capacity(args.len() - limit);
                let mut tuple_ty = Vec::with_capacity(args.len() - limit);
                for (idx, arg) in args.into_iter().enumerate() {
                    if idx < limit {
                        new_args.push(arg);
                    } else {
                        tuple_ty.push(arg.type_of());
                        args_tuple.push(arg);
                    }
                }
                let tuple = t_tuple(args_tuple, Type::Tuple(tuple_ty));
                new_args.push(tuple);

                let func_limited = limit_funcs_expr(*func, params_in_tuple, limit);

                t_app(func_limited, new_args, ty)
            }
        },
        TypedExpr::Lambda(items, typed_expr, _) => todo!(),
    }
}

pub fn limit_funcs_def(def: TypedDef, limit: usize, gensym: &mut Gensym) -> TypedDef {
    match def {
        TypedDef::ValDef(name, ty, typed_expr) => TypedDef::ValDef(name, ty, typed_expr),
        TypedDef::FunDef(name, params, rt_ty, typed_expr) => {
            if params.len() < limit {
                TypedDef::FunDef(name, params, rt_ty, typed_expr)
            } else {
                let tuple_name = gensym.fresh_with_prefix("$tuple");
                let mut tuple_elems_ty = Vec::with_capacity(params.len() - limit);
                for (idx, (_, ty)) in params.iter().enumerate() {
                    if idx >= limit {
                        tuple_elems_ty.push((*ty).clone());
                    }
                }
                let tuple_ty = Type::Tuple(tuple_elems_ty);

                let mut params_in_tuple = HashMap::<Ident, (Ident, Type, usize)>::new();
                let mut new_params = Vec::with_capacity(limit + 1);
                for (idx, (param, ty)) in params.into_iter().enumerate() {
                    if idx < limit {
                        new_params.push((param, ty));
                    } else {
                        params_in_tuple
                            .insert(param, (tuple_name.clone(), tuple_ty.clone(), idx - limit));
                    }
                }
                new_params.push((tuple_name, tuple_ty));

                TypedDef::FunDef(
                    name,
                    new_params,
                    rt_ty,
                    limit_funcs_expr(typed_expr, &params_in_tuple, limit),
                )
            }
        },
    }
}

fn limit_funcs_prog(prog: TypedProgram, limit: usize) -> TypedProgram {
    let mut gensym = Gensym::new();
    let mut defs_limited = Vec::with_capacity(prog.defs.len());
    for def in prog.defs {
        defs_limited.push(limit_funcs_def(def, limit, &mut gensym));
    }

    let main_limited = limit_funcs_expr(prog.main, &HashMap::new(), limit);
    TypedProgram { defs: defs_limited, main: main_limited }
}

// todo: tests
