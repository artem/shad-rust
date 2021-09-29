#![forbid(unsafe_code)]

#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    Number(f64),
    Symbol(String),
}

#[derive(Default)]
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
