use crate::values::*;
use crate::value_trait::ValueTrait;

pub struct NamedVar {
    pub name: String,
    pub typed: bool,
    pub initialized: bool,
    pub value: Value,
}


pub struct Var {    
    pub typed: bool,
    pub initialized: bool,
    pub value: Value,
}

impl Var {

    pub fn copy(&self)->Self{
        Var{
            typed: self.typed,
            initialized: self.initialized,
            value: self.value.copy_to_value()
        }
    }    
}

impl Clone for Var {
    fn clone(&self)->Self{
        Var{
            typed: self.typed,
            initialized: self.initialized,
            value: self.value.clone_to_value()
        }
    }
}


