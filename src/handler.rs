use std::collections::HashMap;

use crate::heap_list::HeapList;
use crate::function::Function;
use crate::package::{Packages,Package};
use crate::native_fn::NativeFnType;

use crate::compiler::Compiler;
use crate::parser::Parser;

pub struct PPLHandler {
    pub heap: HeapList,
    pub packages_elements : Vec<Function>,
    pub packages_dictionary : Packages,    
}

impl PPLHandler{

    /// Creates a new Handler
    pub fn new()->Self{
        Self{
            heap : HeapList::new(),
            packages_elements : Vec::with_capacity(64),
            packages_dictionary : HashMap::new(),            
        }
    }

    /// Registers a Rust function 
    pub fn register_rust_function(&mut self, name: &str, func: NativeFnType, package: &mut Package)->Result<(),String>{
        package.register_rust_func(name, func, &mut self.packages_elements)
    }

    /// Registers a package in the handler
    pub fn register_package(&mut self, package: Package)->Result<(),String>{
        if self.packages_dictionary.contains_key(&package.name){
            return Err(format!("Package '{}' already exists", package.name))
        }
        self.packages_dictionary.insert(package.name.clone(), package);
        Ok(())
    }   


    /// Compiles a source code
    pub fn compile<'a>(&mut self, source: &'a [u8]/*, heap: &mut HeapList, packages_dictionary: &mut Packages, packages_elements: &mut Vec<Function>*/) -> Option<Function> {            
        
        let mut compiler = Compiler::new();
        let mut parser = Parser::new(source);

        parser.program(self, &mut compiler)//heap, packages_dictionary, packages_elements)

    }

}