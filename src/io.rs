// MINIMUM REQUIREMENTS
use crate::package::Package;
use crate::handler::Handler;
use crate::vm::VM;

use crate::value_trait::ValueTrait;

fn print(vm: &mut VM, nvars: usize)->usize{
    
    for _ in 0..nvars{
        let v = vm.pop().unwrap();
        print!("{} ",v.to_string());
    }
    println!("");

    return 0;
}

pub fn register_io_package(handler: &mut Handler){
    
    // Create the packate
    let mut pkg = Package::new(&"io".to_string());

    // Add functions
    pkg.register_rust_func("print",print).unwrap();

    // Register the package
    handler.push_package(pkg);
}