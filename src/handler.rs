use std::collections::HashMap;

use crate::function::Function;
use crate::package::{Packages, Package};
use crate::vm::{VM,InterpretResult};
use crate::parser::Parser;

use crate::values::Value;

pub struct Handler<'a>{
    vm : VM,
    parser: Parser<'a>,
    packages : Packages,    
}

impl <'a>Handler<'a> {

    pub fn new(source: &'a Vec<u8>)->Self{
        Self{
            parser: Parser::new(source),
            packages: HashMap::new(),
            vm: VM::new(),            
        }
    }

    pub fn run(&mut self){
        match self.parser.compile(){
            Some(f)=>{

                let (code, lines) = f.chunk().to_slices();//self.parser.current_function().chunk().to_slices();

                match self.vm.run(code,lines){
                    InterpretResult::Ok => {
    
                    },
                    InterpretResult::CompileError(e)=>panic!("Compile Error: {}",e),
                    InterpretResult::RuntimeError(e)=>panic!("Runtime Error: {}",e),
    
                }
            }
            None=>{                
                panic!("ERROR WHEN PARSING")                
            }   
        }
        
    }

    pub fn pop_stack(&mut self)->Result<Value,&str>{
        self.vm.pop()
    }

    pub fn push_stack(&mut self, v: Value){
        self.vm.push(v)
    }
    

    pub fn push_package(&mut self, pkg: Package){
        let name = pkg.name.clone();
        match self.packages.insert(name, pkg){
            None => {},
            Some(_) => panic!("Package '{}' is already registered")
        }
    }
}