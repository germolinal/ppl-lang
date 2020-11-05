extern crate ppl_lib;
use std::env;
use std::fs;
use ppl_lib::vm::VM;
use ppl_lib::parser::Parser;


pub fn main(){
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    
    //let query = &args[1];
    let filename = &args[1];
    let script = fs::read(filename).unwrap();

    let mut parser = Parser::new(&script);
    parser.program();
    let _vm = VM::new();    

    // file contents.

//    println!("Searching for {}", query);
    println!("In file {}", filename);
    
}

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