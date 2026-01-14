use chumsky::Parser;

mod parser;

fn main() {
    let parser = parser::parse_sexpr();
    match parser.parse("(+ (+ 2 3) 4)").into_result() {
        Ok(x) => println!("{x:#?}"),
        Err(x) => println!("{x:?}"),
    }
}
