
use crate::debug::*;
use crate::scanner::*;
use crate::token::*;
use crate::parse_function::*;
use crate::function::Function;
use crate::operations::Operation;
use crate::compiler::Compiler;
use crate::heap_list::HeapList;
use crate::package::{/*Package,*/ Packages};




#[repr(u8)] 
#[derive(PartialEq,PartialOrd)]
pub enum Precedence{    
    None,
    Assignment,  // =
    Or,          // or
    And,         // and
    Equality,    // == !=
    Comparison,  // < > <= >=
    Term,        // + -
    Factor,      // * /
    Unary,       // ! -
    Call,        // . ()
    Primary,
}


type ParseFn<'a> = fn(can_assign: bool, &mut Parser<'a>, &mut Compiler<'a>, heap: &mut HeapList, packages_dictionary: &mut Packages, packages_elements: &mut Vec<Function>);

pub struct ParseRule<'a> {
    pub prefix: Option<ParseFn<'a>>,
    pub infix: Option<ParseFn<'a>>,
    pub precedence: Precedence,
    pub next_precedence: Option<Precedence>
}


pub struct Parser<'a>{
    current: Token<'a>,
    previous: Token<'a>,
    had_error: bool,
    panic_mode: bool,
    scanner: Scanner<'a>,    
    
    current_function: Option<Function>,        
    //current_package: &'a mut Package
}

impl <'a>Parser<'a>{

    pub fn new(source: &'a Vec<u8>)->Self{
        let scanner = Scanner::new(source);
        let previous = scanner.make_token(TokenType::EOF);
        let current = scanner.make_token(TokenType::EOF);
        
        let main_function = Function::new_script("main".as_bytes());
        
        Self {
            scanner: scanner,
            had_error: false,
            panic_mode: false,
            current: current,
            previous: previous,            
            current_function : Some(main_function),                  
        }
    }

    pub fn take_current_function(&mut self)->Option<Function>{
        self.current_function.take()
    }

    

    pub fn set_function(&mut self, func : Function){
        self.current_function = Some(func);
    }


    pub fn chunk_len(&mut self)->Option<usize>{
        match &self.current_function {
            Some(f)=>Some(f.chunk().unwrap().len()),
            None => {
                self.error_no_current_function();
                None
            }
        }
    }

    pub fn patch_chunk(&mut self, position: usize, op: Operation){
        match &mut self.current_function{
            Some(f)=>{
                let chunk = f.mut_chunk().unwrap();
                let (_, ln) = chunk[position];
                chunk[position] = (op, ln);
            }
            None => {
                self.error_no_current_function()                
            }
        }        
    }

        
    

    /* UTILITY FUNCTIONS */

