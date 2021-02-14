#[cfg(debug_assertions)]
use std::env;

use crate::handler::PPLHandler;
use crate::operations::*;
use crate::values::*;
use crate::value_trait::ValueTrait;
use crate::call_frame::CallFrame;
use crate::function::Function;
use crate::stack::Stack;

#[cfg(debug_assertions)]
use crate::debug;

pub enum InterpretResult {
    Ok(usize),    
    RuntimeError(String),
}



pub struct VM {
    call_frames: Stack<CallFrame>,
    stack: Stack<Value>,    
    handler: PPLHandler
}


impl VM {
    
    pub fn new( handler : PPLHandler)-> Self {
                    
        Self {            
            handler,
            call_frames: Stack::new(),
            stack: Stack::new(),
        }

    }    

    
    
    fn for_loop(&mut self)->Result<(),String>{
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
    }

    //#[inline]
    fn jump_if_false(&mut self, n: u8, frame_n: &u8)->Result<(),String>{
        if let Value::Bool(v) = self.stack.last() {
            if !(*v) {                            
                self.call_frames[*frame_n].jump_forward(n as usize);
            }
            Ok(())
        }else{
            let value = self.pop().unwrap();
            Err(format!("Expression in 'if' statement (i.e., if EXPR {{...}} ) must be a boolean... found a '{}'", value.type_name()))
        }
    }

    //#[inline]
    fn jump_if_true(&mut self, n: u8, frame_n: &u8)->Result<(),String>{
        if let Value::Bool(v) = self.stack.last() {
            if *v {                            
                self.call_frames[*frame_n].jump_forward(n as usize);
            }
            Ok(())
        }else{
            let value = self.pop().unwrap();
            Err(format!("Expression in 'if' statement (i.e., if EXPR {{...}} ) must be a boolean... found a '{}'", value.type_name()))
        }
    }

    //#[inline]
    fn drop_n(&mut self, n: u8)->Result<(),String>{                    
        self.stack.drop_n(n)        
    }

    

    //#[inline]
    fn negate(&mut self)->Result<(),String>{
        match self.pop(){
            Ok(v) => match v.negate(){
                Ok(v)=>{
                    self.push(v);
                    Ok(())
                },
                Err(e)=> Err(e)
            },
            Err(e)=> Err(e.to_string())
        }   
    }

    //#[inline]
    fn not(&mut self)->Result<(),String>{
        match self.pop(){
            Ok(v) => match v.not(){
                Ok(v)=>{
                    self.push(v);
                    Ok(())
                },
                Err(e)=> Err(e)
            },
            Err(e)=> Err(e.to_string())
        }  
    }

    //#[inline]
    fn add(&mut self)->Result<(),String>{
        let b = self.pop().unwrap();
        let a = self.pop().unwrap();                    
        match a.add(&b){
            Ok(v)=>{
                self.push(v);
                Ok(())
            },
            Err(e)=>Err(e)
        }  
    }

    //#[inline]
    fn subtract(&mut self)->Result<(),String>{
        let b = self.pop().unwrap();
        let a = self.pop().unwrap();
        match a.subtract(&b){
            Ok(v)=>{
                self.push(v);
                Ok(())
            },
            Err(e)=> Err(e)
        }      
    }

    //#[inline]
    fn multiply(&mut self)->Result<(),String>{
        let b = self.pop().unwrap();
        let a = self.pop().unwrap();
        match a.multiply(&b){
            Ok(v)=>{
                self.push(v);
                Ok(())
            },
            Err(e)=> Err(e)
        }   
    }

    //#[inline]
    fn divide(&mut self)->Result<(),String>{
        let b = self.pop().unwrap();
        let a = self.pop().unwrap();
        match a.divide(&b){
            Ok(v)=>{
                self.push(v);
                Ok(())
            },
            Err(e)=> Err(e)
        }       
    }


