extern crate ppl_lib;
use std::env;
use std::fs;

//use ppl_lib::handler::Handler;
// Packages
//use ppl_lib::io::register_io_package;



pub fn main(){
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    
    if args.len() > 1{

        //let query = &args[1];
        let filename = &args[1];
        let script = fs::read(filename).unwrap();
    
        //let mut handler = Handler::new(&script);
        
        // Reguster packages
        //register_io_package(&mut handler);
    
        //    println!("Searching for {}", query);
        println!("In file {}", filename);

        // Run file
        //handler.run();

    }else{
        panic!("A script File is required")
    }




    
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