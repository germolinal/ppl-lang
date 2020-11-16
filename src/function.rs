
use crate::rust_fn::RustFn;
use crate::script_fn::ScriptFn;
use crate::value_trait::ValueTrait;
use crate::values::Value;
//use crate::chunk::Chunk;


pub enum Function{
    Rust(Box<RustFn>),
    Script(Box<ScriptFn>)
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

    // Copy and clone
    fn clone_to_value(&self)->Value{
        panic!("Trying to clone a function.")
    }
}