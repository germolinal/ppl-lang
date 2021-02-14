use criterion::{black_box, criterion_group, criterion_main, Criterion};

extern crate ppl_lib;

use ppl_lib::handler::PPLHandler;
use ppl_lib::io;
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

        let mut handler = PPLHandler::new();

        io::register_package(&mut handler);

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
    } 
    ));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);