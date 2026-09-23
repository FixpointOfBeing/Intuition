pub type Ident = String;

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Unit,
    Bool,
    Float,
    Int,
    Tuple(Vec<Type>),
    Arrow(Box<Type>, Box<Type>),
    Var(Ident),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Unit,
    Bool(bool),
    Int(i64),
    Float(f64),
    Var(Ident),
    Tuple(Vec<Expr>),
    PrimIO(PrimIO, Option<Box<Expr>>),
    BinOp(BinOp, Box<Expr>, Box<Expr>),
    UnaryOp(UnaryOp, Box<Expr>),
    Ann(Box<Expr>, Type),
    If(Box<Expr>, Box<Expr>, Box<Expr>),
    Let(Ident, Option<Type>, Box<Expr>, Box<Expr>),
    LetRec(
        Ident,              // function name
        Vec<(Ident, Type)>, // function arguments with their types
        Type,               // function return type
        Box<Expr>,          // function body
        Box<Expr>,          // expression after the let rec
    ),
    App(Box<Expr>, Vec<Expr>),
    Lambda(Vec<(Ident, Type)>, Option<Type>, Box<Expr>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum PrimIO {
    PrintInt,
    PrintFloat,
    PrintBool,
    ReadInt,
    ReadFloat,
}

#[derive(Debug, Clone, PartialEq)]
pub enum BinOp {
    Add,
    Sub,
    Mul,
    Div,
    And,
    Or,
    Eq,
    Neq,
    Lt,
    Gt,
    Leq,
    Geq,
}

#[derive(Debug, Clone, PartialEq)]
pub enum UnaryOp {
    Neg,
    Not,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Def {
    ValDef(
        Ident,        // name
        Option<Type>, // optional type annotation
        Expr,         // expresion
    ),
    FunDef(
        Ident,              // function name
        Vec<(Ident, Type)>, // function arguments with their types
        Option<Type>,       // optional function return type
        Expr,               // function body
    ),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Program {
    pub defs: Vec<Def>,
    pub main: Expr,
}

pub fn ty_unit() -> Type {
    Type::Unit
}

pub fn ty_bool() -> Type {
    Type::Bool
}

pub fn ty_int() -> Type {
    Type::Int
}

pub fn ty_float() -> Type {
    Type::Float
}

pub fn ty_tuple(types: Vec<Type>) -> Type {
    Type::Tuple(types)
}

pub fn ty_arrow(from: Type, to: Type) -> Type {
    Type::Arrow(Box::new(from), Box::new(to))
}

pub fn ty_var(name: impl Into<Ident>) -> Type {
    Type::Var(name.into())
}

pub fn val_def(name: impl Into<Ident>, ann: Option<Type>, expr: Expr) -> Def {
    Def::ValDef(name.into(), ann, expr)
}

pub fn fun_def(
    name: impl Into<Ident>,
    params: Vec<(Ident, Type)>,
    ret_ty: Option<Type>,
    body: Expr,
) -> Def {
    Def::FunDef(name.into(), params, ret_ty, body)
}

pub fn unit() -> Expr {
    Expr::Unit
}

pub fn bool(b: bool) -> Expr {
    Expr::Bool(b)
}

pub fn int(n: i64) -> Expr {
    Expr::Int(n)
}

pub fn float(f: f64) -> Expr {
    Expr::Float(f)
}

pub fn var(name: impl Into<Ident>) -> Expr {
    Expr::Var(name.into())
}

pub fn tuple(exprs: Vec<Expr>) -> Expr {
    Expr::Tuple(exprs)
}

pub fn bin_op(op: BinOp, left: Expr, right: Expr) -> Expr {
    Expr::BinOp(op, Box::new(left), Box::new(right))
}

pub fn unary(op: UnaryOp, expr: Expr) -> Expr {
    Expr::UnaryOp(op, Box::new(expr))
}

pub fn ann(expr: Expr, ty: Type) -> Expr {
    Expr::Ann(Box::new(expr), ty)
}

pub fn if_else(cond: Expr, then_branch: Expr, else_branch: Expr) -> Expr {
    Expr::If(Box::new(cond), Box::new(then_branch), Box::new(else_branch))
}

pub fn a_let(name: impl Into<Ident>, ann: Option<Type>, val: Expr, body: Expr) -> Expr {
    Expr::Let(name.into(), ann, Box::new(val), Box::new(body))
}

pub fn let_rec(
    name: impl Into<Ident>,
    params: Vec<(Ident, Type)>,
    ret_ty: Type,
    body: Expr,
    next: Expr,
) -> Expr {
    Expr::LetRec(name.into(), params, ret_ty, Box::new(body), Box::new(next))
}

pub fn app(func: Expr, args: Vec<Expr>) -> Expr {
    Expr::App(Box::new(func), args)
}

pub fn lambda(params: Vec<(Ident, Type)>, ret_ty: Option<Type>, body: Expr) -> Expr {
    Expr::Lambda(params, ret_ty, Box::new(body))
}

pub fn print_int(expr: Expr) -> Expr {
    Expr::PrimIO(PrimIO::PrintInt, Some(Box::new(expr)))
}

pub fn print_float(expr: Expr) -> Expr {
    Expr::PrimIO(PrimIO::PrintFloat, Some(Box::new(expr)))
}

pub fn print_bool(expr: Expr) -> Expr {
    Expr::PrimIO(PrimIO::PrintBool, Some(Box::new(expr)))
}

pub fn read_int() -> Expr {
    Expr::PrimIO(PrimIO::ReadInt, None)
}

pub fn read_float() -> Expr {
    Expr::PrimIO(PrimIO::ReadFloat, None)
}

// ----------------------------------------------------------------------------------------------------
impl std::fmt::Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Type::Unit => {
                write!(f, "Unit")
            },
            Type::Bool => {
                write!(f, "Bool")
            },
            Type::Int => {
                write!(f, "Int")
            },
            Type::Float => {
                write!(f, "Float")
            },
            Type::Tuple(types) => {
                let types_str = types
                    .iter()
                    .map(|ty| format!("{}", ty))
                    .collect::<Vec<String>>()
                    .join(", ");
                write!(f, "({})", types_str)
            },
            Type::Var(name) => {
                write!(f, "{}", name)
            },
            Type::Arrow(from, to) => {
                let from_str = match **from {
                    Type::Arrow(_, _) => format!("({})", from),
                    _ => format!("{}", from),
                };
                write!(f, "{} -> {}", from_str, to)
            },
        }
    }
}

