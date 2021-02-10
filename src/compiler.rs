use crate::options::Options;
use crate::token::Token;
use crate::function::Function;
use crate::parser::Parser;
use crate::package::{Packages};
use crate::heap_list::HeapList;

pub struct Local<'a> {
    pub name: Token<'a>,
    pub depth: usize,
    initialized: bool    
}


#[derive(PartialEq)]
pub enum CompilerOptions {
    Optimize
}

pub struct Compiler<'a> {

    pub locals: Vec<Local<'a>>,    
    pub scope_depth: usize,

    pub optimize: bool,    
}


pub fn compile<'a>(source: &'a [u8], heap: &mut HeapList, packages_dictionary: &mut Packages, packages_elements: &mut Vec<Function>) -> Option<Function> {            
    let compiler_options : Options<CompilerOptions> = vec![];

    let mut compiler = Compiler::new(compiler_options);
    let mut parser = Parser::new(source);

    parser.program(&mut compiler, heap, packages_dictionary, packages_elements)

}

impl <'a>Compiler<'a> {

    /// Creates an empty compiler.
    pub fn new( options: Options<CompilerOptions> )->Self{        
        Self{
            locals: Vec::with_capacity(200),            
            scope_depth: 0,
            optimize: options.contains(&CompilerOptions::Optimize),

        }
    }

    

    /// Marks a variable as initialized        
    pub fn mark_initialized(&mut self){
        /*
        if self.scope_depth == 0 {
            return;
        }
        */
        let local_count = self.locals.len();
        //self.locals[local_count - 1].depth = self.scope_depth;
        self.locals[local_count - 1].initialized = true;
        
    }

    /// Returns the number of locals
    pub fn local_count(&self)->usize{
        self.locals.len()
    }


    /// Checks if a variable is within the scope of the compler.
    /// 
    /// It iterates the locals vector from the end to the start. 
    /// If we move out of scope (i.e., the depth of the locals is 
    /// different from the current one in the compiler), it stops
    /// iterating and returns false.
    pub fn var_is_in_scope(&self, var: &Token)->bool {
            
        let var_slice = var.source_slice();

        if self.locals.is_empty() {            
            return false;
        }

        for i in (0..self.locals.len()).rev() {
            let local = &self.locals[i];            

            // if the variables are out of scope, break, and declare it is not there
            //if local.depth < self.scope_depth {
            //    println!("local.depth < self.scope_depth");
            //    break;
            //}

            // if not the same length, don't bother
            if local.name.length == var.length && var_slice == local.name.source_slice(){                                
                return true                
            }
        }

        false
    }

    /// Retrieves the position of a local variable in the scope    
    pub fn get_local(&self, var: &Token) -> Option<u8> {
            
        let var_slice = var.source_slice();

        if self.locals.is_empty() {
            return None;
        }

        for i in (0..self.locals.len()).rev() {
            let local = &self.locals[i];
                                    
            // if not the same length, don't bother
            if local.name.length == var.length && var_slice == local.name.source_slice() {                
                return Some(i as u8)
                
            }
        }

        None
    }

    /// Pushes a Local into the locals vector in the 
    /// compiler. It will warn when the vector was resized.
    pub fn add_local(&mut self, var_name: Token<'a>){

        #[cfg(debut_assertions)]
        if self.local_count == self.locals.capacity(){
            println!("WARNING: Increasing the size of the Local arrays in the Compiler ")
        }

        self.locals.push(Local{
            name: var_name,
            depth: self.scope_depth,            
            initialized: false
        });
        
    }
}









/***********/
/* TESTING */
/***********/

#[cfg(test)]
mod tests {
    use super::*;
    use crate::token::TokenType;

    #[test]
    fn test_add_local(){

        let mut compiler = Compiler::new(vec![]);
        let src = vec!['H' as u8,'e' as u8,'l' as u8, 'l' as u8, 'o' as u8];

        let token = Token{
            line: 1,
            length: 2,
            start: 0,
            txt: &src[0..2],            
            token_type: TokenType::Identifier,
        };

        assert_eq!(compiler.locals.len(),0);

        compiler.add_local(token);

        assert_eq!(compiler.locals.len(),1);
        assert_eq!(compiler.locals[0].depth, 0);
        assert_eq!(compiler.locals[0].name.source_text(), "He");
        assert!(!compiler.locals[0].initialized);
    }

    #[test]
    fn test_var_is_in_scope(){
        let mut compiler = Compiler::new(vec![]);
        let src = vec!['H' as u8,'e' as u8,'l' as u8, 'l' as u8, 'o' as u8];

        let token = Token{
            line: 1,
            length: 2,
            start: 0,
            txt: &src[0..2],            
            token_type: TokenType::Identifier,
        };

        assert_eq!(compiler.locals.len(),0);

        assert!(!compiler.var_is_in_scope(&token));
        compiler.add_local(token);

        assert_eq!(compiler.locals.len(),1);
        assert_eq!(compiler.locals[0].depth, 0);
        assert_eq!(compiler.locals[0].name.source_text(), "He");
        assert!(!compiler.locals[0].initialized);

        assert!(compiler.var_is_in_scope(&token));
    }

    #[test]
    fn test_get_local(){

        let mut compiler = Compiler::new(vec![]);
        let src = vec!['H' as u8,'e' as u8,'l' as u8, 'l' as u8, 'o' as u8];

        let token = Token{
            line: 1,
            length: 2,
            start: 0,
            txt: &src[0..2],            
            token_type: TokenType::Identifier,
        };

        let token2 = Token{
            line: 1,
            length: 2,
            start: 2,
            txt: &src[2..4],            
            token_type: TokenType::Identifier,
        };

        let not_added = Token{
            line: 1,
            length: 3,
            start: 0,
            txt: &src[0..3],            
            token_type: TokenType::Identifier,
        };

        assert_eq!(compiler.locals.len(),0);

        assert!(!compiler.var_is_in_scope(&token));
        assert!(!compiler.var_is_in_scope(&token2));
        
        compiler.add_local(token);
        compiler.add_local(token2);

        assert_eq!(compiler.locals.len(),2);

        assert_eq!(compiler.locals[0].depth, 0);
        assert_eq!(compiler.locals[0].name.source_text(), "He");
        assert!(!compiler.locals[0].initialized);

        assert_eq!(compiler.locals[1].depth, 0);
        assert_eq!(compiler.locals[1].name.source_text(), "ll");
        assert!(!compiler.locals[1].initialized);

        assert_eq!(compiler.get_local(&token).unwrap(),0);
        assert_eq!(compiler.get_local(&token2).unwrap(),1);
        assert!(compiler.get_local(&not_added).is_none());

    }
}