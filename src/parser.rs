use crate::debug::*;
use crate::scanner::*;
use crate::operations::*;
use crate::token::*;
use crate::chunk::*;
use crate::values::*;
use crate::parse_function::*;

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
        }
    }

    /* UTILITY FUNCTIONS */

    pub fn compile(&mut self) -> bool {        
        self.advance();

        self.expression();

        
        if !self.consume(TokenType::EOF){            
            self.error_at_current(format!("Expected end of expression"));
        }

        return !self.had_error;
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

    pub fn emit_byte(&mut self, op: Operation){
        self.chunk.write_operation(op, self.previous.line());
    }

    fn end_compiler(&mut self){
        self.emit_return();
    }

    fn emit_return(&mut self){
        self.emit_byte(Operation::Return);
    }

    pub fn add_constant(&mut self,v: Value<'a>)->usize{
        self.chunk.add_constant(v)
    }

    pub fn source(&self)->&Vec<u8>{
        self.scanner.source()
    }

        
    pub fn get_rule(&self, ttype: TokenType)->ParseRule{
        match ttype{
            TokenType::RightParen | 
            TokenType::LeftBrace | TokenType::RightBrace |
            TokenType::Comma | 
            TokenType::Equal |
            //TokenType::Class | 
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
            TokenType::True |TokenType::False | TokenType::Nil => {
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
                println!(" ===> {}",debug::token_type(ttype));
                unimplemented!()
            }
        }
    }

    
    pub fn expression(&mut self){
        self.parse_precedence(Precedence::Assignment);
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
        parser.compile();
        debug::chunk(&parser.chunk, format!("some chunck"));
    }
}