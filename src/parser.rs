use chumsky::extra;
use chumsky::{number::format::RUST_LITERAL, prelude::*};

#[derive(Debug, Clone)]
pub enum Sexpr {
    Ident(String),
    Number(f32),
    List(Vec<Sexpr>),
    LiteralList(Vec<Sexpr>),
    String(String),
}

pub fn parse_sexpr<'a>() -> impl Parser<'a, &'a str, Sexpr, extra::Err<Simple<'a, char>>> {
    recursive(|sexpr| {
        let ident = text::ident::<_, extra::Err<Simple<'a, char>>>()
            .padded()
            .map(|x: &str| x.to_string())
            .or(one_of("+-*/=><").padded().map(|x: char| x.to_string()));

        let string = one_of::<_, _, extra::Err<Simple<'a, char>>>("\"")
            .ignore_then(none_of("\"").repeated().collect::<String>())
            .then_ignore(one_of("\""))
            .padded()
            .map(Sexpr::String);

        let number = number::<RUST_LITERAL, &str, f32, extra::Err<Simple<'a, char>>>()
            .padded()
            .map(|x: f32| Sexpr::Number(x));

        let list = sexpr
            .repeated()
            .collect::<Vec<_>>()
            .delimited_by(just('(').padded(), just(')').padded());
        let literal_list = just('\'').ignore_then(list.clone());
        choice((
            ident.map(|x| Sexpr::Ident(x)),
            number,
            string,
            list.map(Sexpr::List),
            literal_list.map(Sexpr::LiteralList),
        ))
    })
}
