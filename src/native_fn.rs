use crate::vm::VM;

pub type NativeFnType = fn(&mut VM, usize)->usize;


pub struct NativeFn{    
    pub func : NativeFnType,    
    pub name: String
}

impl NativeFn {
    pub fn new(name : &[u8], func: NativeFnType)->Self{
        
        NativeFn {
            name: std::str::from_utf8(name).unwrap().to_string(),
            func: func
        }     
    }
}