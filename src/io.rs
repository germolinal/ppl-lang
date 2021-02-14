use crate::handler::PPLHandler;
use crate::package::Package;
use crate::value_trait::ValueTrait;
use crate::values::Value;
use crate::vm::VM;

fn print(n_args: u8, vm: &mut VM)->u8{
            

    for _ in 0..n_args {    
        let v = vm.pop().unwrap();
        match v {
            Value::HeapRef(_)=>{
                print!("{} ", vm.resolve_heap_reference(v).unwrap().to_string())                
            },
            Value::PackageRef(_)=>{
                print!("{} ", vm.resolve_package_reference(v).unwrap().to_string())                
            },
            _ => print!("{} ", v.to_string()),
        }                
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