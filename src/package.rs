use std::collections::HashMap;
use std::rc::Rc;
use crate::function::Function;

use crate::native_fn::{NativeFn, NativeFnType};

/// A structure that indexes the position of functions
/// in the Package in the package_elements vector
pub struct Package {
    functions: HashMap<String,i16>,
    pub name: String
}

pub type Packages = HashMap<String,Package>;

impl Package {
    pub fn new(name: String)->Self{
        Self {
            name: name.clone(),
            functions: HashMap::new()
        }
    }
    

    pub fn register_rust_func(&mut self, name: &str, func: NativeFnType, elements: &mut Vec<Function>)->Result<(),String> {
        
        let native_fn = NativeFn::new(name.as_bytes(), func);
        
        let function: Function = Function::Native(Rc::new(native_fn));

        self.register_func(function, elements)
    }  


    pub fn register_func(&mut self, func: Function, elements: &mut Vec<Function> )->Result<(),String> {
        
        // Get the name
        let func_name = format!("{}",func.get_name());
        let f_name_2 = func_name.clone();

        // Push it
        let index = elements.len() as i16;
        elements.push(func);

        // register it
        match self.functions.insert(func_name,index){
            None => Ok(()),
            Some(_) => Err(format!("Function '{}' is already in package '{}'", f_name_2, self.name))
        }
        
    }

    pub fn get(&self, func_name: &String)->Option<&i16>{
        self.functions.get(func_name)
    }
}