    #[cfg(debug_assertions)]
    #[allow(dead_code)]
    pub fn show_tokens(&self, msg: &str){
        println!("at {} == previous: {} | current: {}", msg, debug::token(self.previous), debug::token(self.current));
    }
    
    
    pub fn previous(&self)->&Token<'a>{
        &self.previous
    }

    pub fn current(&self)->&Token<'a>{
        &self.current
    }
    
    pub fn advance(&mut self){
        self.previous = self.current;        
        self.current = self.scanner.scan_token();

        loop{
            match self.current.token_type(){
                TokenType::Error => {
                    let msg = self.scanner.error_msg();
                    self.error_at_current(msg);
                    break;
                },
                _ => break
            }
        }
    }

    pub fn consume(&mut self, expected_type: TokenType)->bool{
        if self.current.token_type() == expected_type {
            self.advance();
            return true;
        }        
        false
    }


    /// Checks that the current token is of a certain type
    pub fn check(&self, t: TokenType)->bool{
        self.current.token_type() == t
    }

    /// Checks that the current token is of a certain type
    /// and, if it is, advances.
    pub fn match_token(&mut self, t: TokenType) -> bool{
        if !self.check(t) {
            return false;
        }
        self.advance();
        true
    }
    

    pub fn emit_byte(&mut self, op: Operation){
        match &mut self.current_function{
            Some(f)=>f.mut_chunk().unwrap().push( (op, self.previous.line()) ),
            None => self.error_no_current_function()                
        }        
    }
        

        
    pub fn get_rule(&self, ttype: TokenType)->ParseRule<'a>{
        match ttype{
            TokenType::RightParen | TokenType::RightBracket |            
            TokenType::LeftBrace | TokenType::RightBrace |
            TokenType::Comma | //TokenType::Semicolon |
            TokenType::Equal |
            TokenType::Class | 
            TokenType::Else |
            TokenType::For | 
            TokenType::If |
            TokenType::Return |
            TokenType::Let | 
            TokenType::While | 
            TokenType::Break | TokenType::In |
            TokenType::EOF | TokenType::Error            
            => {
                ParseRule{
                    prefix:None,
                    infix:None,
                    precedence:Precedence::None,
                    next_precedence: None
                }
            },
            TokenType::LeftParen => {
                ParseRule{
                    prefix:Some(grouping),
                    infix:Some(call),
                    precedence:Precedence::Call,
                    next_precedence: Some(Precedence::Primary)
                }
            },
            TokenType::LeftBracket => {
                ParseRule{
                    prefix:Some(array),
                    infix:Some(index),
                    precedence:Precedence::Call,
                    next_precedence: Some(Precedence::Primary)
                }
            },
            TokenType::Number => {
                ParseRule{
                    prefix: Some(number),
                    infix: None,
                    precedence: Precedence::None,
                    next_precedence: Some(Precedence::Assignment),
                }
            },      
            TokenType::Identifier =>{                
                ParseRule{
                    prefix: Some(variable),
                    infix: None,
                    precedence: Precedence::None,
                    next_precedence: Some(Precedence::Assignment),
                }
            },           
            TokenType::Package =>{                
                ParseRule{
                    prefix: Some(package_element),
                    infix: None,
                    precedence: Precedence::None,
                    next_precedence: Some(Precedence::Assignment),
                }
                
            },                        
            TokenType::TokenString => {
                ParseRule{
                    prefix: Some(string),
                    infix: None,
                    precedence: Precedence::None,
                    next_precedence: Some(Precedence::Assignment),
                }
            },
            TokenType::Function => {
                ParseRule{
                    prefix: Some(function_value),
                    infix: None,
                    precedence: Precedence::None,
                    next_precedence: Some(Precedence::Assignment),
                }
            },
            TokenType::Minus => {
                ParseRule{
                    precedence: Precedence::Term,
                    next_precedence: Some(Precedence::Factor),
                    prefix: Some(unary),
                    infix: Some(binary),
                }                
            },
            TokenType::Plus => {
                ParseRule{
                    precedence: Precedence::Term,
                    next_precedence: Some(Precedence::Factor),
                    prefix: None,
                    infix: Some(binary),
                }
            },
            TokenType::Star => {
                ParseRule{
                    precedence: Precedence::Factor,
                    next_precedence: Some(Precedence::Unary),
                    prefix: None,
                    infix: Some(binary),
                }
            },
            TokenType::Bang => {
                ParseRule{
                    precedence: Precedence::None,
                    next_precedence: Some(Precedence::Assignment),
                    prefix: Some(unary),
                    infix: None
                }
            },
            TokenType::Slash => {
                ParseRule{
                    precedence: Precedence::Factor,
                    next_precedence: Some(Precedence::Unary),
                    prefix: None,
                    infix: Some(binary),
                }
            },
            
            TokenType::True | TokenType::False => {
                ParseRule{
                    precedence: Precedence::None,
                    next_precedence: Some(Precedence::Assignment),
                    prefix: Some(literal),
                    infix: None,
                }
            },
            
            TokenType::BangEqual | TokenType::EqualEqual =>{
                ParseRule{
                    precedence: Precedence::Equality,
                    next_precedence: Some(Precedence::Comparison),
                    prefix: None,
                    infix: Some(binary),
                }
            },
            TokenType::Greater | TokenType::GreaterEqual | TokenType::Less | TokenType::LessEqual => {
                ParseRule{
                    precedence: Precedence::Comparison,
                    next_precedence: Some(Precedence::Term),
                    prefix: None,
                    infix: Some(binary),
                }
            },
            TokenType::Or => {
                ParseRule{
                    precedence: Precedence::Or,
                    next_precedence: Some(Precedence::And),
                    prefix: None,
                    infix: Some(binary),
                }
            },
            TokenType::And => {
                ParseRule{
                    precedence: Precedence::And,
                    next_precedence: Some(Precedence::Equality),
                    prefix: None,
                    infix: Some(binary),
                }
            },
            TokenType::Dot => {
                /*
                ParseRule{
                    precedence: Precedence::Call,
                    next_precedence: Some(Precedence::Primary),
                    prefix: None,
                    infix: Some(dot),
                }
                */
                unimplemented!()

            },
            TokenType::Question => {
                /*
                ParseRule{
                    precedence: Precedence::Assignment,
                    next_precedence: Some(Precedence::Or),
                    prefix: None,
                    infix: Some(question),
                }
                */
                unimplemented!();
            },
            
            TokenType::TokenSelf => {
                /*
                ParseRule{
                    prefix: Some(self),
                    infix: None,
                    precedence: Precedence::None,
                    next_precedence: Some(Precedence::Assignment),
                }
                */
                unimplemented!()
            }
            /*
            _ => {
                eprintln!(" ===> {}",debug::token_type(ttype));
                unimplemented!()
            }
            */
        }
    }

    pub fn parse_precedence(&mut self, compiler: &mut Compiler<'a>, precedence: Precedence, heap: &mut HeapList, packages_dictionary: &mut Packages, packages_elements: &mut Vec<Function>){          
        self.advance();
        let rule = match self.get_rule(self.previous.token_type()).prefix {
            Some(r) => r,
            None => {
                self.error_at_current(format!("Expecting expression.")); 
                return;
            }
        };

        let can_assign : bool = precedence <= Precedence::Assignment;

        // Run the rule
        rule(can_assign, self, compiler, heap, packages_dictionary, packages_elements);

        while precedence <= self.get_rule(self.current.token_type()).precedence {
            self.advance();
            match self.get_rule(self.previous.token_type()).infix{
                Some(r)=>r(can_assign, self, compiler, heap, packages_dictionary, packages_elements),
                None => self.internal_error_at_current(format!("No infix rule!"))
            }
        }                
    }

    
    /// Compiles a program, returns an Option<ScriptFn>        
    ///     
    /// # EBNF Grammar
    /// program -> declaration* EOF
    pub fn program(&mut self, compiler: &mut Compiler<'a>, heap: &mut HeapList, packages_dictionary: &mut Packages, packages_elements: &mut Vec<Function>) -> Option<Function> {            
        // Prime the pump
        self.advance();
        //self.advance();

        while !self.match_token(TokenType::EOF) && !self.had_error{
            self.declaration(compiler, heap, packages_dictionary, packages_elements);
        }
                

        if self.had_error{
            #[cfg(debug_assertions)]
            {
                match &self.current_function{
                    Some(f)=>{
                        let ch = f.chunk().unwrap();
                        debug::chunk(ch, format!("main"));
                    },
                    None=> println!("No Main Chunk to debug")
                }
                
            }
            return None;
        }else{
            self.emit_byte(Operation::Return);
            return self.take_current_function();
        }
    }

    /// Compiles a declaration    
    ///     
    /// # EBNF Grammar
    /// declaration -> classDecl | funDecl | varDecl | statement
    fn declaration(&mut self, compiler: &mut Compiler<'a>, heap: &mut HeapList, packages_dictionary: &mut Packages, packages_elements: &mut Vec<Function>){
        match self.current.token_type(){
            TokenType::Class => {
                self.advance();
                unimplemented!();
            },
            TokenType::Function => {
                self.advance();                
                self.fn_declaration(compiler, heap, packages_dictionary, packages_elements);
            },
            TokenType::Let => {
                self.advance();
                let mut n : usize= 0;
                self.var_declaration(compiler, heap, true, &mut n, packages_dictionary, packages_elements);
            },
            _ => {                                
                self.statement(compiler, heap, packages_dictionary, packages_elements);
            }
        }
    }

    
    /// Compiles a function
    /// 
    /// # EBNF Grammar
    /// function -> fn IDENTIFIER (varlist) BLOCK
    fn fn_declaration(&mut self, compiler: &mut Compiler<'a>, heap: &mut HeapList, packages_dictionary: &mut Packages, packages_elements: &mut Vec<Function>){
        
        // fn has been consumed.
        if self.consume(TokenType::Identifier){

            // declare the variable.
            self.declare_variable(compiler);
            
            let func_name = self.previous;

            let func  = match function(self, func_name.txt, compiler, heap, packages_dictionary, packages_elements){
                Some(f)=>f,
                None => return
            };
    
            // Push constant.
            let i = heap.push(Box::new(func));        
    
            // Register the function
            self.emit_byte(Operation::PushHeapRef(i));            


        }else{
            return self.error_at_current(format!("Expecting identifier after 'let'. Found '{}'",self.previous.source_text() ))
        };
                

        
        

    }
    

    /// Compiles a Variable declaration    
    ///     
    /// # EBNF Grammar
    /// var_declaration -> "let" IDENTIFIER ("=" expression)
    pub fn var_declaration(&mut self, compiler: &mut Compiler<'a>, heap: &mut HeapList, define: bool, n_declared_vars : &mut usize, packages_dictionary: &mut Packages, packages_elements: &mut Vec<Function>){        
        
        // Get the token representing the name
        if !self.consume(TokenType::Identifier){
            let txt = self.previous.source_text().to_string();
            return self.error_at_current(format!("Expecting identifier after 'let'. Found '{}'", txt ));
        }       

        // Declare the variable
        self.declare_variable(compiler);

        if define {
            // Define 
            if self.match_token(TokenType::Equal){                                                               
                // Put value of expression on the stack                        
                self.expression(compiler, heap, packages_dictionary, packages_elements);                             
            }else{
                // Or NIL.
                self.emit_byte(Operation::PushNil);            
            }
                
            self.define_variable(compiler);
        }
                

        // Count the declared variable
        *n_declared_vars+=1;        

        
        // Check if there is another variable afterwards
        if self.consume(TokenType::Comma) {
            self.var_declaration(compiler, heap, define, n_declared_vars, packages_dictionary, packages_elements);
        }    
        
    }

    /// Declares a variable, failing if it is a re-declaration
    /// 
    /// Checks if a variable with the same name exists
    /// within the same scope_depth in the compiler. If 
    /// not, it pushes the Local into the locals vector
    /// in the compiler.
    fn declare_variable(&mut self, compiler: &mut Compiler<'a>){
        let var_name = self.previous();
                    
        if compiler.var_is_in_scope(var_name){
            //self.error_at_current(format!("A variable called '{}' already exists in this scope", var_name.source_text(self.source())));
            panic!("A variable called '{}' already exists in this scope", var_name.source_text());
            //return;
        }
                
        compiler.add_local(*var_name)
            
    }

    /// Compiles a while statement
    /// 
    /// # EBNF Grammar:
    /// while_statement -> while EXPRESSION BLOCK
    fn while_statement(&mut self, compiler: &mut Compiler<'a>, heap: &mut HeapList, packages_dictionary: &mut Packages, packages_elements: &mut Vec<Function>){
        let while_start = match self.chunk_len(){
            Some(i)=>i,
            None => return
        };

        // Compile expression (puts a boolean on the stack)
        self.expression(compiler, heap, packages_dictionary, packages_elements);
        
        // This is patched later in this function
        self.emit_byte(Operation::JumpIfFalse(0)); 

        /* PROCESS BODY */
        // Mark the beginning of body (for looping)
        let body_start = match self.chunk_len(){
            Some(i)=>i,
            None => return
        };

        // consume Left Brace
        if !self.consume(TokenType::LeftBrace){
            return self.error_at_current(format!("Expecting '{{' when opening For loop."));
        }
        // Open, process, and close the scope for the body        
        self.begin_scope(compiler);
        self.block(compiler, heap, packages_dictionary, packages_elements);        
        self.end_scope(compiler);

        // Mark the end
        let body_end = match self.chunk_len(){
            Some(i)=>i,
            None => return
        };

        // Add jump back, before the expression
        self.emit_byte(Operation::JumpBack(body_end - while_start));
        
        // Patch jump
        let body_length = body_end - body_start;
        //self.chunk().patch_code(body_start-1 ,Operation::JumpIfFalse(body_length+2)); // Jumping the JumpBack
        self.patch_chunk(body_start-1, Operation::JumpIfFalse(body_length+2));
    }

    /// Compiles an If statement
    /// # EBNF Grammar:
    /// if_statement -> if EXPRESSION BLOCK
    fn if_statement(&mut self, compiler: &mut Compiler<'a>, heap: &mut HeapList, packages_dictionary: &mut Packages, packages_elements: &mut Vec<Function>){
        
        // Compile expression (puts a boolean on the stack)
        self.expression(compiler, heap, packages_dictionary, packages_elements);

        // This is patched later in this function        
        self.emit_byte(Operation::JumpIfFalse(0)); 

        /* PROCESS BODY */
        // Mark the beginning of body (for looping)
        let body_start = match self.chunk_len(){
            Some(i)=>i,
            None => return
        };

        // consume Left Brace
        if !self.consume(TokenType::LeftBrace){
            return self.error_at_current(format!("Expecting '{{' when opening For loop."));
        }
        // Open, process, and close the scope for the body        
        self.begin_scope(compiler);
        self.block(compiler, heap, packages_dictionary, packages_elements);        
        self.end_scope(compiler);

        // Mark the end
        let body_end = match self.chunk_len(){
            Some(i)=>i,
            None => return
        };

        // Patch jump
        let body_length = body_end - body_start;
        //self.chunk().patch_code(body_start-1 ,Operation::JumpIfFalse(body_length+1)); 
        self.patch_chunk(body_start-1, Operation::JumpIfFalse(body_length)); 

        // check else
        if self.consume(TokenType::Else){
            // This is patched after the statement is processed
            self.emit_byte(Operation::JumpIfTrue(0));
            let body_start = match self.chunk_len(){
                Some(i)=>i,
                None => return
            };

            // check if "else if" case 
            if self.consume(TokenType::If){
                // Get rid of old                 
                self.if_statement(compiler, heap, packages_dictionary, packages_elements);

            } else if self.consume(TokenType::LeftBrace){
                // Open, process, and close the scope for the body        
                self.begin_scope(compiler);
                self.block(compiler, heap, packages_dictionary, packages_elements);        
                self.end_scope(compiler);
                
            } else{
                return self.error_at_current(format!("Expecting 'if' or '{{' after 'else' keyword"));
            }

            // Mark the end
            let body_end = match self.chunk_len(){
                Some(i)=>i,
                None => return
            };

            // Patch jump
            let body_length = body_end - body_start;
            //self.chunk().patch_code(body_start-1 ,Operation::JumpIfTrue(body_length+1)); 
            self.patch_chunk(body_start-1, Operation::JumpIfTrue(body_length));

            
        }

        // Pop the Expression that drove this flow
        self.emit_byte(Operation::Pop(1));

    }

    /// Compiles a for statement
    /// 
    /// # EBNF Grammar:
    /// for_statement -> for IDENTIFIER in EXPRESSION  BLOCK
    fn for_statement(&mut self, compiler: &mut Compiler<'a>, heap: &mut HeapList, packages_dictionary: &mut Packages, packages_elements: &mut Vec<Function>){
                
        // Open the main scope for this for statement.
        self.begin_scope(compiler);

        // consume declare the variables
        let mut n_declared_vars : usize = 0;
        self.var_declaration(compiler, heap, true, &mut n_declared_vars, packages_dictionary, packages_elements);                
        
        // consume 'in', or fail
        if !self.consume(TokenType::In) {
            return self.error_at_current(format!("Expecting keyword 'in' when declaring For loop."));
        }
        
        
        // Evaluate the value to iterate over... put it at the end 
        // of the stack.
        self.expression(compiler, heap, packages_dictionary, packages_elements);
        
        
        /* PROCESS BODY */
        // Mark the beginning of body (for looping)
        let body_start = match self.chunk_len(){
            Some(i)=>i,
            None => return
        };

        // consume Left Brace
        if !self.consume(TokenType::LeftBrace){
            return self.error_at_current(format!("Expecting '{{' when opening For loop."));
        }
        // Open, process, and close the scope for the body        
        self.begin_scope(compiler);
        self.block(compiler, heap, packages_dictionary, packages_elements);        
        self.end_scope(compiler);

        // Mark the end of the scope
        let body_end = match self.chunk_len(){
            Some(i)=>i,
            None => return
        };
        
        // Close the main scope
        self.end_scope(compiler);

        // emit Loop operation
        let body_length = body_end - body_start;
        self.emit_byte(Operation::ForLoop(n_declared_vars, body_length))


    }

    /// Compiles a statement
    /// 
    /// # EBNF Grammar:
    /// statement -> expression | forStmt | ifStmt | returnStmt | whileStmt| block ;
    fn statement(&mut self, compiler: &mut Compiler<'a>, heap: &mut HeapList, packages_dictionary: &mut Packages, packages_elements: &mut Vec<Function>){
                
        match self.current.token_type(){
            TokenType::LeftBrace => {
                self.advance();
                self.begin_scope(compiler);
                self.block(compiler, heap, packages_dictionary, packages_elements);
                self.end_scope(compiler);
            },
            TokenType::For => {
                self.advance();
                self.for_statement(compiler, heap, packages_dictionary, packages_elements);
            },
            TokenType::If =>{
                self.advance();
                self.if_statement(compiler, heap, packages_dictionary, packages_elements);
            },
            TokenType::Return =>{
                // only one value can be returned
                self.advance();
                self.expression(compiler, heap, packages_dictionary, packages_elements);
                self.emit_byte(Operation::Return)
            },
            TokenType::While =>{
                self.advance();
                self.while_statement(compiler, heap, packages_dictionary, packages_elements);
            },
            TokenType::EOF=>{
                return
            },
            _ => {
                // Expression-statement
                self.expression(compiler, heap, packages_dictionary, packages_elements);
                self.emit_byte(Operation::Pop(1));
            }
        }
    }


    /// Compiles an expression
    /// # EBFN Grammar:
    /// expression -> literal | unary | binary | grouping;
    pub fn expression(&mut self, compiler: &mut Compiler<'a>, heap: &mut HeapList, packages_dictionary: &mut Packages, packages_elements: &mut Vec<Function>){                    
        self.parse_precedence(compiler, Precedence::Assignment, heap, packages_dictionary, packages_elements);
    }


    /// Opens a scope, after a Left Brace
    pub fn begin_scope(&mut self, compiler: &mut Compiler<'a>){                          
        compiler.scope_depth += 1;        
    }

    /// Closes a scope and emits all the necessary
    /// PopVar operations, removing the local variables
    pub fn end_scope(&mut self, compiler: &mut Compiler<'a>){    
        // reduce scope
        compiler.scope_depth -= 1;
            
        // remove locals in that scope
        let mut local_count = 0;
        while compiler.local_count() > 0 && compiler.locals[compiler.local_count()-1].depth > compiler.scope_depth  {
            local_count+=1;                
            compiler.locals.pop();
        }
        self.emit_byte(Operation::Pop(local_count));
    }

    
    /// Parses a block
    /// 
    /// # EBNF Grammar:
    /// block -> "{" declaration* "}"
    pub fn block(&mut self, compiler: &mut Compiler<'a>, heap: &mut HeapList, packages_dictionary: &mut Packages, packages_elements: &mut Vec<Function>){ 
        // LEFT BRACE HAS BEEN CONSUMED ALREADY
        while !self.check(TokenType::RightBrace) && !self.check(TokenType::EOF){
            self.declaration(compiler, heap, packages_dictionary, packages_elements);
        }
        
        if !self.consume(TokenType::RightBrace){
            return self.error_at_current(format!("Expecting '}}' after block."));            
        }
        
    }

    /// Marks a variable in the compiler as initialized
    fn define_variable(&mut self, compiler: &mut Compiler<'a>){        
        compiler.mark_initialized();        
    }
    
    

    /* ERROR FUNCTIONS */

    pub fn error_no_current_function(&mut self){
        self.error_at_current(format!("Trying to use Parser's current function... found None"))
    }

    pub fn error_at_current(&mut self, msg: String){
        self.error_at(self.current, msg);
    }
    


    fn error_at(&mut self, token: Token, msg:String){
        if self.panic_mode {
            return
        }else{
            self.panic_mode = true;
        }

        eprint!("[line {}] Error", token.line());

        match token.token_type(){
            TokenType::EOF => eprint!(" at end"),
            TokenType::Error => {/*ignore*/},
            _ => eprint!(" at '{}'", token.source_text())
        }
        
        eprintln!(": {}", msg);
        self.had_error = true;                  
    }

    pub fn internal_error_at_current(&mut self, msg: String){
        self.internal_error_at(self.current, msg);
    }


    fn internal_error_at(&mut self, token: Token, msg:String){
        if self.panic_mode {
            return
        }else{
            self.panic_mode = true;
        }

        eprint!("INTERNAL ERROR: [line {}] Error", token.line());

        match token.token_type(){
            TokenType::EOF => eprint!(" at end"),
            TokenType::Error => {/*ignore*/},
            _ => eprint!(" at '{}'", token.source_text())
        }
        
        eprintln!(": {}", msg);
        self.had_error = true;                  
    }
}






