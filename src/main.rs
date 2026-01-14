use chumsky::Parser;

use vm::VM;
mod parser;
mod vm;

fn main() {
    let parser = parser::parse_sexpr();
    let vm = VM::new();
    match parser.parse("(= (+ 2 2) 4)").into_result() {
        Ok(x) => {
            println!("{:?}", vm.evaluate(&x));
        }
        Err(x) => println!("{x:?}"),
    }
}
