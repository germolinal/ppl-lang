use crate::handler::PPLHandler;
use crate::package::Package;
use crate::value_trait::ValueTrait;
use crate::vm::VM;

fn print(n_args: u8, vm: &mut VM)->u8{
            

    for _ in 0..n_args {        
        print!("{} ", vm.pop().unwrap().to_string());
    }
    println!();

    0
}

pub fn register_package(handler : &mut PPLHandler/*packages: &mut Packages, elements : &mut Vec<Function>*/){
    
    // Create the packate
    let mut pkg = Package::new("io".to_string());

    // Add functions    
    handler.register_rust_function("print", print, &mut pkg).unwrap();

    // Register the package
    handler.packages_dictionary.insert(pkg.name.clone(), pkg);
    
}