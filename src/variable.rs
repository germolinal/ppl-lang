use crate::values::*;

pub struct NamedVar<'a> {
    pub name: String,
    pub value_type: ValueType,
    pub value: Value<'a>,
}

#[derive(Copy,Clone)]
pub struct Var<'a> {    
    pub value_type: ValueType,
    pub value: Value<'a>,
}



