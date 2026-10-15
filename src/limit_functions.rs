use crate::closure_conversion::{
    ClosureDef, ClosureExpr, ClosureProgram, ClosureType, clos_app, clos_bin_op, clos_bool,
    clos_clos, clos_clos_free_var, clos_clos_fun_ptr, clos_float, clos_if, clos_int, clos_let,
    clos_prim_input, clos_prim_output, clos_tuple, clos_tuple_projection, clos_unary, clos_unit,
    clos_var,
};

use std::collections::HashMap;

use crate::{gensym::Gensym, syntax::Ident};

// limit表示函数参数的最大数量
pub fn limit_funcs_expr(
    clos_expr: ClosureExpr,
    params_in_tuple: &HashMap<Ident, (Ident, ClosureType, usize)>,
    limit: usize,
) -> ClosureExpr {
    match clos_expr {
        ClosureExpr::Unit => clos_unit(),
        ClosureExpr::Bool(b) => clos_bool(b),
        ClosureExpr::Int(i) => clos_int(i),
        ClosureExpr::Float(f) => clos_float(f),
        ClosureExpr::Var(name, clos_ty) => {
            if let Some((tuple_name, tuple_ty, idx)) = params_in_tuple.get(&name) {
                clos_tuple_projection(
                    clos_var(tuple_name.to_string(), (*tuple_ty).clone()),
                    *idx,
                    clos_ty,
                )
            } else {
                clos_var(name, clos_ty)
            }
        },
        ClosureExpr::BinOp(op, left, right, clos_ty) => {
            let left = limit_funcs_expr(*left, params_in_tuple, limit);
            let right = limit_funcs_expr(*right, params_in_tuple, limit);
            clos_bin_op(op, left, right, clos_ty)
        },
        ClosureExpr::Closure(clos, ty) => clos_clos(clos, ty),
        ClosureExpr::ClosureFunPtr(clos_expr, clos_ty) => {
            let clos = limit_funcs_expr(*clos_expr, params_in_tuple, limit);
            clos_clos_fun_ptr(clos, clos_ty)
        },
        ClosureExpr::ClosureFreeVar(clos_expr, index, clos_ty) => {
            let clos = limit_funcs_expr(*clos_expr, params_in_tuple, limit);
            clos_clos_free_var(clos, index, clos_ty)
        },
        ClosureExpr::Tuple(clos_exprs, clos_ty) => {
            let elems = clos_exprs
                .into_iter()
                .map(|clos_expr| limit_funcs_expr(clos_expr, params_in_tuple, limit))
                .collect();
            clos_tuple(elems, clos_ty)
        },
        ClosureExpr::TupleProj(tuple, index, clos_ty) => {
            let tuple = limit_funcs_expr(*tuple, params_in_tuple, limit);
            clos_tuple_projection(tuple, index, clos_ty)
        },
        ClosureExpr::PrimInput(prim, clos_ty) => clos_prim_input(prim, clos_ty),
        ClosureExpr::PrimOutput(prim, clos_expr, clos_ty) => {
            let expr = limit_funcs_expr(*clos_expr, params_in_tuple, limit);
            clos_prim_output(prim, expr, clos_ty)
        },
        ClosureExpr::UnaryOp(op, clos_expr, clos_ty) => {
            let expr = limit_funcs_expr(*clos_expr, params_in_tuple, limit);
            clos_unary(op, expr, clos_ty)
        },
        ClosureExpr::If(cond, thn, els, clos_ty) => {
            let cond = limit_funcs_expr(*cond, params_in_tuple, limit);
            let thn = limit_funcs_expr(*thn, params_in_tuple, limit);
            let els = limit_funcs_expr(*els, params_in_tuple, limit);
            clos_if(cond, thn, els, clos_ty)
        },
        ClosureExpr::Let(name, rhs_ty, rhs, body, let_ty) => {
            let rhs = limit_funcs_expr(*rhs, params_in_tuple, limit);
            let body = limit_funcs_expr(*body, params_in_tuple, limit);
            clos_let(name, rhs_ty, rhs, body, let_ty)
        },
        ClosureExpr::App(func, args, clos_ty) => {
            let func = limit_funcs_expr(*func, params_in_tuple, limit);
            let args = args
                .into_iter()
                .map(|arg| limit_funcs_expr(arg, params_in_tuple, limit))
                .collect::<Vec<_>>();

            if args.len() <= limit {
                clos_app(func, args, clos_ty)
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
                let tuple = clos_tuple(args_tuple, ClosureType::Tuple(tuple_ty));
                new_args.push(tuple);

                clos_app(func, new_args, clos_ty)
            }
        },
    }
}

