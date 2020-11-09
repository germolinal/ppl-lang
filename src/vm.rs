use crate::chunk::*;
use crate::operations::*;
use crate::values::*;
use crate::variable::Var;
use crate::stack::Stack;
use crate::value_trait::ValueTrait;

#[cfg(debug_assertions)]
use crate::debug::*;

pub enum InterpretResult {
    Ok,
    CompileError(String),
    RuntimeError(String),
}

impl InterpretResult {
    pub fn is_ok(&self)->bool{
        match self {
            InterpretResult::Ok => true,
            _ => false
        }
    }
}

pub struct VM {
    stack: Stack<Value>,
    var_stack: Stack<Var>,
}


impl VM {
    
    pub fn new()-> Self{
                    
        Self{
            var_stack: Stack::new(Var::new()),//Vec::with_capacity(256), 
            stack: Stack::new(Value::Nil),//Vec::with_capacity(256),            
        }
    }    

    pub fn interpret(&mut self, _source : &Vec<u8>) -> InterpretResult {
        
        //compile(source);        
        return InterpretResult::Ok;
    }

    
    pub fn run(&mut self, chunk: & Chunk) -> InterpretResult {
        
        for (_offset,op) in chunk.code().iter().enumerate(){

            /*****************************/
            /* Dissassemble when testing */
            /*****************************/
            #[cfg(debug_assertions)]
            {
                // report stack
                print!("  --> Stack: [");
                                            
                for i in 0..self.stack.len() {
                    let val = self.stack[i];
                    print!("{}, ", val.to_string());                    
                }
                print!("]\n");

                // Report operation                
                debug::operation(chunk, _offset);                
            }
            /*****************************/
            /*****************************/
            /*****************************/
                        
            match op {
                Operation::Return =>{   

                    return InterpretResult::Ok;
                },
                /*
                Operation::Constant(c_index) => {
                    let c = &chunk.constants()[*c_index];
                    self.push(*c)                    
                },
                */
                Operation::PushBool(v)=>{
                    self.push(Value::Bool(*v))
                },                
                Operation::PushNumber(v)=>{
                    self.push(Value::Number(*v))
                },
                Operation::PushVar(v)=>{
                    self.push_var(*v);
                },
                Operation::PopVars(n)=>{                    
                    for _ in 0..*n {
                        self.pop_var();                    
                    }
                },
                Operation::DefineVar(n)=>{
                    let v = self.pop();                    
                    self.var_stack[*n].value = v;
                    self.var_stack[*n].initialized = true;
                },
                // Unary operations
                Operation::Negate =>{                    
                    match self.pop().negate(){
                        Ok(v)=>self.push(v),
                        Err(e)=>return InterpretResult::RuntimeError(e)
                    }
                    
                },
                Operation::Not =>{
                    match self.pop().not(){
                        Ok(v)=>self.push(v),
                        Err(e)=>return InterpretResult::RuntimeError(e)
                    }                                
                },

                // Binary operations
                Operation::Add => {    
                    let b = self.pop();
                    let a = self.pop();
                    match a.add(b){
                        Ok(v)=>self.push(v),
                        Err(e)=>return InterpretResult::RuntimeError(e)
                    }  
                },                
                Operation::Subtract => {    
                    let b = self.pop();
                    let a = self.pop();
                    match a.subtract(b){
                        Ok(v)=>self.push(v),
                        Err(e)=>return InterpretResult::RuntimeError(e)
                    }               
                },                
                Operation::Multiply => {    
                    let b = self.pop();
                    let a = self.pop();
                    match a.multiply(b){
                        Ok(v)=>self.push(v),
                        Err(e)=>return InterpretResult::RuntimeError(e)
                    }           
                },                
                Operation::Divide => {    
                    let b = self.pop();
                    let a = self.pop();
                    match a.divide(b){
                        Ok(v)=>self.push(v),
                        Err(e)=>return InterpretResult::RuntimeError(e)
                    }       
                },
                Operation::Equal => {
                    let b = self.pop();
                    let a = self.pop();
                    match a.compare_equal(b){
                        Ok(v)=>self.push(v),
                        Err(e)=>return InterpretResult::RuntimeError(e)
                    }       
                                                        
                },

                Operation::NotEqual => {
                    let b = self.pop();
                    let a = self.pop();
                    match a.compare_not_equal(b){
                        Ok(v)=>self.push(v),
                        Err(e)=>return InterpretResult::RuntimeError(e)
                    }       
                                                        
                },
                Operation::Greater => {
                    let b = self.pop();
                    let a = self.pop();
                    match a.greater(b){
                        Ok(v)=>self.push(v),
                        Err(e)=>return InterpretResult::RuntimeError(e)
                    }   
                },
                Operation::Less => {
                    let b = self.pop();
                    let a = self.pop();
                    match a.less(b){
                        Ok(v)=>self.push(v),
                        Err(e)=>return InterpretResult::RuntimeError(e)
                    }                       
                },
                Operation::GreaterEqual => {
                    let b = self.pop();
                    let a = self.pop();
                    match a.greater_equal(b){
                        Ok(v)=>self.push(v),
                        Err(e)=>return InterpretResult::RuntimeError(e)
                    }   
                },
                Operation::LessEqual => {
                    let b = self.pop();
                    let a = self.pop();
                    match a.less_equal(b){
                        Ok(v)=>self.push(v),
                        Err(e)=>return InterpretResult::RuntimeError(e)
                    }   
                    
                }

            }
        }

        return InterpretResult::RuntimeError("No RETURN operation found".to_string());
        
    }

