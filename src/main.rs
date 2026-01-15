use chumsky::{IterParser, Parser};

use rustyline::{error::ReadlineError, DefaultEditor};
use vm::VM;

use crate::vm::Value;
mod parser;
mod vm;

fn handle_repl_command(line: &str, vm: &mut VM) {
    if line.len() == 1 {
        return;
    }

    let line = &line[1..].split(" ").collect::<Vec<&str>>();
    match line[0] {
        "load" | "l" => {
            let Some(&file) = line.get(1) else {
                println!("provide file to load");
                return;
            };
            let input = std::fs::read_to_string(file);
            let Ok(input_string) = input else {
                println!("error loading file {file}: {input:?}");
                return;
            };
            let parser = parser::parse_sexpr().repeated().collect::<Vec<_>>();
            let parse_result = parser.parse(&input_string).into_result();
            match parse_result {
                Ok(parsed) => {
                    for sexpr in parsed {
                        vm.evaluate(&sexpr);
                    }
                },
                Err(e) => {
                    for error in e {
                        println!("parsing error: {error:?}");
                    }
                }
            }
        },

        _ => {
            println!("unknown repl command: {}", line[0]);
        }
    }
}

fn main() {
    let mut rl = DefaultEditor::new().unwrap();
    let mut vm = VM::new();
    loop {
        let readline = rl.readline("λ ");
        match readline {
            Ok(line) => {
                if line.starts_with(':') {
                    handle_repl_command(&line, &mut vm);
                } else {
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
