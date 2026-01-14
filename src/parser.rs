use chumsky::{number::format::RUST_LITERAL, prelude::*};

#[derive(Debug)]
pub enum Sexpr {
    Ident(String),
    Number(f32),
    List(Vec<Sexpr>),
}

pub fn parse_sexpr<'a>() -> impl Parser<'a, &'a str, Sexpr> {
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
