use std::{collections::HashMap, path::PathBuf};

use crate::{uniquify::rename_top, syntax::{BinOp, Expr, Ident, Type, UnaryOp}};
use inkwell::{
    builder::Builder,
    context::Context,
    module::Module,
    types::{BasicType, BasicTypeEnum},
    values::BasicValueEnum,
};

pub struct CodeGen<'ctx> {
    context: &'ctx Context,
    module: Module<'ctx>,
    builder: Builder<'ctx>,
    env: HashMap<Ident, BasicValueEnum<'ctx>>,
}

impl<'ctx> CodeGen<'ctx> {
    pub fn new(context: &'ctx Context, module_name: &str, top_level_ty: &Type) -> Self {
        let module = context.create_module(module_name);
        let builder = context.create_builder();
        let codegen = CodeGen {
            context,
            module,
            builder,
            env: HashMap::new(),
        };

        let ret_type = Self::compile_type(context, top_level_ty);
        let fn_type = ret_type.fn_type(&[], false);
        let function = codegen.module.add_function("program", fn_type, None);

        let entry = context.append_basic_block(function, "entry");
        codegen.builder.position_at_end(entry);
        codegen
    }

    pub fn emit_c_main(&self) {
        let i32_type = self.context.i32_type();
        let main_fn_type = i32_type.fn_type(&[], false);
        let main_fn = self.module.add_function("main", main_fn_type, None);

        let entry = self.context.append_basic_block(main_fn, "entry");
        self.builder.position_at_end(entry);

        let program_fn = self.module.get_function("program").unwrap();
        self.builder.build_call(program_fn, &[], "result").unwrap();

        self.builder
            .build_return(Some(&i32_type.const_int(0, false)))
            .unwrap();
    }

    fn compile_type(context: &'ctx Context, ty: &Type) -> BasicTypeEnum<'ctx> {
        match ty {
            Type::Unit => context.struct_type(&[], false).into(),
            Type::Bool => context.bool_type().into(),
            Type::Int => context.i64_type().into(),
            Type::Float => context.f64_type().into(),
            Type::Arrow(_, _) => panic!("函数类型不直接对应一个值类型，需要单独处理函数签名"),
            Type::Var(_) => panic!("到这一步类型变量应该已经被类型推断解析掉了"),
        }
    }

    fn compile_expr(&mut self, expr: &Expr) -> BasicValueEnum<'ctx> {
        match expr {
            Expr::Unit => self.context.struct_type(&[], false).const_zero().into(),
            Expr::Bool(b) => {
                let v = if *b { 1 } else { 0 };
                self.context.bool_type().const_int(v, false).into()
            }
            Expr::Int(i) => self.context.i64_type().const_int(*i as u64, false).into(),
            Expr::Float(f) => self.context.f64_type().const_float(*f).into(),
            Expr::BinOp(op, left, right) => {
                let left_value = self.compile_expr(left);
                let right_value = self.compile_expr(right);
                self.compile_binop(op, left_value, right_value)
            }
            Expr::UnaryOp(op, expr) => {
                let value = self.compile_expr(expr);
                self.compile_unaryop(op, value)
            }
            Expr::Ann(expr, _) => self.compile_expr(expr),
            Expr::If(cond, then_e, else_e) => self.compile_if(cond, then_e, else_e),
            Expr::Let(name, _, rhs, body) => self.compile_let(name, rhs, body),
            Expr::Var(name) => *self.env.get(name).expect("未绑定的变量"),
            Expr::LetRec(_, items, _, expr, expr1) => todo!(),
            Expr::App(expr, exprs) => todo!(),
            Expr::Lambda(items, _, expr) => todo!(),
        }
    }

