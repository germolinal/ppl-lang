use crate::debug::*;
use crate::scanner::*;
use crate::operations::*;
use crate::token::*;
use crate::chunk::*;
use crate::values::*;
use crate::parse_function::*;
use crate::variable::*;

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
    chunk: Chunk<'a>,
    
    variables: Vec<NamedVar<'a>>,
    variable_count: Vec<usize>,
}

impl <'a>Parser<'a>{

    pub fn new(source: &'a Vec<u8>)->Self{
        let scanner = Scanner::new(source);
        let previous = Token::new(&scanner,TokenType::EOF);
        let current = Token::new(&scanner,TokenType::EOF);
        Self{
            scanner: scanner,
            had_error: false,
            panic_mode: false,
            current: current,
            previous: previous,
            chunk: Chunk::new(),

            variables: vec![],
            variable_count: vec![0],
        }
    }

    /* UTILITY FUNCTIONS */

    fn scope_depth(&self)->usize{
        self.variable_count.len()
    }    

    fn current_scope_has_variable(&self, var_name: &String)->bool{
        let n = self.variable_count[self.scope_depth()-1];
        let fin = self.variables.len();
        let ini = fin - n;
        for i in ini..fin{
            if var_name == &self.variables[i].name {
                return true
            }
        }
        false
    }
    
    pub fn previous(&self)->Token{
        self.previous
    }
    
    fn advance(&mut self){
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

    fn check(&self, t: TokenType)->bool{
        self.current.token_type() == t
    }

    
    fn match_token(&mut self, t: TokenType) -> bool{
        if !self.check(t) {
            return false;
        }
        self.advance();
        true
    }
    

    pub fn emit_byte(&mut self, op: Operation<'a>){
        self.chunk.write_operation(op, self.previous.line());
    }
    /*
    fn end_compiler(&mut self){
        self.emit_return();
    }
    
    fn emit_return(&mut self){
        self.emit_byte(Operation::Return);
    }
    */

    /*
    pub fn add_constant(&mut self,v: Value<'a>)->usize{
        self.chunk.add_constant(v)
    }
    */

    pub fn source(&self)->&Vec<u8>{
        self.scanner.source()
    }

        
    pub fn get_rule(&self, ttype: TokenType)->ParseRule{
        match ttype{
            TokenType::RightParen | 
            TokenType::LeftBrace | TokenType::RightBrace |
            TokenType::Comma | TokenType::Semicolon |
            TokenType::Equal |
            TokenType::Class | 
            TokenType::Else |
            TokenType::For | 
            TokenType::Function | 
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
                    infix:None,//Some(call),
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
            }
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
            /*
            TokenType::True |TokenType::False | TokenType::Nil => {
                ParseRule{
                    precedence: Precedence::None,
                    next_precedence: Some(Precedence::Assignment),
                    prefix: Some(literal),
                    infix: None,
                }
            },
            */
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
            _ => {
                eprintln!(" ===> {}",debug::token_type(ttype));
                unimplemented!()
            }
        }
    }

