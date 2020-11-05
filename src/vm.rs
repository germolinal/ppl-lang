use crate::chunk::*;
use crate::operations::*;
use crate::values::*;
use crate::variable::Var;

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

pub struct VM<'a> {
    stack: Vec<Value<'a>>,//[Value<'a>;256],    
    var_stack: Vec<Var<'a>>,//[Var<'a>;256],    
}

impl <'a>VM<'a> {
    
    pub fn new()-> Self{
                    
        Self{
            var_stack: Vec::with_capacity(256), 
            stack: Vec::with_capacity(256),            
        }
    }    

    pub fn interpret(&mut self, _source : &Vec<u8>) -> InterpretResult {
        
        //compile(source);        
        return InterpretResult::Ok;
    }

    pub fn get_a_b_numbers(&mut self)->Result<(f64,f64),InterpretResult>{
        let errmsg = "Expecting two 'Numbers'";

        // Get b 
        let b = if let Value::Number(v) = self.pop(){
            v
        }else{
            return Err(InterpretResult::RuntimeError(errmsg.to_string()));
        };

        // get a        
        let a = if let Value::Number(v) = self.pop(){
            v
        }else{
            return Err(InterpretResult::RuntimeError(errmsg.to_string()));
        };

        return Ok((a,b));
    }

    pub fn get_number(&mut self)->Result<f64,InterpretResult>{
        // Get b 
        if let Value::Number(v) = self.pop(){
            return Ok(v)
        }else{
            return Err(InterpretResult::RuntimeError(format!("Expecting 'Number'")));
        };
    }

    pub fn get_boolean(&mut self)->Result<bool,InterpretResult>{     
        if let Value::Bool(v) = self.pop(){
            return Ok(v)
        }else{
            return Err(InterpretResult::RuntimeError(format!("Expecting 'Boolean'")));
        };
    }