/***********/
/* TESTING */
/***********/

#[cfg(test)]
mod tests {
    use super::*;    
    use crate::vm::VM;
    use crate::chunk::Chunk;
    use crate::value_trait::ValueTrait;
    

    // this is used in some calses below.
    impl <'a>Parser<'a> {
        pub fn chunk(&self)->Option<&Chunk> {
            if self.current_function.is_none(){
                // We need this to avoid double borrowing.
                panic!("No curent function!");
                //return None
            }else{
                match &self.current_function{
                    Some(f)=>Some(f.chunk().unwrap()),
                    None=>None
                }
            }
        }
    }
    

    #[test]
    fn test_parser_new(){
        let raw_source = "".to_string();
        let source : Vec<u8> = raw_source.into_bytes();
        let parser = Parser::new(&source);
        
        assert!(!parser.had_error);
        assert!(!parser.panic_mode);
    }

    #[test]
    fn test_parser_advance(){
        let raw_source = "({".to_string();
        let source : Vec<u8> = raw_source.into_bytes();
        let mut parser = Parser::new(&source);
        parser.advance();
        match parser.current.token_type(){
            TokenType::LeftParen => {},
            _ => {panic!("Expecting left paren")}
        }
    }

    #[test]
    fn test_parser_consume(){
        let raw_source = "({".to_string();
        let source : Vec<u8> = raw_source.into_bytes();
        let mut parser = Parser::new(&source);
        parser.advance();
        parser.advance();

        match parser.previous.token_type(){
            TokenType::LeftParen => {},
            _ => {panic!("Expecting left paren")}
        }
        match parser.current.token_type(){
            TokenType::LeftBrace => {},
            _ => {panic!("Expecting left Brace")}
        }

        assert!(!parser.consume(TokenType::Number));
        assert!(parser.consume(TokenType::LeftBrace));
    }

