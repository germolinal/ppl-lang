use crate::chunk::Chunk;
use crate::value_trait::ValueTrait;

pub struct ScriptFn {
    pub name: String,
    chunk: Chunk,
    pub n_args: usize,
    //n_outs: usize,
}


impl ScriptFn {

    
    pub fn new(name: &String)->Self{
        Self {
            name: name.clone(),
            chunk: Chunk::new(),
            n_args: 0
        }
    }
    
    pub fn push_constant(&mut self, v: Box<dyn ValueTrait>)->usize{
        self.chunk.push_constant(v)
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

    pub fn set_n_args(&mut self, n_args: usize){
        self.n_args = n_args
    }
        
}