    pub fn parse_precedence(&mut self, precedence: Precedence){          
        self.advance();
        let rule = match self.get_rule(self.previous.token_type()).prefix {
            Some(r)=>{
                r
            },
            None =>{self.error_at_current(format!("Expecting expression.")); return;}
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
    pub fn program(&mut self) -> bool {            
        // Prime the pump
        self.advance();
        self.advance();

        while !self.match_token(TokenType::EOF){
            self.declaration();
        }
                

        return !self.had_error;
    }

    /// Compiles a declaration    
    ///     
    /// # EBNF Grammar
    /// declaration -> classDecl | funDecl | varDecl | statement
    fn declaration(&mut self){        
        match self.previous.token_type(){
            TokenType::Class => {
                self.advance();
                unimplemented!();
            },
            TokenType::Function => {
                self.advance();
                unimplemented!();
            },
            TokenType::Let => {
                self.advance();
                self.var_declaration();
            },
            _ => {                
                self.statement();
            }
        }
    }

    /// Compiles a Variable declaration    
    ///     
    /// # EBNF Grammar
    /// var_declaration -> "let" IDENTIFIER (":" TYPE ) ("=" expression)
    fn var_declaration(&mut self){        
        // Name is previous.
        let var_name = if let TokenType::Identifier = self.previous.token_type(){            
            self.previous.source_text(self.source())
        }else{
            return self.error_at_current(format!("Expecting identifier after 'let'. Found '{}'",self.previous.source_text(self.source()) ))
        };

        // Check if variable is the current scope            
        if self.current_scope_has_variable(&var_name){
            return self.error_at_current(format!("Redeclaration of variable '{}'",var_name));
        }
    
        // All good to go. Create default values
        let var_type = ValueType::Nil;
        let val = Value::Nil;

        // Check if it has a type                        
        if let TokenType::Colon = self.current.token_type(){
            // let x : String            
            unimplemented!();
            // Expect type, consume
        }
                
        // Register variable in the parser
        self.variables.push(NamedVar{
            name: var_name,
            value: val,
            value_type: var_type,
        });
        let depth = self.variable_count.len();
        self.variable_count[depth-1] += 1;

        // Emit operation for the VM.
        self.emit_byte(Operation::PushVar(Var{
            value: val,
            value_type: var_type
        }));


        // Define it if needed
        if self.match_token(TokenType::Equal){            
            // Put value of expression on the stack                        
            self.expression();            

            self.emit_byte(Operation::DefineVar(self.variables.len()-1))            
        }




        // Leave the semicolon behind                    
        if !self.consume(TokenType::Semicolon){
            return self.error_at_current(format!("Expecting ';' after variable declaration."));
        }        

        // Make the semicolon disappear...
        self.advance(); 
    }

    /// Compiles a statement
    /// 
    /// # EBNF Grammar:
    /// statement -> expression | forStmt | ifStmt | returnStmt | whileStmt| block ;
    fn statement(&mut self){  
        
        println!("previous {} | current {}",debug::token(self.previous, self.scanner.source()),debug::token(self.current, self.scanner.source()));        

        match self.previous().token_type(){
            TokenType::LeftBrace => {
                self.advance();
                self.begin_scope();
                self.block();
                self.end_scope();
            },
            TokenType::For =>{
                self.advance();
                unimplemented!();
            },
            TokenType::If =>{
                self.advance();
                unimplemented!();
            },
            TokenType::Return =>{
                self.advance();
                unimplemented!();
            },
            TokenType::While =>{
                self.advance();
                unimplemented!();
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
    fn begin_scope(&mut self){          
        // add an empty variable counter                
        self.variable_count.push(0)        
    }

    /// Closes a scope and emits all the necessary
    /// PopVar operations, removing the local variables
    fn end_scope(&mut self){           
        if let Some(n) = self.variable_count.pop(){
            // Pop variables from parser stack
            for _ in 0..n{
                if let Some(_) = self.variables.pop(){

                }else{
                    panic!("Trying to pop variables from empty scope");
                }
            }

            // Emit operation to clean the VM.
            self.emit_byte(Operation::PopVars(n));

        }else{
            self.internal_error_at_current(format!("Trying to end a scope but there are no scopes"))
        }        
    }

    
    /// Parses a block
    /// 
    /// # EBNF Grammar:
    /// block -> "{" declaration* "}"
    fn block(&mut self){
        while !self.check(TokenType::RightBrace) && !self.check(TokenType::EOF){
            self.declaration();
        }
        if !self.consume(TokenType::RightBrace){
            self.error_at_current(format!("Expecting '}}' after block."));
        }
    }

    

    /* ERROR FUNCTIONS */

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
    use crate::debug::*;

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
        if let Operation::PushNumber(found) = parser.chunk.code().last().unwrap() {            
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
        if let Operation::PushNumber(found) = parser.chunk.code().last().unwrap() {            
            assert_eq!(2.1,*found);            
        }else{
            assert!(false);
        }

    }

    #[test]
    fn test_add_expression(){
        let raw_source = "!(5 - 4 > 3 * 2 2 == !nil 3*2".to_string();
        let source : Vec<u8> = raw_source.into_bytes();

        let mut parser = Parser::new(&source);
        parser.program();
        debug::chunk(&parser.chunk, format!("some chunck"));
    }

    #[test]
    fn test_var_declaration(){
        let raw_source = "let x = 2; let y; let z = 5;".to_string();
        let source : Vec<u8> = raw_source.into_bytes();

        let mut parser = Parser::new(&source);
        parser.program();

        assert_eq!(3,parser.variables.len());
        assert_eq!(parser.variables[0].name,"x".to_string());
        assert_eq!(parser.variables[1].name,"y".to_string());
        assert_eq!(parser.variables[2].name,"z".to_string());
        
    }

    #[test]
    fn test_current_scope_has_variable(){
        let raw_source = "{let x; let y; let z;}".to_string();
        let source : Vec<u8> = raw_source.into_bytes();

        let mut parser = Parser::new(&source);
        parser.program();
        assert!(parser.current_scope_has_variable(&format!("x")));
    }

    #[test]
    fn test_scopes(){
        let raw_source = "let x; {let y;} let z;".to_string();
        let source : Vec<u8> = raw_source.into_bytes();

        let mut parser = Parser::new(&source);
        
        // Prime the pump... this is done by program()
        parser.advance();parser.advance();
        
        // Check empty variables   
        assert_eq!(parser.scope_depth(),1);     
        assert_eq!(parser.variables.len(),0);
        assert_eq!(parser.variable_count.len(),1);
        assert_eq!(parser.variable_count[0],0);
        

        // Consume the first variable           
        parser.declaration();        
        
        // There is only the "main" scope
        assert_eq!(parser.scope_depth(),1);     
        assert_eq!(parser.variables.len(),1);
        assert_eq!(parser.variables[0].name,"x".to_string());
        assert_eq!(parser.variable_count.len(),1);
        assert_eq!(parser.variable_count[0],1);
        if let Operation::PushVar(Var{
            value_type: ValueType::Nil,
            value: Value::Nil
        }) = parser.chunk.code()[0]{
            assert!(true);
        }else{assert!(false)};


        // Consume the block, with the variable                
        assert!(parser.consume(TokenType::LeftBrace));
        parser.advance(); 
        parser.begin_scope();
        parser.block();                
        
        // There is the "main" scope, and the new one
        assert_eq!(parser.scope_depth(),2);     
        assert_eq!(parser.variables.len(),2);
        assert_eq!(parser.variables[1].name,"y".to_string());
        assert_eq!(parser.variable_count.len(),2);
        assert_eq!(parser.variable_count[0],1);
        assert_eq!(parser.variable_count[1],1);
        if let Operation::PushVar(Var{
            value_type: ValueType::Nil,
            value: Value::Nil
        }) = parser.chunk.code()[1]{
            assert!(true);
        }else{assert!(false)};

        parser.end_scope();
        assert_eq!(parser.scope_depth(),1);     
        assert_eq!(parser.variables.len(),1);        
        assert_eq!(parser.variable_count.len(),1);
        assert_eq!(parser.variable_count[0],1);
        // y should not be there any more.
        assert!(!parser.current_scope_has_variable(&"y".to_string()));        
        parser.advance();

        // Consume what is after the block, with the variable                
        parser.declaration();                
        
        // There is the "main" scope, and the new one
        assert_eq!(parser.scope_depth(),1);     
        assert_eq!(parser.variable_count.len(),1);
        assert_eq!(parser.variables.len(),2);
        assert_eq!(parser.variables[1].name,"z".to_string());
        assert_eq!(parser.variable_count[0],2);
        if let Operation::PushVar(Var{
            value_type: ValueType::Nil,
            value: Value::Nil
        }) = parser.chunk.code()[1]{
            assert!(true);
        }else{assert!(false)};
        
        debug::chunk(&parser.chunk, format!("some chunck"));
    }
}