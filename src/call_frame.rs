use crate::function::Function;
use crate::operations::Operation;

pub struct CallFrame{
    function: Function,
    first_slot: u8,
    ip: usize,
}

impl CallFrame{
        
    pub fn new(first_slot: u8, function: Function)->Self{
        Self{
            function: function,
            first_slot: first_slot,
            ip: 0
        }
    }

    pub fn first_slot(&self)->u8{
        self.first_slot
    }

    pub fn ip(&self)->usize{
        self.ip
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
    pub fn current_instruction_and_line(&self)->Result<(Operation,usize), String>{
        if self.function.is_native() {
            return Err(format!("Trying to get Operation from function '{}' which is native", self.function.get_name()));
        }else{
            let ops_lines =self.function.chunk().unwrap().as_slice();
            let i = self.ip;
            Ok(ops_lines[i])
        }
    }

    /// Increases the callframe 'ip' value by 'n'
    pub fn jump_forward(&mut self, n: usize){
        self.ip += n;
    }

    /// Reduces the callframe 'ip' value by 'n'
    pub fn jump_backwards(&mut self, n: usize){
        if self.ip < n {
            panic!("Trying to set a CallFrame's 'ip' to a negative value")
        }
        self.ip -= n;
    }


}