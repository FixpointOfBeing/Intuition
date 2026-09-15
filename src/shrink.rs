use crate::syntax::BinOp;
use crate::typechecker::TypedExpr;

pub fn shrink(expr: TypedExpr) -> TypedExpr {
    match &expr {
        TypedExpr::BinOp(op, left, right, ty) => match op {
            BinOp::And => TypedExpr::If(
                (*left).clone(),
                (*right).clone(),
                Box::new(TypedExpr::Bool(false)),
                (*ty).clone(),
            ),
            BinOp::Or => TypedExpr::If(
                (*left).clone(),
                Box::new(TypedExpr::Bool(true)),
                (*right).clone(),
                (*ty).clone(),
            ),
            _ => expr,
        },
        _ => expr,
    }
}
