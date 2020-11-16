
use crate::debug::*;
use crate::scanner::*;
//use crate::operations::*;
use crate::token::*;
use crate::chunk::*;
use crate::values::*;
use crate::parse_function::*;
use crate::function::Function;
use crate::script_fn::ScriptFn;
use crate::value_trait::ValueTrait;
use crate::operations::Operation;
//use crate::package::Packages;


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


type ParseFn = fn(&mut Parser);

pub struct ParseRule{
    pub prefix: Option<ParseFn>,
    pub infix: Option<ParseFn>,
    pub precedence: Precedence,
    pub next_precedence: Option<Precedence>
}


pub struct Parser<'a>{
    current: Token,
    previous: Token,
    had_error: bool,
    panic_mode: bool,
    scanner: Scanner<'a>,
    //chunk: Chunk,
    
    variables: Vec<String>,
    variable_count: Vec<usize>,
    constants: Vec<Box<dyn ValueTrait>>,

    optimize: bool,

    // No need to cover RustFn because it is not scanned.
    current_function: Option<Box<ScriptFn>>,        
    //current_package: &'a mut Package

}

impl <'a>Parser<'a>{

    pub fn new(source: &'a Vec<u8>)->Self{
        let scanner = Scanner::new(source);
        let previous = Token::new(&scanner,TokenType::EOF);
        let current = Token::new(&scanner,TokenType::EOF);
        
        let main_function = Box::new(Function::new_script(&"main".to_string()));

        Self{
            scanner: scanner,
            had_error: false,
            panic_mode: false,
            current: current,
            previous: previous,

            variables: vec![],
            variable_count: vec![0],
            constants: Vec::with_capacity(256),

            optimize: false,      
            current_function : Some(main_function),                  
        }
    }

    pub fn take_current_function(&mut self)->Option<Box<ScriptFn>>{
        self.current_function.take()
    }

    /*
    pub fn current_function(&self)->Result<&Box<ScriptFn>,String>{
        match self.current_function {
            Some(v) =>Ok(&v),
            None => {
                return Err(format!("Trying to get Parser's current compilation function, but found None"));                
            }
        }
    }
    */
    

    pub fn set_function(&mut self, func : Box<ScriptFn>){
        self.current_function = Some(func);
    }


    pub fn chunk_len(&mut self)->Option<usize>{
        match &self.current_function {
            Some(f)=>Some(f.chunk().code().len()),
            None => {
                self.error_no_current_function();
                None
            }
        }
    }
    

    
    pub fn chunk(&mut self)->Option<&Chunk> {
        if self.current_function.is_none(){
            // We need this to avoid double borrowing.
            self.error_no_current_function();
            return None
        }else{
            match &self.current_function{
                Some(f)=>Some(f.chunk()),
                None=>None
            }
        }
    }
    

    pub fn patch_chunk(&mut self, position: usize, op: Operation){
        match &mut self.current_function{
            Some(f)=>{
                f.mut_chunk().patch_code(position,op)
            }
            None => {
                self.error_no_current_function()                
            }
        }        
    }

    fn push_to_heap(&mut self, v: Box<dyn ValueTrait>)->usize{
        let i = self.constants.len();

        self.constants.push(v);

        return i;
    }

    /* UTILITY FUNCTIONS */

    #[cfg(debug_assertions)]
    #[allow(dead_code)]
    pub fn show_tokens(&self, msg: &str){
        println!("at {} == previous: {} | current: {}", msg, debug::token(self.previous,self.scanner.source()), debug::token(self.current, self.scanner.source()));
    }

    fn scope_depth(&self)->usize{
        self.variable_count.len()
    }    

    fn current_scope_has_variable(&self, var_name: &String)->bool{
        match self.find_var_in_scope(var_name){
            Some(_)=>true,
            None => false
        }
    }
    
