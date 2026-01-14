use std::num;

use chumsky::{IterParser, Parser, number::{format::RUST_LITERAL, number}, prelude::{choice, just, one_of, recursive}, text};

#[derive(Debug)]
enum Sexpr {
    Ident(String),
    Number(f32),
    List(Vec<Sexpr>),
}

fn parse_sexpr<'a>() -> impl Parser<'a, &'a str, Sexpr> {
    recursive(|sexpr| {
        let ident = text::ident()
            .padded()
            .map(|x: &str| x.to_string())
            .or(one_of("+-*/").map(|x: char| x.to_string()));

        let number = number::<RUST_LITERAL, &str, f32, _>().padded().map(|x: f32| Sexpr::Number(x));

        let list = sexpr.repeated().collect::<Vec<_>>().delimited_by(just('(').padded(), just(')').padded()).map(Sexpr::List);

        choice((
                ident.map(|x| Sexpr::Ident(x)),
                number,
                list,
        ))
    })
}

fn main() {
    let parser = parse_sexpr();
    match parser.parse("(+ (+ 2 3) 4)").into_result() {
        Ok(x) => println!("{x:#?}"),
        Err(x) => println!("{x:?}"),
    }
}
