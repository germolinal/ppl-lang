use criterion::{black_box, criterion_group, criterion_main, Criterion};

extern crate ppl_lib;
use ppl_lib::heap_list::HeapList;
use ppl_lib::package::Packages;
use ppl_lib::function::Function;
use std::collections::HashMap;
use ppl_lib::io::register_io_package;
use ppl_lib::compiler;
use ppl_lib::call_frame::CallFrame;
use ppl_lib::vm::{VM, InterpretResult};

fn criterion_benchmark(c: &mut Criterion) {

    let script = black_box("
    fn fib(n){    
        if n < 2 {
            return n
        }else{
            return fib(n - 1) + fib(n - 2)
        }
    }
    
    let y = fib(25)".as_bytes().to_vec());

    c.bench_function("fib 25", |b| b.iter(||{

        let mut heap = HeapList::new();
        let mut packages_elements : Vec<Function> = Vec::with_capacity(64);
        let mut packages_dictionary : Packages = HashMap::new();

        register_io_package(&mut packages_dictionary, &mut packages_elements);

        let main_function = match compiler::compile(&script, &mut heap, &mut packages_dictionary, &mut packages_elements){
            None => panic!("Compilation error!"),
            Some(f) => f
        };

        
        let mut vm = VM::new();
        vm.push_call_frame(CallFrame::new(0, main_function));

        match vm.run(&mut heap, &packages_elements) {
            InterpretResult::Ok(_)=>{},
            InterpretResult::RuntimeError(e)=>panic!(e)
        }
    } 
    ));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);