    pub fn previous(&self)->Token{
        self.previous
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

    fn search_var(&self,var_name: &String, n_vars: usize)->Option<usize>{

        
        let fin = self.variables.len();
        let ini = fin - n_vars;
        for i in (ini..fin).rev(){
            if &self.variables[i] == var_name{
                return Some(i)
            }
        }
        None
    }

    pub fn find_var_in_scope(&self, var_name: &String)->Option<usize>{
        
        let n_vars = self.variable_count[self.scope_depth()-1];
        
        self.search_var(var_name, n_vars)
    }

    pub fn find_var(&self, var_name: &String)->Option<usize>{
        self.search_var(var_name, self.variables.len())        
    }
    

    pub fn push_variable(&mut self, name: String, v: Value){
        // Register variable in the parser
        self.variables.push(name);


        // Register the variable in the VM
        self.emit_byte(Operation::PushVar(v));
    
        // Take note of the variable registration
        let depth = self.variable_count.len();
        self.variable_count[depth-1] += 1;

    }

    /*
    pub fn consume_previous(&mut self, expected_type: TokenType)->bool{
        if self.previous.token_type() == expected_type {
            self.advance();
            return true;
        }        
        false
    }
    */

    pub fn check(&self, t: TokenType)->bool{
        self.current.token_type() == t
    }

    
    fn match_token(&mut self, t: TokenType) -> bool{
        if !self.check(t) {
            return false;
        }
        self.advance();
        true
    }
    

    pub fn emit_byte(&mut self, op: Operation){
        match &mut self.current_function{
            Some(f)=>f.mut_chunk().write_operation(op, self.previous.line()),
            None => self.error_no_current_function()                
        }        
    }
    

    pub fn source(&self)->&Vec<u8>{
        self.scanner.source()
    }

        
    pub fn get_rule(&self, ttype: TokenType)->ParseRule{
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
            }   
            _ => {
                eprintln!(" ===> {}",debug::token_type(ttype));
                unimplemented!()
            }
            /*
            */
        }
    }

    pub fn parse_precedence(&mut self, precedence: Precedence){          
        self.advance();
        let rule = match self.get_rule(self.previous.token_type()).prefix {
            Some(r) => r,
            None =>{
                self.error_at_current(format!("Expecting expression.")); 
                return;
            }
        };

        // Run the rule
        rule(self);

        while precedence <= self.get_rule(self.current.token_type()).precedence {
            self.advance();
            match self.get_rule(self.previous.token_type()).infix{
                Some(r)=>r(self),
                None => self.internal_error_at_current(format!("No infix rule!"))
            }
        }                
    }

    
    /// Compiles a program, returns a boolean
    /// indicating if it worked.
    ///     
    /// # EBNF Grammar
    /// program -> declaration* EOF
    pub fn compile(&mut self) -> Option<Box<ScriptFn>> {            
        // Prime the pump
        self.advance();
        //self.advance();

        while !self.match_token(TokenType::EOF){
            self.declaration();
        }
                

        if self.had_error{
            #[cfg(debug_assertions)]
            {
                match &self.current_function{
                    Some(f)=>{
                        let ch = f.chunk();
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
    fn declaration(&mut self){    
        self.show_tokens("declaration() - start");
        match self.current.token_type(){
            TokenType::Class => {
                self.advance();
                unimplemented!();
            },
            TokenType::Function => {
                self.advance();
                self.fn_declaration();
            },
            TokenType::Let => {
                self.advance();
                let mut n : usize= 0;
                self.var_declaration(&mut n);
            },
            _ => {                
                self.statement();
            }
        }
    }

    /// Compiles a function
    /// 
    /// # EBNF Grammar
    /// function -> fn IDENTIFIER (varlist) BLOCK
    fn fn_declaration(&mut self ){
        // fn has been consumed.
        let func_name = if self.consume(TokenType::Identifier){
            self.previous.source_text(self.source())
        }else{
            return self.error_at_current(format!("Expecting identifier after 'let'. Found '{}'",self.previous.source_text(self.source()) ))
        };

        let mut func = match function(self, &func_name){
            Some(f)=>f,
            None => return
        };

        // Push constant.
        let i = self.push_to_heap(Box::new(Function::Script(func)));

        // Register the function
        self.push_variable(func_name, Value::HeapRef(i));
    }

    /// Compiles a Variable declaration    
    ///     
    /// # EBNF Grammar
    /// var_declaration -> "let" IDENTIFIER ("=" expression)
    pub fn var_declaration(&mut self, n_declared_vars : &mut usize){        
        self.show_tokens("var_declaration - Start");
        
        let var_name = if self.consume(TokenType::Identifier){
            self.previous.source_text(self.source())
        }else{
            return self.error_at_current(format!("Expecting identifier after 'let'. Found '{}'",self.previous.source_text(self.source()) ))
        };
        
        self.show_tokens("var_declaration - After getting Var_Name");

        // Check if variable is the current scope            
        if self.current_scope_has_variable(&var_name){
            return self.error_at_current(format!("Redeclaration of variable '{}'",var_name));
        }
    
        self.show_tokens("var_declaration - After current scope");

        // All good to go. Create default values
        let typed = false;        
        let initialized = false;

        
        // Register Variable as NIL... define later
        self.push_variable(var_name, Value::Nil);


        // Count the declared variable
        *n_declared_vars+=1;        

        self.show_tokens("var_declaration - Before defining");
        // Define it if needed
        if self.match_token(TokenType::Equal){                        
            // Put value of expression on the stack                        
            self.expression();                        
            self.emit_byte(Operation::DefineVar(self.variables.len()-1));        
        }

        // Check if there is another variable afterwards
        if self.consume(TokenType::Comma) {
            self.var_declaration(n_declared_vars);
        }    

        self.show_tokens("var_declaration - Before leaving");        
    }

    /// Compiles a while statement
    /// 
    /// # EBNF Grammar:
    /// while_statement -> while EXPRESSION BLOCK
    fn while_statement(&mut self){
        let while_start = match self.chunk_len(){
            Some(i)=>i,
            None => return
        };

        // Compile expression (puts a boolean on the stack)
        self.expression();
        
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
        self.begin_scope();
        self.block();        
        self.end_scope();

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
    fn if_statement(&mut self){
        
        // Compile expression (puts a boolean on the stack)
        self.expression();

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
        self.begin_scope();
        self.block();        
        self.end_scope();

        // Mark the end
        let body_end = match self.chunk_len(){
            Some(i)=>i,
            None => return
        };

        // Patch jump
        let body_length = body_end - body_start;
        //self.chunk().patch_code(body_start-1 ,Operation::JumpIfFalse(body_length+1)); 
        self.patch_chunk(body_start-1, Operation::JumpIfFalse(body_length+1)); 

        // check else
        if self.consume(TokenType::Else){
            // This is patched after the statement is processed
            self.emit_byte(Operation::JumpIfTrue(0));
            let body_start = match self.chunk_len(){
                Some(i)=>i,
                None => return
            };
            self.statement();

            // Mark the end
            let body_end = match self.chunk_len(){
                Some(i)=>i,
                None => return
            };

            // Patch jump
            let body_length = body_end - body_start;
            //self.chunk().patch_code(body_start-1 ,Operation::JumpIfTrue(body_length+1)); 
            self.patch_chunk(body_start-1, Operation::JumpIfTrue(body_length+1));
        }

    }

    /// Compiles a for statement
    /// 
    /// # EBNF Grammar:
    /// for_statement -> for IDENTIFIER in EXPRESSION  BLOCK
    fn for_statement(&mut self){
                
        // Open the main scope for this for statement.
        self.begin_scope();

        // consume declare the variables
        self.show_tokens("for_statement - Before VAR Declaration");
        let mut n_declared_vars : usize = 0;
        self.var_declaration(&mut n_declared_vars);        
        self.show_tokens("for_statement - AFTER VAR Declaration");
        println!(" ----> Declared {}",n_declared_vars);
        
        // consume 'in', or fail
        if !self.consume(TokenType::In) {
            return self.error_at_current(format!("Expecting keyword 'in' when declaring For loop."));
        }
        self.show_tokens("for_statement - Before Expression()");
        
        // Evaluate the value to iterate over... put it at the end 
        // of the stack.
        self.expression();
        self.show_tokens("for_statement - AFTER Expression()");

        
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
        self.begin_scope();
        self.block();        
        self.end_scope();

        // Mark the end of the scope
        let body_end = match self.chunk_len(){
            Some(i)=>i,
            None => return
        };
        
        // Close the main scope
        self.end_scope();

        // emit Loop operation
        let body_length = body_end - body_start;
        self.emit_byte(Operation::ForLoop(n_declared_vars, body_length))


    }

    /// Compiles a statement
    /// 
    /// # EBNF Grammar:
    /// statement -> expression | forStmt | ifStmt | returnStmt | whileStmt| block ;
    fn statement(&mut self){  
                
        match self.current.token_type(){
            TokenType::LeftBrace => {
                self.show_tokens("statement - LeftBrace before advance()");
                self.advance();
                self.show_tokens("statement - LeftBrace after advance()");
                self.begin_scope();
                self.block();
                self.show_tokens("statement - after block()");
                self.end_scope();
            },
            TokenType::For => {
                self.advance();
                self.for_statement();
            },
            TokenType::If =>{
                self.advance();
                self.if_statement();
            },
            TokenType::Return =>{
                self.advance();
                unimplemented!();
            },
            TokenType::While =>{
                self.advance();
                self.while_statement();
            },
            TokenType::EOF=>{
                return
            },
            _ => self.expression()
        }
    }


    /// Compiles an expression
    /// # EBFN Grammar:
    /// expression -> literal | unary | binary | grouping;
    pub fn expression(&mut self){                    
        self.parse_precedence(Precedence::Assignment);
    }


    /// Opens a scope, after a Left Brace
    pub fn begin_scope(&mut self){          
        // add an empty variable counter                
        self.variable_count.push(0)        
    }

    /// Closes a scope and emits all the necessary
    /// PopVar operations, removing the local variables
    pub fn end_scope(&mut self){           
        if let Some(n) = self.variable_count.pop(){
            // Pop variables from parser stack
            for _ in 0..n{
                if let Some(_) = self.variables.pop(){

                }else{
                    panic!("Trying to pop variables from empty scope");
                }
            }

            // Emit operation to clean the VM.
            if n > 0 || !self.optimize {
                self.emit_byte(Operation::PopVars(n));
            }

        }else{
            self.internal_error_at_current(format!("Trying to end a scope but there are no scopes"))
        }        
    }

    
    /// Parses a block
    /// 
    /// # EBNF Grammar:
    /// block -> "{" declaration* "}"
    pub fn block(&mut self){ 
        // LEFT BRACE HAS BEEN CONSUMED ALREADY
        println!("====== <<<< BEGIN");
        

        self.show_tokens("block - before loop");       
        while !self.check(TokenType::RightBrace) && !self.check(TokenType::EOF){
            self.declaration();
        }
        self.show_tokens("block - after loop");
        if !self.consume(TokenType::RightBrace){
            return self.error_at_current(format!("Expecting '}}' after block."));            
        }

        self.show_tokens("block end (if not error)");
        println!("====== <<<< END");
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
            _ => eprint!(" at '{}'", token.source_text(self.scanner.source()))
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
            _ => eprint!(" at '{}'", token.source_text(self.scanner.source()))
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
        let mut parser = Parser::new(&source);
        
        parser.advance();
        parser.advance();

        match parser.previous.token_type(){
            TokenType::Number => {},
            _ => {panic!("Expecting Number, found {}", debug::token(parser.previous, parser.scanner.source()))}
        }
        
        number(&mut parser);        
        if let Operation::PushNumber(found) = parser.chunk().unwrap().code().last().unwrap() {            
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
            _ => {panic!("Expecting Number, found {}", debug::token(parser.previous, parser.scanner.source()))}
        }
        
        number(&mut parser);
        number(&mut parser);        
        if let Operation::PushNumber(found) = parser.chunk().unwrap().code().last().unwrap() {            
            assert_eq!(2.1,*found);            
        }else{
            assert!(false);
        }

    }

    #[test]
    fn test_add_expression(){
        let raw_source = "= !( 5 - 4 > 3 * 22 == !false) ".to_string();
        let source : Vec<u8> = raw_source.into_bytes();

        let mut parser = Parser::new(&source);
        parser.compile();
        debug::chunk(&parser.chunk().unwrap(), format!("some chunck"));

        let mut vm = VM::new();
        let (code,lines)=parser.chunk().unwrap().to_slices();        

        vm.run(code, lines);
        if let Ok(v) = vm.pop(){
            if let Value::Bool(b) = v {
                assert!(b);
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

        let mut parser = Parser::new(&source);
        parser.compile();
        assert!(!parser.had_error);
        

        // Do it slowly now

        let mut parser = Parser::new(&source);
        // prime the pump (done in self.compile())
        parser.advance();

        /* DECLARE X */

        // previous is LET, consume.
        // (done in self.declaration())
        parser.advance();
        
        let mut n : usize= 0;
        parser.var_declaration(&mut n);
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
        parser.var_declaration(&mut n);
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

        let mut parser = Parser::new(&source);
        if let Some(f) = parser.compile(){
            assert_eq!(3,parser.variables.len());
        
            assert_eq!(parser.variables[0],"x".to_string());                        
            assert_eq!(parser.variables[1],"y".to_string());        
            assert_eq!(parser.variables[2],"z".to_string());        
            
            debug::chunk(f.chunk(),"the_chunk".to_string());
        }else{
            assert!(false)
        }

        
    }


    #[test]
    fn test_var_list_declaration(){
        let raw_source = "let x = 2, y, z = true".to_string();
        let source : Vec<u8> = raw_source.into_bytes();

        // 
        let mut parser = Parser::new(&source);
        parser.advance();parser.advance();
        let mut n : usize = 0;
        parser.var_declaration(&mut n);
        assert_eq!(n,3);


        let mut parser = Parser::new(&source);
        if let Some(f) = parser.compile(){
            debug::chunk( f.chunk() ,"the_chunk".to_string());
        }else{
            assert!(false)
        }

        assert_eq!(3,parser.variables.len());
        
        assert_eq!(parser.variables[0],"x".to_string());                
        assert_eq!(parser.variables[1],"y".to_string());
        assert_eq!(parser.variables[2],"z".to_string());
        
        
        
    }

    #[test]
    fn test_current_scope_has_variable(){
        let raw_source = "let x let x2 {let y} let z".to_string();
        let source : Vec<u8> = raw_source.into_bytes();

        let mut parser = Parser::new(&source);
        parser.compile();
        assert!(parser.current_scope_has_variable(&format!("x")));
        assert!(!parser.current_scope_has_variable(&format!("y")));
    }

    #[test]
    fn test_block(){
        let raw_source = "let x { let y } let z".to_string();
        let source : Vec<u8> = raw_source.into_bytes();

        let mut parser = Parser::new(&source);
        parser.compile();
        assert!(!parser.had_error);

        // Now do it more slowly

        // prime the pump (done in self.compile())
        let mut parser = Parser::new(&source);
        parser.advance();

        
        /* DECLARE X */

        // Check if there is a let, and consume it.
        assert!(parser.match_token(TokenType::Let));        
        let mut n : usize= 0;
        parser.var_declaration(&mut n);
        assert_eq!(n,1);

        // Check that the next one is {, and consume it
        assert!(parser.match_token(TokenType::LeftBrace));       
                

        /* CONSUME BLOCK */
        parser.block();

        // declare Z (check that there is a LET, and consume it first)
        assert!(parser.match_token(TokenType::Let));       
        let mut n : usize= 0;
        parser.var_declaration(&mut n);
        assert_eq!(n,1);
    }

    #[test]
    fn test_scopes(){
        let raw_source = "let x { let y } let z".to_string();
        let source : Vec<u8> = raw_source.into_bytes();

        let mut parser = Parser::new(&source);
        
        
        
        // Check empty variables   
        assert_eq!(parser.scope_depth(),1);     
        assert_eq!(parser.variables.len(),0);
        assert_eq!(parser.variable_count.len(),1);
        assert_eq!(parser.variable_count[0],0);
        

        // Prime the pump... this is done by compile()
        parser.advance();

        // Consume the first variable  
        parser.declaration();  
        assert!(!parser.had_error); 
         
        
        // There is only the "main" scope
        assert_eq!(parser.scope_depth(),1);     
        assert_eq!(parser.variables.len(),1);
        assert_eq!(parser.variables[0],"x".to_string());
        assert_eq!(parser.variable_count.len(),1);
        assert_eq!(parser.variable_count[0],1);

        if let Operation::PushVar(Value::Nil) = parser.chunk().unwrap().code()[0]{
            assert!(true);
        }else{assert!(false)};


        // Consume the block, with the variable                        
        assert!(parser.consume(TokenType::LeftBrace));        
        parser.begin_scope();
        parser.block();   
        assert!(!parser.had_error);             
        
        // There is the "main" scope, and the new one
        assert_eq!(parser.scope_depth(),2);     
        assert_eq!(parser.variables.len(),2);
        assert_eq!(parser.variables[1],"y".to_string());
        assert_eq!(parser.variable_count.len(),2);
        assert_eq!(parser.variable_count[0],1);
        assert_eq!(parser.variable_count[1],1);

        
        if let Operation::PushVar(Value::Nil) = parser.chunk().unwrap().code()[1]{
            assert!(true);
        }else{assert!(false)};

        // Pop scope
        parser.end_scope();
        
        assert_eq!(parser.scope_depth(),1);     
        assert_eq!(parser.variables.len(),1);        
        assert_eq!(parser.variable_count.len(),1);
        assert_eq!(parser.variable_count[0],1);
        // y should not be there any more.
        assert!(!parser.current_scope_has_variable(&"y".to_string()));        
        

        // Consume what is after the block, with the variable                
        parser.declaration();                
        
        // There is the "main" scope, and the new one
        assert_eq!(parser.scope_depth(),1);     
        assert_eq!(parser.variable_count.len(),1);
        assert_eq!(parser.variables.len(),2);
        assert_eq!(parser.variables[1],"z".to_string());
        assert_eq!(parser.variable_count[0],2);
        if let Operation::PushVar(Value::Nil) = parser.chunk().unwrap().code()[1]{
            assert!(true);
        }else{assert!(false)};
        
        debug::chunk(&parser.chunk().unwrap(), format!("some chunck"));
    }

    #[test]
    fn test_if(){
        let raw_source = "if 3 { }else if 4{}".to_string();
        let source : Vec<u8> = raw_source.into_bytes();

        let mut parser = Parser::new(&source);
        if let Some(f) = parser.compile(){
            // Check the operations...
        
            // Push 3
            if let Operation::PushNumber(v) = f.chunk().code()[0]{
                assert_eq!(3.0,v);
            }else{assert!(false)};

            // set jump.
            if let Operation::JumpIfFalse(v) = f.chunk().code()[1]{
                assert_eq!(2,v);
            }else{assert!(false)};

            // Pop variables.
            if let Operation::PopVars(n) = f.chunk().code()[2]{
                assert_eq!(0,n);
            }else{assert!(false)};

            

            debug::chunk(f.chunk(), format!("for loop chunck"));
        }else{
            assert!(false)
        }

        
    }

    #[test]
    fn test_while_loop(){
        let raw_source = "while 3 { }".to_string();
        let source : Vec<u8> = raw_source.into_bytes();

        let mut parser = Parser::new(&source);
        
        if let Some(f) = parser.compile(){
    // Check the operations...
            
            // Push 3
            if let Operation::PushNumber(v) = f.chunk().code()[0]{
                assert_eq!(3.0,v);
            }else{assert!(false)};

            // set jump.
            if let Operation::JumpIfFalse(v) = f.chunk().code()[1]{
                assert_eq!(3,v);
            }else{assert!(false)};

            // Pop variables.
            if let Operation::PopVars(n) = f.chunk().code()[2]{
                assert_eq!(0,n);
            }else{assert!(false)};

            // Jump back.
            if let Operation::JumpBack(n) = f.chunk().code()[3]{
                assert_eq!(3,n);
            }else{assert!(false)};

            debug::chunk(f.chunk(), format!("for loop chunck"));
        }else{
            assert!(false)
        }
        

        
    }

    #[test]
    fn test_for_loop(){
        let raw_source = "for i,j in 3 {}".to_string();
        let source : Vec<u8> = raw_source.into_bytes();

        let mut parser = Parser::new(&source);
        if let Some(f) = parser.compile(){
            // Check the operations...
            
            // Push i
            if let Operation::PushVar(_) = f.chunk().code()[0]{
                assert!(true);
            }else{assert!(false)};

            // Push j
            if let Operation::PushVar(_) = f.chunk().code()[1]{
                assert!(true);
            }else{assert!(false)};

            // Push 3
            if let Operation::PushNumber(v) = f.chunk().code()[2]{
                assert_eq!(3.0,v);
            }else{assert!(false)};

            // ... body happens

            // Pop vars from body        
            if let Operation::PopVars(_) = f.chunk().code()[3]{
                assert!(true);
            }else{assert!(false)};

            // Pop vars from main scope        
            if let Operation::PopVars(_) = f.chunk().code()[4]{
                assert!(true);
            }else{assert!(false)};

            // FOR LOOP
            if let Operation::ForLoop(n_vars,length) = f.chunk().code()[5]{
                assert_eq!(n_vars,2);
                assert_eq!(length,1); // The PopVars operation
            }else{assert!(false)};

            debug::chunk(f.chunk(), format!("for loop chunck"));
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
        if let Some(f) = parser.compile(){
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
    fn test_function_expression(){
        let raw_source = "let x = fn(b,c) { }".to_string();
        let source : Vec<u8> = raw_source.into_bytes();

        let mut parser = Parser::new(&source);        
        if let Some(f) = parser.compile(){
            assert!(!parser.had_error);

            debug::chunk(f.chunk(), format!("function expression chunck"));

            let ch = f.chunk().code();
            if let Operation::PushFunction(v) = &ch[1] {
                match v {
                    Function::Script(s)=>{
                        assert_eq!(s.n_args,2);
                        debug::chunk(s.chunk(), format!("fn {}()", s.name));
                    },
                    Function::Rust(_)=>assert!(false)
                }
            }else {
                panic!(" Found -> '{}'",&parser.variables[0].to_string())
            }
        }else{
            assert!(false)
        }
        
        
    }


    #[test]
    fn test_function_delaration(){
        let raw_source = "fn x(a,b,c) { }".to_string();
        let source : Vec<u8> = raw_source.into_bytes();

        let mut parser = Parser::new(&source);        
        if let Some(f) = parser.compile(){
            assert!(!parser.had_error);

            debug::chunk(f.chunk(), format!("function expression chunck"));
        }else{
            assert!(false)
        }
        
                
    }
}