pub mod a_normal_form;
pub use a_normal_form::{AExpr, AnfDef, AnfProgram, CompExpr};
pub mod closure_conversion;
pub use closure_conversion::{Closure, ClosureDef, ClosureExpr, ClosureProgram, ClosureType};
pub mod env;
pub use env::Value;
pub mod eval;
pub mod explicate_control;
pub use explicate_control::{CAtom, CBlock, CDef, CExpr, CProgram, CStmt, CTail};
pub mod expose_allocation;
pub use expose_allocation::{AllocDef, AllocExpr, AllocProgram, GlobalValue};
pub mod gensym;
pub use gensym::Gensym;
pub mod limit_functions;
pub mod llvm;
pub mod repl;
pub mod reveal_def;
pub use reveal_def::{RevealDef, RevealExpr, RevealProgram};
pub mod riscv;
pub mod shrink;
pub mod syntax;
pub use syntax::{BinOp, Def, Expr, HasType, Ident, PrimInput, PrimOutput, Program, Type, UnaryOp};
pub mod typechecker;
pub use typechecker::{TypedDef, TypedExpr, TypedProgram};
pub mod uniquify;
pub mod wasm;

use crate::eval::eval_file;
use crate::repl::repl;
use crate::riscv::compile::compile_file;
use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(name = "intu")]
#[command(version = "1.0")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Repl,
    Eval {
        #[arg(value_name = "FILE")]
        file: PathBuf,
    },
    Compile {
        #[arg(value_name = "FILE")]
        file: PathBuf,

        #[arg(short, long, value_name = "OUTPUT")]
        output: Option<PathBuf>,
    },
}
fn main() {
    let cli = Cli::parse();

    match &cli.command {
        None | Some(Commands::Repl) => {
            repl();
        },
        Some(Commands::Compile { file, output }) => {
            compile_file(file, output).unwrap_or_else(|e| {
                println!("Error: {}", e);
            });
        },
        Some(Commands::Eval { file }) => match eval_file(file) {
            Ok(v) => {
                println!("{}", v);
            },
            Err(e) => {
                println!("{}", e)
            },
        },
    }
}