    fn compile_binop(
        &mut self,
        op: &BinOp,
        left: BasicValueEnum<'ctx>,
        right: BasicValueEnum<'ctx>,
    ) -> BasicValueEnum<'ctx> {
        match (left, right) {
            (BasicValueEnum::IntValue(lv), BasicValueEnum::IntValue(rv)) => match op {
                BinOp::Add => self.builder.build_int_add(lv, rv, "").unwrap().into(),
                BinOp::Sub => self.builder.build_int_sub(lv, rv, "").unwrap().into(),
                BinOp::Mul => self.builder.build_int_mul(lv, rv, "").unwrap().into(),
                BinOp::Div => self
                    .builder
                    .build_int_signed_div(lv, rv, "")
                    .unwrap()
                    .into(),
                BinOp::Eq => self
                    .builder
                    .build_int_compare(inkwell::IntPredicate::EQ, lv, rv, "")
                    .unwrap()
                    .into(),
                BinOp::Neq => self
                    .builder
                    .build_int_compare(inkwell::IntPredicate::NE, lv, rv, "")
                    .unwrap()
                    .into(),
                BinOp::Lt => self
                    .builder
                    .build_int_compare(inkwell::IntPredicate::SLT, lv, rv, "")
                    .unwrap()
                    .into(),
                BinOp::Gt => self
                    .builder
                    .build_int_compare(inkwell::IntPredicate::SGT, lv, rv, "")
                    .unwrap()
                    .into(),
                BinOp::Leq => self
                    .builder
                    .build_int_compare(inkwell::IntPredicate::SLE, lv, rv, "")
                    .unwrap()
                    .into(),
                BinOp::Geq => self
                    .builder
                    .build_int_compare(inkwell::IntPredicate::SGE, lv, rv, "")
                    .unwrap()
                    .into(),
                BinOp::And => self.builder.build_and(lv, rv, "").unwrap().into(),
                BinOp::Or => self.builder.build_or(lv, rv, "").unwrap().into(),
            },
            (BasicValueEnum::FloatValue(lv), BasicValueEnum::FloatValue(rv)) => match op {
                BinOp::Add => self.builder.build_float_add(lv, rv, "").unwrap().into(),
                BinOp::Sub => self.builder.build_float_sub(lv, rv, "").unwrap().into(),
                BinOp::Mul => self.builder.build_float_mul(lv, rv, "").unwrap().into(),
                BinOp::Div => self.builder.build_float_div(lv, rv, "").unwrap().into(),
                BinOp::Eq => self
                    .builder
                    .build_float_compare(inkwell::FloatPredicate::OEQ, lv, rv, "")
                    .unwrap()
                    .into(),
                BinOp::Neq => self
                    .builder
                    .build_float_compare(inkwell::FloatPredicate::ONE, lv, rv, "")
                    .unwrap()
                    .into(),
                BinOp::Lt => self
                    .builder
                    .build_float_compare(inkwell::FloatPredicate::OLT, lv, rv, "")
                    .unwrap()
                    .into(),
                BinOp::Gt => self
                    .builder
                    .build_float_compare(inkwell::FloatPredicate::OGT, lv, rv, "")
                    .unwrap()
                    .into(),
                BinOp::Leq => self
                    .builder
                    .build_float_compare(inkwell::FloatPredicate::OLE, lv, rv, "")
                    .unwrap()
                    .into(),
                BinOp::Geq => self
                    .builder
                    .build_float_compare(inkwell::FloatPredicate::OGE, lv, rv, "")
                    .unwrap()
                    .into(),
                _ => panic!("浮点类型不支持的二元操作"),
            },
            _ => panic!("类型不匹配的二元操作，类型检查阶段应该已经排除这种情况"),
        }
    }

    fn compile_unaryop(
        &mut self,
        op: &UnaryOp,
        value: BasicValueEnum<'ctx>,
    ) -> BasicValueEnum<'ctx> {
        match value {
            BasicValueEnum::IntValue(iv) => match op {
                UnaryOp::Neg => self.builder.build_int_neg(iv, "").unwrap().into(),
                UnaryOp::Not => self.builder.build_not(iv, "").unwrap().into(),
            },
            BasicValueEnum::FloatValue(fv) => match op {
                UnaryOp::Neg => self.builder.build_float_neg(fv, "").unwrap().into(),
                UnaryOp::Not => panic!("浮点类型不支持的一元操作"),
            },
            _ => panic!("类型不匹配的一元操作，类型检查阶段应该已经排除这种情况"),
        }
    }

    fn compile_if(&mut self, cond: &Expr, then_e: &Expr, else_e: &Expr) -> BasicValueEnum<'ctx> {
        let cond_val = self.compile_expr(cond).into_int_value();
        let func = self
            .builder
            .get_insert_block()
            .unwrap()
            .get_parent()
            .unwrap();

        let then_bb = self.context.append_basic_block(func, "then");
        let else_bb = self.context.append_basic_block(func, "else");
        let merge_bb = self.context.append_basic_block(func, "merge");

        self.builder
            .build_conditional_branch(cond_val, then_bb, else_bb)
            .unwrap();

        // then
        self.builder.position_at_end(then_bb);
        let then_val = self.compile_expr(then_e);
        self.builder.build_unconditional_branch(merge_bb).unwrap();
        let then_bb = self.builder.get_insert_block().unwrap(); 

        // else
        self.builder.position_at_end(else_bb);
        let else_val = self.compile_expr(else_e);
        self.builder.build_unconditional_branch(merge_bb).unwrap();
        let else_bb = self.builder.get_insert_block().unwrap();

        // merge
        self.builder.position_at_end(merge_bb);
        let phi = self
            .builder
            .build_phi(then_val.get_type(), "ifval")
            .unwrap();
        phi.add_incoming(&[(&then_val, then_bb), (&else_val, else_bb)]);
        phi.as_basic_value()
    }

    fn compile_let(&mut self, name: &str, rhs: &Expr, body: &Expr) -> BasicValueEnum<'ctx> {
        let rhs_value = self.compile_expr(rhs);
        let old_value = self.env.insert(name.to_string(), rhs_value);
        let result = self.compile_expr(body);
        match old_value {
            Some(v) => self.env.insert(name.to_string(), v),
            None => self.env.remove(name),
        };
        result
    }
}
pub fn compile_file(file_path: &std::path::Path, output: &Option<PathBuf>) -> Result<(), String> {
    use crate::typechecker::typecheck;
    use lalrpop_util::lalrpop_mod;
    use std::fs::read_to_string;
    lalrpop_mod!(pub parser);

    let source = read_to_string(file_path).map_err(|e| e.to_string())?;
    let ast = parser::ExprParser::new()
        .parse(&source)
        .map_err(|e| e.to_string())?;

    let module_name = file_path
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("main_module");

    let context = Context::create();
    let top_level_ty = typecheck(&ast).map_err(|e| format!("类型检查错误: {}", e))?;
    let mut codegen = CodeGen::new(&context, module_name, &top_level_ty);
    
    let ast = rename_top(&ast);
    let result = codegen.compile_expr(&ast);

    codegen
        .builder
        .build_return(Some(&result))
        .map_err(|e| e.to_string())?;

    codegen.emit_c_main();

    if let Err(e) = codegen.module.verify() {
        return Err(format!("生成的 LLVM IR 不合法: {}", e.to_string()));
    }
    if let Some(output_path) = output {
        codegen
            .module
            .print_to_file(output_path)
            .map_err(|e| e.to_string())?;
    } else {
        codegen.module.print_to_stderr();
    }

    Ok(())
}
