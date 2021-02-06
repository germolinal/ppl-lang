use crate::vm::VM;


pub type NativeFnType = fn(&mut VM, n_args: u8)->u8;


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