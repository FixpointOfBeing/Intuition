pub mod compile;
pub mod repl;
pub mod syntax;
pub mod eval;
pub mod typechecker;

use crate::repl::repl;
use crate::compile::compile_file;
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
    Build {
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
        Some(Commands::Build { file, output }) => {
            if let Err(e) = compile_file(file, output) {
                std::process::exit(1);
            }
        }
    }
}