pub fn limit_funcs_def(def: ClosureDef, limit: usize, gensym: &mut Gensym) -> ClosureDef {
    match def {
        ClosureDef::ValDef(name, clos_ty, clos_expr) => {
            ClosureDef::ValDef(name, clos_ty, limit_funcs_expr(clos_expr, &HashMap::new(), limit))
        },
        ClosureDef::FunDef(name, params, ret_ty, clos_expr) => {
            if params.len() <= limit {
                ClosureDef::FunDef(
                    name,
                    params,
                    ret_ty,
                    limit_funcs_expr(clos_expr, &HashMap::new(), limit),
                )
            } else {
                let tuple_name = gensym.fresh_with_prefix("tuple");
                let tuple_ty = {
                    let mut tuple_elems_ty = Vec::with_capacity(params.len() - limit);
                    for (idx, (_, ty)) in params.iter().enumerate() {
                        if idx >= limit {
                            tuple_elems_ty.push((*ty).clone());
                        }
                    }
                    ClosureType::Tuple(tuple_elems_ty)
                };

                let mut params_in_tuple = HashMap::<Ident, (Ident, ClosureType, usize)>::new();
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

                ClosureDef::FunDef(
                    name,
                    new_params,
                    ret_ty,
                    limit_funcs_expr(clos_expr, &params_in_tuple, limit),
                )
            }
        },
    }
}

