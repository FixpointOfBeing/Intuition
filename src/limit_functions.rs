// use std::collections::HashMap;
// use crate::syntax::HasType;
//
// use crate::{
//     gensym::Gensym,
//     syntax::{Ident, Type},
//     typechecker::{
//         TypedDef, TypedExpr, TypedProgram, t_app, t_bin_op, t_bool, t_float, t_if, t_int,
//         t_let, t_let_rec, t_lambda, t_prim_input,t_prim_output, t_tuple, t_tuple_projection, t_unary, t_unit,
//     },
// };
//
// pub fn limit_funcs_expr(
//     typed_expr: TypedExpr,
//     params_in_tuple: &HashMap<Ident, (Ident, Type, usize)>,
//     limit: usize,
// ) -> TypedExpr {
//     match typed_expr {
//         TypedExpr::Unit => t_unit(),
//         TypedExpr::Bool(b) => t_bool(b),
//         TypedExpr::Int(n) => t_int(n),
//         TypedExpr::Float(f) => t_float(f),
//         TypedExpr::Var(name, ty) => {
//             if let Some((tuple_name, tuple_ty, idx)) = params_in_tuple.get(&name) {
//                 t_tuple_projection(
//                     TypedExpr::Var(tuple_name.to_string(), (*tuple_ty).clone()),
//                     *idx,
//                     ty,
//                 )
//             } else {
//                 TypedExpr::Var(name, ty)
//             }
//         },
//         TypedExpr::BinOp(bin_op, lhs, rhs, ty) => t_bin_op(
//             bin_op,
//             limit_funcs_expr(*lhs, params_in_tuple, limit),
//             limit_funcs_expr(*rhs, params_in_tuple, limit),
//             ty,
//         ),
//         TypedExpr::Tuple(exprs, ty) => t_tuple(
//             exprs
//                 .into_iter()
//                 .map(|expr| limit_funcs_expr(expr, params_in_tuple, limit))
//                 .collect(),
//             ty,
//         ),
//         TypedExpr::TupleProj(expr, index, ty) => {
//             t_tuple_projection(limit_funcs_expr(*expr, params_in_tuple, limit), index, ty)
//         },
//         TypedExpr::PrimIO(prim_io, expr, ty) => t_prim_io(
//             prim_io,
//             expr.map(|expr| limit_funcs_expr(*expr, params_in_tuple, limit)),
//             ty,
//         ),
//         TypedExpr::UnaryOp(unary_op, expr, ty) => {
//             t_unary(unary_op, limit_funcs_expr(*expr, params_in_tuple, limit), ty)
//         },
//         // TypedExpr::Ann(typed_expr, _) => todo!(),
//         TypedExpr::If(cond, thn, els, ty) => t_if(
//             limit_funcs_expr(*cond, params_in_tuple, limit),
//             limit_funcs_expr(*thn, params_in_tuple, limit),
//             limit_funcs_expr(*els, params_in_tuple, limit),
//             ty,
//         ),
//         TypedExpr::Let(name, rhs_ty, rhs, body, ty) => t_let(
//             name,
//             rhs_ty,
//             limit_funcs_expr(*rhs, params_in_tuple, limit),
//             limit_funcs_expr(*body, params_in_tuple, limit),
//             ty,
//         ),
//         TypedExpr::LetRec(name, params, ret_ty, fbody, body, ty) => t_let_rec(
//             name,
//             params,
//             ret_ty,
//             limit_funcs_expr(*fbody, params_in_tuple, limit),
//             limit_funcs_expr(*body, params_in_tuple, limit),
//             ty,
//         ),
//         TypedExpr::App(func, args, ty) => {
//             if args.len() < limit {
//                 TypedExpr::App(func, args, ty)
//             } else {
//                 let mut new_args = Vec::with_capacity(limit + 1);
//                 let mut args_tuple = Vec::with_capacity(args.len() - limit);
//                 let mut tuple_ty = Vec::with_capacity(args.len() - limit);
//                 for (idx, arg) in args.into_iter().enumerate() {
//                     if idx < limit {
//                         new_args.push(arg);
//                     } else {
//                         tuple_ty.push(arg.type_of());
//                         args_tuple.push(arg);
//                     }
//                 }
//                 let tuple = t_tuple(args_tuple, Type::Tuple(tuple_ty));
//                 new_args.push(tuple);
//
//                 let func_limited = limit_funcs_expr(*func, params_in_tuple, limit);
//
//                 t_app(func_limited, new_args, ty)
//             }
//         },
//         TypedExpr::Lambda(params, body, ty) => {
//             t_lambda(params, limit_funcs_expr(*body, params_in_tuple, limit), ty)
//         },
//     }
// }
//
// pub fn limit_funcs_def(def: TypedDef, limit: usize, gensym: &mut Gensym) -> TypedDef {
//     match def {
//         TypedDef::ValDef(name, ty, typed_expr) => TypedDef::ValDef(name, ty, typed_expr),
//         TypedDef::FunDef(name, params, rt_ty, typed_expr) => {
//             if params.len() < limit {
//                 TypedDef::FunDef(name, params, rt_ty, typed_expr)
//             } else {
//                 let tuple_name = gensym.fresh_with_prefix("tuple");
//                 let mut tuple_elems_ty = Vec::with_capacity(params.len() - limit);
//                 for (idx, (_, ty)) in params.iter().enumerate() {
//                     if idx >= limit {
//                         tuple_elems_ty.push((*ty).clone());
//                     }
//                 }
//                 let tuple_ty = Type::Tuple(tuple_elems_ty);
//
//                 let mut params_in_tuple = HashMap::<Ident, (Ident, Type, usize)>::new();
//                 let mut new_params = Vec::with_capacity(limit + 1);
//                 for (idx, (param, ty)) in params.into_iter().enumerate() {
//                     if idx < limit {
//                         new_params.push((param, ty));
//                     } else {
//                         params_in_tuple
//                             .insert(param, (tuple_name.clone(), tuple_ty.clone(), idx - limit));
//                     }
//                 }
//                 new_params.push((tuple_name, tuple_ty));
//
//                 TypedDef::FunDef(
//                     name,
//                     new_params,
//                     rt_ty,
//                     limit_funcs_expr(typed_expr, &params_in_tuple, limit),
//                 )
//             }
//         },
//     }
// }
//
// fn limit_funcs_prog(prog: TypedProgram, limit: usize) -> TypedProgram {
//     let mut gensym = Gensym::new();
//     let mut defs_limited = Vec::with_capacity(prog.defs.len());
//     for def in prog.defs {
//         defs_limited.push(limit_funcs_def(def, limit, &mut gensym));
//     }
//
//     let main_limited = limit_funcs_expr(prog.main, &HashMap::new(), limit);
//     TypedProgram { defs: defs_limited, main: main_limited }
// }
//
//
//
// #[cfg(test)]
// mod tests {
//     use super::*;
//     use crate::{
//         syntax::{BinOp, Type},
//         typechecker::{t_bin_op, t_if, t_tuple_projection, t_var},
//     };
//     use std::collections::HashMap;
//
//     fn packed_mapping() -> HashMap<Ident, (Ident, Type, usize)> {
//         let tuple_ty = Type::Tuple(vec![Type::Int, Type::Bool]);
//         HashMap::from([
//             (
//                 "c".to_string(),
//                 ("tuple$0".to_string(), tuple_ty.clone(), 0usize),
//             ),
//             (
//                 "d".to_string(),
//                 ("tuple$0".to_string(), tuple_ty, 1usize),
//             ),
//         ])
//     }
//
//     #[test]
//     fn rewrites_packed_vars_and_leaves_others_alone() {
//         let mapping = packed_mapping();
//         let tuple_ty = Type::Tuple(vec![Type::Int, Type::Bool]);
//
//         let input = t_bin_op(
//             BinOp::Add,
//             t_var("c", Type::Int),
//             t_var("x", Type::Int),
//             Type::Int,
//         );
//
//         let expected = t_bin_op(
//             BinOp::Add,
//             t_tuple_projection(t_var("tuple$0", tuple_ty), 0, Type::Int),
//             t_var("x", Type::Int),
//             Type::Int,
//         );
//
//         assert_eq!(limit_funcs_expr(input, &mapping, 2), expected);
//     }
//
//     #[test]
//     fn packs_extra_app_args_into_tuple() {
//         let mapping = HashMap::new();
//         let func = t_var("f", Type::Dummy);
//         let args = vec![t_int(1), t_int(2), t_int(3), t_bool(true)];
//
//         let expected = t_app(
//             func.clone(),
//             vec![
//                 t_int(1),
//                 t_int(2),
//                 t_tuple(
//                     vec![t_int(3), t_bool(true)],
//                     Type::Tuple(vec![Type::Int, Type::Bool]),
//                 ),
//             ],
//             Type::Int,
//         );
//
//         assert_eq!(
//             limit_funcs_expr(t_app(func, args, Type::Int), &mapping, 2),
//             expected
//         );
//     }
//
//     #[test]
//     fn leaves_short_app_unchanged() {
//         let mapping = HashMap::new();
//         let func = t_var("f", Type::Dummy);
//         let args = vec![t_int(1), t_int(2)];
//
//         assert_eq!(
//             limit_funcs_expr(t_app(func.clone(), args.clone(), Type::Int), &mapping, 3),
//             t_app(func, args, Type::Int)
//         );
//     }
//
//     #[test]
//     fn recurses_through_if_and_let() {
//         let mapping = packed_mapping();
//         let tuple_ty = Type::Tuple(vec![Type::Int, Type::Bool]);
//
//         let input = t_if(
//             t_bool(true),
//             t_var("d", Type::Bool),
//             t_let(
//                 "x",
//                 Type::Int,
//                 t_var("c", Type::Int),
//                 t_var("x", Type::Int),
//                 Type::Int,
//             ),
//             Type::Bool,
//         );
//
//         let expected = t_if(
//             t_bool(true),
//             t_tuple_projection(t_var("tuple$0", tuple_ty.clone()), 1, Type::Bool),
//             t_let(
//                 "x",
//                 Type::Int,
//                 t_tuple_projection(t_var("tuple$0", tuple_ty), 0, Type::Int),
//                 t_var("x", Type::Int),
//                 Type::Int,
//             ),
//             Type::Bool,
//         );
//
//         assert_eq!(limit_funcs_expr(input, &mapping, 2), expected);
//     }
//
//     #[test]
//     fn packs_extra_def_params_and_rewrites_body() {
//         let def = TypedDef::FunDef(
//             "f".to_string(),
//             vec![
//                 ("a".to_string(), Type::Int),
//                 ("b".to_string(), Type::Int),
//                 ("c".to_string(), Type::Int),
//                 ("d".to_string(), Type::Bool),
//             ],
//             Type::Int,
//             t_bin_op(
//                 BinOp::Add,
//                 t_var("a", Type::Int),
//                 t_var("c", Type::Int),
//                 Type::Int,
//             ),
//         );
//
//         let mut gensym = Gensym::new();
//         let result = limit_funcs_def(def, 2, &mut gensym);
//
//         let tuple_ty = Type::Tuple(vec![Type::Int, Type::Bool]);
//         let expected = TypedDef::FunDef(
//             "f".to_string(),
//             vec![
//                 ("a".to_string(), Type::Int),
//                 ("b".to_string(), Type::Int),
//                 ("tuple$0".to_string(), tuple_ty.clone()),
//             ],
//             Type::Int,
//             t_bin_op(
//                 BinOp::Add,
//                 t_var("a", Type::Int),
//                 t_tuple_projection(t_var("tuple$0", tuple_ty), 0, Type::Int),
//                 Type::Int,
//             ),
//         );
//
//         assert_eq!(result, expected);
//     }
//
//     #[test]
//     fn leaves_short_def_unchanged() {
//         let def = TypedDef::FunDef(
//             "g".to_string(),
//             vec![("a".to_string(), Type::Int), ("b".to_string(), Type::Int)],
//             Type::Int,
//             t_var("a", Type::Int),
//         );
//
//         let mut gensym = Gensym::new();
//         assert_eq!(
//             limit_funcs_def(def.clone(), 3, &mut gensym),
//             def
//         );
//     }
// }
