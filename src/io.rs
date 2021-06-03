use crate::handler::PPLHandler;
use crate::package::Package;
use crate::value_trait::ValueTrait;
use crate::values::Value;
use crate::vm::VM;

fn print(n_args: u8, vm: &mut VM)->u8{
           
    let fin = vm.stack_length();
    let ini = fin - n_args;
    // Print them first (in the right order)
    for i in ini..fin {    
        let v = vm.borrow_stack_element(i);//vm.pop().unwrap();
        match v {
            Value::HeapRef(index)=>{
                print!("{} ", vm.borrow_heap_reference(*index).unwrap().to_string())                
            },
            Value::PackageRef(index)=>{
                print!("{} ", vm.borrow_package_reference(*index).unwrap().to_string())                
            },
            _ => print!("{} ", v.to_string()),
        }                
    }
    println!();
    // Pop them all
    for _ in ini..fin{
        vm.pop().unwrap();
    }

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