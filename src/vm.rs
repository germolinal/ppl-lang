use crate::chunk::*;
use crate::operations::*;
use crate::values::*;
use crate::parser::*;

#[cfg(test)]
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
    stack: [PPLValue<'a>;256],
    stack_top: usize,
}

impl <'a>VM<'a> {
    
    pub fn new()-> Self{
                    
        Self{
            stack: [PPLValue::PPLNil; 256],
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
            #[cfg(test)]
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

                // Unary operations
                Operation::Negate =>{
                    let value = self.pop();// This panics if there is nothing there
                    match value { 
                        PPLValue::PPLFloat(v)=>{self.push(PPLValue::PPLFloat(-v))}
                        PPLValue::PPLInt(v)=>{self.push(PPLValue::PPLInt(-v))}
                        _ => {return InterpretResult::RuntimeError(format!("Operand '-' does not work on type '{}'",value.ppl_type()))}
                    }
                },
                Operation::Not =>{
                    let value = self.pop();// This panics if there is nothing there
                    match value { 
                        PPLValue::PPLBool(v)=>{self.push(PPLValue::PPLBool(!v))}                        
                        _ => {return InterpretResult::RuntimeError(format!("Operand '!' does not work on type '{}'",value.ppl_type()))}
                    }
                },

                // Binary operations
                Operation::Add => {    
                    // Get b (from a + b)            
                    let value_b = self.pop();
                    match value_b {
                        // Result needs to be a float
                        PPLValue::PPLFloat(b)=>{
                            // get a (from a + b)
                            let value_a = self.pop();
                            match value_a {
                                PPLValue::PPLFloat(a) => self.push(PPLValue::PPLFloat(b + a)),
                                PPLValue::PPLInt(a) => self.push(PPLValue::PPLFloat(b + (a as f64) )),
                                _ => return InterpretResult::RuntimeError(format!("Trying to add over type '{}'", value_a.ppl_type()))
                            }
                        },
                        PPLValue::PPLInt(b)=>{
                            // get a (from a + b)
                            let value_a = self.pop();
                            match value_a {
                                PPLValue::PPLFloat(a) => self.push(PPLValue::PPLFloat((b as f64) + a)),
                                PPLValue::PPLInt(a) => self.push(PPLValue::PPLInt(b + a )),
                                _ => return InterpretResult::RuntimeError(format!("Trying to add over type '{}'", value_a.ppl_type()))
                            }
                        },
                        _ => return InterpretResult::RuntimeError(format!("Trying to add over type '{}'", value_b.ppl_type()))
                        
                    }                    
                },                
                Operation::Subtract => {    
                    // Get b (from a - b)            
                    let value_b = self.pop();
                    match value_b {
                        // Result needs to be a float
                        PPLValue::PPLFloat(b)=>{
                            // get a (from a - b)
                            let value_a = self.pop();
                            match value_a {
                                PPLValue::PPLFloat(a) => self.push(PPLValue::PPLFloat(a - b)),
                                PPLValue::PPLInt(a) => self.push(PPLValue::PPLFloat( (a as f64) - b )),
                                _ => return InterpretResult::RuntimeError(format!("Trying to subtract over type '{}'", value_a.ppl_type()))
                            }
                        },
                        PPLValue::PPLInt(b)=>{
                            // get a (from a + b)
                            let value_a = self.pop();
                            match value_a {
                                PPLValue::PPLFloat(a) => self.push(PPLValue::PPLFloat( a - (b as f64) )),
                                PPLValue::PPLInt(a) => self.push(PPLValue::PPLInt( a - b )),
                                _ => return InterpretResult::RuntimeError(format!("Trying to subtract over type '{}'", value_a.ppl_type()))
                            }
                        },
                        _ => return InterpretResult::RuntimeError(format!("Trying to subtract over type '{}'", value_b.ppl_type()))
                        
                    }                    
                },                
                Operation::Multiply => {    
                    // Get b (from a * b)            
                    let value_b = self.pop();
                    match value_b {
                        // Result needs to be a float
                        PPLValue::PPLFloat(b)=>{
                            // get a (from a * b)
                            let value_a = self.pop();
                            match value_a {
                                PPLValue::PPLFloat(a) => self.push(PPLValue::PPLFloat(a * b)),
                                PPLValue::PPLInt(a) => self.push(PPLValue::PPLFloat( (a as f64) * b )),
                                _ => return InterpretResult::RuntimeError(format!("Trying to multiply over type '{}'", value_a.ppl_type()))
                            }
                        },
                        PPLValue::PPLInt(b)=>{
                            // get a (from a * b)
                            let value_a = self.pop();
                            match value_a {
                                PPLValue::PPLFloat(a) => self.push(PPLValue::PPLFloat( a * (b as f64) )),
                                PPLValue::PPLInt(a) => self.push(PPLValue::PPLInt( a * b )),
                                _ => return InterpretResult::RuntimeError(format!("Trying to multiply over type '{}'", value_a.ppl_type()))
                            }
                        },
                        _ => return InterpretResult::RuntimeError(format!("Trying to multiply over type '{}'", value_b.ppl_type()))                        
                    }                    
                },                
                Operation::Divide => {    
                    // Get b (from a / b)            
                    let value_b = self.pop();
                    match value_b {
                        // Result needs to be a float
                        PPLValue::PPLFloat(b)=>{
                            // get a (from a / b)
                            let value_a = self.pop();
                            match value_a {
                                PPLValue::PPLFloat(a) => self.push(PPLValue::PPLFloat(a / b)),
                                PPLValue::PPLInt(a) => self.push(PPLValue::PPLFloat((a as f64) / b )),
                                _ => return InterpretResult::RuntimeError(format!("Trying to divide over type '{}'", value_a.ppl_type()))
                            }
                        },
                        PPLValue::PPLInt(b)=>{
                            // get a (from a * b)
                            let value_a = self.pop();
                            match value_a {
                                PPLValue::PPLFloat(a) => self.push(PPLValue::PPLFloat(a / (b as f64) )),
                                PPLValue::PPLInt(a) => self.push(PPLValue::PPLInt(a / b )),
                                _ => return InterpretResult::RuntimeError(format!("Trying to divide over type '{}'", value_a.ppl_type()))
                            }
                        },
                        _ => return InterpretResult::RuntimeError(format!("Trying to divide over type '{}'", value_b.ppl_type()))                        
                    }                    
                }

            }
        }

