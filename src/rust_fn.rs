use crate::vm::VM;

pub type RustFnType = fn(&mut VM, usize)->usize;


pub struct RustFn {    
    pub func : RustFnType,    
    pub name : String,
}

impl RustFn{
    
}