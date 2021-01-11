

use crate::operations::*;
//use crate::value_trait::ValueTrait;
//use crate::heap_list::HeapList;

pub type Chunk = Vec<(Operation, usize)>;



/*

/// Represents a set of operations and values
pub struct Chunk {

    /// The sequential instructions to carry out
    code : Vec<(Operation, usize)>,

    /// The values contained in the code
    heap : HeapList,

    // The lines from which each instruction was created
    //lines : Vec<usize>,
}
*/

/*
impl Chunk { 

    /// Crates a new empty Chunk    
    pub fn new() -> Self {
        Vec::with_capacity(1024)
        //Self{
        //    code: Vec::with_capacity(1024),
            //heap: HeapList::new(),
            //lines: Vec::with_capacity(1024),
        //}
    }

    /*
    /// Borrows the code array
    pub fn code(&self)->&Vec<Operation>{
        &self.code
    }

    pub fn lines(&self)->&Vec<usize>{
        &self.lines
    }
    */
    /// Replaces an operation in the Chunk's code
    pub fn patch_code(&mut self, i: usize, op: Operation){
        let (_,ln)=self[i];
        self[i]=(op, ln);
    }

    /*
    pub fn get_heap_value(&self, i: usize)->Option<&Box<dyn ValueTrait>>{
        self.heap.get(i)
    }
    */

    pub fn n_operations(&self)->usize{
        self.len()
    }
    
    /*
    pub fn to_slice(&self)->(&[Operation],&[usize]){
        &[self]
        let code : &[Operation] = &self.code;
        let lines : &[usize] = &self.lines;
        
        (code,lines)
    }
    
    pub fn get_sub_slices(&self, ini: usize, fin: usize)->(&[Operation],&[usize]){
        let code : &[Operation] = &self.code[ini..fin];
        let lines : &[usize] = &self.lines[ini..fin];
        
        (code,lines)
    }
    */

    

    /// Writes an operation into the Chunk
    /// # Arguments
    /// * op: The Operation
    /// * line: The line of the script from which the operation was dispatched    
    pub fn write_operation(&mut self, op : Operation, line: usize){       
        
        //self.code.push(op);
        //self.lines.push(line);
        self.push((op,line));
    }
    
    /*
    /// Adds a value to the Heap in the Chunk
    /// # Arguments
    /// * v: the value to add    
    /// 
    /// # Panic
    /// Panics when the heap is full
    pub fn push_to_heap(&mut self, v : Box<dyn ValueTrait>)-> usize {        
        self.heap.push(v)                
    }

    /// Borrows the Heap
    pub fn heap(&self)-> &HeapList {
        &self.heap
    }
    */
    

}


/***********/
/* TESTING */
/***********/

#[cfg(test)]
mod tests {
    use super::*;
    //use crate::values::*;

    #[test]
    fn test_new() {
        let c = Chunk::new();        
        assert_eq!(0, c.code.len());             
    }

    #[test]
    fn test_write() {
        let mut c = Chunk::new();        
        c.write_operation(Operation::Return, 0);

        assert_eq!(1, c.code.len());
        
    }

    
    
}
*/