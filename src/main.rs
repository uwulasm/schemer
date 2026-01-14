use chumsky::Parser;

use rustyline::{DefaultEditor, error::ReadlineError};
use vm::VM;

use crate::vm::Value;
mod parser;
mod vm;

fn main() {
    let mut rl = DefaultEditor::new().unwrap();
    let vm = VM::new();
    loop {
        let readline = rl.readline("λ ");
        match readline {
            Ok(line) => {
                let parser = parser::parse_sexpr();
                let parsed = parser.parse(&line);
                match parsed.into_result() {
                    Ok(p) => {
                        let value = vm.evaluate(&p);
                        match value {
                            Value::Nothing => {},
                            _ => println!("{value}"),
                        }
                    },

                    Err(e) => {
                        println!("{e:?}");
                    }
                }
            }
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break;
            },
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break;
            },
            Err(e) => {
                println!("{e:?}");
            }
        }
    }
}
