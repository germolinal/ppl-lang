use std::ops::{Deref, DerefMut};
use crate::chunk::Chunk;


pub struct ScriptFn {
    pub name: String,
    chunk: Chunk,
    n_args: usize,
    //n_outs: usize,
}


impl ScriptFn {

    
    pub fn new(name: String)->Self{
        Self {
            name: name,
            chunk: Chunk::new(),
            n_args: 0
        }
    }
    

    pub fn chunk(&self)->&Chunk{
        &self.chunk
    }

    pub fn mut_chunk(&mut self)->&mut Chunk{
        &mut self.chunk
    }

    pub fn set_name(&mut self,name:&String){        
        self.name = name.clone();
    }
        
}


/*
use std::rc::Rc;
impl Deref for ScriptFn{
    
    type Target = Chunk;
    
    fn deref(&self)->&Self::Target{
        &self.chunk
    }
}



impl DerefMut for ScriptFn{
    
    //type Target = ScriptFn;
    
    fn deref_mut(&mut self)->&mut Self::Target{
        &mut self.chunk
    }
}

*/