    //#[inline]
    fn equal(&mut self)->Result<(),String>{
        let b = self.pop().unwrap();
        let a = self.pop().unwrap();
        match a.compare_equal(&b){
            Ok(v)=>{
                self.push(v);
                Ok(())
            },
            Err(e)=> Err(e)
        }      
    }

    //#[inline]
    fn not_equal(&mut self)->Result<(),String>{
        let b = self.pop().unwrap();
        let a = self.pop().unwrap();
        match a.compare_not_equal(&b){
            Ok(v)=>{
                self.push(v);
                Ok(())
            },
            Err(e)=> Err(e)
        }       
    }

    //#[inline]
    fn greater(&mut self)->Result<(),String>{
        let b = self.pop().unwrap();
        let a = self.pop().unwrap();
        match a.greater(&b){
            Ok(v)=>{
                self.push(v);
                Ok(())
            },
            Err(e)=> Err(e)
        }   
    }

    //#[inline]
    fn less(&mut self)->Result<(),String>{
        let b = self.pop().unwrap();
        let a = self.pop().unwrap();
        match a.less(&b){
            Ok(v)=>{
                self.push(v);
                Ok(())
            },
            Err(e)=> Err(e)
        }    
    }

    //#[inline]
    fn greater_equal(&mut self)->Result<(),String>{
        let b = self.pop().unwrap();
        let a = self.pop().unwrap();
        match a.greater_equal(&b){
            Ok(v)=>{
                self.push(v);
                Ok(())
            },
            Err(e)=> Err(e)
        }
    }

    //#[inline]
    fn less_equal(&mut self)->Result<(),String>{
        let b = self.pop().unwrap();
        let a = self.pop().unwrap();
        match a.less_equal(&b){
            Ok(v)=>{
                self.push(v);
                Ok(())
            },
            Err(e)=> Err(e)
        }   
    }

    
    fn and(&mut self)->Result<(),String>{
        let b = self.pop().unwrap();
        let a = self.pop().unwrap();
        match a {
            Value::Bool(v)=>{
                if !v { // If not A then A and B can't be true
                    self.push(Value::Bool(false));
                    Ok(())
                }else{
                    // If A, then check B
                    match b {
                        Value::Bool(v)=>{
                            if v {
                                self.push(Value::Bool(true));                                
                            }else{
                                self.push(Value::Bool(false));                                
                            }
                            Ok(())
                        },
                        _ => Err( "Cannot use 'and' operator because expression at the right of 'and' is not a Boolean".to_string() )
                    }
                }
            },
            _ => Err( "Cannot use 'and' operator because expression at the left of 'and' is not a Boolean".to_string() )
        }
    }

    
    fn or(&mut self)->Result<(),String>{
        let b = self.pop().unwrap();
        let a = self.pop().unwrap();
        match a {
            Value::Bool(v)=>{
                if v { // If A then A or B must be true
                    self.push(Value::Bool(true));
                    Ok(())
                }else{
                    // If not A, then check B
                    match b {
                        // if B, then it is true
                        Value::Bool(v)=>{
                            if v {
                                self.push(Value::Bool(true));                                
                            }else{
                                self.push(Value::Bool(false));
                            }
                            Ok(())
                        },
                        _ =>Err( "Cannot use 'or' operator because expression at the right of 'or' is not a Boolean".to_string() )
                    }
                }
            },
            _ => Err( "Cannot use 'or' operator because expression at the left of 'or' is not a Boolean".to_string() )
        }
    }


    