    pub fn run(&mut self, chunk: &'a Chunk) -> InterpretResult {
        
        for (_offset,op) in chunk.code().iter().enumerate(){

            /*****************************/
            /* Dissassemble when testing */
            /*****************************/
            #[cfg(debug_assertions)]
            {
                // report stack
                print!("  --> Stack: [");
                                            
                for op in &self.stack {
                    print!("{}, ", op.to_string());                    
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
                    self.push(Value::new_bool(*v))
                },                
                Operation::PushNumber(v)=>{
                    self.push(Value::new_number(*v))
                },
                Operation::PushVar(v)=>{
                    self.push_var(*v);
                },
                Operation::PopVars(n)=>{                    
                    self.pop_vars(*n);                    
                },
                Operation::DefineVar(n)=>{
                    let v = self.pop();
                    self.var_stack[*n].value = v;
                },
                // Unary operations
                Operation::Negate =>{
                    match self.get_number(){
                        Ok(v)=>self.push(Value::new_number(-v)),
                        Err(e)=>return e
                    }
                },
                Operation::Not =>{
                    match self.get_boolean(){
                        Ok(v)=>self.push(Value::new_bool(!v)),
                        Err(e)=>return e
                    }                                
                },

                // Binary operations
                Operation::Add => {    
                    match self.get_a_b_numbers(){
                        Ok((a,b))=>{
                            self.push(Value::new_number(a + b));                 
                        },
                        Err(e)=>return e
                    };
                },                
                Operation::Subtract => {    
                    match self.get_a_b_numbers(){
                        Ok((a,b))=>{
                            self.push(Value::new_number(a - b));                 
                        },
                        Err(e)=>return e
                    };              
                },                
                Operation::Multiply => {    
                    match self.get_a_b_numbers(){
                        Ok((a,b))=>{
                            self.push(Value::new_number(a * b));                 
                        },
                        Err(e)=>return e
                    };           
                },                
                Operation::Divide => {    
                    match self.get_a_b_numbers(){
                        Ok((a,b))=>{
                            self.push(Value::new_number(a / b));                 
                        },
                        Err(e)=>return e
                    };
                },
                Operation::Equal => {
                    // Get b (from a == b)            
                    let value_b = self.pop();
                    let type_b = value_b.value_type();
                    // Get a (from a == b)            
                    let value_a = self.pop();
                    let type_a = value_a.value_type();

                    //
                    if type_a != type_b {
                        self.push(Value::new_bool(false));                                           
                    }else{
                        let b = match value_b.value_type(){
                            ValueType::Number => {
                                match value_b.unrwap_number(){
                                    Ok(v)=>v,
                                    Err(e)=>return InterpretResult::RuntimeError(e)
                                }
                            },
                            _ => return InterpretResult::RuntimeError(format!("Trying to == over type '{}'", value_b.typename()))
                        };
                        
                        
                        let a = match value_a.value_type(){
                            ValueType::Number => {
                                match value_a.unrwap_number(){
                                    Ok(v)=>v,
                                    Err(e)=>return InterpretResult::RuntimeError(e)
                                }
                            },
                            _ => return InterpretResult::RuntimeError(format!("Trying to == over type '{}'", value_b.typename()))
                        };
    
                        // Emit operation
                        self.push(Value::new_bool(a == b));   
                    }
                                                        
                },
                Operation::Greater => {
                    // Get b (from a == b)            
                    let value_b = self.pop();
                    let type_b = value_b.value_type();
                    // Get a (from a == b)            
                    let value_a = self.pop();
                    let type_a = value_a.value_type();

                    //
                    if type_a != type_b {
                        self.push(Value::new_bool(false));                   
                        
                    }else{
                        let b = match value_b.value_type(){
                            ValueType::Number => {
                                match value_b.unrwap_number(){
                                    Ok(v)=>v,
                                    Err(e)=>return InterpretResult::RuntimeError(e)
                                }
                            },
                            _ => return InterpretResult::RuntimeError(format!("Trying to == over type '{}'", value_b.typename()))
                        };
                        
                        
                        let a = match value_a.value_type(){
                            ValueType::Number => {
                                match value_a.unrwap_number(){
                                    Ok(v)=>v,
                                    Err(e)=>return InterpretResult::RuntimeError(e)
                                }
                            },
                            _ => return InterpretResult::RuntimeError(format!("Trying to == over type '{}'", value_b.typename()))
                        };
    
                        // Emit operation
                        self.push(Value::new_bool(a > b));   
                    }
                },
                Operation::Less => {
                    // Get b (from a == b)            
                    let value_b = self.pop();
                    let type_b = value_b.value_type();
                    // Get a (from a == b)            
                    let value_a = self.pop();
                    let type_a = value_a.value_type();

                    //
                    if type_a != type_b {
                        self.push(Value::new_bool(false));                   
                        
                    }else{
                        let b = match value_b.value_type(){
                            ValueType::Number => {
                                match value_b.unrwap_number(){
                                    Ok(v)=>v,
                                    Err(e)=>return InterpretResult::RuntimeError(e)
                                }
                            },
                            _ => return InterpretResult::RuntimeError(format!("Trying to == over type '{}'", value_b.typename()))
                        };
                        
                        
                        let a = match value_a.value_type(){
                            ValueType::Number => {
                                match value_a.unrwap_number(){
                                    Ok(v)=>v,
                                    Err(e)=>return InterpretResult::RuntimeError(e)
                                }
                            },
                            _ => return InterpretResult::RuntimeError(format!("Trying to == over type '{}'", value_b.typename()))
                        };
    
                        // Emit operation
                        self.push(Value::new_bool(a < b));   
                    }
                }

            }
        }

        return InterpretResult::RuntimeError("No RETURN operation found".to_string());
        
    }

    pub fn push(&mut self, value: Value<'a> ) {        
        self.stack.push(value);        
    }

    fn push_var(&mut self,var: Var<'a>){
        self.var_stack.push(var);     
    }

    fn pop_vars(&mut self, n: usize){
        for _ in 0..n{
            if !self.var_stack.pop().is_some(){
                panic!("Trying to pop an empty variable stack")
            }            
        }
    }

    pub fn pop(&mut self)->Value<'a>{
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
    fn test_pop_empty_stack(){
        let mut vm = VM::new();
        vm.pop();
    }

    #[test]
    fn test_push_pop(){
        let mut vm = VM::new();
        
        assert_eq!(vm.stack.len(),0);
        match vm.stack[0].value_type(){
            ValueType::Nil => {},
            _ => {assert!(false)}
        }
        
        
        vm.push(Value::new_number(1.2));
        assert_eq!(vm.stack.len(),1);

        match vm.stack[0].value_type(){
            ValueType::Number => {
                assert_eq!(vm.stack[0].unrwap_number().unwrap(),1.2);
            },
            _ => {assert!(false)}
        }

        let value = vm.pop();
        assert_eq!(vm.stack.len(),0);
        
        match value.value_type() {
            ValueType::Number => {
                assert_eq!(value.unrwap_number().unwrap(),1.2);
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
        let v2 = vm.pop().to_f64().unwrap();
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

        let c = vm.pop().to_f64().unwrap();
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

        let c = vm.pop().to_f64().unwrap();
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

        let c = vm.pop().to_f64().unwrap();
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

        let c = vm.pop().to_f64().unwrap();
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