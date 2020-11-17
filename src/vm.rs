
use crate::operations::*;
use crate::values::*;
use crate::value_trait::ValueTrait;
//use crate::array::Array;

#[cfg(debug_assertions)]
use crate::debug::*;

pub enum InterpretResult {
    Ok(usize),
    //CompileError(String),
    RuntimeError(String),
}



pub struct VM {
    stack: Vec<Value>,
    var_stack: Vec<Value>,        
}


impl VM {
    
    pub fn new()-> Self{
                    
        Self {
            var_stack: Vec::with_capacity(1024),
            stack: Vec::with_capacity(1024),                                    
        }

    }    

    /*
    pub fn interpret(&mut self, _source : &Vec<Operation>) -> InterpretResult {
        
        //compile(source);        
        return InterpretResult::Ok;
    }
    
    fn define_specific_var(&mut self, var_index: usize, v: Value){
        self.var_stack[var_index] = v;        
    }
    */
    
    pub fn run(&mut self, code: &[Operation], lines: &[usize], constants: &Vec<Box<dyn ValueTrait>>) -> InterpretResult {
               
        let mut ip = 0;
        loop{          

            if ip >= code.len(){
                break;
            }   
            /*****************************/
            /* Dissassemble when developing */
            /*****************************/
            #[cfg(debug_assertions)]
            {
                // report stack
                print!("  --> Stack: [");
                                            
                for val in self.stack.iter() {                    
                    print!("{}, ", val.to_string());                    
                }
                print!("]\n");

                // Report operation                
                debug::operation(code, lines, ip);                
            }
            /*****************************/
            /*****************************/
            /*****************************/

            match &code[ip] {
                Operation::Return(n) => {   
                    return InterpretResult::Ok(*n);
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
                /*       
                Operation::PushString(v)=>{
                    self.push(Value::StringV( *v ) )
                },        
                Operation::PushFunction(v)=>{
                    self.push(Value::Function( *v ))
                },     
                Operation::PushArray(n)=>{
                    let mut ret = Array::with_capacity(*n);
                    for i in 0..*n {
                        if let Ok(v) = self.pop(){
                            ret[n - 1 -i]=v;
                        }else{
                            return InterpretResult::RuntimeError(format!("Ran out of element in stack when building Array of {} elements",n))
                        }
                    }
                    self.push(Value::Array( Box::new(ret)) )
                },                
                Operation::PushObject(v)=>{
                    self.push(Value::Object( *v ) )
                },                
                Operation::PushGeneric(v)=>{
                    self.push(Value::Generic( *v ))
                },
                
                
                */
                Operation::PushVar(v)=>{
                    // Pushes an object
                    self.push_var(*v);
                },
                Operation::PushVarRef(i)=>{
                    self.push(Value::VarRef(*i))
                }
                
                Operation::EvalVar(i)=>{                                         
                    match self.var_stack[*i] {
                        // Copied.
                        Value::Nil => self.push(Value::Nil),
                        Value::Number(v) => self.push(Value::Number(v)),
                        Value::Bool(v) => self.push(Value::Bool(v)),
                        /*
                        Value::Function(_)=> self.push(Value::VarRef(*i)),
                        Value::Array(v) => {
                            
                            self.push(v.clone_to_value());
                        },
                        Value::StringV(v) => {
                            let s = *v.clone();
                            self.push(Value::StringV(Box::new(s)))
                        },

                        // Referenced
                        Value::Object(_)=> self.push(Value::VarRef(*i)),
                        Value::Generic(_)=> self.push(Value::VarRef(*i)),
                        */
                        Value::HeapRef(i) => self.push(Value::HeapRef(i)),
                        Value::Usize(_)=>panic!("Unexpected behaviour... Trying to Evaluate a usize"),
                        Value::VarRef(_)=>panic!("Unexpected behaviour... trying to evaluate a VarRef")


                    }                                        
                    
                },
                

                Operation::PopVars(n)=>{                    
                    for _ in 0..*n {
                        self.pop_var().unwrap();                    
                    }
                },
                
                Operation::DefineVars(n)=>{
                    for _ in 0..*n{
                        match self.pop(){
                            Ok(v) => {
                                let length = self.stack.len();
                                let dest = length-*n;
                                if let Value::VarRef(i) = self.stack[dest]{
                                    self.var_stack[i] = v;
                                }else{
                                    // ignore returned value
                                }
                            },
                            Err(e)=>return InterpretResult::RuntimeError(e.to_string())
                        }
                    }
                },
                // Unary operations
                Operation::Negate =>{   
                    match self.pop(){
                        Ok(v) => match v.negate(){
                            Ok(v)=>self.push(v),
                            Err(e)=>return InterpretResult::RuntimeError(e)
                        },
                        Err(e)=>return InterpretResult::RuntimeError(format!("{}",e))
                    }                                                                             
                },
                Operation::Not =>{
                    match self.pop(){
                        Ok(v) => match v.not(){
                            Ok(v)=>self.push(v),
                            Err(e)=>return InterpretResult::RuntimeError(e)
                        },
                        Err(e)=>return InterpretResult::RuntimeError(format!("{}",e))
                    }                                                           
                },

                // Binary operations
                Operation::Add => {    
                    let b = self.pop().unwrap();
                    let a = self.pop().unwrap();                    
                    match a.add(&b){
                        Ok(v)=>self.push(v),
                        Err(e)=>return InterpretResult::RuntimeError(e)
                    }  
                },                
                Operation::Subtract => {    
                    let b = self.pop().unwrap();
                    let a = self.pop().unwrap();
                    match a.subtract(&b){
                        Ok(v)=>self.push(v),
                        Err(e)=>return InterpretResult::RuntimeError(e)
                    }               
                },                
                Operation::Multiply => {    
                    let b = self.pop().unwrap();
                    let a = self.pop().unwrap();
                    match a.multiply(&b){
                        Ok(v)=>self.push(v),
                        Err(e)=>return InterpretResult::RuntimeError(e)
                    }           
                },                
                Operation::Divide => {    
                    let b = self.pop().unwrap();
                    let a = self.pop().unwrap();
                    match a.divide(&b){
                        Ok(v)=>self.push(v),
                        Err(e)=>return InterpretResult::RuntimeError(e)
                    }       
                },
                Operation::Equal => {
                    let b = self.pop().unwrap();
                    let a = self.pop().unwrap();
                    match a.compare_equal(&b){
                        Ok(v)=>self.push(v),
                        Err(e)=>return InterpretResult::RuntimeError(e)
                    }       
                                                        
                },

                Operation::NotEqual => {
                    let b = self.pop().unwrap();
                    let a = self.pop().unwrap();
                    match a.compare_not_equal(&b){
                        Ok(v)=>self.push(v),
                        Err(e)=>return InterpretResult::RuntimeError(e)
                    }       
                                                        
                },
                Operation::Greater => {
                    let b = self.pop().unwrap();
                    let a = self.pop().unwrap();
                    match a.greater(&b){
                        Ok(v)=>self.push(v),
                        Err(e)=>return InterpretResult::RuntimeError(e)
                    }   
                },
                Operation::Less => {
                    let b = self.pop().unwrap();
                    let a = self.pop().unwrap();
                    match a.less(&b){
                        Ok(v)=>self.push(v),
                        Err(e)=>return InterpretResult::RuntimeError(e)
                    }                       
                },
                Operation::GreaterEqual => {
                    let b = self.pop().unwrap();
                    let a = self.pop().unwrap();
                    match a.greater_equal(&b){
                        Ok(v)=>self.push(v),
                        Err(e)=>return InterpretResult::RuntimeError(e)
                    }   
                },
                Operation::LessEqual => {
                    let b = self.pop().unwrap();
                    let a = self.pop().unwrap();
                    match a.less_equal(&b){
                        Ok(v)=>self.push(v),
                        Err(e)=>return InterpretResult::RuntimeError(e)
                    }   
                    
                },
                Operation::And =>{
                    let b = self.pop().unwrap();
                    let a = self.pop().unwrap();
                    match a {
                        Value::Bool(v)=>{
                            if !v { // If not A then A and B can't be true
                                self.push(Value::Bool(false))                                
                            }else{
                                // If A, then check B
                                match b {
                                    Value::Bool(v)=>{
                                        if v {
                                            self.push(Value::Bool(true))
                                        }else{
                                            self.push(Value::Bool(false))
                                        }
                                    },
                                    _ =>return InterpretResult::RuntimeError(format!("Cannot use 'and' operator because expression at the right of 'and' is not a Boolean"))
                                }
                            }
                        },
                        _ => return InterpretResult::RuntimeError(format!("Cannot use 'and' operator because expression at the left of 'and' is not a Boolean"))
                    }
                },
                Operation::Or =>{
                    let b = self.pop().unwrap();
                    let a = self.pop().unwrap();
                    match a {
                        Value::Bool(v)=>{
                            if v { // If A then A or B must be true
                                self.push(Value::Bool(true))                                
                            }else{
                                // If not A, then check B
                                match b {
                                    // if B, then it is true
                                    Value::Bool(v)=>{
                                        if v {
                                            self.push(Value::Bool(true))
                                        }else{
                                            self.push(Value::Bool(false))
                                        }
                                    },
                                    _ =>return InterpretResult::RuntimeError(format!("Cannot use 'or' operator because expression at the right of 'or' is not a Boolean"))
                                }
                            }
                        },
                        _ => return InterpretResult::RuntimeError(format!("Cannot use 'or' operator because expression at the left of 'or' is not a Boolean"))
                    }
                },

                Operation::ForLoop(n_vars, body_length)=>{

                    let range = self.pop().unwrap();
                    let mut first_iter = true;
                    // Check number of variables
                    if *n_vars > 2 || *n_vars == 0 {
                        return InterpretResult::RuntimeError(format!("1 or 2 variables should be defined within a For loop: {} were given",n_vars));
                    }
                    // Loop
                    loop {
                        // Get variables
                        let (var1,var2) = match range.get_next(){
                            Some(v)=>{ 
                                first_iter = false; 
                                v
                            },
                            None => {
                                if first_iter {
                                    // If this was the first iteration and returned None, 
                                    // then fail
                                    let e = format!("Cannot iterate type '{}'", range.type_name());
                                    return InterpretResult::RuntimeError(e)
                                }else{
                                    // If it is finished...
                                    break;
                                }
                            }
                        };                                                
                        
                        // Not finished... push variables, these
                        // should be evaluated within the body loop
                        self.push(var1);
                        if *n_vars == 2 {                            
                            self.push(var2);
                        }

                        // Run body... lets do this:
                        let ini = ip;
                        let fin = ip + body_length;                        
                        let sub_code = &code[ini..fin];
                        let sub_lines = &lines[ini..fin];
                        match self.run(sub_code, sub_lines, constants){
                            InterpretResult::Ok(_) => {},
                            InterpretResult::RuntimeError(e) => return InterpretResult::RuntimeError(e),
                            //InterpretResult::CompileError(e) => return InterpretResult::CompileError(e),
                        };                                                
                    }

                    // Skip the whole length of the body
                    ip += body_length;
                },// End of for_loop operation
                Operation::JumpIfFalse(n)=>{
                    let value = self.pop().unwrap();
                    if let Value::Bool(v) = value {
                        if !v {
                            ip += n;
                        }
                    }else{
                        return InterpretResult::RuntimeError(format!("Expression in while loop ( while EXPR {{...}} ) must be a boolean... found a '{}'", value.type_name()));
                    }
                },
                Operation::JumpIfTrue(n)=>{
                    let value = self.pop().unwrap();
                    if let Value::Bool(v) = value {
                        if v {
                            ip += n;
                        }
                    }else{
                        return InterpretResult::RuntimeError(format!("Expression in while loop ( while EXPR {{...}} ) must be a boolean... found a '{}'", value.type_name()));
                    }
                },
                Operation::JumpBack(n)=>{
                    ip -= n;
                },    
                Operation::PushHeapRef(i)=>{
                    self.stack.push(Value::HeapRef(*i))
                },     
                Operation::Call(n)=>{
                    let f_ref = self.stack[ self.stack.len() - n - 1 ];
                    if let Value::HeapRef(i) = f_ref {
                                                
                        // This will push all the returned values; and on top,
                        // the number of returned values 
                        match constants[i].call(self,*n){
                            Ok(ret_n)=>self.push(Value::Usize(ret_n)),
                            Err(e)=>return InterpretResult::RuntimeError(e)
                        }

                    }else{
                        match f_ref.call(self,*n){
                            // This should never be successful because all 
                            // objects that can be called as functions 
                            // are in the heap (thus, HeapRef)
                            Ok(_)=>unreachable!(),
                            Err(e)=>return InterpretResult::RuntimeError(e)
                        }
                    }                    
                }    

            }// end of match
            ip += 1;
        }// end of loop.

        return InterpretResult::RuntimeError("No RETURN operation found".to_string());
        
    }

    pub fn push(&mut self, value: Value ) {        
        self.stack.push(value);        
    }

    fn push_var(&mut self,var: Value){
        self.var_stack.push(var);     
    }

    pub fn pop_var(&mut self)->Result<Value,&str>{        
        if let Some(v)= self.var_stack.pop(){
            Ok(v)
        }else{
            Err("Trying to pop an empty Var-stack")
        }   
    }

    pub fn pop(&mut self)->Result<Value,&str>{
        if let Some(v)= self.stack.pop(){
            Ok(v)
        }else{
            Err("Trying to pop an empty stack")
        }
    }

    
}


/***********/
/* TESTING */
/***********/

#[cfg(test)]
mod tests {
    use super::*;
    use crate::chunk::*;

