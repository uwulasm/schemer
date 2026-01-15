use std::{collections::HashMap, fmt::Display};

use crate::parser::Sexpr;

#[derive(Debug, Clone)]
pub enum Value {
    LiteralList(Vec<Value>),
    String(String),
    Function(Function),
    Number(f32),
    Nothing,
    True,
    False,
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::String(x) => write!(f, "{x}")?,
            Value::Function(_f) => write!(f, "function")?,
            Value::Number(x) => write!(f, "{x}")?,
            Value::Nothing => {}
            Value::True => write!(f, "true")?,
            Value::False => write!(f, "false")?,
            Value::LiteralList(l) => {
                write!(f, "'(")?;
                if let Some(first) = l.first() {
                    write!(f, "{first}")?;
                }
                for value in l.iter().skip(1) {
                    write!(f, " {value}")?;
                }
                write!(f, ")")?;
            }
        };
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub enum Function {
    LangFunction(Vec<String>, Sexpr),
    Builtin(fn(&[Value]) -> Value),
}

pub struct VM {
    special_cases: HashMap<String, fn(&mut Self, &[Sexpr]) -> Value>,
    variables: Vec<HashMap<String, Value>>,
}

impl VM {
    pub fn new() -> VM {
        VM {
            special_cases: HashMap::from([
                (
                    "if".to_string(),
                    VM::if_special_case as fn(&mut Self, &[Sexpr]) -> Value,
                ),
                (
                    "define".to_string(),
                    VM::define_special_case as fn(&mut Self, &[Sexpr]) -> Value,
                ),
                (
                    "begin".to_string(),
                    VM::begin_special_case as fn(&mut Self, &[Sexpr]) -> Value,
                ),
            ]),
            variables: vec![HashMap::from([
                (
                    "+".to_string(),
                    Value::Function(Function::Builtin(VM::add_builtin)),
                ),
                (
                    "*".to_string(),
                    Value::Function(Function::Builtin(VM::mul_builtin)),
                ),
                (
                    "-".to_string(),
                    Value::Function(Function::Builtin(VM::sub_builtin)),
                ),
                (
                    "display".to_string(),
                    Value::Function(Function::Builtin(VM::display_builtin)),
                ),
                (
                    "=".to_string(),
                    Value::Function(Function::Builtin(VM::eq_builtin)),
                ),
            ])],
        }
    }

    fn if_special_case(&mut self, exprs: &[Sexpr]) -> Value {
        let condition = self.evaluate(&exprs[1]);

        let truth_value = {
            match condition {
                Value::False => false,
                _ => true,
            }
        };

        if truth_value {
            self.evaluate(&exprs[2])
        } else {
            if let Some(else_cond) = exprs.get(3) {
                self.evaluate(else_cond)
            } else {
                Value::Nothing
            }
        }
    }

    fn define_special_case(&mut self, exprs: &[Sexpr]) -> Value {
        match exprs.get(1) {
            Some(Sexpr::Ident(x)) => {
                let value = self.evaluate(&exprs[2]);
                self.variables.last_mut().unwrap().insert(x.clone(), value);
            }

            Some(Sexpr::List(x)) => {
                let mut args = Vec::new();
                let Sexpr::Ident(function_name) = x[0].clone() else {
                    todo!()
                };

                for arg in &x[1..] {
                    let Sexpr::Ident(argument) = arg else { todo!() };
                    args.push(argument.clone());
                }

                self.variables.last_mut().unwrap().insert(
                    function_name,
                    Value::Function(Function::LangFunction(args, exprs[2].clone())),
                );
            }
            _ => {}
        };

        Value::Nothing
    }

    fn begin_special_case(&mut self, exprs: &[Sexpr]) -> Value {
        let mut output = Value::Nothing;
        for expr in &exprs[1..] {
            output = self.evaluate(expr);
        }
        output
    }

    fn add_builtin(values: &[Value]) -> Value {
        let mut total = 0f32;
        for value in values {
            let Value::Number(x) = value else { todo!() };
            total += x;
        }

        Value::Number(total)
    }

    fn mul_builtin(values: &[Value]) -> Value {
        let mut total = 1f32;
        for value in values {
            let Value::Number(x) = value else { todo!() };
            total *= x;
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
            println!("{value}");
        }

        Value::Nothing
    }

    fn eq_builtin(values: &[Value]) -> Value {
        let Value::Number(a) = values[0] else { todo!() };

        let Value::Number(b) = values[1] else { todo!() };

        if a == b {
            Value::True
        } else {
            Value::False
        }
    }

    fn get_variable(&self, x: &str) -> Option<Value> {
        for variable_set in self.variables.iter().rev() {
            if let Some(v) = variable_set.get(x) {
                return Some(v.clone());
            }
        }

        None
    }

    pub fn evaluate(&mut self, expr: &Sexpr) -> Value {
        match expr {
            Sexpr::String(x) => Value::String(x.clone()),
            Sexpr::Number(x) => Value::Number(*x),
            Sexpr::Ident(x) => self.get_variable(&x).unwrap(),
            Sexpr::LiteralList(list) => {
                Value::LiteralList(list.iter().map(|expr| self.evaluate(expr)).collect())
            }
            Sexpr::List(list) => {
                if let Some(prefix) = list.first() {
                    match prefix {
                        Sexpr::Ident(x) => {
                            if let Some(&special_case) = self.special_cases.get(x) {
                                special_case(self, list)
                            } else if let Some(v) = self.get_variable(x) {
                                match &v {
                                    Value::Function(f) => {
                                        let arguments = list
                                            .iter()
                                            .skip(1)
                                            .map(|x| self.evaluate(x))
                                            .collect::<Vec<_>>();
                                        match f {
                                            Function::Builtin(builtin) => builtin(&arguments),
                                            Function::LangFunction(args, body) => {
                                                self.variables.push(HashMap::new());
                                                for (e, arg) in (&list[1..]).iter().zip(args) {
                                                    let value = self.evaluate(&e);
                                                    self.variables
                                                        .last_mut()
                                                        .unwrap()
                                                        .insert(arg.clone(), value);
                                                }
                                                let output = self.evaluate(&body);
                                                self.variables.pop();
                                                output
                                            }
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
