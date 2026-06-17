use rustyline::error::ReadlineError;
use rustyline::{DefaultEditor };
use lalrpop_util::lalrpop_mod;

use crate::typechecker::typecheck;
use crate::eval::eval_top;
lalrpop_mod!(pub parser);

pub fn repl() {
    const HISTORY_FILE: &str = ".history.txt";

    let mut rl = DefaultEditor::new().unwrap();
    if rl.load_history(HISTORY_FILE).is_err() {
        println!("No previous history.");
    }

    loop {
        let readline = rl.readline(">> ");
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str()).unwrap();
                match parser::ExprParser::new().parse(line.trim()) {
                    Ok(expr) => {
                        match typecheck(&expr) {
                            Ok(_) => {
                                match eval_top(&expr) {
                                    Ok(val) => println!("{}", val),
                                    Err(e) => println!("Evaluation error{}", e),
                                }
                            }
                            Err(e) => println!("Type error: {}", e),
                        }
                    },
                    Err(e) => println!("Parse error: {}", e),
                }
            }
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break;
            }
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }

    rl.save_history(HISTORY_FILE).unwrap();
}