    #[test]
    fn test_parse_number(){
        let raw_source = "2".to_string();
        let source : Vec<u8> = raw_source.into_bytes();
        let mut compiler = Compiler::new(vec![]);
        let mut parser = Parser::new(&source);
        let mut packages_elements : Vec<Function> = Vec::with_capacity(64);
        let mut packages_dictionary : Packages = HashMap::new();

        parser.advance();
        parser.advance();

        match parser.previous.token_type(){
            TokenType::Number => {},
            _ => {panic!("Expecting Number, found {}", debug::token(parser.previous))}
        }
        
        let mut heap = HeapList::new();
        
        number(false, &mut parser, &mut compiler, &mut heap, &mut packages_dictionary, &mut packages_elements);        
        if let (Operation::PushNumber(found), _) = parser.chunk().unwrap().last().unwrap() {            
            assert_eq!(2.0,*found);            
        }else{
            assert!(false);
        }


        let raw_source = "2.1".to_string();
        let source : Vec<u8> = raw_source.into_bytes();
        let mut parser = Parser::new(&source);
        
        parser.advance();
        parser.advance();

        match parser.previous.token_type(){
            TokenType::Number => {},
            _ => {panic!("Expecting Number, found {}", debug::token(parser.previous))}
        }
        
        number(false, &mut parser,&mut compiler, &mut heap, &mut packages_dictionary, &mut packages_elements);
        number(false, &mut parser,&mut compiler, &mut heap, &mut packages_dictionary, &mut packages_elements);        
        if let (Operation::PushNumber(found),_) = parser.chunk().unwrap().last().unwrap() {            
            assert_eq!(2.1,*found);            
        }else{
            assert!(false);
        }

    }

