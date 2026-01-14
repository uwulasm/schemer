use std::collections::HashMap;

use crate::parser::Sexpr;

#[derive(Debug, Clone)]
pub enum Value {
    String(String),
    Function(Function),
    Number(f32),
    Nothing,
    True,
    False,
}

#[derive(Debug, Clone)]
pub enum Function {
    LangFunction(Sexpr),
    Builtin(fn(&[Value]) -> Value),
}

#[derive(Debug, Clone)]
struct Variable {
    name: String,
    value: Value,
}

pub struct VM {
    special_cases: HashMap<String, fn(&Self, &[Sexpr]) -> Value>,
    variables: Vec<Variable>,
}

impl VM {
    pub fn new() -> VM {
        VM {
            //special_cases: HashMap::from([("+".to_string(), VM::add_special_case as fn(&Self, &[Sexpr]) -> Value)]),
            special_cases: HashMap::new(),
            variables: vec![Variable {
                name: "+".to_string(),
                value: Value::Function(Function::Builtin(VM::add_builtin)),
            },

            Variable {
                name: "-".to_string(),
                value: Value::Function(Function::Builtin(VM::sub_builtin)),
            },

            Variable {
                name: "display".to_string(),
                value: Value::Function(Function::Builtin(VM::display_builtin)),
            },
            ],
        }
    }

    fn add_builtin(values: &[Value]) -> Value {
        let mut total = 0f32;
        for value in values {
            let Value::Number(x) = value else { todo!() };
            total += x;
        }

        Value::Number(total)
    }

    fn sub_builtin(values: &[Value]) -> Value {
        let Value::Number(mut total) = values[0] else {
            todo!()
        };

        for value in values.iter().skip(1) {
            let Value::Number(x) = value else { todo!() };
            total -= x;
        }

        Value::Number(total)
    }

    fn display_builtin(values: &[Value]) -> Value {
        for value in values {
            match value {
                Value::String(x) => println!("{x}"),
                _ => todo!(),
            }
        }

        Value::Nothing
    }

    pub fn evaluate(&self, expr: &Sexpr) -> Value {
        match expr {
            Sexpr::String(x) => Value::String(x.clone()),
            Sexpr::Number(x) => Value::Number(*x),
            Sexpr::Ident(_) => todo!(),
            Sexpr::List(list) => {
                if let Some(prefix) = list.first() {
                    match prefix {
                        Sexpr::Ident(x) => {
                            if let Some(&special_case) = self.special_cases.get(x) {
                                special_case(self, list)
                            } else if let Some(v) = self.variables.iter().find(|&v| *x == v.name) {
                                match &v.value {
                                    Value::Function(f) => {
                                        let arguments = list
                                            .iter()
                                            .skip(1)
                                            .map(|x| self.evaluate(x))
                                            .collect::<Vec<_>>();
                                        match f {
                                            Function::Builtin(builtin) => builtin(&arguments),
                                            _ => todo!(),
                                        }
                                    }
                                    _ => todo!(),
                                }
                            } else {
                                todo!()
                            }
                        }
                        _ => todo!(),
                    }
                } else {
                    Value::Nothing
                }
            }
        }
    }
}