    impl InterpretResult {
        pub fn is_ok(&self)->bool{
            match self {
                InterpretResult::Ok(_) => true,
                _ => false
            }
        }
    }

    #[test]
    #[should_panic]
    fn test_pop_empty_stack(){
        let mut vm = VM::new();
        vm.pop().unwrap();
    }

    #[test]
    fn test_push_pop(){
        let mut vm = VM::new();
        
        assert_eq!(vm.stack.len(),0);
                
        vm.push(Value::Number(1.2));
        assert_eq!(vm.stack.len(),1);

        match vm.stack[0]{
            Value::Number(v) => {
                assert_eq!(v,1.2);
            },
            _ => {assert!(false)}
        }

        let value = vm.pop().unwrap();
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
        c.write_operation(Operation::Return(0), 0);                        
        let (code,lines)=c.to_slices();

        let mut vm = VM::new();
        assert!(vm.run(code, lines, c.constants()).is_ok()); 

        let v2 = vm.pop().unwrap().get_number().unwrap();
        assert_eq!(v2,-v);
        
            
        
    }

    #[test]
    fn test_not(){
        
        // Over a Float... should not work
        let v = 1.2;
        let mut c = Chunk::new();        
        c.write_operation(Operation::PushNumber(v), 123);                
        c.write_operation(Operation::Not, 124);
        c.write_operation(Operation::Return(0), 0);                        
        let (code,lines)=c.to_slices();

        let mut vm = VM::new();
        assert!(!vm.run(code,lines, c.constants()).is_ok());                
        
            

        // Over a boolean... should work
        let v = true;
        let mut c = Chunk::new();        
        c.write_operation(Operation::PushBool(v), 123);                
        c.write_operation(Operation::Not, 124);
        c.write_operation(Operation::Return(0), 0);                        
        let (code,lines)=c.to_slices();
        
        let mut vm = VM::new();
        assert!(vm.run(code,lines, c.constants()).is_ok());                
        
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

        chunk.write_operation(Operation::Return(0), 0);                        
        let (code,lines)=chunk.to_slices();
        
        let mut vm = VM::new();
        assert!(vm.run(code,lines, chunk.constants()).is_ok());                                

        let c = vm.pop().unwrap().get_number().unwrap();
        assert_eq!(a+b,c);

        
        // Int over something else... should not work
        let a = 11.2;
        let b = true;
        
        let mut chunk = Chunk::new();        
        chunk.write_operation(Operation::PushNumber(a), 123);                        
        chunk.write_operation(Operation::PushBool(b), 123);                        
        chunk.write_operation(Operation::Add, 124);

        chunk.write_operation(Operation::Return(0), 0);                        
        let (code,lines)=chunk.to_slices();
        
        let mut vm = VM::new();
        assert!(!vm.run(code,lines, chunk.constants()).is_ok());                             

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
        chunk.write_operation(Operation::Return(0), 0);                        
        let (code,lines)=chunk.to_slices();
        
        let mut vm = VM::new();
        assert!(vm.run(code,lines, chunk.constants()).is_ok());                              

        let c = vm.pop().unwrap().get_number().unwrap();
        assert_eq!(a-b,c);

        
        
        // Int over something else... should not work
        let a = 12.;
        let b = true;
        
        let mut chunk = Chunk::new();
        
        chunk.write_operation(Operation::PushNumber(a), 123);                        
        chunk.write_operation(Operation::PushBool(b), 123);                        
        chunk.write_operation(Operation::Subtract, 124);

        chunk.write_operation(Operation::Return(0), 0);                        
        let (code,lines)=chunk.to_slices();
        
        let mut vm = VM::new();
        assert!(!vm.run(code,lines, chunk.constants()).is_ok());                             
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

        chunk.write_operation(Operation::Return(0), 0);                        
        let (code,lines)=chunk.to_slices();
        
        let mut vm = VM::new();
        assert!(vm.run(code,lines, chunk.constants()).is_ok());                

        let c = vm.pop().unwrap().get_number().unwrap();
        assert_eq!(a*b,c);

        
        // Int over something else... should not work
        let a = 12.2;
        let b = true;
        
        let mut chunk = Chunk::new();        
        chunk.write_operation(Operation::PushNumber(a), 123);                        
        chunk.write_operation(Operation::PushBool(b), 123);                        
        chunk.write_operation(Operation::Multiply, 124);

        chunk.write_operation(Operation::Return(0), 0);                        
        let (code,lines)=chunk.to_slices();
        
        let mut vm = VM::new();
        assert!(!vm.run(code,lines, chunk.constants()).is_ok());                              

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

        chunk.write_operation(Operation::Return(0), 0);                        
        let (code,lines)=chunk.to_slices();
        
        let mut vm = VM::new();
        assert!(vm.run(code,lines, chunk.constants()).is_ok());                              

        let c = vm.pop().unwrap().get_number().unwrap();
        assert_eq!(a / b,c);

        
        // Int over something else... should not work
        let a = 12.1;
        let b = true;
        
        let mut chunk = Chunk::new();              
        chunk.write_operation(Operation::PushNumber(a), 123);                        
        chunk.write_operation(Operation::PushBool(b), 123);                        
        chunk.write_operation(Operation::Divide, 124);

        chunk.write_operation(Operation::Return(0), 0);                        
        let (code,lines)=chunk.to_slices();
        
        let mut vm = VM::new();
        assert!(!vm.run(code,lines, chunk.constants()).is_ok());                                
    }
}