//use std::collections::HashMap;
extern crate ppl_lib;
use std::env;
use std::fs;

use ppl_lib::handler::PPLHandler;
// Packages
use ppl_lib::io;

use ppl_lib::vm::{VM, InterpretResult};
use ppl_lib::compiler;
use ppl_lib::call_frame::CallFrame;
//use ppl_lib::heap_list::HeapList;
//use ppl_lib::package::Packages;
//use ppl_lib::function::Function;




pub fn main(){
    let args: Vec<String> = env::args().collect();
    
    if args.len() > 1 {

        //let query = &args[1];
        let filename = &args[1];
        let script = fs::read(filename).unwrap();
        
        let mut handler = PPLHandler::new();

        /*
        let mut heap = HeapList::new();
        let mut packages_elements : Vec<Function> = Vec::with_capacity(64);
        let mut packages_dictionary : Packages = HashMap::new();
        */

        //register_io_package(&mut packages_dictionary, &mut packages_elements);
        io::register_package(&mut handler);

        //let main_function = match compiler::compile(&script, &mut heap, &mut packages_dictionary, &mut packages_elements){
        let main_function = match handler.compile(&script){
            None => panic!("Compilation error!"),
            Some(f) => f
        };

        
        let mut vm = VM::new(handler);
        vm.push_call_frame(CallFrame::new(0, main_function));

        match vm.run() {
            InterpretResult::Ok(_)=>{},
            InterpretResult::RuntimeError(e)=>panic!(e)
        }
            
        //let mut handler = Handler::new(&script);
        
        
    
        

        // Run file
        //handler.run();

    }else{
        panic!("A script File is required")
    }




    
}

/*
struct PPLOptions {
    pub filename: Option<String>,    
}

impl PPLOptions {
    pub fn new(args: &mut Vec<String>)->Self{
        let flnm = match args.len() {
            1 => { None },
            _ => { args.pop() }
        };
        Self{
            filename : flnm
        }
    }
}
*/