    use crate::values::*;
    use crate::call_frame::CallFrame;
    use std::collections::HashMap;

    #[test]
    fn test_expression(){
        let raw_source = "let y = !( 5 - 4 > 3 * 22 == !false) ".to_string();
        let source : Vec<u8> = raw_source.into_bytes();

        let mut compiler = Compiler::new(vec![]);
        let mut parser = Parser::new(&source);
        let mut heap = HeapList::new();
        let mut packages_elements : Vec<Function> = Vec::with_capacity(64);
        let mut packages_dictionary : Packages = HashMap::new();

        if let Some(f) = parser.program(&mut compiler, &mut heap, &mut packages_dictionary, &mut packages_elements){
            
            let chunk = f.chunk().unwrap();
            debug::chunk(chunk, format!("test_expression chunck"));

            let mut vm = VM::new();   

            vm.push_call_frame(CallFrame::new(0,f.clone_rc()));
            vm.run(&mut heap, &packages_elements);            

            
            
            if let Ok(v) = vm.pop(){
                if let Value::Bool(b) = v {
                    assert!(b);
                }else{
                    assert!(false);
                }
            }else{
                assert!(false);
            }
            
        }else{
            assert!(false);
        }
        
    }

    #[test]
    fn test_var_declaration_1(){        
        let raw_source = "let x let y = 2 let i ".to_string();
        let source : Vec<u8> = raw_source.into_bytes();

        let mut compiler = Compiler::new(vec![]);
        let mut parser = Parser::new(&source);
        let mut heap = HeapList::new();
        let mut packages_elements : Vec<Function> = Vec::with_capacity(64);
        let mut packages_dictionary : Packages = HashMap::new();

        if let Some(f) = parser.program(&mut compiler, &mut heap, &mut packages_dictionary, &mut packages_elements){
            
            let chunk = f.chunk().unwrap();
            
            // Check the operations...
            debug::chunk(chunk, format!("test_var_declaration_1"));
            
            // define X (should be nil)
            if let (Operation::PushNil,_) = chunk[0]{
                assert!(true);
            }else{
                let ops_lines = chunk.as_slice();
                debug::operation(ops_lines, 0);
                assert!(false)
            };

            // Push y, should be 2
            if let (Operation::PushNumber(v),_) = chunk[1]{
                assert_eq!(v, 2.0);
            }else{
                let ops_lines = chunk.as_slice();
                debug::operation(ops_lines, 1);
                assert!(false)
            };

            // Push i, should be Nil
            if let (Operation::PushNil,_) = chunk[2]{
                assert!(true);
            }else{
                let ops_lines = chunk.as_slice();
                debug::operation(ops_lines, 2);
                assert!(false)
            };

           
            
        }else{
            assert!(false)
        }
        assert!(!parser.had_error);
        

        // Do it slowly now

        let mut compiler = Compiler::new(vec![]);
        let mut parser = Parser::new(&source);
        let mut heap = HeapList::new();
        let mut packages_elements : Vec<Function> = Vec::with_capacity(64);
        let mut packages_dictionary : Packages = HashMap::new();

        // prime the pump (done in self.program())
        parser.advance();

        /* DECLARE X */

        // previous is LET, consume.
        // (done in self.declaration())
        parser.advance();
        
        let mut n : usize= 0;
        parser.var_declaration(&mut compiler, &mut heap, true, &mut n, &mut packages_dictionary, &mut packages_elements);
        assert_eq!(n,1);        

        // Check that the next one is let
        if let TokenType::Let = parser.current.token_type(){
            assert!(true);
        }else{
            assert!(false);
        }

        /* DECLARE Y */

        // previous is LET, consume.
        // (done in self.declaration())
        parser.advance();
        
        let mut n : usize= 0;
        parser.var_declaration(&mut compiler, &mut heap, true, &mut n, &mut packages_dictionary, &mut packages_elements);
        assert_eq!(n,1);

        // Check that the next one is let
        if let TokenType::Let = parser.current.token_type(){
            assert!(true);
        }else{
            assert!(false);
        }

        //debug::chunk(&parser.chunk,"the_chunk".to_string());
    }

    

    
    #[test]
    fn test_var_declaration_2(){
        let raw_source = "let x = 2 let y let z = true".to_string();
        let source : Vec<u8> = raw_source.into_bytes();

        let mut compiler = Compiler::new(vec![]);
        let mut parser = Parser::new(&source);
        let mut heap = HeapList::new();
        let mut packages_elements : Vec<Function> = Vec::with_capacity(64);
        let mut packages_dictionary : Packages = HashMap::new();

        if let Some(f) = parser.program(&mut compiler, &mut heap, &mut packages_dictionary, &mut packages_elements){                          
            let chunk = f.chunk().unwrap();
            debug::chunk(chunk,"the_chunk".to_string());
                        
            // define X (should be 2.0)
            if let (Operation::PushNumber(v),_) = chunk[0]{
                assert_eq!(v,2.0);
            }else{
                let ops_lines = chunk.as_slice();
                debug::operation(ops_lines, 0);
                assert!(false)
            };

            // Push y, should be Nil
            if let (Operation::PushNil,_) = chunk[1]{
                assert!(true);
            }else{
                let ops_lines = chunk.as_slice();
                debug::operation(ops_lines, 1);
                assert!(false)
            };

            // Push z, should be true
            if let (Operation::PushBool(v),_) = chunk[2]{
                assert!(v);
            }else{
                let ops_lines = chunk.as_slice();
                debug::operation(ops_lines, 2);
                assert!(false)
            };

           
        }else{
            assert!(false)
        }

        
    }
    


    
    #[test]
    fn test_var_list_declaration(){
        let raw_source = "let x = 2, y, z = true".to_string();
        let source : Vec<u8> = raw_source.into_bytes();

        // 
        let mut compiler = Compiler::new(vec![]);
        let mut parser = Parser::new(&source);
        let mut heap = HeapList::new();
        let mut packages_elements : Vec<Function> = Vec::with_capacity(64);
        let mut packages_dictionary : Packages = HashMap::new();

        parser.advance();parser.advance();
        let mut n : usize = 0;
        parser.var_declaration(&mut compiler, &mut heap, true, &mut n, &mut packages_dictionary, &mut packages_elements);
        assert_eq!(n,3);


        let mut compiler = Compiler::new(vec![]);
        let mut parser = Parser::new(&source);
        if let Some(f) = parser.program(&mut compiler, &mut heap, &mut packages_dictionary, &mut packages_elements){
            let chunk = f.chunk().unwrap();

            debug::chunk( chunk ,"the_chunk".to_string());

             // define X (should be 2.0)
             if let (Operation::PushNumber(v),_) = chunk[0]{
                assert_eq!(v,2.0);
            }else{
                let ops_lines = chunk.as_slice();
                debug::operation(ops_lines, 0);
                assert!(false)
            };

            // Push y, should be Nil
            if let (Operation::PushNil,_) = chunk[1]{
                assert!(true);
            }else{
                let ops_lines = chunk.as_slice();
                debug::operation(ops_lines, 1);
                assert!(false)
            };

            // Push z, should be True
            if let (Operation::PushBool(v),_) = chunk[2]{
                assert!(v);
            }else{
                let ops_lines = chunk.as_slice();
                debug::operation(ops_lines, 2);
                assert!(false)
            };

        }else{
            assert!(false)
        }
                
    }
    

    
    #[test]
    fn test_current_scope_has_variable(){
        let raw_source = "let x let x2 {let y} let z".to_string();
        let source : Vec<u8> = raw_source.into_bytes();

        let mut compiler = Compiler::new(vec![]);
        let mut parser = Parser::new(&source);
        let mut heap = HeapList::new();
        let mut packages_elements : Vec<Function> = Vec::with_capacity(64);
        let mut packages_dictionary : Packages = HashMap::new();

        parser.program(&mut compiler, &mut heap, &mut packages_dictionary, &mut packages_elements);

        let x = Token{
            line: 1,
            length: 1,
            start: 4,
            txt: &source[4..5],
            token_type: TokenType::Identifier
        };
        assert_eq!(x.source_text(), format!("x"));

        assert!(compiler.var_is_in_scope(&x));

        let y = Token{
            line: 1,
            length: 1,
            start: 18,
            txt: &source[18..19],
            token_type: TokenType::Identifier
        };
        assert_eq!(y.source_text(), format!("y"));
        assert!(!compiler.var_is_in_scope(&y));
    }
    

