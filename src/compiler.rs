use crate::options::Options;
use crate::token::Token;
use crate::function::Function;
use crate::parser::Parser;

pub struct Local {
    pub name: Token,
    pub depth: usize,
    initialized: bool    
}


#[derive(PartialEq)]
pub enum CompilerOptions {
    Optimize
}

pub struct Compiler {

    pub locals: Vec<Local>,    
    pub scope_depth: usize,

    pub optimize: bool,    
}


pub fn compile(source: &Vec<u8>) -> Option<Function> {            
    let compiler_options : Options<CompilerOptions> = vec![];

    let mut compiler = Compiler::new(compiler_options);
    let mut parser = Parser::new(source);

    return parser.program(&mut compiler);

}

impl Compiler {

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
    pub fn var_is_in_scope(&self, var: &Token, source: &Vec<u8>)->bool {
            
        let var_slice = var.source_slice(source);

        if self.locals.len() == 0 {
            println!("No locals!");
            return false;
        }

        for i in (0..self.locals.len()).rev() {
            let local = &self.locals[i];
            println!("Checking local {}", i);

            // if the variables are out of scope, break, and declare it is not there
            //if local.depth < self.scope_depth {
            //    println!("local.depth < self.scope_depth");
            //    break;
            //}

            // if not the same length, don't bother
            if local.name.length == var.length {
                println!("local.name.length == var.length");
                if var_slice == local.name.source_slice(source){
                    println!("var_slice == local.name.source_slice(source)");
                    return true
                }
            }
        }

        return false;
    }

    /// Retrieves the position of a local variable in the scope    
    pub fn get_local(&self, var: &Token, source: &Vec<u8>) -> Option<usize> {
            
        let var_slice = var.source_slice(source);

        if self.locals.len() == 0 {
            return None;
        }

        for i in (0..self.locals.len()).rev() {
            let local = &self.locals[i];
                                    
            // if not the same length, don't bother
            if local.name.length == var.length {
                if var_slice == local.name.source_slice(source){
                    return Some(i)
                }
            }
        }

        return None;
    }

    /// Pushes a Local into the locals vector in the 
    /// compiler. It will warn when the vector was resized.
    pub fn add_local(&mut self, var_name: &Token){

        #[cfg(debut_assertions)]
        if self.local_count == self.locals.capacity(){
            println!("WARNING: Increasing the size of the Local arrays in the Compiler ")
        }

        self.locals.push(Local{
            name: *var_name,
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
            token_type: TokenType::Identifier,
        };

        assert_eq!(compiler.locals.len(),0);

        compiler.add_local(&token);

        assert_eq!(compiler.locals.len(),1);
        assert_eq!(compiler.locals[0].depth, 0);
        assert_eq!(compiler.locals[0].name.source_text(&src), "He");
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
            token_type: TokenType::Identifier,
        };

        assert_eq!(compiler.locals.len(),0);

        assert!(!compiler.var_is_in_scope(&token, &src));
        compiler.add_local(&token);

        assert_eq!(compiler.locals.len(),1);
        assert_eq!(compiler.locals[0].depth, 0);
        assert_eq!(compiler.locals[0].name.source_text(&src), "He");
        assert!(!compiler.locals[0].initialized);

        assert!(compiler.var_is_in_scope(&token, &src));
    }

    #[test]
    fn test_get_local(){

        let mut compiler = Compiler::new(vec![]);
        let src = vec!['H' as u8,'e' as u8,'l' as u8, 'l' as u8, 'o' as u8];

        let token = Token{
            line: 1,
            length: 2,
            start: 0,            
            token_type: TokenType::Identifier,
        };

        let token2 = Token{
            line: 1,
            length: 2,
            start: 2,            
            token_type: TokenType::Identifier,
        };

        let not_added = Token{
            line: 1,
            length: 3,
            start: 0,            
            token_type: TokenType::Identifier,
        };

        assert_eq!(compiler.locals.len(),0);

        assert!(!compiler.var_is_in_scope(&token, &src));
        assert!(!compiler.var_is_in_scope(&token2, &src));
        
        compiler.add_local(&token);
        compiler.add_local(&token2);

        assert_eq!(compiler.locals.len(),2);

        assert_eq!(compiler.locals[0].depth, 0);
        assert_eq!(compiler.locals[0].name.source_text(&src), "He");
        assert!(!compiler.locals[0].initialized);

        assert_eq!(compiler.locals[1].depth, 0);
        assert_eq!(compiler.locals[1].name.source_text(&src), "ll");
        assert!(!compiler.locals[1].initialized);

        assert_eq!(compiler.get_local(&token, &src).unwrap(),0);
        assert_eq!(compiler.get_local(&token2, &src).unwrap(),1);
        assert!(compiler.get_local(&not_added, &src).is_none());

    }
}