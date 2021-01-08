use crate::function::Function;
use crate::operations::Operation;
use std::rc::Rc;

pub struct CallFrame{
    function: Function,
    first_slot: usize,
    ip: usize,
}

impl CallFrame{
        
    pub fn new(first_slot: usize, function: Function)->Self{
        Self{
            function: function,
            first_slot: first_slot,
            ip: 0
        }
    }

    pub fn first_slot(&self)->usize{
        self.first_slot
    }

    pub fn ip(&self)->usize{
        self.ip
    }

    pub fn n_operations(&self)->Result<usize, String>{
        if self.function.is_native(){
            return Err(format!("Trying to get the number of operations out of function '{}' which is native", self.function.get_name()));
        }else{
            Ok(self.function.chunk().unwrap().n_operations())
        }
    }

    pub fn function(&self)->&Function{
        &self.function
    }

    pub fn code_lines(&self)->Result<(&[Operation],&[usize]), String>{
        if self.function.is_native(){
            return Err(format!("Trying to get the code and lines out of function '{}' which is native", self.function.get_name()));
        }else{
            Ok(self.function.chunk().unwrap().to_slices())
        }
    }

    /// Gets the current Operation and line of that operation
    pub fn current_instruction_and_line(&self)->Result<(Operation,usize), String>{
        if self.function.is_native() {
            return Err(format!("Trying to get Operation from function '{}' which is native", self.function.get_name()));
        }else{
            let (code,lines)=self.function.chunk().unwrap().to_slices();
            let i = self.ip;
            Ok((code[i], lines[i]))
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