    /// Gets a local variable
    fn get_local(&mut self, absolute_position: u8)->Result<(),String>{
        let local = self.stack[absolute_position].clone();
        
        
        // Let the HEAP know that we are referencing this
        if let Value::HeapRef(i) = local {
            self.handler.heap.add_reference(i);
        }

        // Push it    
        self.push(local.clone());  
        Ok(())   
    }

    
    /// Sets local variable
    fn set_local(&mut self, absolute_position: u8)->Result<(),String>{
        let last = self.stack.len()-1;
                
        // If the value that will be replaced pointed to 
        // the heap, let the heap know
        if let Value::HeapRef(heap_ref) = self.stack[absolute_position] {
            self.handler.heap.drop_reference(heap_ref);
        }

        // Check if the value to be assigned is a Function...
        // we don't allow that.                    
        if let Value::HeapRef(heap_ref) = self.stack[last]{
            if self.handler.heap.get(heap_ref).unwrap().is_function(){
                return Err("Cannot assign a function into a variable".to_string());
            }
        }

        // Replace
        self.stack[absolute_position] = self.stack[last].clone();
        Ok(())
    }

    /// Gets a global variable
    //#[inline]
    fn get_global(&mut self, i: u8)->Result<(),String>{
        if !self.handler.heap.get(i).unwrap().is_function(){
            return Err( "Trying to get a reference to a non-function global variable".to_string() )
        }
        self.handler.heap.add_reference(i);
        self.push(Value::HeapRef(i));
        Ok(())
    }

    /// Gets a value from package
    //#[inline]
    fn get_from_package(&mut self, i: usize)->Result<(),String>{
        self.push(Value::PackageRef(i));
        Ok(())
    }

    /// Calls a Script Function
    fn call_script(&mut self, function: Function, n_vars: u8)-> Result<(),String> {
        
        // get the function from the surrounding function (i.e. the current one)                                                                                
        match function.call(self, n_vars){
            Ok(_n_returns)=>{
                // Add the function to the stack, and continue 
                // in business as usual.                       
                let first_slot = self.stack.len() as u8 - n_vars;                  
                self.push_call_frame(CallFrame::new(first_slot,function));                        
                                
                Ok(())
            },
            Err(e)=>Err(e)
        }
    }

    

    /// Calls a function
    fn call(&mut self, n_args: u8, frame_n: &mut u8, advance: &mut bool)->Result<(),String>{
        
        let f_ref = self.stack[ self.stack.len() as u8 - n_args - 1 ].clone();

        let function = match f_ref {
            Value::HeapRef(i) => {
                match self.handler.heap.get(i).unwrap()
                    .as_any()
                    .downcast_ref::<Function>(){
                        Some(f)=>f.clone_rc(),
                        None => return Err(format!("Trying to call from a '{}' object as if it was a function", self.handler.heap.get(i).unwrap().type_name()))
                    }                                                
            },
            Value::PackageRef(i) => {
                                
                // get the function from the surrounding function (i.e. the current one)                                                                        
                //let function = 
                self.handler.packages_elements[i as usize].clone_rc()
                                                                                            
            },
            _ => {
                // THis is an error... this is here just to send a 
                // better error message
                match f_ref.call(self, n_args){
                    // This should never be successful because all 
                    // objects that can be called as functions 
                    // are in the heap (thus, HeapRef)
                    Ok(_)=>unreachable!(),
                    Err(e)=>return Err(e)
                }
            }
        };// end of retrieve the function

        if function.is_native(){
            
            let stack_before = self.stack.len();

            // Call the funciton
            match function.call(self, n_args){
                Ok(n_returns)=>{
                    
                    
                    // CHECK INTEGRITY OF THE STACK
                    // it should have grown by n_return elements 
                    // (pushed as results), and reduced by n_args
                    // (popped when using the arguments)
                    debug_assert_eq!(self.stack.len(), stack_before + n_returns - n_args);

                    // At this stage, the stack should be
                    // [..., NativeFn<>, return_value] if something was returned, or
                    // simply [..., NativeFn<>] if nothing was returned
                    
                    // Get the returned value (or nil, if there is no return)
                    let ret : Value;
                    if n_returns == 0 {
                        ret = Value::Nil;
                    }else if n_returns == 1{
                        ret = self.pop().unwrap();
                    }else{                                        
                        panic!("Function '{}' returns more than one argument... this is a bug in that function.", function.get_name())
                    }                        
                                                        
                    // Pop the function itself... should not need
                    // for dropping references
                    self.stack.drop_last().unwrap();
    
                    // Push result
                    self.push(ret);
                                        
                },
                Err(e)=>return Err(e)
            }
            


        }else{
            match self.call_script(function, n_args){
                Ok(_)=>{
                    *frame_n += 1;
                    *advance = false;
                },
                Err(e)=>return Err(e)
            }
        }

        Ok(())
    }



