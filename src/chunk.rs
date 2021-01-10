

use crate::operations::*;
use crate::value_trait::ValueTrait;
use crate::heap_list::HeapList;


/// Represents a set of operations and values


pub struct Chunk {

    /// The sequential instructions to carry out
    code : Vec<Operation>,

    /// The values contained in the code
    heap : HeapList,

    /// The lines at which each instruction was created
    lines : Vec<usize>,
}


impl Chunk { 

    /// Crates a new empty Chunk    
    pub fn new()->Self{
        Self{
            code: Vec::with_capacity(1024),
            heap: HeapList::new(),
            lines: Vec::with_capacity(1024),
        }
    }

    
    pub fn code(&self)->&Vec<Operation>{
        &self.code
    }

    pub fn patch_code(&mut self, i: usize, op: Operation){
        self.code[i]=op;
    }

    pub fn get_constant(&self, i: usize)->Option<&Box<dyn ValueTrait>>{
        self.heap.get(i)
    }
    
    pub fn n_operations(&self)->usize{
        self.code.len()
    }
    
    pub fn lines(&self)->&Vec<usize>{
        &self.lines
    }

    pub fn to_slices(&self)->(&[Operation],&[usize]){
        let code : &[Operation] = &self.code;
        let lines : &[usize] = &self.lines;
        
        (code,lines)
    }

    pub fn get_sub_slices(&self, ini: usize, fin: usize)->(&[Operation],&[usize]){
        let code : &[Operation] = &self.code[ini..fin];
        let lines : &[usize] = &self.lines[ini..fin];
        
        (code,lines)
    }

    

    /// Writes an operation into the Chunk
    /// # Arguments
    /// * op: The Operation
    /// * line: The line of the script from which the operation was dispatched    
    pub fn write_operation(&mut self, op : Operation, line: usize){       
        self.code.push(op);
        self.lines.push(line);
    }
    
    
    /// Adds a value to the Heap in the Chunk
    /// # Arguments
    /// * v: the value to add    
    /// 
    /// # Panic
    /// Panics when the heap is full
    pub fn push_to_heap(&mut self, v : Box<dyn ValueTrait>)-> usize {        
        self.heap.push(v)                
    }

    pub fn heap(&self)-> &HeapList {
        &self.heap
    }
    

}


/***********/
/* TESTING */
/***********/

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let c = Chunk::new();        
        assert_eq!(0, c.code.len());             
    }

    #[test]
    fn test_write() {
        let mut c = Chunk::new();        
        c.write_operation(Operation::Return(0), 0);

        assert_eq!(1, c.code.len());
        
    }

    /*
    #[test]
    fn test_add_constant(){
        let v = 1.2;
        let mut c = Chunk::new();
        let i = c.add_constant(Value::new_number(v));

        let found = c.heap[0];
        assert_eq!(v,found.unrwap_number().unwrap());
        assert_eq!(i,0);
        


        let i = c.add_constant(Value::new_number(2.0*v));

        let found = c.heap[1];
        assert_eq!(v*2.0,found.unrwap_number().unwrap());
        assert_eq!(i,1);
        
    }
    */
}