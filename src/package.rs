use std::collections::HashMap;
use std::rc::Rc;

use crate::function::Function;
use crate::native_fn::{NativeFn, NativeFnType};

pub struct Package {
    functions: HashMap<String,Function>,
    pub name: String
}

pub type Packages = HashMap<String,Package>;

impl Package{
    pub fn new(name: &String)->Self{
        Self {
            name: name.clone(),
            functions: HashMap::new()
        }
    }

    pub fn register_rust_func(&mut self, name: &str, func: NativeFnType )->Result<(),String> {
        
        let native_fn = NativeFn {
            name: name.to_string(),
            func: func
        };
        
        let function: Function = Function::Native(Rc::new(native_fn));

        self.register_func(function)
    }  


    pub fn register_func(&mut self, func: Function)->Result<(),String> {
        
        let func_name = func.get_name().clone();
        let f_name_2 = func_name.clone();
        match self.functions.insert(func_name,func){
            None => Ok(()),
            Some(_) => Err(format!("Function '{}' is already in package '{}'", f_name_2, self.name))
        }
        
    }

    pub fn get(&self, func_name: &String)->Option<&Function>{
        self.functions.get(func_name)
    }
}