    /// Return operation
    fn return_op(&mut self, frame_n: &mut u8)->Result<(),String>{
        
                            
        // Get the value
        let ret_value = match self.pop(){
            Ok(v)=>v,
            Err(e)=>return Err(e.to_string()),
        };
                
        // Whatever was added + the name of the function
        let n_drops = 1 + self.stack.len() - self.call_frames[*frame_n].first_slot();
        if let Err(msg) = self.stack.drop_n(n_drops){
            return Err(msg);
        }
        
        // Go back one call_frame
        if let Err(msg) = self.drop_call_frame(){
            return Err( msg );
        }          

        *frame_n -= 1;
        
        self.push(ret_value);

        Ok(())                                
        
    }

    /// Grabs an operation and performs the appropriate action
    fn perform_operation(&mut self, /*current_operation: Operation, handler: &mut PPLHandler,*/ frame_n: &mut u8, first_call_frame_slot: u8, advance: &mut bool)->Result<(),String>{
        let current_operation =self.call_frames[*frame_n].current_instruction().unwrap();            

        match current_operation {
            Operation::Return => {                                       
                unreachable!();                
            },                
            Operation::PushBool(v)=>{
                self.push(Value::Bool(v)); 
                Ok(())               
            },                
            Operation::PushNumber(v)=>{
                self.push(Value::Number(v));    
                Ok(())            
            },      
            Operation::PushNil=>{
                self.push(Value::Nil);     
                Ok(())           
            }                                   
            Operation::GetLocal(i)=>{                      
                let absolute_position = i  + first_call_frame_slot;
                self.get_local(absolute_position)                                                                                              
            },
            Operation::SetLocal(i)=>{      
                let absolute_position = i + first_call_frame_slot;
                self.set_local(absolute_position)
            },    
            Operation::GetGlobal(i)=>{                    
                self.get_global(i)
            },         
            Operation::GetFromPackage(i)=>{                                        
                self.get_from_package(i)
            },         
            Operation::Pop(n)=>{                    
                self.drop_n(n)
            },
                        
            // Unary operations
            Operation::Negate =>{   
                self.negate()                                                                         
            },
            Operation::Not =>{
                self.not()                                                      
            },
            // Binary operations
            Operation::Add => {    
                self.add()
            },                
            Operation::Subtract => {    
                self.subtract()        
            },                
            Operation::Multiply => {    
                self.multiply()      
            },                
            Operation::Divide => {    
                self.divide()
            },
            Operation::Equal => {
                self.equal()                                                     
            },

            Operation::NotEqual => {
                self.not_equal()                                                   
            },
            Operation::Greater => {
                self.greater()
            },
            Operation::Less => {
                self.less()                  
            },
            Operation::GreaterEqual => {
                self.greater_equal()  
            },
            Operation::LessEqual => {
                self.less_equal()                
            },
            Operation::And =>{
                self.and()
            },
            Operation::Or =>{
                self.or()
            },

            Operation::ForLoop(_n_vars, _body_length)=>{
                self.for_loop()                
            },
            Operation::JumpIfFalse(n)=>{                                
                self.jump_if_false(n, frame_n)
            },
            Operation::JumpIfTrue(n)=>{                    
                self.jump_if_true(n, frame_n)
            },
            Operation::JumpBack(n)=>{                    
                self.call_frames[*frame_n].jump_backwards(n as usize);
                Ok(())
            },                    
            Operation::PushHeapRef(i)=>{
                self.stack.push(Value::HeapRef(i)).unwrap();
                Ok(())
            },                 
            Operation::Call(n_vars)=>{
                                    
                self.call(n_vars, frame_n, advance)

            }// end of Operation::Call    

            /* ****** */
        }// end of match OPERATION {}
                
    }// end of match_operation function




