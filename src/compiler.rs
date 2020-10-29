use crate::scanner::*;
#[cfg(test)]
use crate::debug::*;
use crate::chunk::*;

pub fn compile(source : &Vec<u8>, chunk: &mut Chunk) {
    let mut scanner = Scanner::new(source);

    let mut line : usize = 0;
    loop {
        let token = scanner.scan_token();
        
        
        #[cfg(test)]
        println!("{}",debug::debug_token(token, source));

        /*
        match token.token_type() {            
            TokenType::EOF => break,

            TokenType::Error => panic!("ERROR!"),

            TokenType::LeftBrace => 
            _ =>{}

        }  
        */
    }
}