    #[test]
    fn test_block(){
        let raw_source = "let x { let y } let z".to_string();
        let source : Vec<u8> = raw_source.into_bytes();

        let mut compiler = Compiler::new(vec![]);
        let mut parser = Parser::new(&source);
        let mut heap = HeapList::new();
        let mut packages_elements : Vec<Function> = Vec::with_capacity(64);
        let mut packages_dictionary : Packages = HashMap::new();

        parser.program(&mut compiler, &mut heap, &mut packages_dictionary, &mut packages_elements);
        assert!(!parser.had_error);

        // Now do it more slowly

        // prime the pump (done in self.program())
        let mut compiler = Compiler::new(vec![]);
        let mut parser = Parser::new(&source);
        let mut heap = HeapList::new();
        let mut packages_elements : Vec<Function> = Vec::with_capacity(64);
        let mut packages_dictionary : Packages = HashMap::new();

        parser.advance();

        
        /* DECLARE X */

        // Check if there is a let, and consume it.
        assert!(parser.match_token(TokenType::Let));        
        let mut n : usize= 0;
        parser.var_declaration(&mut compiler, &mut heap, true, &mut n, &mut packages_dictionary, &mut packages_elements);
        assert_eq!(n,1);

        // Check that the next one is {, and consume it
        assert!(parser.match_token(TokenType::LeftBrace));       
                

        /* CONSUME BLOCK */
        parser.block(&mut compiler, &mut heap, &mut packages_dictionary, &mut packages_elements);

        // declare Z (check that there is a LET, and consume it first)
        assert!(parser.match_token(TokenType::Let));       
        let mut n : usize= 0;
        parser.var_declaration(&mut compiler, &mut heap, true, &mut n, &mut packages_dictionary, &mut packages_elements);
        assert_eq!(n,1);
    }

    