    /// Runs the last CallFrame in the call_stack
    pub fn run( &mut self) -> InterpretResult {
                        
        let mut frame_n = self.call_frames.len() - 1;
        
        loop {  
            
            // This variable allows some operation to stop
            // advancing through the code ONCE. It is used
            // when calling functions (because we need to start
            // at 0, not 1, which is what would happen if we advanced)        
            let mut advance = true;

            // Get some general data            
            let first_call_frame_slot = self.call_frames[frame_n].first_slot();
            let ip = self.call_frames[frame_n].ip_index();
            
            if ip >= self.call_frames[frame_n].n_operations().unwrap(){                                
                break;
            }   

            /********************************/
            /* Dissassemble when developing */
            /********************************/
            
            #[cfg(debug_assertions)]
            if env::var("TRACE_PPL_EXECUTION").is_ok(){
                
                // report stack_frame depth
                eprintln!("  --> n_frames: {}", frame_n);

                // Print the stack
                eprint!(" | Stack: [");                                            
                for val in 0..self.stack.len() {                    
                    let v = self.stack[val].clone();
                    eprint!("{}, ", v.to_string());                    
                }
                eprintln!("]");

                // Print the heap
                eprint!(" | Heap: [");                                            
                for hi in 0..self.handler.heap.len(){
                    if let Some(v) = self.handler.heap.get(hi){
                        eprint!("({},{}); ", hi, v.to_string())
                    }
                }
                eprintln!("]\n");

                // Report operation                 
                let code_lines = self.call_frames[frame_n].code_lines().unwrap();               
                debug::operation(code_lines, ip);                
                
            
            }
            
            
            /*****************************/
            /*****************************/
            /*****************************/

            let current_operation = self.call_frames[frame_n].current_instruction().unwrap();            
            
            if let Operation::Return = current_operation {
                /* IF THIS SI THE RETURN FROM A FUNCTION */
                if frame_n > 0 {                        
                    match self.return_op(&mut frame_n){
                        Ok(_)=>{},
                        Err(e)=>return InterpretResult::RuntimeError(e)
                    }
                }else{
                    /* OTHERWISE, RETURN FROM THE PROGRAM */
                    return InterpretResult::Ok(1);
                }
            }else{                

                match self.perform_operation(/*current_operation, &mut self.handler,*/ &mut frame_n, first_call_frame_slot, &mut advance){
                    Ok(_)=>{},
                    Err(e)=>{return InterpretResult::RuntimeError(e)}
                }
            }


            // Advance one space
            if advance {
                self.call_frames[frame_n].jump_forward(1);
            }

        }// end of loop.

        
        let current_function = self.call_frames[frame_n].function();
        let f_name = current_function.get_name();

        InterpretResult::RuntimeError(format!("No RETURN operation found in function '{}' (this is a bug, not a user error)", f_name))
        
    }
    
    pub fn push(&mut self, value: Value ) {                
        match self.stack.push(value){
            Ok(_)=>{},
            Err(e)=>panic!(format!("Stack: {}", e))
        }
    }

    pub fn push_call_frame(&mut self, call_frame: CallFrame ) {        
       
        match self.call_frames.push(call_frame){
            Ok(_)=>{},
            Err(e)=>panic!(format!("CallFrame: {} ", e))
        };        
    }

    pub fn drop_call_frame(&mut self) ->Result<(), String> {        
        
        if let Err(msg) = self.call_frames.drop_last(){
            return Err(format!("StackFrame: {}", msg));
        }
        Ok(())
    }

       
    /// Pops the stack. Returns an error if empty
    pub fn pop(&mut self)->Result<Value,&str>{
        if let Some(v) = self.stack.pop(){
            Ok(v)
        }else{
            Err("Trying to pop an empty stack")
        }
    }

