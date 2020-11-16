use crate::handler::Handler;

pub type RustFnType = fn(&mut Handler, usize)->usize;


pub struct RustFn {    
    pub func : RustFnType,    
    pub name : String,
}

impl RustFn {
    
    
}