    #[test]
    fn test_if(){
        let raw_source = "if 3 { }else if 4{}".to_string();
        let source : Vec<u8> = raw_source.into_bytes();

        let mut compiler = Compiler::new(vec![]);
        let mut parser = Parser::new(&source);
        let mut heap = HeapList::new();
        let mut packages_elements : Vec<Function> = Vec::with_capacity(64);
        let mut packages_dictionary : Packages = HashMap::new();

        if let Some(f) = parser.program(&mut compiler, &mut heap, &mut packages_dictionary, &mut packages_elements){

            let chunk = f.chunk().unwrap();

            // Check the operations...
        
            // Push 3
            if let (Operation::PushNumber(v),_) = chunk[0]{
                assert_eq!(3.0,v);
            }else{assert!(false)};

            // set jump.
            if let (Operation::JumpIfFalse(v),_) = chunk[1]{
                assert_eq!(2,v);
            }else{assert!(false)};

            // Pop variables.
            if let (Operation::Pop(n),_) = chunk[2]{
                assert_eq!(0,n);
            }else{assert!(false)};

            

            debug::chunk(chunk, format!("for loop chunck"));
        }else{
            assert!(false)
        }

        
    }

    #[test]
    fn test_while_loop(){        
        let raw_source = "while 3 { }".to_string();
        let source : Vec<u8> = raw_source.clone().into_bytes();

        let mut compiler = Compiler::new(vec![]);
        let mut parser = Parser::new(&source);
        let mut heap = HeapList::new();
        let mut packages_elements : Vec<Function> = Vec::with_capacity(64);
        let mut packages_dictionary : Packages = HashMap::new();
        
        if let Some(f) = parser.program(&mut compiler, &mut heap, &mut packages_dictionary, &mut packages_elements){
            // Check the operations...
            let chunk = f.chunk().unwrap();
            debug::chunk(chunk, raw_source);


            // Push 3
            if let (Operation::PushNumber(v),_) = chunk[0]{
                assert_eq!(3.0,v);
            }else{assert!(false)};

            // set jump.
            if let (Operation::JumpIfFalse(v),_) = chunk[1]{
                assert_eq!(3,v);
            }else{assert!(false)};

            // Pop variables.
            if let (Operation::Pop(n),_) = chunk[2]{
                assert_eq!(0,n);
            }else{assert!(false)};

            // Jump back.
            if let (Operation::JumpBack(n),_) = chunk[3]{
                assert_eq!(3,n);
            }else{assert!(false)};

            
        }else{
            assert!(false)
        }
        

        
    }

    #[test]
    fn test_for_loop(){
        let raw_source = "for i,j in 3 {}".to_string();
        let source : Vec<u8> = raw_source.clone().into_bytes();

        let mut compiler = Compiler::new(vec![]);
        let mut parser = Parser::new(&source);
        let mut heap = HeapList::new();
        let mut packages_elements : Vec<Function> = Vec::with_capacity(64);
        let mut packages_dictionary : Packages = HashMap::new();

        if let Some(f) = parser.program(&mut compiler, &mut heap, &mut packages_dictionary, &mut packages_elements){
            // Check the operations...
            let chunk = f.chunk().unwrap();
            debug::chunk(chunk, raw_source);
            
            // Push i
            if let (Operation::PushNil,_) = chunk[0]{
                assert!(true);
            }else{
                let ops_lines = chunk.as_slice();
                debug::operation(ops_lines, 0);
                assert!(false)
            };

            // Push j
            if let (Operation::PushNil,_) = chunk[1]{
                assert!(true);
            }else{
                let ops_lines = chunk.as_slice();
                debug::operation(ops_lines, 1);
                assert!(false)
            };

            // Push 3
            if let (Operation::PushNumber(v),_) = chunk[2]{
                assert_eq!(3.0,v);
            }else{assert!(false)};

            // ... body happens

            // Pop vars from body        
            if let (Operation::Pop(_),_) = chunk[3]{
                assert!(true);
            }else{assert!(false)};

            // Pop vars from main scope        
            if let (Operation::Pop(_),_) = chunk[4]{
                assert!(true);
            }else{assert!(false)};

            // FOR LOOP
            if let (Operation::ForLoop(n_vars,length),_) = chunk[5]{
                assert_eq!(n_vars,2);
                assert_eq!(length,1); // The PopVars operation
            }else{assert!(false)};
            
        }else{
            assert!(false)
        }

        
        
    }