    pub fn push(&mut self, value: Value ) {        
        self.stack.push(value);        
    }

    fn push_var(&mut self,var: Var){
        self.var_stack.push(var);     
    }

    fn pop_var(&mut self)->Var{        
        if let Some(v)= self.var_stack.pop(){
            v
        }else{
            panic!("Trying to pop an empty stack")
        }   
    }

    pub fn pop(&mut self)->Value{
        if let Some(v)= self.stack.pop(){
            v
        }else{
            panic!("Trying to pop an empty stack")
        }
    }
}


/***********/
/* TESTING */
/***********/

#[cfg(test)]
mod tests {
    use super::*;

    
    #[test]
    #[should_panic]
    fn test_pocp_empty_stack(){
        let mut vm = VM::new();
        vm.pop();
    }

    #[test]
    fn test_push_pop(){
        let mut vm = VM::new();
        
        assert_eq!(vm.stack.len(),0);
        match vm.stack[0]{
            Value::Nil => {},
            _ => {assert!(false)}
        }
        
        
        vm.push(Value::Number(1.2));
        assert_eq!(vm.stack.len(),1);

        match vm.stack[0]{
            Value::Number(v) => {
                assert_eq!(v,1.2);
            },
            _ => {assert!(false)}
        }

        let value = vm.pop();
        assert_eq!(vm.stack.len(),0);
        
        match value{
            Value::Number(v) => {
                assert_eq!(v,1.2);
            },
            _ => {assert!(false)}
        }
        
    }

    /*
    #[test]
    fn test_constant(){
        let v = 1.2;
        let mut c = Chunk::new();

        let constant_i = c.add_constant(Value::new_number(v));                        
        c.write_operation(Operation::Constant(constant_i), 123);                
        c.write_operation(Operation::Return, 0);
        c.write_operation(Operation::Return, 0);
        
        assert_eq!(c.code().len(),3);

        let mut vm = VM::new();
        vm.run(&c);        
    }
    */

    #[test]
    fn test_negate(){
        
        // Over a number... should work
        let v = 1.2;
        let mut c = Chunk::new();        
        c.write_operation(Operation::PushNumber(v), 123);                
        c.write_operation(Operation::Negate, 124);
        c.write_operation(Operation::Return, 0);                        
        let mut vm = VM::new();
        assert!(vm.run(&c).is_ok()); 

        let v2 = vm.pop().get_number().unwrap();
        assert_eq!(v2,-v);
        
            
        
    }

    #[test]
    fn test_not(){
        
        // Over a Float... should not work
        let v = 1.2;
        let mut c = Chunk::new();        
        c.write_operation(Operation::PushNumber(v), 123);                
        c.write_operation(Operation::Not, 124);
        c.write_operation(Operation::Return, 0);                        
        let mut vm = VM::new();
        assert!(!vm.run(&c).is_ok());                
        
            

        // Over a boolean... should work
        let v = true;
        let mut c = Chunk::new();        
        c.write_operation(Operation::PushBool(v), 123);                
        c.write_operation(Operation::Not, 124);
        c.write_operation(Operation::Return, 0);                        
        let mut vm = VM::new();
        assert!(vm.run(&c).is_ok());   
        
    }


