#![forbid(unsafe_code)]

use crate::Value::{Number, Symbol};
use std::collections::HashMap;
use std::fmt;
use std::fmt::{Debug, Display, Formatter};

#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    Number(f64),
    Symbol(String),
}

impl Display for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Number(x) => fmt::Display::fmt(&x, f),
            Symbol(x) => fmt::Display::fmt(&x, f),
        }
    }
}

#[derive(Default)]
pub struct Interpreter {
    stack: Vec<Value>,
    vars: HashMap<String, Value>,
}

impl Interpreter {
    pub fn new() -> Self {
        Interpreter {
            stack: vec![],
            vars: HashMap::new(),
        }
    }

    pub fn eval(&mut self, expr: &str) {
        let toks = expr.split_ascii_whitespace();
        for tok in toks {
            match tok {
                "+" => {
                    if let Number(a) = self.stack.pop().unwrap() {
                        if let Number(b) = self.stack.pop().unwrap() {
                            self.stack.push(Number(a + b));
                            continue;
                        }
                    }
                    panic!("invalid operands");
                }
                "-" => {
                    if let Number(a) = self.stack.pop().unwrap() {
                        if let Number(b) = self.stack.pop().unwrap() {
                            self.stack.push(Number(a - b));
                            continue;
                        }
                    }
                    panic!("invalid operands");
                }
                "*" => {
                    if let Number(a) = self.stack.pop().unwrap() {
                        if let Number(b) = self.stack.pop().unwrap() {
                            self.stack.push(Number(a * b));
                            continue;
                        }
                    }
                    panic!("invalid operands");
                }
                "/" => {
                    if let Number(a) = self.stack.pop().unwrap() {
                        if let Number(b) = self.stack.pop().unwrap() {
                            self.stack.push(Number(a / b));
                            continue;
                        }
                    }
                    panic!("invalid operands");
                }
                "set" => {
                    if let Symbol(v) = self.stack.pop().unwrap() {
                        self.vars.insert(v, self.stack.pop().unwrap());
                    } else {
                        panic!("expected a Symbol");
                    }
                }
                _ if tok.starts_with('\'') => self.stack.push(Symbol(tok[1..].to_owned())),
                _ if tok.starts_with('$') => {
                    self.stack.push(self.vars.get(&tok[1..]).unwrap().clone())
                }
                _ => {
                    let num: i32 = tok.parse().unwrap();
                    self.stack.push(Number(num as f64));
                }
            }
        }
    }

    pub fn stack(&self) -> &[Value] {
        self.stack.as_slice()
    }
}
