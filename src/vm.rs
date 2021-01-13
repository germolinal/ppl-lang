
use crate::operations::*;
use crate::values::*;
use crate::value_trait::ValueTrait;
use crate::call_frame::CallFrame;
use crate::function::Function;
use crate::heap_list::HeapList;

#[cfg(debug_assertions)]
use crate::debug::*;

pub enum InterpretResult {
    Ok(usize),    
    RuntimeError(String),
}



pub struct VM {
    call_frames: Vec<CallFrame>,
    stack: Vec<Value>,    
}


impl VM {
    
    pub fn new()-> Self{
                    
        Self {
            //var_stack: Vec::with_capacity(1024),
            call_frames: Vec::with_capacity(64),
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
    
    
    /// Runs the last CallFrame in the call_stack
    pub fn run( &mut self, heap: &mut HeapList ) -> InterpretResult {
                
        
        let mut frame_n = self.call_frames.len() - 1;
        
        loop {  
            // This variable allows some operation to stop
            // advancing through the code ONCE. It is used
            // when calling functions (because we need to start
            // at 0, not 1, which is what would happen if we advanced)        
            let mut advance = true;

            // Get some general data
            let first_call_frame_slot = self.call_frames[frame_n].first_slot();
            let ip = self.call_frames[frame_n].ip();

            if ip >= self.call_frames[frame_n].n_operations().unwrap(){                
                break;
            }   

            /*****************************/
            /* Dissassemble when developing */
            /*****************************/
            #[cfg(debug_assertions)]
            {
                // report stack
                print!("  --> n_frames: {} | Stack: [", frame_n);
                                            
                for val in self.stack.iter() {                    
                    print!("{}, ", val.to_string());                    
                }
                print!("]\n");

                // Report operation 
                let code_lines = self.call_frames[frame_n].code_lines().unwrap();               
                debug::operation(code_lines, ip);                
            }
            /*****************************/
            /*****************************/
            /*****************************/

            let (current_operation, _)=self.call_frames[frame_n].current_instruction_and_line().unwrap();
            
            match current_operation {
                Operation::Return => {   
                                        
                    /* IF THIS SI THE RETURN FROM A FUNCTION */
                    if frame_n > 0 {                        
                        // Get the value
                        let ret_value = self.pop().unwrap();
                                                
                        // Restore stack to what was before this                                         
                        while self.stack.len() > self.call_frames[frame_n].first_slot(){
                            self.pop().unwrap();
                        }                   
                        
                        // (and also the function) itself
                        if self.stack.len() > 0 {
                            self.pop().unwrap();
                        }
    
                        // Go back one call_frame
                        self.pop_call_frame().unwrap();                        
                        frame_n -= 1;
                        
    
                        self.push(ret_value);
                    }else{
                        /* OTHERWISE, RETURN FROM THE PROGRAM */
                        return InterpretResult::Ok(1);
                    }
                    
                },                
                Operation::PushBool(v)=>{
                    self.push(Value::Bool(v))
                },                
                Operation::PushNumber(v)=>{
                    self.push(Value::Number(v))
                },      
                Operation::PushNil=>{
                    self.push(Value::Nil)
                }                                   
                Operation::GetLocal(i)=>{  
                    println!("first slot: {}", first_call_frame_slot);
                    let absolute_position = i + first_call_frame_slot;
                    let local = self.stack[absolute_position];
                    // Check if it has been initialized
                    if local.is_nil() {
                        panic!(format!("Variable '{}' has not been initialized", "TODO: Add &source in GetLocal operation" ));
                    }
                    // Push it    
                    self.push(self.stack[absolute_position]);                                                                                                      
                },
                Operation::SetLocal(i)=>{      
                    let absolute_position = i + first_call_frame_slot;
                    let last = self.stack.len()-1;
                    self.stack[absolute_position] = self.stack[last];//self.pop().unwrap();                                                                                                                 
                },                
                Operation::Pop(n)=>{                    
                    for _ in 0..n {
                        self.pop().unwrap();
                    }
                },
                
                Operation::DefineVars(n)=>{
                    for _ in 0..n{
                        match self.pop(){
                            Ok(_v) => {
                                unimplemented!();
                                /*
                                let length = self.stack.len();
                                let dest = length - *n;
                                if let Value::VarRef(i) = self.stack[dest]{                                    
                                }else{
                                    // ignore returned value
                                }
                                */
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

                Operation::ForLoop(_n_vars, _body_length)=>{
                    unimplemented!();
                    /*                
                    let range = self.pop().unwrap();
                    let mut first_iter = true;
                    // Check number of variables
                    if n_vars > 2 || n_vars == 0 {
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
                        if n_vars == 2 {                            
                            self.push(var2);
                        }

                        // Run body... lets do this:
                        let _ini = ip;
                        let _fin = ip + body_length;   
                             
                        let sub_code = &code[ini..fin];
                        let sub_lines = &lines[ini..fin];
                        match self.run(sub_code, sub_lines, constants){
                            InterpretResult::Ok(_) => {},
                            InterpretResult::RuntimeError(e) => return InterpretResult::RuntimeError(e),
                            //InterpretResult::CompileError(e) => return InterpretResult::CompileError(e),
                        };    
                    }
                    
                    // Skip the whole length of the body
                    self.call_frames[frame_n].jump_forward(body_length);
                    */                                            

                },// End of for_loop operation
                Operation::JumpIfFalse(n)=>{                    

                    let value = self.pop().unwrap();
                    if let Value::Bool(v) = value {
                        if !v {                            
                            self.call_frames[frame_n].jump_forward(n);
                        }
                    }else{
                        return InterpretResult::RuntimeError(format!("Expression in while loop ( while EXPR {{...}} ) must be a boolean... found a '{}'", value.type_name()));
                    }
                },
                Operation::JumpIfTrue(n)=>{
                    let value = self.pop().unwrap();
                    if let Value::Bool(v) = value {
                        if v {                            
                            self.call_frames[frame_n].jump_forward(n);
                        }
                    }else{
                        return InterpretResult::RuntimeError(format!("Expression in while loop ( while EXPR {{...}} ) must be a boolean... found a '{}'", value.type_name()));
                    }
                },
                Operation::JumpBack(n)=>{                    
                    self.call_frames[frame_n].jump_backwards(n);
                },                    
                Operation::PushHeapRef(i)=>{
                    self.stack.push(Value::HeapRef(i))
                },                 
                Operation::Call(n_vars)=>{
                    let f_ref = self.stack[ self.stack.len() - n_vars - 1 ];
                    if let Value::HeapRef(i) = f_ref {
                                         
                        // get the function from the surrounding function (i.e. the current one)                        
                        println!("FUNCTION {}", i);                        
                        
                        let function = match heap.get(i).unwrap()
                            .as_any()
                            .downcast_ref::<Function>(){
                                Some(f)=>f.clone_rc(),
                                None => return InterpretResult::RuntimeError(format!("Trying to call from a '{}' object as if it was a function", heap.get(i).unwrap().type_name()))
                            };
                        
                            let f_name = function.get_name();
                        
                            debug::chunk(function.chunk().unwrap(), format!("{}",f_name));
                            

                        match function.call(self,n_vars){
                            Ok(_n_returns)=>{
                                // Add the function to the stack, and continue 
                                // in business as usual.                       
                                let first_slot = self.stack.len() - n_vars;                  
                                self.push_call_frame(CallFrame::new(first_slot,function));                        
                                frame_n += 1;
                                advance = false;
                                //self.run();
                                //self.pop_call_frame().unwrap();
                            },
                            Err(e)=>panic!(e)
                        }
                        
                        
                        //unimplemented!();
                        /*
                        // This will push all the returned values; and on top,
                        // the number of returned values 
                        match constants[i].call(self,*n){
                            Ok(ret_n)=>self.push(Value::Usize(ret_n)),
                            Err(e)=>return InterpretResult::RuntimeError(e)
                        }
                        */

                    }else{
                        // THis is an error... this is here just to send a 
                        // better error message
                        match f_ref.call(self,n_vars){
                            // This should never be successful because all 
                            // objects that can be called as functions 
                            // are in the heap (thus, HeapRef)
                            Ok(_)=>unreachable!(),
                            Err(e)=>return InterpretResult::RuntimeError(e)
                        }
                    }                    
                }    

            }// end of match

            // Advance one space
            if advance {
                self.call_frames[frame_n].jump_forward(1);
            }

        }// end of loop.

        let current_function = self.call_frames[frame_n].function();
        let f_name = current_function.get_name();

        return InterpretResult::RuntimeError(format!("No RETURN operation found in function '{}'", f_name));
        
    }

    pub fn push(&mut self, value: Value ) {        
        self.stack.push(value);        
    }

    pub fn push_call_frame(&mut self, call_frame: CallFrame ) {        
        self.call_frames.push(call_frame);        
    }

    pub fn pop_call_frame(&mut self) ->Result<CallFrame, &str> {        
        if let Some(v) = self.call_frames.pop(){
            Ok(v)
        }else{
            Err("Trying to pop an empty CallFrame stack")
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

    

    use crate::function::Function;
    #[test]
    fn test_negate(){
        
        let v = 1.2;        
        let mut function = Function::new_script("test_negate".as_bytes());
        {
            let c = function.mut_chunk().unwrap();
            
            // Over a number... should work
            c.push((Operation::PushNumber(v), 123));                
            c.push((Operation::Negate, 124));
            c.push((Operation::Return, 0));                        
        }
        
        let mut vm = VM::new();
        let mut heap = HeapList::new();
        vm.push_call_frame(CallFrame::new(0, function.clone_rc() ));
        
        //let c = function.chunk().unwrap();
        assert!(vm.run(&mut heap).is_ok()); 

        let v2 = vm.pop().unwrap().get_number().unwrap();
        assert_eq!(v2,-v);
        
            
        
    }

    #[test]
    fn test_not(){
        
        // Over a Float... should not work
        let v = 1.2;        
        let mut function = Function::new_script("test_not".as_bytes());
        {
            let c = function.mut_chunk().unwrap();
    
            c.push((Operation::PushNumber(v), 123));                
            c.push((Operation::Not, 124));
            c.push((Operation::Return, 0));                        

        }
                
        let mut vm = VM::new();
        let mut heap = HeapList::new();
        vm.push_call_frame(CallFrame::new(0,function.clone_rc()));
        assert!(!vm.run(&mut heap).is_ok());
        
                    

        // Over a boolean... should work
        let v = true;
        let mut function = Function::new_script("test_not".as_bytes());
        {
            let c = function.mut_chunk().unwrap();
            c.push((Operation::PushBool(v), 123));                
            c.push((Operation::Not, 124));
            c.push((Operation::Return, 0));                        
    
        }        
        
        let mut vm = VM::new();
        let mut heap = HeapList::new();
        vm.push_call_frame(CallFrame::new(0,function.clone_rc()));
        assert!(vm.run(&mut heap).is_ok());
                        
        
    }


    #[test]
    fn test_add(){
        
        // Float with Float... should work
        let a = 1.2;
        let b = 12.21231;
        
        let mut function = Function::new_script("test_add".as_bytes());
        {
            let chunk = function.mut_chunk().unwrap();
                    
            chunk.push((Operation::PushNumber(a), 123));                        
            chunk.push((Operation::PushNumber(b), 123));                        
            chunk.push((Operation::Add, 124));
            chunk.push((Operation::Return, 0));                        
        }
        
        
        let mut vm = VM::new();
        let mut heap = HeapList::new();
        vm.push_call_frame(CallFrame::new(0,function.clone_rc()));
        assert!(vm.run(&mut heap).is_ok());                                

        let c = vm.pop().unwrap().get_number().unwrap();
        assert_eq!(a+b,c);

        
        // Int over something else... should not work
        let a = 11.2;
        let b = true;
        
        let mut function = Function::new_script("test_add".as_bytes());
        {
            let chunk = function.mut_chunk().unwrap();
            
            chunk.push((Operation::PushNumber(a), 123));                        
            chunk.push((Operation::PushBool(b), 123));                        
            chunk.push((Operation::Add, 124));
            chunk.push((Operation::Return, 0));                                        
        }
        
        let mut vm = VM::new();
        let mut heap = HeapList::new();
        vm.push_call_frame(CallFrame::new(0,function.clone_rc()));
        assert!(!vm.run(&mut heap).is_ok());                             

    }

    #[test]
    fn test_subtract(){
        
        // Float with Float... should work
        let a = 1.2;
        let b = 12.21231;
        
        let mut function = Function::new_script("test_subtract".as_bytes());
        {
            let chunk = function.mut_chunk().unwrap();
                    
            chunk.push((Operation::PushNumber(a), 123));                        
            chunk.push((Operation::PushNumber(b), 123));                        
            chunk.push((Operation::Subtract, 124));
            chunk.push((Operation::Return, 0));                                
        }
        
        let mut vm = VM::new();
        let mut heap = HeapList::new();
        vm.push_call_frame(CallFrame::new(0,function.clone_rc()));
        assert!(vm.run(&mut heap).is_ok());                              

        let c = vm.pop().unwrap().get_number().unwrap();
        assert_eq!(a-b,c);

        
        
        // Int over something else... should not work
        let a = 12.;
        let b = true;

        let mut function = Function::new_script("test_subtract".as_bytes());
        {
            let chunk = function.mut_chunk().unwrap();
            
            
            chunk.push((Operation::PushNumber(a), 123));                        
            chunk.push((Operation::PushBool(b), 123));                        
            chunk.push((Operation::Subtract, 124));
            chunk.push((Operation::Return, 0));                        

        }
        
        
        
        let mut vm = VM::new();
        let mut heap = HeapList::new();
        vm.push_call_frame(CallFrame::new(0,function.clone_rc()));
        assert!(!vm.run(&mut heap).is_ok());                             
    }

    #[test]
    fn test_multiply(){
        
        // Float with Float... should work
        let a = 1.2;
        let b = 12.21231;
        
        let mut function = Function::new_script("test_multiply".as_bytes());
        {

            let chunk = function.mut_chunk().unwrap();
            
            chunk.push((Operation::PushNumber(a), 123));                        
            chunk.push((Operation::PushNumber(b), 123));                        
            chunk.push((Operation::Multiply, 124));
            chunk.push((Operation::Return, 0));                                        
        }
        

        let mut vm = VM::new();
        let mut heap = HeapList::new();
        vm.push_call_frame(CallFrame::new(0,function.clone_rc()));

        assert!(vm.run(&mut heap).is_ok());                

        let c = vm.pop().unwrap().get_number().unwrap();
        assert_eq!(a*b,c);

        
        // Int over something else... should not work
        let a = 12.2;
        let b = true;
        
        let mut function = Function::new_script("test_multiply".as_bytes());
        {

            let chunk = function.mut_chunk().unwrap();
            
            chunk.push((Operation::PushNumber(a), 123));                        
            chunk.push((Operation::PushBool(b), 123));                        
            chunk.push((Operation::Multiply, 124));
            chunk.push((Operation::Return, 0));                        
        }
        
        let mut vm = VM::new();
        let mut heap = HeapList::new();
        vm.push_call_frame(CallFrame::new(0,function.clone_rc()));                
        assert!(!vm.run(&mut heap).is_ok());                              

    }

    #[test]
    fn test_divide(){
        
        // Float with Float... should work
        let a = 1.2;
        let b = 12.21231;
        
        let mut function = Function::new_script("test_divide".as_bytes());
        {

            let chunk = function.mut_chunk().unwrap();
            
            chunk.push((Operation::PushNumber(a), 123));                        
            chunk.push((Operation::PushNumber(b), 123));                        
            chunk.push((Operation::Divide, 124));            
            chunk.push((Operation::Return, 0));                        
        }
        
        let mut vm = VM::new();
        let mut heap = HeapList::new();
        vm.push_call_frame(CallFrame::new(0,function.clone_rc()));
        assert!(vm.run(&mut heap).is_ok());                              

        let c = vm.pop().unwrap().get_number().unwrap();
        assert_eq!(a / b,c);

        
        // Int over something else... should not work
        let a = 12.1;
        let b = true;
        
        let mut function = Function::new_script("test_divide".as_bytes());
        {
            let chunk = function.mut_chunk().unwrap();
            
            chunk.push((Operation::PushNumber(a), 123));                        
            chunk.push((Operation::PushBool(b), 123));                        
            chunk.push((Operation::Divide, 124));
            chunk.push((Operation::Return, 0));                        

        }
        
        let mut vm = VM::new();
        let mut heap = HeapList::new();
        vm.push_call_frame(CallFrame::new(0,function.clone_rc()));
        assert!(!vm.run(&mut heap).is_ok());                                
    }
}