        return InterpretResult::RuntimeError("No RETURN operation found".to_string());
        
    }

    pub fn push(&mut self, value: PPLValue<'a> ) {        
        self.stack[self.stack_top] = value;
        self.stack_top+=1;        
    }

    pub fn pop(&mut self)->PPLValue<'a>{
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
        match vm.stack[0]{
            PPLValue::PPLNil => {},
            _ => {assert!(false)}
        }
        
        
        vm.push(PPLValue::PPLFloat(1.2));
        assert_eq!(vm.stack_top,1);

        match vm.stack[0]{
            PPLValue::PPLFloat(v) => {
                assert_eq!(v,1.2);
            },
            _ => {assert!(false)}
        }

        let value = vm.pop();
        assert_eq!(vm.stack_top,0);
        
        match value {
            PPLValue::PPLFloat(v) => {
                assert_eq!(v,1.2);
            },
            _ => {assert!(false)}
        }
        
    }

    #[test]
    fn test_constant(){
        let v = 1.2;
        let mut c = Chunk::new();

        let constant_i = c.add_constant(PPLValue::PPLFloat(v));                        
        c.write_operation(Operation::Constant(constant_i), 123);                
        c.write_operation(Operation::Return, 0);
        c.write_operation(Operation::Return, 0);
        
        assert_eq!(c.code().len(),3);

        let mut vm = VM::new();
        vm.run(&c);        
    }

    #[test]
    fn test_negate(){
        
        // Over a Float... should work
        let v = 1.2;
        let mut c = Chunk::new();
        let constant_i = c.add_constant(PPLValue::PPLFloat(v));                        
        c.write_operation(Operation::Constant(constant_i), 123);                
        c.write_operation(Operation::Negate, 124);
        c.write_operation(Operation::Return, 0);                        
        let mut vm = VM::new();
        assert!(vm.run(&c).is_ok());        
        let v2 = vm.pop().to_f64();
        assert_eq!(v2,-v);
        
        // Over an Int... should work
        let v = 12;
        let mut c = Chunk::new();
        let constant_i = c.add_constant(PPLValue::PPLInt(v));                        
        c.write_operation(Operation::Constant(constant_i), 123);                
        c.write_operation(Operation::Negate, 124);
        c.write_operation(Operation::Return, 0);                        
        let mut vm = VM::new();
        assert!(vm.run(&c).is_ok());        
        let v2 = vm.pop().to_i32();
        assert_eq!(v2,-v);
        
        // Over a Nil... should not        
        let mut c = Chunk::new();
        let constant_i = c.add_constant(PPLValue::PPLNil);                        
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
        let constant_i = c.add_constant(PPLValue::PPLFloat(v));                        
        c.write_operation(Operation::Constant(constant_i), 123);                
        c.write_operation(Operation::Not, 124);
        c.write_operation(Operation::Return, 0);                        
        let mut vm = VM::new();
        assert!(!vm.run(&c).is_ok());                
        
        // Over an Int... should not work
        let v = 12;
        let mut c = Chunk::new();
        let constant_i = c.add_constant(PPLValue::PPLInt(v));                        
        c.write_operation(Operation::Constant(constant_i), 123);                
        c.write_operation(Operation::Not, 124);
        c.write_operation(Operation::Return, 0);                        
        let mut vm = VM::new();
        assert!(!vm.run(&c).is_ok());        
        
        
        // Over a Nil... should not        
        let mut c = Chunk::new();
        let constant_i = c.add_constant(PPLValue::PPLNil);                        
        c.write_operation(Operation::Constant(constant_i), 123);                
        c.write_operation(Operation::Not, 124);
        c.write_operation(Operation::Return, 0);                        
        let mut vm = VM::new();
        assert!(!vm.run(&c).is_ok());   

        // Over a boolean... should work
        let v = true;
        let mut c = Chunk::new();
        let constant_i = c.add_constant(PPLValue::PPLBool(v));                        
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
        let a_index = chunk.add_constant(PPLValue::PPLFloat(a));                        
        chunk.write_operation(Operation::Constant(a_index), 123);                
        let b_index = chunk.add_constant(PPLValue::PPLFloat(b));                        
        chunk.write_operation(Operation::Constant(b_index), 123);                        
        chunk.write_operation(Operation::Add, 124);

        chunk.write_operation(Operation::Return, 0);                        
        let mut vm = VM::new();
        assert!(vm.run(&chunk).is_ok());                

        let c = vm.pop().to_f64();
        assert_eq!(a+b,c);

        // Float with Int... should work, return Float
        let a = 1.2;
        let b = 12;
        
        let mut chunk = Chunk::new();
        let a_index = chunk.add_constant(PPLValue::PPLFloat(a));                        
        chunk.write_operation(Operation::Constant(a_index), 123);                
        let b_index = chunk.add_constant(PPLValue::PPLInt(b));                        
        chunk.write_operation(Operation::Constant(b_index), 123);                        
        chunk.write_operation(Operation::Add, 124);

        chunk.write_operation(Operation::Return, 0);                        
        let mut vm = VM::new();
        assert!(vm.run(&chunk).is_ok());                

        let c = vm.pop().to_f64();
        assert_eq!(a+ b as f64,c);

        // Int with Float... should work, return Float
        let a = 12;
        let b = 1.123122;
        
        let mut chunk = Chunk::new();
        let a_index = chunk.add_constant(PPLValue::PPLInt(a));                        
        chunk.write_operation(Operation::Constant(a_index), 123);                
        let b_index = chunk.add_constant(PPLValue::PPLFloat(b));                        
        chunk.write_operation(Operation::Constant(b_index), 123);                        
        chunk.write_operation(Operation::Add, 124);

        chunk.write_operation(Operation::Return, 0);                        
        let mut vm = VM::new();
        assert!(vm.run(&chunk).is_ok());                

        let c = vm.pop().to_f64();
        assert_eq!(a as f64 + b,c);

        // Int with Int... should work, return Int
        let a = 12;
        let b = 31;
        
        let mut chunk = Chunk::new();
        let a_index = chunk.add_constant(PPLValue::PPLInt(a));                        
        chunk.write_operation(Operation::Constant(a_index), 123);                
        let b_index = chunk.add_constant(PPLValue::PPLInt(b));                        
        chunk.write_operation(Operation::Constant(b_index), 123);                        
        chunk.write_operation(Operation::Add, 124);

        chunk.write_operation(Operation::Return, 0);                        
        let mut vm = VM::new();
        assert!(vm.run(&chunk).is_ok());                

        let c = vm.pop().to_i32();
        assert_eq!(a + b,c);
        
        // Int over something else... should not work
        let a = 12;
        let b = true;
        
        let mut chunk = Chunk::new();
        let a_index = chunk.add_constant(PPLValue::PPLInt(a));                        
        chunk.write_operation(Operation::Constant(a_index), 123);                
        let b_index = chunk.add_constant(PPLValue::PPLBool(b));                        
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
        let a_index = chunk.add_constant(PPLValue::PPLFloat(a));                        
        chunk.write_operation(Operation::Constant(a_index), 123);                
        let b_index = chunk.add_constant(PPLValue::PPLFloat(b));                        
        chunk.write_operation(Operation::Constant(b_index), 123);                        
        chunk.write_operation(Operation::Subtract, 124);

        chunk.write_operation(Operation::Return, 0);                        
        let mut vm = VM::new();
        assert!(vm.run(&chunk).is_ok());                

        let c = vm.pop().to_f64();
        assert_eq!(a-b,c);

        // Float with Int... should work, return Float
        let a = 1.2;
        let b = 12;
        
        let mut chunk = Chunk::new();
        let a_index = chunk.add_constant(PPLValue::PPLFloat(a));                        
        chunk.write_operation(Operation::Constant(a_index), 123);                
        let b_index = chunk.add_constant(PPLValue::PPLInt(b));                        
        chunk.write_operation(Operation::Constant(b_index), 123);                        
        chunk.write_operation(Operation::Subtract, 124);

        chunk.write_operation(Operation::Return, 0);                        
        let mut vm = VM::new();
        assert!(vm.run(&chunk).is_ok());                

        let c = vm.pop().to_f64();
        assert_eq!(a - b as f64,c);

        // Int with Float... should work, return Float
        let a = 12;
        let b = 1.123122;
        
        let mut chunk = Chunk::new();
        let a_index = chunk.add_constant(PPLValue::PPLInt(a));                        
        chunk.write_operation(Operation::Constant(a_index), 123);                
        let b_index = chunk.add_constant(PPLValue::PPLFloat(b));                        
        chunk.write_operation(Operation::Constant(b_index), 123);                        
        chunk.write_operation(Operation::Subtract, 124);

        chunk.write_operation(Operation::Return, 0);                        
        let mut vm = VM::new();
        assert!(vm.run(&chunk).is_ok());                

        let c = vm.pop().to_f64();
        assert_eq!(a as f64 - b,c);

        // Int with Int... should work, return Int
        let a = 12;
        let b = 31;
        
        let mut chunk = Chunk::new();
        let a_index = chunk.add_constant(PPLValue::PPLInt(a));                        
        chunk.write_operation(Operation::Constant(a_index), 123);                
        let b_index = chunk.add_constant(PPLValue::PPLInt(b));                        
        chunk.write_operation(Operation::Constant(b_index), 123);                        
        chunk.write_operation(Operation::Subtract, 124);

        chunk.write_operation(Operation::Return, 0);                        
        let mut vm = VM::new();
        assert!(vm.run(&chunk).is_ok());                

        let c = vm.pop().to_i32();
        assert_eq!(a - b,c);
        
        // Int over something else... should not work
        let a = 12;
        let b = true;
        
        let mut chunk = Chunk::new();
        let a_index = chunk.add_constant(PPLValue::PPLInt(a));                        
        chunk.write_operation(Operation::Constant(a_index), 123);                
        let b_index = chunk.add_constant(PPLValue::PPLBool(b));                        
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
        let a_index = chunk.add_constant(PPLValue::PPLFloat(a));                        
        chunk.write_operation(Operation::Constant(a_index), 123);                
        let b_index = chunk.add_constant(PPLValue::PPLFloat(b));                        
        chunk.write_operation(Operation::Constant(b_index), 123);                        
        chunk.write_operation(Operation::Multiply, 124);

        chunk.write_operation(Operation::Return, 0);                        
        let mut vm = VM::new();
        assert!(vm.run(&chunk).is_ok());                

        let c = vm.pop().to_f64();
        assert_eq!(a*b,c);

        // Float with Int... should work, return Float
        let a = 1.2;
        let b = 12;
        
        let mut chunk = Chunk::new();
        let a_index = chunk.add_constant(PPLValue::PPLFloat(a));                        
        chunk.write_operation(Operation::Constant(a_index), 123);                
        let b_index = chunk.add_constant(PPLValue::PPLInt(b));                        
        chunk.write_operation(Operation::Constant(b_index), 123);                        
        chunk.write_operation(Operation::Multiply, 124);

        chunk.write_operation(Operation::Return, 0);                        
        let mut vm = VM::new();
        assert!(vm.run(&chunk).is_ok());                

        let c = vm.pop().to_f64();
        assert_eq!(a * b as f64,c);

        // Int with Float... should work, return Float
        let a = 12;
        let b = 1.123122;
        
        let mut chunk = Chunk::new();
        let a_index = chunk.add_constant(PPLValue::PPLInt(a));                        
        chunk.write_operation(Operation::Constant(a_index), 123);                
        let b_index = chunk.add_constant(PPLValue::PPLFloat(b));                        
        chunk.write_operation(Operation::Constant(b_index), 123);                        
        chunk.write_operation(Operation::Multiply, 124);

        chunk.write_operation(Operation::Return, 0);                        
        let mut vm = VM::new();
        assert!(vm.run(&chunk).is_ok());                

        let c = vm.pop().to_f64();
        assert_eq!(a as f64 * b,c);

        // Int with Int... should work, return Int
        let a = 12;
        let b = 31;
        
        let mut chunk = Chunk::new();
        let a_index = chunk.add_constant(PPLValue::PPLInt(a));                        
        chunk.write_operation(Operation::Constant(a_index), 123);                
        let b_index = chunk.add_constant(PPLValue::PPLInt(b));                        
        chunk.write_operation(Operation::Constant(b_index), 123);                        
        chunk.write_operation(Operation::Multiply, 124);

        chunk.write_operation(Operation::Return, 0);                        
        let mut vm = VM::new();
        assert!(vm.run(&chunk).is_ok());                

        let c = vm.pop().to_i32();
        assert_eq!(a * b,c);
        
        // Int over something else... should not work
        let a = 12;
        let b = true;
        
        let mut chunk = Chunk::new();
        let a_index = chunk.add_constant(PPLValue::PPLInt(a));                        
        chunk.write_operation(Operation::Constant(a_index), 123);                
        let b_index = chunk.add_constant(PPLValue::PPLBool(b));                        
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
        let a_index = chunk.add_constant(PPLValue::PPLFloat(a));                        
        chunk.write_operation(Operation::Constant(a_index), 123);                
        let b_index = chunk.add_constant(PPLValue::PPLFloat(b));                        
        chunk.write_operation(Operation::Constant(b_index), 123);                        
        chunk.write_operation(Operation::Divide, 124);

        chunk.write_operation(Operation::Return, 0);                        
        let mut vm = VM::new();
        assert!(vm.run(&chunk).is_ok());                

        let c = vm.pop().to_f64();
        assert_eq!(a / b,c);

        // Float with Int... should work, return Float
        let a = 1.2;
        let b = 12;
        
        let mut chunk = Chunk::new();
        let a_index = chunk.add_constant(PPLValue::PPLFloat(a));                        
        chunk.write_operation(Operation::Constant(a_index), 123);                
        let b_index = chunk.add_constant(PPLValue::PPLInt(b));                        
        chunk.write_operation(Operation::Constant(b_index), 123);                        
        chunk.write_operation(Operation::Divide, 124);

        chunk.write_operation(Operation::Return, 0);                        
        let mut vm = VM::new();
        assert!(vm.run(&chunk).is_ok());                

        let c = vm.pop().to_f64();
        assert_eq!(a / b as f64,c);

        // Int with Float... should work, return Float
        let a = 12;
        let b = 1.123122;
        
        let mut chunk = Chunk::new();
        let a_index = chunk.add_constant(PPLValue::PPLInt(a));                        
        chunk.write_operation(Operation::Constant(a_index), 123);                
        let b_index = chunk.add_constant(PPLValue::PPLFloat(b));                        
        chunk.write_operation(Operation::Constant(b_index), 123);                        
        chunk.write_operation(Operation::Divide, 124);

        chunk.write_operation(Operation::Return, 0);                        
        let mut vm = VM::new();
        assert!(vm.run(&chunk).is_ok());                

        let c = vm.pop().to_f64();
        assert_eq!(a as f64 / b,c);

        // Int with Int... should work, return Int
        let a = 12;
        let b = 31;
        
        let mut chunk = Chunk::new();
        let a_index = chunk.add_constant(PPLValue::PPLInt(a));                        
        chunk.write_operation(Operation::Constant(a_index), 123);                
        let b_index = chunk.add_constant(PPLValue::PPLInt(b));                        
        chunk.write_operation(Operation::Constant(b_index), 123);                        
        chunk.write_operation(Operation::Divide, 124);

        chunk.write_operation(Operation::Return, 0);                        
        let mut vm = VM::new();
        assert!(vm.run(&chunk).is_ok());                

        let c = vm.pop().to_i32();
        assert_eq!(a / b,c);
        
        // Int over something else... should not work
        let a = 12;
        let b = true;
        
        let mut chunk = Chunk::new();
        let a_index = chunk.add_constant(PPLValue::PPLInt(a));                        
        chunk.write_operation(Operation::Constant(a_index), 123);                
        let b_index = chunk.add_constant(PPLValue::PPLBool(b));                        
        chunk.write_operation(Operation::Constant(b_index), 123);                        
        chunk.write_operation(Operation::Divide, 124);

        chunk.write_operation(Operation::Return, 0);                        
        let mut vm = VM::new();
        assert!(!vm.run(&chunk).is_ok());                

    }
}