use crate::operations::*;
use crate::values::Value;

/// Represents a set of operations and values
pub struct Chunk<'a> {

    /// The sequential instructions to carry out
    code : Vec<Operation>,

    /// The values contained in the code
    constants : Vec<Value<'a>>,

    /// The lines at which each instruction was created
    lines : Vec<usize>,
}

impl <'a>Chunk<'a> { 

    /// Crates a new empty Chunk
    pub fn new()->Self{
        Self{
            code: Vec::with_capacity(1024),
            constants: Vec::with_capacity(1024),
            lines: Vec::with_capacity(1024),
        }
    }

    pub fn code(&self)->&Vec<Operation>{
        &self.code
    }

    
    pub fn constants(&self)->&Vec<Value>{
        &self.constants
    }

    pub fn lines(&self)->&Vec<usize>{
        &self.lines
    }

    

    /// Writes an operation into the Chunk
    /// # Arguments
    /// * op: The Operation
    /// * line: The line of the script from which the operation was dispatched
    pub fn write_operation(&mut self, op : Operation, line: usize){       
        self.code.push(op);
        self.lines.push(line);
    }
    
    /// Adds a value to the Chunk
    /// # Arguments
    /// * v: the value to add
    pub fn add_constant(&mut self, v : Value<'a>)-> usize {
        //if self.constants.len() >= (std::u8::MAX-1) as usize {
        //    panic!("The max number of constants in chunk ({}) has been exceeded", std::u8::MAX);
        //}

        let ret = self.constants.len();// as u8;
        self.constants.push(v);
        return ret;
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
        assert_eq!(0, c.constants.len());        
    }

    #[test]
    fn test_write() {
        let mut c = Chunk::new();        
        c.write_operation(Operation::Return, 0);

        assert_eq!(1, c.code.len());
        
    }

    #[test]
    fn test_add_constant(){
        let v = 1.2;
        let mut c = Chunk::new();
        let i = c.add_constant(Value::new_number(v));

        if let found = c.constants[0] {            
            assert_eq!(v,found.unrwap_number().unwrap());
            assert_eq!(i,0);
        }else{
            assert!(false);
        }


        let i = c.add_constant(Value::new_number(2.0*v));

        if let found = c.constants[1] {            
            assert_eq!(v*2.0,found.unrwap_number().unwrap());
            assert_eq!(i,1);
        }else{
            assert!(false);
        }
    }
}