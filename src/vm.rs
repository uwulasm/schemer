use std::collections::HashMap;
use crate::parser::Sexpr;

#[derive(Debug, Clone)]
pub enum Value {
    Number(f32),
    Nothing,
}

pub struct VM {
    special_cases: HashMap<String, fn(&Self, &[Sexpr]) -> Value>,
}

impl VM {
    pub fn new() -> VM {
        VM {
            special_cases: HashMap::from([("+".to_string(), VM::add_special_case as fn(&Self, &[Sexpr]) -> Value)]),
        }
    }

    fn add_special_case(&self, exprs: &[Sexpr]) -> Value {
        let a = self.evaluate(&exprs[1]);
        let b = self.evaluate(&exprs[2]);
        match (a, b) {
            (Value::Number(a), Value::Number(b)) => Value::Number(a + b),
            _ => todo!(),
        }
    }

    pub fn evaluate(&self, expr: &Sexpr) -> Value {
        match expr {
            Sexpr::Number(x) => Value::Number(*x),
            Sexpr::Ident(_) => todo!(),
            Sexpr::List(list) => {
                if let Some(prefix) = list.first() {
                    match prefix {
                        Sexpr::Ident(x) => {
                            if let Some(&special_case) = self.special_cases.get(x) {
                                special_case(self, list)
                            } else {
                                todo!()
                            }
                        }
                        _ => todo!(),
                    }
                } else {
                    Value::Nothing
                }
            },
        }
    }
}
