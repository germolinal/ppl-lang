use crate::values::*;

pub struct NamedVar {
    pub name: String,
    pub typed: bool,
    pub initialized: bool,
    pub value: Value,
}

#[derive(Copy,Clone)]
pub struct Var {    
    pub typed: bool,
    pub initialized: bool,
    pub value: Value,
}

impl Var {
    pub fn new()->Self{
        Var{
            typed: false,
            initialized: false,
            value: Value::Nil
        }
    }
}



