pub mod compile;
pub mod eval;
pub mod repl;
pub mod syntax;
pub mod typechecker;

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
        }
        Some(Commands::Compile { file, output }) => {
            match compile_file(file, output) {
                Ok(()) => {
                    println!("Compilation successful");
                }
                Err(e) => {
                    println!("{}", e);
                }
            }
        }
        Some(Commands::Eval { file }) => match eval_file(file) {
            Ok(v) => {
                println!("{}", v);
            }
            Err(e) => {
                println!("{}", e)
            }
        },
    }
}