fn limit_funcs_prog(prog: ClosureProgram, limit: usize) -> ClosureProgram {
    let mut gensym = Gensym::new();
    let mut defs_limited = Vec::with_capacity(prog.defs.len());
    for def in prog.defs {
        defs_limited.push(limit_funcs_def(def, limit, &mut gensym));
    }

    let main_limited = limit_funcs_expr(prog.main, &HashMap::new(), limit);
    ClosureProgram { defs: defs_limited, main: main_limited }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::syntax::BinOp;

    fn packed_mapping() -> HashMap<Ident, (Ident, ClosureType, usize)> {
        let tuple_ty = ClosureType::Tuple(vec![ClosureType::Int, ClosureType::Bool]);
        HashMap::from([
            ("c".to_string(), ("tuple$0".to_string(), tuple_ty.clone(), 0usize)),
            ("d".to_string(), ("tuple$0".to_string(), tuple_ty, 1usize)),
        ])
    }

    #[test]
    fn rewrites_packed_vars_and_leaves_others_alone() {
        let mapping = packed_mapping();
        let tuple_ty = ClosureType::Tuple(vec![ClosureType::Int, ClosureType::Bool]);

        let input = clos_bin_op(
            BinOp::Add,
            clos_var("c", ClosureType::Int),
            clos_var("x", ClosureType::Int),
            ClosureType::Int,
        );

        let expected = clos_bin_op(
            BinOp::Add,
            clos_tuple_projection(clos_var("tuple$0", tuple_ty), 0, ClosureType::Int),
            clos_var("x", ClosureType::Int),
            ClosureType::Int,
        );

        assert_eq!(limit_funcs_expr(input, &mapping, 2), expected);
    }

    #[test]
    fn packs_extra_app_args_into_tuple() {
        let mapping = HashMap::new();
        let func = clos_var("f", ClosureType::Int);
        let args = vec![clos_int(1), clos_int(2), clos_int(3), clos_bool(true)];

        let expected = clos_app(
            func.clone(),
            vec![
                clos_int(1),
                clos_int(2),
                clos_tuple(
                    vec![clos_int(3), clos_bool(true)],
                    ClosureType::Tuple(vec![ClosureType::Int, ClosureType::Bool]),
                ),
            ],
            ClosureType::Int,
        );

        assert_eq!(
            limit_funcs_expr(clos_app(func, args, ClosureType::Int), &mapping, 2),
            expected
        );
    }

    #[test]
    fn leaves_short_app_unchanged() {
        let mapping = HashMap::new();
        let func = clos_var("f", ClosureType::Int);
        let args = vec![clos_int(1), clos_int(2)];

        assert_eq!(
            limit_funcs_expr(clos_app(func.clone(), args.clone(), ClosureType::Int), &mapping, 3),
            clos_app(func, args, ClosureType::Int)
        );
    }

    #[test]
    fn recurses_through_if_and_let() {
        let mapping = packed_mapping();
        let tuple_ty = ClosureType::Tuple(vec![ClosureType::Int, ClosureType::Bool]);

        let input = clos_if(
            clos_bool(true),
            clos_var("d", ClosureType::Bool),
            clos_let(
                "x",
                ClosureType::Int,
                clos_var("c", ClosureType::Int),
                clos_var("x", ClosureType::Int),
                ClosureType::Int,
            ),
            ClosureType::Bool,
        );

        let expected = clos_if(
            clos_bool(true),
            clos_tuple_projection(clos_var("tuple$0", tuple_ty.clone()), 1, ClosureType::Bool),
            clos_let(
                "x",
                ClosureType::Int,
                clos_tuple_projection(clos_var("tuple$0", tuple_ty), 0, ClosureType::Int),
                clos_var("x", ClosureType::Int),
                ClosureType::Int,
            ),
            ClosureType::Bool,
        );

        assert_eq!(limit_funcs_expr(input, &mapping, 2), expected);
    }

    #[test]
    fn packs_extra_def_params_and_rewrites_body() {
        let def = ClosureDef::FunDef(
            "f".to_string(),
            vec![
                ("a".to_string(), ClosureType::Int),
                ("b".to_string(), ClosureType::Int),
                ("c".to_string(), ClosureType::Int),
                ("d".to_string(), ClosureType::Bool),
            ],
            ClosureType::Int,
            clos_bin_op(
                BinOp::Add,
                clos_var("a", ClosureType::Int),
                clos_var("c", ClosureType::Int),
                ClosureType::Int,
            ),
        );

        let mut gensym = Gensym::new();
        let result = limit_funcs_def(def, 2, &mut gensym);

        let tuple_ty = ClosureType::Tuple(vec![ClosureType::Int, ClosureType::Bool]);
        let expected = ClosureDef::FunDef(
            "f".to_string(),
            vec![
                ("a".to_string(), ClosureType::Int),
                ("b".to_string(), ClosureType::Int),
                ("tuple$0".to_string(), tuple_ty.clone()),
            ],
            ClosureType::Int,
            clos_bin_op(
                BinOp::Add,
                clos_var("a", ClosureType::Int),
                clos_tuple_projection(clos_var("tuple$0", tuple_ty), 0, ClosureType::Int),
                ClosureType::Int,
            ),
        );

        assert_eq!(result, expected);
    }

    #[test]
    fn leaves_short_def_unchanged() {
        let def = ClosureDef::FunDef(
            "g".to_string(),
            vec![("a".to_string(), ClosureType::Int), ("b".to_string(), ClosureType::Int)],
            ClosureType::Int,
            clos_var("a", ClosureType::Int),
        );

        let mut gensym = Gensym::new();
        assert_eq!(limit_funcs_def(def.clone(), 3, &mut gensym), def);
    }

    // New tests that expose the missing-recursion bugs:

    #[test]
    fn rewrites_packed_var_inside_short_app() {
        let mapping = packed_mapping();
        let tuple_ty = ClosureType::Tuple(vec![ClosureType::Int, ClosureType::Bool]);
        let func = clos_var("f", ClosureType::Int);
        let input =
            clos_app(func.clone(), vec![clos_var("c", ClosureType::Int)], ClosureType::Int);
        let expected = clos_app(
            func,
            vec![clos_tuple_projection(clos_var("tuple$0", tuple_ty), 0, ClosureType::Int)],
            ClosureType::Int,
        );
        assert_eq!(limit_funcs_expr(input, &mapping, 2), expected);
    }

    #[test]
    fn limits_nested_app() {
        let mapping = HashMap::new();
        let h = clos_var("h", ClosureType::Int);
        let g = clos_var("g", ClosureType::Int);
        // g (h 1 2 3): outer app has 1 arg <= limit, inner has 3 > limit
        let inner =
            clos_app(h.clone(), vec![clos_int(1), clos_int(2), clos_int(3)], ClosureType::Int);
        let input = clos_app(g.clone(), vec![inner], ClosureType::Int);

        let expected_inner = clos_app(
            h,
            vec![
                clos_int(1),
                clos_int(2),
                clos_tuple(vec![clos_int(3)], ClosureType::Tuple(vec![ClosureType::Int])),
            ],
            ClosureType::Int,
        );
        let expected = clos_app(g, vec![expected_inner], ClosureType::Int);

        assert_eq!(limit_funcs_expr(input, &mapping, 2), expected);
    }

    #[test]
    fn rewrites_packed_vars_packed_into_app_tuple() {
        let mapping = packed_mapping();
        let tuple_ty = ClosureType::Tuple(vec![ClosureType::Int, ClosureType::Bool]);
        let func = clos_var("f", ClosureType::Int);
        // f a c d  (limit 2); c and d are packed params
        let input = clos_app(
            func.clone(),
            vec![
                clos_var("a", ClosureType::Int),
                clos_var("c", ClosureType::Int),
                clos_var("d", ClosureType::Bool),
            ],
            ClosureType::Int,
        );
        let expected = clos_app(
            func,
            vec![
                clos_var("a", ClosureType::Int),
                clos_tuple_projection(clos_var("tuple$0", tuple_ty.clone()), 0, ClosureType::Int),
                clos_tuple(
                    vec![clos_tuple_projection(
                        clos_var("tuple$0", tuple_ty.clone()),
                        1,
                        ClosureType::Bool,
                    )],
                    ClosureType::Tuple(vec![ClosureType::Bool]),
                ),
            ],
            ClosureType::Int,
        );
        assert_eq!(limit_funcs_expr(input, &mapping, 2), expected);
    }
}