    /// Fetches a Heap reference, returning a mutable reference to ut
    pub fn resolve_heap_reference(&mut self, v: Value) -> Result<&mut Box<dyn ValueTrait>, &str> {
        
        if let Value::HeapRef(i)= v {
            let v = self.handler.heap.get_mut(i);
            match v {
                None => Err("Reference points to NONE"),
                Some(ret)=>Ok(ret)
            }
        } else {
            Err("resolve_heap_reference() requires a Value::HeapRef() as an argument")
        }
    }

    /// Fetches a package references, returning a reference to it
    pub fn resolve_package_reference(&self, v: Value) -> Result<&Function, &str> {
        
        if let Value::PackageRef(i)= v {
            Ok(&self.handler.packages_elements[i])
        } else {
            Err("resolve_package_reference() requires a Value::PackageRef() as an argument")
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
        let handler = PPLHandler::new();
        let mut vm = VM::new(handler);
        vm.pop().unwrap();
    }

    #[test]
    fn test_push_pop(){
        let handler = PPLHandler::new();
        let mut vm = VM::new(handler);
        
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
        
        let handler = PPLHandler::new();
        let mut vm = VM::new(handler);
        
        vm.push_call_frame(CallFrame::new(0, function.clone_rc() ));

        assert!(vm.run().is_ok()); 

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
                
        let handler = PPLHandler::new();
        let mut vm = VM::new(handler);

        vm.push_call_frame(CallFrame::new(0,function.clone_rc()));
        assert!(!vm.run().is_ok());
        
                    

        // Over a boolean... should work
        let v = true;
        let mut function = Function::new_script("test_not".as_bytes());
        {
            let c = function.mut_chunk().unwrap();
            c.push((Operation::PushBool(v), 123));                
            c.push((Operation::Not, 124));
            c.push((Operation::Return, 0));                        
    
        }        
        
        let handler = PPLHandler::new();
        let mut vm = VM::new(handler);        
        
        vm.push_call_frame(CallFrame::new(0,function.clone_rc()));
        assert!(vm.run().is_ok());
                        
        
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
        
        
        let handler = PPLHandler::new();
        let mut vm = VM::new(handler);

        vm.push_call_frame(CallFrame::new(0,function.clone_rc()));
        assert!(vm.run().is_ok());                                

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
        
        let handler = PPLHandler::new();
        let mut vm = VM::new(handler);

        vm.push_call_frame(CallFrame::new(0,function.clone_rc()));
        assert!(!vm.run().is_ok());                             

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
        
        let handler = PPLHandler::new();
        let mut vm = VM::new(handler);

        vm.push_call_frame(CallFrame::new(0,function.clone_rc()));
        assert!(vm.run().is_ok());                              

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
        
        
        
        let handler = PPLHandler::new();
        let mut vm = VM::new(handler);

        vm.push_call_frame(CallFrame::new(0,function.clone_rc()));
        assert!(!vm.run().is_ok());                             
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
        

        let handler = PPLHandler::new();
        let mut vm = VM::new(handler);
        
        vm.push_call_frame(CallFrame::new(0,function.clone_rc()));
        assert!(vm.run().is_ok());                

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
        
        let handler = PPLHandler::new();
        let mut vm = VM::new(handler);
        
        vm.push_call_frame(CallFrame::new(0,function.clone_rc()));                
        assert!(!vm.run().is_ok());                              

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
        
        
        let handler = PPLHandler::new();
        let mut vm = VM::new(handler);
        
        vm.push_call_frame(CallFrame::new(0,function.clone_rc()));
        assert!(vm.run().is_ok());                              

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
        
        let handler = PPLHandler::new();
        let mut vm = VM::new(handler);
        
        vm.push_call_frame(CallFrame::new(0,function.clone_rc()));
        assert!(!vm.run().is_ok());                                
    }
}