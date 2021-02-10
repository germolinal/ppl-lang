
use crate::function::Function;
use crate::package::{Package, Packages};
use crate::vm::VM;
use crate::value_trait::ValueTrait;

fn print(vm: &mut VM, n_args: u8)->u8{
        
    let args = vm.get_last_stack(n_args);    

    for arg in args{        
        print!("{} ",arg.unwrap().to_string());
    }
    println!();

    0
}

pub fn register_io_package(packages: &mut Packages, elements : &mut Vec<Function>){
    
    // Create the packate
    let mut pkg = Package::new("io".to_string());

    // Add functions
    pkg.register_rust_func("print",print, elements).unwrap();

    // Register the package
    packages.insert(pkg.name.clone(), pkg);
    //handler.push_package(pkg);
}