    #[test]
    fn test_add(){
        
        // Float with Float... should work
        let a = 1.2;
        let b = 12.21231;
        
        let mut chunk = Chunk::new();
        
        chunk.write_operation(Operation::PushNumber(a), 123);                        
        chunk.write_operation(Operation::PushNumber(b), 123);                        
        chunk.write_operation(Operation::Add, 124);

        chunk.write_operation(Operation::Return, 0);                        
        let mut vm = VM::new();
        assert!(vm.run(&chunk).is_ok());                

        let c = vm.pop().get_number().unwrap();
        assert_eq!(a+b,c);

        
        // Int over something else... should not work
        let a = 11.2;
        let b = true;
        
        let mut chunk = Chunk::new();        
        chunk.write_operation(Operation::PushNumber(a), 123);                        
        chunk.write_operation(Operation::PushBool(b), 123);                        
        chunk.write_operation(Operation::Add, 124);

        chunk.write_operation(Operation::Return, 0);                        
        let mut vm = VM::new();
        assert!(!vm.run(&chunk).is_ok());                

    }

    #[test]
    fn test_subtract(){
        
        // Float with Float... should work
        let a = 1.2;
        let b = 12.21231;
        
        let mut chunk = Chunk::new();        
        chunk.write_operation(Operation::PushNumber(a), 123);                        
        chunk.write_operation(Operation::PushNumber(b), 123);                        
        chunk.write_operation(Operation::Subtract, 124);

        chunk.write_operation(Operation::Return, 0);                        
        let mut vm = VM::new();
        assert!(vm.run(&chunk).is_ok());                

        let c = vm.pop().get_number().unwrap();
        assert_eq!(a-b,c);

        
        
        // Int over something else... should not work
        let a = 12.;
        let b = true;
        
        let mut chunk = Chunk::new();
        
        chunk.write_operation(Operation::PushNumber(a), 123);                        
        chunk.write_operation(Operation::PushBool(b), 123);                        
        chunk.write_operation(Operation::Subtract, 124);

        chunk.write_operation(Operation::Return, 0);                        
        let mut vm = VM::new();
        assert!(!vm.run(&chunk).is_ok());                
    }

    #[test]
    fn test_multiply(){
        
        // Float with Float... should work
        let a = 1.2;
        let b = 12.21231;
        
        let mut chunk = Chunk::new();        
        chunk.write_operation(Operation::PushNumber(a), 123);                        
        chunk.write_operation(Operation::PushNumber(b), 123);                        
        chunk.write_operation(Operation::Multiply, 124);

        chunk.write_operation(Operation::Return, 0);                        
        let mut vm = VM::new();
        assert!(vm.run(&chunk).is_ok());                

        let c = vm.pop().get_number().unwrap();
        assert_eq!(a*b,c);

        
        // Int over something else... should not work
        let a = 12.2;
        let b = true;
        
        let mut chunk = Chunk::new();        
        chunk.write_operation(Operation::PushNumber(a), 123);                        
        chunk.write_operation(Operation::PushBool(b), 123);                        
        chunk.write_operation(Operation::Multiply, 124);

        chunk.write_operation(Operation::Return, 0);                        
        let mut vm = VM::new();
        assert!(!vm.run(&chunk).is_ok());                

    }

    #[test]
    fn test_divide(){
        
        // Float with Float... should work
        let a = 1.2;
        let b = 12.21231;
        
        let mut chunk = Chunk::new();        
        chunk.write_operation(Operation::PushNumber(a), 123);                        
        chunk.write_operation(Operation::PushNumber(b), 123);                        
        chunk.write_operation(Operation::Divide, 124);

        chunk.write_operation(Operation::Return, 0);                        
        let mut vm = VM::new();
        assert!(vm.run(&chunk).is_ok());                

        let c = vm.pop().get_number().unwrap();
        assert_eq!(a / b,c);

        
        // Int over something else... should not work
        let a = 12.1;
        let b = true;
        
        let mut chunk = Chunk::new();              
        chunk.write_operation(Operation::PushNumber(a), 123);                        
        chunk.write_operation(Operation::PushBool(b), 123);                        
        chunk.write_operation(Operation::Divide, 124);

        chunk.write_operation(Operation::Return, 0);                        
        let mut vm = VM::new();
        assert!(!vm.run(&chunk).is_ok());                

    }
}