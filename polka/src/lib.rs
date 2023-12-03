#![forbid(unsafe_code)]

use std::{collections::HashMap, fmt::Display};

////////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    Number(f64),
    Symbol(String),
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Number(num) => write!(f, "{}", num),
            Self::Symbol(sym) => write!(f, "'{}", sym),
        }
    }
}

////////////////////////////////////////////////////////////////////////////////

pub struct Interpreter {
    // TODO: your code here.
}

impl Interpreter {
    pub fn new() -> Self {
        // TODO: your code here.
        unimplemented!()
    }

    pub fn eval(&mut self, expr: &str) {
        // TODO: your code here.
        unimplemented!()
    }

    pub fn stack(&self) -> &[Value] {
        // TODO: your code here.
        unimplemented!()
    }

}