impl std::fmt::Display for BinOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let op_str = match self {
            BinOp::Add => "+",
            BinOp::Sub => "-",
            BinOp::Mul => "*",
            BinOp::Div => "/",
            BinOp::And => "&&",
            BinOp::Or => "||",
            BinOp::Eq => "=",
            BinOp::Neq => "<>",
            BinOp::Lt => "<",
            BinOp::Gt => ">",
            BinOp::Leq => "<=",
            BinOp::Geq => ">=",
        };
        write!(f, "{}", op_str)
    }
}

impl std::fmt::Display for UnaryOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let op_str = match self {
            UnaryOp::Neg => "-",
            UnaryOp::Not => "!",
        };
        write!(f, "{}", op_str)
    }
}

impl std::fmt::Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expr::Unit => {
                write!(f, "()")
            },
            Expr::Bool(b) => {
                write!(f, "{}", b)
            },
            Expr::Int(n) => {
                write!(f, "{}", n)
            },
            Expr::Float(fl) => {
                write!(f, "{}", fl)
            },
            Expr::Var(name) => {
                write!(f, "{}", name)
            },
            Expr::Tuple(exprs) => {
                let exprs_str = exprs
                    .iter()
                    .map(|expr| format!("{}", expr))
                    .collect::<Vec<String>>()
                    .join(", ");
                write!(f, "({})", exprs_str)
            },

            Expr::PrimIO(prim_io, Some(expr)) => match prim_io {
                PrimIO::PrintInt => write!(f, "print_int {}", *expr),
                PrimIO::PrintFloat => write!(f, "print_float {}", *expr),
                PrimIO::PrintBool => write!(f, "print_bool {}", *expr),
                _ => unreachable!(),
            },
            Expr::PrimIO(prim_io, None) => match prim_io {
                PrimIO::ReadInt => write!(f, "read_int ()"),
                PrimIO::ReadFloat => write!(f, "read_float ()"),
                _ => unreachable!(),
            },
            Expr::BinOp(op, left, right) => {
                write!(f, "({} {} {})", left, op, right)
            },
            Expr::UnaryOp(op, expr) => {
                write!(f, "({}{})", op, expr)
            },
            Expr::Ann(expr, ty) => {
                write!(f, "({} : {})", expr, ty)
            },
            Expr::If(cond, then_branch, else_branch) => {
                write!(f, "if {} then {} else {}", cond, then_branch, else_branch)
            },
            Expr::Let(name, ann, val, body) => {
                if let Some(ty) = ann {
                    write!(f, "let {}: {} = {} in {}", name, ty, val, body)
                } else {
                    write!(f, "let {} = {} in {}", name, val, body)
                }
            },
            Expr::LetRec(fname, fparams, fret_ty, fbody, body) => {
                let params_str = fparams
                    .iter()
                    .map(|(param_name, param_ty)| format!("({}: {})", param_name, param_ty))
                    .collect::<Vec<_>>()
                    .join(" ");
                write!(f, "let rec {} {} : {} = {} in {}", fname, params_str, fret_ty, fbody, body)
            },
            Expr::App(func, args) => {
                let args_str = args
                    .iter()
                    .map(|arg| format!("{}", arg))
                    .collect::<Vec<_>>()
                    .join(" ");
                write!(f, "({} {})", func, args_str)
            },
            Expr::Lambda(params, ty, body) => {
                let params_str = params
                    .iter()
                    .map(|(param_name, param_ty)| format!("({}: {})", param_name, param_ty))
                    .collect::<Vec<_>>()
                    .join(" ");
                if let Some(ret_ty) = ty {
                    write!(f, "fun {} : {} => {}", params_str, ret_ty, body)
                } else {
                    write!(f, "fun {} => {}", params_str, body)
                }
            },
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use lalrpop_util::lalrpop_mod;
    lalrpop_mod!(pub parser);

    #[test]
    fn test_parse_int() {
        let expr = parser::ExprParser::new().parse("42").unwrap();
        match *expr {
            Expr::Int(n) => assert_eq!(n, 42),
            _ => panic!("Expected Expr::Int"),
        }
    }

    #[test]
    fn test_parse_binop1() {
        let expr = parser::ExprParser::new().parse("1 + 2 * 4").unwrap();
        match *expr {
            Expr::BinOp(BinOp::Add, left, right) => match (*left, *right) {
                (Expr::Int(1), Expr::BinOp(BinOp::Mul, left, right)) => {
                    assert_eq!(*left, int(2));
                    assert_eq!(*right, int(4));
                },
                _ => panic!("Unexpected structure in right operand"),
            },
            _ => panic!("Expected Expr::BinOp"),
        }
    }

    #[test]
    fn test_parse_binop2() {
        let expr = parser::ExprParser::new().parse("(3 - 5) / 2").unwrap();
        match *expr {
            Expr::BinOp(BinOp::Div, left, right) => match (*left, *right) {
                (Expr::BinOp(BinOp::Sub, left_sub, right_sub), Expr::Int(2)) => {
                    assert_eq!(*left_sub, int(3));
                    assert_eq!(*right_sub, int(5));
                },
                _ => {
                    panic!("Unexpected structure in left operand")
                },
            },
            _ => panic!("Expected Expr::BinOp"),
        }
    }

    #[test]
    fn test_parse_if() {
        let expr = parser::ExprParser::new().parse("if true then 1 else 0").unwrap();
        assert_eq!(*expr, if_else(bool(true), int(1), int(0)));
    }

    #[test]
    fn test_parse_lambda() {
        let expr = parser::ExprParser::new()
            .parse("fun (x: Int) (y: Int) : Int => x + y")
            .unwrap();
        assert_eq!(
            *expr,
            lambda(
                vec![("x".to_string(), ty_int()), ("y".to_string(), ty_int())],
                Some(ty_int()),
                bin_op(BinOp::Add, var("x"), var("y"))
            )
        );
    }

    #[test]
    fn test_parse_unit() {
        let expr = parser::ExprParser::new().parse("()").unwrap();
        assert_eq!(*expr, unit());
    }

    #[test]
    fn test_parse_bool_true() {
        let expr = parser::ExprParser::new().parse("true").unwrap();
        assert_eq!(*expr, bool(true));
    }

    #[test]
    fn test_parse_bool_false() {
        let expr = parser::ExprParser::new().parse("false").unwrap();
        assert_eq!(*expr, bool(false));
    }

    #[test]
    fn test_parse_float() {
        let expr = parser::ExprParser::new().parse("3.14").unwrap();
        match *expr {
            Expr::Float(f) => assert!((f - 3.14).abs() < 1e-10),
            _ => panic!("Expected Expr::Float"),
        }
    }

    #[test]
    fn test_parse_negative_int() {
        let expr = parser::ExprParser::new().parse("-42").unwrap();
        assert_eq!(*expr, unary(UnaryOp::Neg, int(42)));
    }

    #[test]
    fn test_parse_negative_float() {
        let expr = parser::ExprParser::new().parse("-1.5").unwrap();
        match *expr {
            Expr::UnaryOp(UnaryOp::Neg, inner) => match *inner {
                Expr::Float(f) => assert!((f - 1.5).abs() < 1e-10),
                _ => panic!("Expected Expr::Float inside Neg"),
            },
            _ => panic!("Expected Expr::UnaryOp(Neg, ...)"),
        }
    }

    #[test]
    fn test_parse_var() {
        let expr = parser::ExprParser::new().parse("foo").unwrap();
        assert_eq!(*expr, var("foo"));
    }

    #[test]
    fn test_parse_read_int() {
        let expr = parser::ExprParser::new().parse("read_int ()").unwrap();
        assert_eq!(*expr, read_int());
    }

    #[test]
    fn test_parse_print_int() {
        let expr = parser::ExprParser::new().parse("print_int 42").unwrap();
        assert_eq!(*expr, print_int(int(42)));
    }

    #[test]
    fn test_parse_read_float() {
        let expr = parser::ExprParser::new().parse("read_float ()").unwrap();
        assert_eq!(*expr, read_float());
    }

    #[test]
    fn test_parse_print_float() {
        let expr = parser::ExprParser::new().parse("print_float 3.14").unwrap();
        assert_eq!(*expr, print_float(float(3.14)));
    }

    #[test]
    fn test_parse_print_bool() {
        let expr = parser::ExprParser::new().parse("print_bool true").unwrap();
        assert_eq!(*expr, print_bool(bool(true)));
    }

    #[test]
    fn test_parse_unary_not() {
        let expr = parser::ExprParser::new().parse("!true").unwrap();
        assert_eq!(*expr, unary(UnaryOp::Not, bool(true)));
    }

    #[test]
    fn test_parse_unary_neg() {
        let expr = parser::ExprParser::new().parse("-x").unwrap();
        assert_eq!(*expr, unary(UnaryOp::Neg, var("x")));
    }

    #[test]
    fn test_parse_binop_sub() {
        let expr = parser::ExprParser::new().parse("10 - 3").unwrap();
        assert_eq!(*expr, bin_op(BinOp::Sub, int(10), int(3)));
    }

    #[test]
    fn test_parse_binop_mul() {
        let expr = parser::ExprParser::new().parse("6 * 7").unwrap();
        assert_eq!(*expr, bin_op(BinOp::Mul, int(6), int(7)));
    }

    #[test]
    fn test_parse_binop_div() {
        let expr = parser::ExprParser::new().parse("8 / 2").unwrap();
        assert_eq!(*expr, bin_op(BinOp::Div, int(8), int(2)));
    }

    #[test]
    fn test_parse_binop_eq() {
        let expr = parser::ExprParser::new().parse("x == y").unwrap();
        assert_eq!(*expr, bin_op(BinOp::Eq, var("x"), var("y")));
    }

    #[test]
    fn test_parse_binop_neq() {
        let expr = parser::ExprParser::new().parse("x != y").unwrap();
        assert_eq!(*expr, bin_op(BinOp::Neq, var("x"), var("y")));
    }

    #[test]
    fn test_parse_binop_lt() {
        let expr = parser::ExprParser::new().parse("1 < 2").unwrap();
        assert_eq!(*expr, bin_op(BinOp::Lt, int(1), int(2)));
    }

    #[test]
    fn test_parse_binop_gt() {
        let expr = parser::ExprParser::new().parse("2 > 1").unwrap();
        assert_eq!(*expr, bin_op(BinOp::Gt, int(2), int(1)));
    }

    #[test]
    fn test_parse_binop_leq() {
        let expr = parser::ExprParser::new().parse("1 <= 2").unwrap();
        assert_eq!(*expr, bin_op(BinOp::Leq, int(1), int(2)));
    }

    #[test]
    fn test_parse_binop_geq() {
        let expr = parser::ExprParser::new().parse("2 >= 1").unwrap();
        assert_eq!(*expr, bin_op(BinOp::Geq, int(2), int(1)));
    }

    #[test]
    fn test_parse_binop_and() {
        let expr = parser::ExprParser::new().parse("true && false").unwrap();
        assert_eq!(*expr, bin_op(BinOp::And, bool(true), bool(false)));
    }

    #[test]
    fn test_parse_binop_or() {
        let expr = parser::ExprParser::new().parse("true || false").unwrap();
        assert_eq!(*expr, bin_op(BinOp::Or, bool(true), bool(false)));
    }

    #[test]
    fn test_precedence_add_vs_mul() {
        let expr = parser::ExprParser::new().parse("2 + 3 * 4").unwrap();
        assert_eq!(*expr, bin_op(BinOp::Add, int(2), bin_op(BinOp::Mul, int(3), int(4))));
    }

    #[test]
    fn test_precedence_compare_vs_arith() {
        let expr = parser::ExprParser::new().parse("1 + 2 < 3 + 4").unwrap();
        assert_eq!(
            *expr,
            bin_op(
                BinOp::Lt,
                bin_op(BinOp::Add, int(1), int(2)),
                bin_op(BinOp::Add, int(3), int(4))
            )
        );
    }

    #[test]
    fn test_left_associativity_sub() {
        let expr = parser::ExprParser::new().parse("10 - 3 - 2").unwrap();
        assert_eq!(*expr, bin_op(BinOp::Sub, bin_op(BinOp::Sub, int(10), int(3)), int(2)));
    }

    #[test]
    fn test_parse_ann_arrow_type() {
        let expr = parser::ExprParser::new()
            .parse("((fun (x: Int) : Int => x) : Int -> Int)")
            .unwrap();
        assert_eq!(
            *expr,
            ann(
                lambda(vec![("x".to_string(), ty_int())], Some(ty_int()), var("x")),
                ty_arrow(ty_int(), ty_int())
            )
        );
    }

    #[test]
    fn test_parse_let_no_annotation() {
        let expr = parser::ExprParser::new().parse("let x = 1 in x").unwrap();
        assert_eq!(*expr, a_let("x", None, int(1), var("x")));
    }

    #[test]
    fn test_parse_let_with_annotation() {
        let expr = parser::ExprParser::new().parse("let x: Int = 1 in x").unwrap();
        assert_eq!(*expr, a_let("x", Some(ty_int()), int(1), var("x")));
    }

    #[test]
    fn test_let_lambda() {
        let expr = parser::ExprParser::new()
            .parse("let f (b: Bool) (x: Int) (y: Int) : Int = if b then x + y else x - y in f true 3 5")
            .unwrap();
        assert_eq!(
            *expr,
            a_let(
                "f",
                Some(ty_arrow(ty_bool(), ty_arrow(ty_int(), ty_arrow(ty_int(), ty_int())))),
                lambda(
                    vec![
                        ("b".to_string(), ty_bool()),
                        ("x".to_string(), ty_int()),
                        ("y".to_string(), ty_int()),
                    ],
                    Some(ty_int()),
                    if_else(
                        var("b"),
                        bin_op(BinOp::Add, var("x"), var("y")),
                        bin_op(BinOp::Sub, var("x"), var("y"))
                    )
                ),
                app(var("f"), vec![bool(true), int(3), int(5)])
            )
        );
    }

    #[test]
    fn test_parse_let_nested() {
        let expr = parser::ExprParser::new()
            .parse("let x = 1 in let y = 2 in x + y")
            .unwrap();
        assert_eq!(
            *expr,
            a_let(
                "x",
                None,
                int(1),
                a_let("y", None, int(2), bin_op(BinOp::Add, var("x"), var("y")))
            )
        );
    }

    #[test]
    fn test_parse_letrec_factorial() {
        let expr = parser::ExprParser::new()
            .parse("let rec fact (n: Int) : Int = if n == 0 then 1 else n * fact(n - 1) in fact(5)")
            .unwrap();
        assert_eq!(
            *expr,
            let_rec(
                "fact",
                vec![("n".to_string(), ty_int())],
                ty_int(),
                if_else(
                    bin_op(BinOp::Eq, var("n"), int(0)),
                    int(1),
                    bin_op(
                        BinOp::Mul,
                        var("n"),
                        app(var("fact"), vec![bin_op(BinOp::Sub, var("n"), int(1))])
                    )
                ),
                app(var("fact"), vec![int(5)])
            )
        );
    }

    #[test]
    fn test_parse_letrec_multi_args() {
        let expr = parser::ExprParser::new()
            .parse("let rec add (x: Int) (y: Int) : Int = x + y in add 1 2")
            .unwrap();
        assert_eq!(
            *expr,
            let_rec(
                "add",
                vec![("x".to_string(), ty_int()), ("y".to_string(), ty_int())],
                ty_int(),
                bin_op(BinOp::Add, var("x"), var("y")),
                app(var("add"), vec![int(1), int(2)])
            )
        );
    }

    #[test]
    fn test_parse_app_single_arg() {
        let expr = parser::ExprParser::new().parse("f 1").unwrap();
        assert_eq!(*expr, app(var("f"), vec![int(1)]));
    }

    #[test]
    fn test_parse_app_multi_args() {
        let expr = parser::ExprParser::new().parse("f 1 2 3").unwrap();
        assert_eq!(*expr, app(var("f"), vec![int(1), int(2), int(3)]));
    }

    #[test]
    fn test_parse_app_lambda_immediately() {
        let expr = parser::ExprParser::new()
            .parse("(fun (x: Int) : Int => x) 42")
            .unwrap();
        assert_eq!(
            *expr,
            app(lambda(vec![("x".to_string(), ty_int())], Some(ty_int()), var("x")), vec![int(42)])
        );
    }

    #[test]
    fn test_parse_lambda_single_param() {
        let expr = parser::ExprParser::new().parse("fun (x: Bool) => !x").unwrap();
        assert_eq!(
            *expr,
            lambda(vec![("x".to_string(), ty_bool())], None, unary(UnaryOp::Not, var("x")))
        );
    }

    #[test]
    fn test_parse_lambda_unit_param() {
        let expr = parser::ExprParser::new().parse("fun (x: Unit) : Unit => ()").unwrap();
        assert_eq!(*expr, lambda(vec![("x".to_string(), ty_unit())], Some(ty_unit()), unit()));
    }

    #[test]
    fn test_parse_if_nested() {
        let source = r#"
            if read_int () > 0 then
                if read_int () < 10 then 1 else 2
            else
                3
        "#;
        let expr = parser::ExprParser::new().parse(source).unwrap();
        assert_eq!(
            *expr,
            if_else(
                bin_op(BinOp::Gt, read_int(), int(0)),
                if_else(bin_op(BinOp::Lt, read_int(), int(10)), int(1), int(2)),
                int(3)
            )
        );
    }

    #[test]
    fn test_parse_if_with_binop_condition() {
        let expr = parser::ExprParser::new().parse("if x > 0 then x else -x").unwrap();
        assert_eq!(
            *expr,
            if_else(bin_op(BinOp::Gt, var("x"), int(0)), var("x"), unary(UnaryOp::Neg, var("x")))
        );
    }

    #[test]
    fn test_parse_tuple() {
        let expr = parser::ExprParser::new().parse("(1, true, 3.14)").unwrap();
        match *expr {
            Expr::Tuple(elements) => {
                assert_eq!(elements.len(), 3);
                assert_eq!(elements[0], int(1));
                assert_eq!(elements[1], bool(true));
                assert!(match elements[2] {
                    Expr::Float(f) => (f - 3.14).abs() < 1e-10,
                    _ => false,
                });
            },
            _ => panic!("Expected Expr::Tuple"),
        }
    }

    #[test]
    fn test_parse_tuple_nested() {
        let expr = parser::ExprParser::new().parse("(true, 1, (3.14, false))").unwrap();
        match *expr {
            Expr::Tuple(elements) => {
                assert_eq!(elements.len(), 3);
                assert_eq!(elements[0], bool(true));
                assert_eq!(elements[1], int(1));
                match &elements[2] {
                    Expr::Tuple(inner) => {
                        assert_eq!(inner.len(), 2);
                        assert!(match inner[0] {
                            Expr::Float(f) => (f - 3.14).abs() < 1e-10,
                            _ => false,
                        });
                        assert_eq!(inner[1], bool(false));
                    },
                    _ => panic!("Expected nested Expr::Tuple"),
                }
            },
            _ => panic!("Expected Expr::Tuple"),
        }
    }

    #[test]
    fn test_parse_val_def() {
        let def = parser::DefParser::new().parse("let x: Int = 42;").unwrap();
        assert_eq!(*def, val_def("x".to_string(), Some(ty_int()), int(42)));
    }

    #[test]
    fn test_parse_fun_def() {
        let def = parser::DefParser::new()
            .parse("let add (x: Int) (y: Int) : Int = x + y;")
            .unwrap();
        assert_eq!(
            *def,
            fun_def(
                "add".to_string(),
                vec![("x".to_string(), ty_int()), ("y".to_string(), ty_int())],
                Some(ty_int()),
                bin_op(BinOp::Add, var("x"), var("y"))
            )
        );
    }

    #[test]
    fn test_parse_program() {
        let source = r#"
            let x: Int = read_int ();
            let y: Bool = true;
            let add (a: Int) (b: Int) : Int = a + b;
            
            if x == 42 then add x 1 else add x (-1)
        "#;

        let program = parser::ProgramParser::new().parse(source).unwrap();
        assert_eq!(program.defs.len(), 3);
        assert_eq!(program.defs[0], val_def("x".to_string(), Some(ty_int()), read_int()));
        assert_eq!(program.defs[1], val_def("y".to_string(), Some(ty_bool()), bool(true)));
        assert_eq!(
            program.defs[2],
            fun_def(
                "add".to_string(),
                vec![("a".to_string(), ty_int()), ("b".to_string(), ty_int())],
                Some(ty_int()),
                bin_op(BinOp::Add, var("a"), var("b"))
            )
        );
        assert_eq!(
            program.main,
            if_else(
                bin_op(BinOp::Eq, var("x"), int(42)),
                app(var("add"), vec![var("x"), int(1)]),
                app(var("add"), vec![var("x"), unary(UnaryOp::Neg, int(1))])
            )
        );
    }

    #[test]
    fn test_parse_type_arrow_right_assoc() {
        let expr = parser::ExprParser::new().parse("(f : Int -> Int -> Bool)").unwrap();
        match *expr {
            Expr::Ann(_, ty) => {
                assert_eq!(ty, ty_arrow(ty_int(), ty_arrow(ty_int(), ty_bool())));
            },
            _ => panic!("Expected Expr::Ann"),
        }
    }

    #[test]
    fn test_parse_type_var() {
        let expr = parser::ExprParser::new().parse("(x : a)").unwrap();
        match *expr {
            Expr::Ann(_, ty) => assert_eq!(ty, ty_var("a")),
            _ => panic!("Expected Expr::Ann with type var"),
        }
    }

    #[test]
    fn test_parse_type_tuple() {
        let expr = parser::ExprParser::new().parse("(x : (Int, Bool))").unwrap();
        match *expr {
            Expr::Ann(_, ty) => assert_eq!(ty, ty_tuple(vec![ty_int(), ty_bool()])),
            _ => panic!("Expected Expr::Ann with tuple type"),
        }
    }

    #[test]
    fn test_parse_type_unit() {
        let expr = parser::ExprParser::new().parse("(x : Unit)").unwrap();
        match *expr {
            Expr::Ann(_, ty) => assert_eq!(ty, ty_unit()),
            _ => panic!("Expected Expr::Ann with Unit type"),
        }
    }

    #[test]
    fn test_parse_error_empty() {
        assert!(parser::ExprParser::new().parse("").is_err());
    }

    #[test]
    fn test_parse_error_unmatched_paren() {
        assert!(parser::ExprParser::new().parse("(1 + 2").is_err());
    }

    #[test]
    fn test_parse_error_missing_else() {
        assert!(parser::ExprParser::new().parse("if true then 1").is_err());
    }

    #[test]
    fn test_parse_error_dangling_op() {
        assert!(parser::ExprParser::new().parse("1 +").is_err());
    }
}
