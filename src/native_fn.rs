use crate::vm::VM;

pub type NativeFnType = fn(&mut VM, usize)->usize;


pub struct NativeFn {    
    pub func : NativeFnType,    
    pub name : String,
}

impl NativeFn{
    
}