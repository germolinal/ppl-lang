
use crate::function::Function;
use crate::operations::Operation;

#[derive(Clone)]
pub struct CallFrame{

    /// The function associated to this CallFrame
    function: Function,
    
    /// The first element in the stack corresponding
    /// to this callframe
    first_slot: u8,
    
    /// The index of the line/operation
    ip_index: usize,
    
}

impl CallFrame{
        
    pub fn new(first_slot: u8, function: Function)->Self{        
        Self{
            function,
            first_slot,
            ip_index: 0,            
        }
    }

    pub fn first_slot(&self)->u8{
        self.first_slot
    }

    pub fn ip_index(&self)->usize{
        self.ip_index
    }
    
    pub fn n_operations(&self)->Result<usize, String>{
        if self.function.is_native(){
            return Err(format!("Trying to get the number of operations out of function '{}' which is native", self.function.get_name()));
        }else{
            Ok(self.function.chunk().unwrap().len())
        }
    }

    pub fn function(&self)->&Function{
        &self.function
    }

    pub fn code_lines(&self)->Result<&[(Operation, usize)], String>{
        if self.function.is_native(){
            return Err(format!("Trying to get the code and lines out of function '{}' which is native", self.function.get_name()));
        }else{
            Ok(self.function.chunk().unwrap().as_slice())
        }
    }

    /// Gets the current Operation and line of that operation
    pub fn current_instruction(&self)->Result<Operation, String>{
        if self.function.is_native() {
            return Err(format!("Trying to get Operation from function '{}' which is native", self.function.get_name()));
        }else{
            let ops_lines =self.function.chunk().unwrap().as_slice();
            let i = self.ip_index;
            Ok(ops_lines[i].0)            
        }
    }

    /// Increases the callframe 'ip' value by 'n'
    pub fn jump_forward(&mut self, n: usize){
        debug_assert!(self.ip_index + n < self.n_operations().unwrap());
        self.ip_index += n;        
    }

    /// Reduces the callframe 'ip' value by 'n'
    pub fn jump_backwards(&mut self, n: usize){
        debug_assert!(self.ip_index <= n);            
        self.ip_index -= n;        
    }


}