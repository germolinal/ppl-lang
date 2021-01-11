use std::any::Any;
use std::rc::Rc;

use crate::native_fn::NativeFn;
use crate::script_fn::ScriptFn;
use crate::value_trait::ValueTrait;
use crate::values::Value;
use crate::chunk::Chunk;
use crate::vm::VM;

pub enum Function{
    Native(Rc<NativeFn>),
    Script(Rc<ScriptFn>)
}


impl Function {

    pub fn clone_rc(&self)->Function{
        match self {
            Function::Native(v)=>Function::Native(Rc::clone(v)),
            Function::Script(v)=>Function::Script(Rc::clone(v))
        }
    }

    pub fn new_script(name: &String)->Function{
        Function::Script(Rc::new(ScriptFn::new(name)))
    }

    pub fn get_name(&self)->&String{
        match self {
            Function::Native(v)=>&v.name,
            Function::Script(v)=>&v.name
        }
    }

    pub fn is_native(&self)->bool{
        match self {
            Function::Native(_)=>true,
            Function::Script(_)=>false
        }
    }

    pub fn chunk(&self)->Option<&Chunk>{
        match self{
            Function::Native(_)=>None,
            Function::Script(f)=>Some(f.chunk())
        }
    }

    pub fn mut_chunk(&mut self)->Option<&mut Chunk>{
        match self{
            Function::Native(_)=>None,
            Function::Script(f)=>{
                match Rc::get_mut(f){
                    Some(a)=>Some(a.mut_chunk()),
                    None => panic!("Trying to get mut_chunk of a Function already shared")
                }
                //Some(f.mut_chunk())
            }
        }
    }
    
    pub fn push_constant(&mut self,v: Box<dyn ValueTrait>)->usize{
        match self{
            Function::Native(_)=>panic!("Trying to push constant to a native function"),
            Function::Script(f)=> {
                match Rc::get_mut(f){
                    Some(a)=>a.push_to_heap(v),
                    None => panic!("Trying push_constant to a Function already shared")
                }                                
            }
        }
    }

    pub fn set_n_args(&mut self, n: usize){
        match self{
            Function::Native(_)=>panic!("Trying to set the number of arguments on a native function"),
            Function::Script(f)=>{
                match Rc::get_mut(f){
                    Some(a)=>a.set_n_args(n),
                    None => panic!("Trying set_n_args of a Function already shared")
                }  
            }                           
        }
    }

}

impl ValueTrait for Function {
    // Basic i/o
    fn to_string(&self)->String {
        format!("fn {}()",match self {
            Function::Native(v)=>&v.name,
            Function::Script(v)=>&v.name
        })
    }

    fn type_name(&self)->String{
        format!("Function")
    }

    fn as_any(&self) -> &dyn Any{
        self
    }

    // Copy and clone
    fn clone_to_value(&self)->Value{
        panic!("Trying to clone a function.")
    }

    fn call(&self, vm: &mut VM, n_args: usize)->Result<usize,String> {

        // Call

        match self {
            Function::Script(f) => {
                // Check number of arguments
                if n_args != f.n_args {
                    return Err(format!("Incorrect number of arguments. Found {}, required {}", n_args, f.n_args));
                }
                return Ok(1);
                /*
                unimplemented!();
                if let InterpretResult::Ok(n) = vm.run(){                    
                    return Ok(n);
                }else{
                    return Err(format!("Error when running function '{}'", f.name));
                }
                */
            },
            Function::Native(f)=>{
                // Get the function
                let rust_fn = f.func;                
                // Call it
                Ok(rust_fn(vm,n_args))
            }
        }
    }
}