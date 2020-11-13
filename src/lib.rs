
mod values;
mod value_trait;

mod nil;
mod number;
mod boolean;
mod object;
mod array;
mod string;
mod function;
mod rust_fn;
mod script_fn;

mod variable;

mod operations;
mod chunk;
mod debug;
mod scanner;
mod token;
mod parse_function;
pub mod parser;
pub mod vm;