// pub mod compile_llvm;
pub mod a_normal_form;
pub mod allocate_registers;
pub mod closure_conversion;
pub mod compile;
pub mod emit_llvm;
pub mod env;
pub mod eval;
pub mod explicate_control;
pub mod gensym;
pub mod intu_ir;
pub mod liveness;
pub mod repl;
pub mod riscv;
pub mod syntax;
pub mod typechecker;
pub mod uniquify;
pub mod wasm;

use crate::compile::compile_file;
use crate::eval::eval_file;
use crate::repl::repl;
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
