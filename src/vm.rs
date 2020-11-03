use crate::chunk::*;
use crate::operations::*;
use crate::values::*;


#[cfg(debug_assertions)]
use crate::debug::*;

enum InterpretResult {
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

struct VM<'a> {
    stack: [Value<'a>;256],
    stack_top: usize,
}

impl <'a>VM<'a> {
    
    pub fn new()-> Self{
                    
        Self{
            stack: [Value::Nil; 256],
            stack_top:0,
        }
    }    

    pub fn interpret(&mut self, source : &Vec<u8>) -> InterpretResult {
        
        //compile(source);        
        return InterpretResult::Ok;
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
                                            
                for i in 0..self.stack_top {
                    print!("{}, ", self.stack[i].to_string());                    
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
                Operation::Constant(c_index) => {
                    let c = &chunk.constants()[*c_index];
                    self.push(*c)                    
                },
                Operation::PushBool(v)=>{
                    self.push(Value::new_bool(*v))
                },
                Operation::PushNil=>{
                    self.push(Value::new_nil())
                },                
                Operation::PushNumber(v)=>{
                    self.push(Value::new_number(*v))
                },

                // Unary operations
                Operation::Negate =>{
                    let value = self.pop(); // This panics if there is nothing there
                    match value.value_type(){ 
                        ValueType::Number => {
                            match value.unrwap_number(){
                                Ok(v)=>self.push(Value::new_number(-v)),
                                Err(e)=> return InterpretResult::RuntimeError(e)
                            }
                            
                        }                        
                        _ => {return InterpretResult::RuntimeError(format!("Operand '-' does not work on type '{}'",value.typename()))}                        
                    } 
                },
                Operation::Not =>{
                    let value = self.pop();// This panics if there is nothing there
                    match value.value_type() {
                        ValueType::Bool => {
                            match value.unrwap_boolean(){
                                Ok(v)=>self.push(Value::new_bool(!v)),
                                Err(e)=> return InterpretResult::RuntimeError(e)
                            }
                        },
                        _ => {return InterpretResult::RuntimeError(format!("Operand '!' does not work on type '{}'",value.typename()))}                        
                    }
                },

                // Binary operations
                Operation::Add => {    
                    // Get b (from a + b)            
                    let value_b = self.pop();
                    let b = match value_b.value_type(){
                        ValueType::Number => {
                            match value_b.unrwap_number(){
                                Ok(v)=>v,
                                Err(e)=>return InterpretResult::RuntimeError(e)
                            }
                        },
                        _ => return InterpretResult::RuntimeError(format!("Trying to add over type '{}'", value_b.typename()))
                    };
                    
                    // Get a (from a + b)            
                    let value_a = self.pop();
                    let a = match value_a.value_type(){
                        ValueType::Number => {
                            match value_a.unrwap_number(){
                                Ok(v)=>v,
                                Err(e)=>return InterpretResult::RuntimeError(e)
                            }
                        },
                        _ => return InterpretResult::RuntimeError(format!("Trying to add over type '{}'", value_b.typename()))
                    };

                    // Emit operation
                    self.push(Value::new_number(a + b));                 
                },                
                Operation::Subtract => {    
                    // Get b (from a - b)            
                    let value_b = self.pop();
                    let b = match value_b.value_type(){
                        ValueType::Number => {
                            match value_b.unrwap_number(){
                                Ok(v)=>v,
                                Err(e)=>return InterpretResult::RuntimeError(e)
                            }
                        },
                        _ => return InterpretResult::RuntimeError(format!("Trying to subtract over type '{}'", value_b.typename()))
                    };
                    
                    // Get a (from a - b)            
                    let value_a = self.pop();
                    let a = match value_a.value_type(){
                        ValueType::Number => {
                            match value_a.unrwap_number(){
                                Ok(v)=>v,
                                Err(e)=>return InterpretResult::RuntimeError(e)
                            }
                        },
                        _ => return InterpretResult::RuntimeError(format!("Trying to subtract over type '{}'", value_b.typename()))
                    };

                    // Emit operation
                    self.push(Value::new_number(a - b));                
                },                
                Operation::Multiply => {    
                    // Get b (from a * b)            
                    let value_b = self.pop();
                    let b = match value_b.value_type(){
                        ValueType::Number => {
                            match value_b.unrwap_number(){
                                Ok(v)=>v,
                                Err(e)=>return InterpretResult::RuntimeError(e)
                            }
                        },
                        _ => return InterpretResult::RuntimeError(format!("Trying to multiply over type '{}'", value_b.typename()))
                    };
                    
                    // Get a (from a * b)            
                    let value_a = self.pop();
                    let a = match value_a.value_type(){
                        ValueType::Number => {
                            match value_a.unrwap_number(){
                                Ok(v)=>v,
                                Err(e)=>return InterpretResult::RuntimeError(e)
                            }
                        },
                        _ => return InterpretResult::RuntimeError(format!("Trying to multiply over type '{}'", value_b.typename()))
                    };

                    // Emit operation
                    self.push(Value::new_number(a * b));             
                },                
                Operation::Divide => {    
                    // Get b (from a / b)            
                    let value_b = self.pop();
                    let b = match value_b.value_type(){
                        ValueType::Number => {
                            match value_b.unrwap_number(){
                                Ok(v)=>v,
                                Err(e)=>return InterpretResult::RuntimeError(e)
                            }
                        },
                        _ => return InterpretResult::RuntimeError(format!("Trying to divide over type '{}'", value_b.typename()))
                    };
                    
                    // Get a (from a / b)            
                    let value_a = self.pop();
                    let a = match value_a.value_type(){
                        ValueType::Number => {
                            match value_a.unrwap_number(){
                                Ok(v)=>v,
                                Err(e)=>return InterpretResult::RuntimeError(e)
                            }
                        },
                        _ => return InterpretResult::RuntimeError(format!("Trying to divide over type '{}'", value_b.typename()))
                    };

                    // Emit operation
                    self.push(Value::new_number(a / b));                  
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
        self.stack[self.stack_top] = value;
        self.stack_top+=1;        
    }

    pub fn pop(&mut self)->Value<'a>{
        if self.stack_top == 0 {
            panic!("Trying to pop an empty stack")
        }
        self.stack_top-=1;
        self.stack[self.stack_top]
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
        
        assert_eq!(vm.stack_top,0);
        match vm.stack[0].value_type(){
            ValueType::Nil => {},
            _ => {assert!(false)}
        }
        
        
        vm.push(Value::new_number(1.2));
        assert_eq!(vm.stack_top,1);

        match vm.stack[0].value_type(){
            ValueType::Number => {
                assert_eq!(vm.stack[0].unrwap_number().unwrap(),1.2);
            },
            _ => {assert!(false)}
        }

        let value = vm.pop();
        assert_eq!(vm.stack_top,0);
        
        match value.value_type() {
            ValueType::Number => {
                assert_eq!(value.unrwap_number().unwrap(),1.2);
            },
            _ => {assert!(false)}
        }
        
    }

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

    #[test]
    fn test_negate(){
        
        // Over a number... should work
        let v = 1.2;
        let mut c = Chunk::new();
        let constant_i = c.add_constant(Value::new_number(v));                        
        c.write_operation(Operation::Constant(constant_i), 123);                
        c.write_operation(Operation::Negate, 124);
        c.write_operation(Operation::Return, 0);                        
        let mut vm = VM::new();
        assert!(vm.run(&c).is_ok());        
        let v2 = vm.pop().to_f64().unwrap();
        assert_eq!(v2,-v);
        
        
        
        // Over a Nil... should not        
        let mut c = Chunk::new();
        let constant_i = c.add_constant(Value::new_nil());                        
        c.write_operation(Operation::Constant(constant_i), 123);                
        c.write_operation(Operation::Negate, 124);
        c.write_operation(Operation::Return, 0);                        
        let mut vm = VM::new();
        assert!(!vm.run(&c).is_ok());   
        
    }

    #[test]
    fn test_not(){
        
        // Over a Float... should not work
        let v = 1.2;
        let mut c = Chunk::new();
        let constant_i = c.add_constant(Value::new_number(v));                        
        c.write_operation(Operation::Constant(constant_i), 123);                
        c.write_operation(Operation::Not, 124);
        c.write_operation(Operation::Return, 0);                        
        let mut vm = VM::new();
        assert!(!vm.run(&c).is_ok());                
        
        
        
        
        // Over a Nil... should not        
        let mut c = Chunk::new();
        let constant_i = c.add_constant(Value::new_nil());                        
        c.write_operation(Operation::Constant(constant_i), 123);                
        c.write_operation(Operation::Not, 124);
        c.write_operation(Operation::Return, 0);                        
        let mut vm = VM::new();
        assert!(!vm.run(&c).is_ok());   

        // Over a boolean... should work
        let v = true;
        let mut c = Chunk::new();
        let constant_i = c.add_constant(Value::new_bool(v));                        
        c.write_operation(Operation::Constant(constant_i), 123);                
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
        let a_index = chunk.add_constant(Value::new_number(a));                        
        chunk.write_operation(Operation::Constant(a_index), 123);                
        let b_index = chunk.add_constant(Value::new_number(b));                        
        chunk.write_operation(Operation::Constant(b_index), 123);                        
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
        let a_index = chunk.add_constant(Value::new_number(a));                        
        chunk.write_operation(Operation::Constant(a_index), 123);                
        let b_index = chunk.add_constant(Value::new_bool(b));                        
        chunk.write_operation(Operation::Constant(b_index), 123);                        
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
        let a_index = chunk.add_constant(Value::new_number(a));                        
        chunk.write_operation(Operation::Constant(a_index), 123);                
        let b_index = chunk.add_constant(Value::new_number(b));                        
        chunk.write_operation(Operation::Constant(b_index), 123);                        
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
        let a_index = chunk.add_constant(Value::new_number(a));                        
        chunk.write_operation(Operation::Constant(a_index), 123);                
        let b_index = chunk.add_constant(Value::new_bool(b));                        
        chunk.write_operation(Operation::Constant(b_index), 123);                        
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
        let a_index = chunk.add_constant(Value::new_number(a));                        
        chunk.write_operation(Operation::Constant(a_index), 123);                
        let b_index = chunk.add_constant(Value::new_number(b));                        
        chunk.write_operation(Operation::Constant(b_index), 123);                        
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
        let a_index = chunk.add_constant(Value::new_number(a));                        
        chunk.write_operation(Operation::Constant(a_index), 123);                
        let b_index = chunk.add_constant(Value::new_bool(b));                        
        chunk.write_operation(Operation::Constant(b_index), 123);                        
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
        let a_index = chunk.add_constant(Value::new_number(a));                        
        chunk.write_operation(Operation::Constant(a_index), 123);                
        let b_index = chunk.add_constant(Value::new_number(b));                        
        chunk.write_operation(Operation::Constant(b_index), 123);                        
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
        let a_index = chunk.add_constant(Value::new_number(a));                        
        chunk.write_operation(Operation::Constant(a_index), 123);                
        let b_index = chunk.add_constant(Value::new_bool(b));                        
        chunk.write_operation(Operation::Constant(b_index), 123);                        
        chunk.write_operation(Operation::Divide, 124);

        chunk.write_operation(Operation::Return, 0);                        
        let mut vm = VM::new();
        assert!(!vm.run(&chunk).is_ok());                

    }
}