    /*
    #[test]
    fn test_array_expression(){
        let raw_source = "let x = [0,1,2,3]".to_string();
        let source : Vec<u8> = raw_source.into_bytes();

        let mut parser = Parser::new(&source);
        if let Some(f) = parser.program(){
            let n_ops = f.chunk().code().len();

            if let Operation::PushArray(4) = f.chunk().code()[n_ops-3]{
                assert!(true);
            }else{
                assert!(false)
            };
            
            for i in 0..4{            
                match f.chunk().code()[i+1]{
                    Operation::PushNumber(v) => assert_eq!(i as f64,v),
                    _ => panic!("At i = {}", i)
                }

            }
                    
            debug::chunk(f.chunk(), format!("array chunck"));
        }else{
            assert!(false)
        }

        
    }
    */

    #[test]
    #[should_panic]
    fn test_wrong_function_declaration(){
        let raw_source = "let x = fn(2) { \nlet i = 123 \nreturn i }".to_string();
        let source : Vec<u8> = raw_source.into_bytes();

        let mut compiler = Compiler::new(vec![]);
        let mut parser = Parser::new(&source);   
        let mut heap = HeapList::new();
        let mut packages_elements : Vec<Function> = Vec::with_capacity(64);
        let mut packages_dictionary : Packages = HashMap::new();

        if let Some(_) = parser.program(&mut compiler, &mut heap, &mut packages_dictionary, &mut packages_elements){
        }else{
            assert!(false);
        }
    }

    #[test]
    fn test_function_declaration_no_params(){
        let raw_source = "fn x() { \nlet i = 123 \nreturn i }".to_string();
        let source : Vec<u8> = raw_source.into_bytes();

        let mut compiler = Compiler::new(vec![]);
        let mut parser = Parser::new(&source);  
        let mut heap = HeapList::new();
        let mut packages_elements : Vec<Function> = Vec::with_capacity(64);
        let mut packages_dictionary : Packages = HashMap::new();

        if let Some(_) = parser.program(&mut compiler, &mut heap, &mut packages_dictionary, &mut packages_elements){
        }else{
            assert!(false);
        }
    }

    
    #[test]
    fn test_function_expression(){
        let raw_source = "let x = fn(a) { let i = fn(){} return i }".to_string();
        let source : Vec<u8> = raw_source.clone().into_bytes();

        let mut compiler = Compiler::new(vec![]);
        let mut parser = Parser::new(&source);  
        let mut heap = HeapList::new();
        let mut packages_elements : Vec<Function> = Vec::with_capacity(64);
        let mut packages_dictionary : Packages = HashMap::new();

        if let Some(main) = parser.program(&mut compiler, &mut heap, &mut packages_dictionary, &mut packages_elements){
            assert!(!parser.had_error);

            let chunk = main.chunk().unwrap();

            debug::chunk(chunk, raw_source);

            let x = heap.get(0).unwrap()                    
                    .as_any()
                    .downcast_ref::<Function>()
                    .expect("Wasn't a Function");
                    
                    if let Function::Script(s) = x {
                        
                        debug::chunk(s.chunk(), format!("{}", x.to_string()));
                        
                    }else{
                        assert!(false);
                    }

            
            if let (Operation::PushHeapRef(v),_) = chunk[0] {
                assert_eq!(v, 1 as usize);                
                match heap.get(v){
                    Some(_s)=>{
                        
                    },
                    None => {assert!(false)}                    
                }
                
            }else {                
                print!("Found wrong operation... ");
                debug::operation(chunk.as_slice(), 1);
                panic!("wrong operation")
            }
        }else{
            assert!(false)
        }
        
        
    }


    

    #[test]
    fn test_function_declaration(){
        let raw_source = "fn x (a,b,c) { }".to_string();
        let source : Vec<u8> = raw_source.clone().into_bytes();

        
        let mut compiler = Compiler::new(vec![]);
        let mut parser = Parser::new(&source); 
        let mut heap = HeapList::new();
        let mut packages_elements : Vec<Function> = Vec::with_capacity(64);
        let mut packages_dictionary : Packages = HashMap::new();

        if let Some(f) = parser.program(&mut compiler, &mut heap, &mut packages_dictionary, &mut packages_elements){
            assert!(!parser.had_error);
            let chunk = f.chunk().unwrap();

            debug::chunk(chunk, raw_source);

            let x_token = Token{
                line: 1,
                length: 1,
                start: 3,
                txt: &source[3..4],            
                token_type: TokenType::Identifier,
            };
            assert_eq!(compiler.local_count(),1);
            println!("locals[0] -> '{}'", compiler.locals[0].name.source_text());
            assert_eq!(x_token.source_text(),format!("x"));
            assert!(compiler.var_is_in_scope(&x_token));

            let x = heap.get(0).unwrap()            
                    .as_any()
                    .downcast_ref::<Function>()
                    .expect("Wasn't a Function");

            if let Function::Script(s) = x {
                debug::chunk(s.chunk(), format!("{}", x.to_string()));

            }else{
                assert!(false);
            }            
                    

        }else{
            assert!(false)
        }           
                                
    }

    

    #[test]
    fn test_call(){
        let raw_source = "fn x(a,b,c) { }\n x()".to_string();
        let source : Vec<u8> = raw_source.clone().into_bytes();

        let mut compiler = Compiler::new(vec![]);
        let mut parser = Parser::new(&source);
        let mut heap = HeapList::new();
        let mut packages_elements : Vec<Function> = Vec::with_capacity(64);
        let mut packages_dictionary : Packages = HashMap::new();

        if let Some(f) = parser.program(&mut compiler, &mut heap, &mut packages_dictionary, &mut packages_elements){
            assert!(!parser.had_error);
            let chunk = f.chunk().unwrap();

            debug::chunk(chunk, raw_source);
        }else{
            assert!(false)
        }   
        
        
        let raw_source = "fn x(a,b,c) { }\n x(1,123,1*2*3)".to_string();
        let source : Vec<u8> = raw_source.clone().into_bytes();

        let mut compiler = Compiler::new(vec![]);
        let mut parser = Parser::new(&source);  
        let mut heap = HeapList::new();
        let mut packages_elements : Vec<Function> = Vec::with_capacity(64);
        let mut packages_dictionary : Packages = HashMap::new();

        if let Some(f) = parser.program(&mut compiler, &mut heap, &mut packages_dictionary, &mut packages_elements){
            assert!(!parser.had_error);
            let chunk = f.chunk().unwrap();
            
            debug::chunk(chunk, raw_source);
            
            // the function x() should be there... nothing else
            assert_eq!(heap.len(),1);              

        } else {
            assert!(false)
        }   
    }
}