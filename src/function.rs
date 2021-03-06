use std::any::Any;
use std::rc::Rc;

use crate::native_fn::NativeFn;
use crate::script_fn::ScriptFn;
use crate::value_trait::ValueTrait;
use crate::chunk::Chunk;
use crate::vm::VM;
use crate::heap_list::HeapList;

#[derive(Clone)]
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

    pub fn new_script(name: &[u8])->Function{
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
    
    pub fn push_constant(&mut self,v: Box<dyn ValueTrait>, heap: &mut HeapList)->u8{
        match self{
            Function::Native(_)=>panic!("Trying to push constant to a native function"),
            Function::Script(f)=> {
                match Rc::get_mut(f){
                    Some(a)=>a.push_to_heap(v, heap),
                    None => panic!("Trying push_constant to a Function already shared")
                }                                
            }
        }
    }

    pub fn set_n_args(&mut self, n: u8){
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
        format!("fn {}()",self.get_name())
    }

    fn type_name(&self)->String {
        "Function".to_string()
    }

    fn is_function(&self)->bool {
        true
    }
    
    fn as_any(&self) -> &dyn Any{
        self
    }    

    fn drop_references(&self, _h: &mut HeapList){
        panic!("Do Functions need to drop references?")
    }

    

    fn call(&self, vm: &mut VM, n_args: u8)->Result<u8,String> {

        // Call

        match self {
            Function::Script(f) => {
                // Check number of arguments
                if n_args != f.n_args {
                    return Err(format!("Incorrect number of arguments. Found {}, required {}", n_args, f.n_args));
                }
                Ok(1)                
            },
            Function::Native(f)=>{
                // Get the function
                let rust_fn = f.func;                                
                // Call it
                Ok(rust_fn(n_args, vm))
            }
        }
    }
}