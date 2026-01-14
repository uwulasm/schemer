use chumsky::Parser;

use rustyline::{error::ReadlineError, DefaultEditor};
use vm::VM;

use crate::vm::Value;
mod parser;
mod vm;

fn main() {
    let mut rl = DefaultEditor::new().unwrap();
    let mut vm = VM::new();
    loop {
        let readline = rl.readline("λ ");
        match readline {
            Ok(line) => {
                rl.add_history_entry(&line).unwrap();
                let parser = parser::parse_sexpr();
                let parsed = parser.parse(&line);
                match parsed.into_result() {
                    Ok(p) => {
                        let value = vm.evaluate(&p);
                        match value {
                            Value::Nothing => {}
                            _ => println!("{value}"),
                        }
                    }

                    Err(e) => {
                        println!("{e:?}");
                    }
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
            Err(e) => {
                println!("{e:?}");
            }
        }
    }
}
