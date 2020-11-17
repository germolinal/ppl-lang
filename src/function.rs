use std::any::Any;

use crate::rust_fn::RustFn;
use crate::script_fn::ScriptFn;
use crate::value_trait::ValueTrait;
use crate::values::Value;
use crate::vm::{VM, InterpretResult};
//use crate::chunk::Chunk;


pub enum Function{
    Rust(RustFn),
    Script(ScriptFn)
}

impl Function {

    pub fn new_script(name: &String)->ScriptFn{
        ScriptFn::new(name)
    }

    pub fn get_name(&self)->&String{
        match self {
            Function::Rust(v)=>&v.name,
            Function::Script(v)=>&v.name
        }
    }
    

}

impl ValueTrait for Function {
    // Basic i/o
    fn to_string(&self)->String {
        format!("fn {}()",match self {
            Function::Rust(v)=>&v.name,
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

    fn call(&self, vm: &mut VM, n: usize)->Result<usize,String> {
        match self {
            Function::Script(f) => {
                let (code, lines) = f.chunk().to_slices();
                if let InterpretResult::Ok(n) = vm.run(code,lines, f.chunk().constants()){                    
                    return Ok(n);
                }else{
                    return Err(format!("Error when running function '{}'", f.name));
                }
            },
            Function::Rust(f)=>{
                // Get the function
                let rust_fn = f.func;                
                // Call it
                Ok(rust_fn(vm,n))
            }
        }
    }
}