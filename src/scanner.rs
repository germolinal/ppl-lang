#[cfg(test)]
use crate::debug::*;

use crate::token::*;


pub struct Scanner<'a> {
    
    /// A pointer to the 
    start : *const u8, // Raw pointer

    current : *const u8, // Raw pointer

    line : usize,    

    source: &'a Vec<u8>,

    current_index: usize,

    start_index: usize,
}

impl <'a>Scanner<'a> {
    
    pub fn new(source : &'a Vec<u8>)->Self{
        Self {
            line: 1,            
            start: source.as_ptr(),            
            current: source.as_ptr(),
            source: source,
            current_index: 0,
            start_index: 0,
        }
    }

    pub fn line(&self)->usize{
        self.line
    }

    pub fn current_index(&self)->usize{
        self.current_index
    }

    pub fn start_index(&self)->usize{
        self.start_index
    }

    pub fn start(&self)-> *const u8{
        self.start
    }

    unsafe fn match_char(&mut self, expected : char)->bool{
        if self.is_at_end(){
            return false;
        }
        
        let c = *self.current as char;
        if c != expected {
            return false
        }

        self.current = self.current.add(1);
        return true;
        
        
    }

    unsafe fn advance (&mut self )->Option<char>{
        // Check if this is done
        if self.is_at_end() {
            return None;
        }
        let ret = *self.current;
        self.current = self.current.add(1);
        self.current_index += 1;
        return Some(ret as char);
    }

    fn is_at_end(&self)->bool{    
        self.current_index == self.source.len()
    }

    unsafe fn peek(&self)->char{
        *self.current as char
    }

    unsafe fn peek_next(&self)->char{
        if self.is_at_end(){
            return '\0';
        }
        return *self.current.clone().add(1) as char;
        
    }

    fn skip_white_space(&mut self){
        loop {            
            unsafe{
                match self.peek(){
                    ' ' => {self.advance().unwrap();},
                    '\r' => {self.advance().unwrap();},
                    '\t' => {self.advance().unwrap();},
                    '\n' => {
                        self.line += 1;
                        self.advance().unwrap();
                    },
                    '/' => {                        
                        if self.peek_next() == '/'{
                            // Single line comment
                            while self.peek() != '\n' && !self.is_at_end() {
                                self.advance().unwrap();
                            }

                        }else if self.peek_next() == '*'{
                            // Consume slash and star
                            self.advance();
                            self.advance();
                            // Block comment
                            loop {                                
                                // Check if it is end
                                if self.is_at_end(){                                    
                                    return;
                                }

                                // Check if end of blovk comment
                                if self.peek() == '*' && self.peek_next() == '/' {                                    
                                    // Consume slash and star
                                    self.advance();
                                    self.advance();
                                    return;
                                }
                                match self.advance().unwrap(){
                                    '\n' => {
                                        self.line += 1;
                                        self.advance().unwrap();
                                    },
                                    _ =>{}
                                };
                            }
                        }else{
                            return;
                        }
                    }
                    _ => return ()
                };
            }
        }
    }

    fn string(&mut self)->Token{        
        // Token will have this line reported
        let start_line = self.line;

        let mut next : char;
        unsafe{            
            next = self.peek();
        }

        // Advance as much as possible
        while next != '"' && !self.is_at_end(){                        
            if next == '\n' {
                self.line +=1 ;                
            }
            unsafe{
                next = match self.advance(){
                    Some(v) => v,
                    None => return Token::new(self,TokenType::Error)
                };
            }
        }
        

        return Token::new_with_line(self,TokenType::TokenString, start_line);
    }

    fn number(&mut self)->Token{        
        unsafe{
            // Scan the first part
            while self.peek().is_ascii_digit(){            
                self.advance();            
            }
            if self.peek()=='.' && self.peek_next().is_ascii_digit(){            
                // Consume the .            
                self.advance();
                while self.peek().is_ascii_digit() {                
                    self.advance();                
                }        
            }
        }

        Token::new(self,TokenType::Number)
        
    }

    fn identifier(&mut self)->Token{        
        unsafe{

            // scan the whole thing.            
            let mut c = self.peek();
            while c.is_ascii_alphabetic() || c.is_ascii_digit() || c == '_' {
                match self.advance(){
                    Some(_) => {c = self.peek()},
                    None => return Token::new(self,TokenType::EOF)
                }
            }

            let mut c = self.start.clone();
            match *c as char {
                'a' => { // break
                    if self.check_keyword("and"){
                        return Token::new(self,TokenType::And);
                    };
                },
                'b' => { // break
                    if self.check_keyword("break"){
                        return Token::new(self,TokenType::Break);
                    };
                },
                'e' => { // else
                    if self.check_keyword("else"){
                        return Token::new(self,TokenType::Else);
                    }
                },
                'f' => {  
                    c = c.add(1);
                    match *c as char {
                        'a' => {// false
                            if self.check_keyword("false"){
                                return Token::new(self,TokenType::False);
                            }
                        },                
                        'n' => {// fn
                            if self.check_keyword("fn"){
                                return Token::new(self,TokenType::Function);
                            }
                        },              
                        'o' => {// for
                            if self.check_keyword("for"){
                                return Token::new(self,TokenType::For);
                            }
                        },
                        _ => return Token::new(self,TokenType::Identifier)                        
                    }
                },
                'i' => {
                    c = c.add(1);
                    match *c as char {
                        'f' => {// if
                            if self.check_keyword("if"){
                                return Token::new(self,TokenType::If);
                            }
                        },                
                        'n' => {// in
                            if self.check_keyword("in"){
                                return Token::new(self,TokenType::In);
                            }
                        },
                        _ => return Token::new(self,TokenType::Identifier)                        
                    }
                },
                'l' => { // let
                    if self.check_keyword("let"){
                        return Token::new(self,TokenType::Let);
                    }
                },
                'n' => {//nil
                    if self.check_keyword("nil"){
                        return Token::new(self,TokenType::Nil);
                    }
                },
                'o' => {//nil
                    if self.check_keyword("or"){
                        return Token::new(self,TokenType::Or);
                    }
                },
                'r' => { // return
                    if self.check_keyword("return"){
                        return Token::new(self,TokenType::Return);
                    }
                },
                's' => {//self
                    if self.check_keyword("self"){
                        return Token::new(self,TokenType::TokenSelf);
                    }
                },
                't' => {//true
                    if self.check_keyword("true"){
                        return Token::new(self,TokenType::True);
                    }
                },
                'w' => {//while
                    if self.check_keyword("while"){
                        return Token::new(self,TokenType::While);
                    }
                },
                _ => return Token::new(self,TokenType::Identifier)                        
            }
            // If not a keyword,
            return Token::new(self,TokenType::Identifier);                        
        }
    }
    
    fn check_keyword(&self, word: &str)-> bool {
        
        let length = self.current_index - self.start_index;
        
        // If they are of different length, don't bother
        // checking... they are not the same word
        if word.len() != length {
            return false;
        }

        unsafe{            
            let mut c = self.start;            
            
            // For each character in keyword
            for ch in word.bytes() {                                                
                if *c != ch {
                    return false;
                }
                
                // Move one char ahead
                c = c.add(1);                
            }
        }// end of unsafe
        return true
    }

    pub fn scan_token(&mut self) -> Token {
        self.skip_white_space();
        self.start = self.current.clone();        
        self.start_index = self.current_index;

        let c : char; 
        unsafe {
            c = match self.advance(){
                Some(v)=>v,                
                None=>return Token::new(self, TokenType::EOF)
            };
        }

        // Alphabetic or underscore allowed
        if c.is_ascii_alphabetic() || c == '_'{
            return self.identifier();
        }  
        // 0..9 allowed
        if c.is_ascii_digit(){
            return self.number();
        }

        match c {
            
            // Single character
            '(' => Token::new(self, TokenType::LeftParen),
            ')' => Token::new(self, TokenType::RightParen),
            '{' => Token::new(self, TokenType::LeftBrace),
            '}' => Token::new(self, TokenType::RightBrace),
            ',' => Token::new(self, TokenType::Comma),
            '.' => Token::new(self, TokenType::Dot),
            '-' => Token::new(self, TokenType::Minus),
            '+' => Token::new(self, TokenType::Plus),
            ';' => Token::new(self, TokenType::Semicolon),
            '/' => Token::new(self, TokenType::Slash),
            '*' => Token::new(self, TokenType::Star),
            
            // Single or Double char
            '!' => {  
                unsafe{              
                    if self.match_char('=') {
                        Token::new(self,TokenType::BangEqual)
                    }else{
                        Token::new(self,TokenType::Bang)
                    }
                }                
            },
            '=' => {
                unsafe{
                    if self.match_char('=') {
                        Token::new(self,TokenType::EqualEqual)
                    }else{
                        Token::new(self,TokenType::Equal)
                    }
                }
            },
            '>' => {
                unsafe{
                    if self.match_char('=') {
                        Token::new(self,TokenType::GreaterEqual)
                    }else{
                        Token::new(self,TokenType::Greater)
                    }
                }
            },
            '<' => {
                unsafe{
                    if self.match_char('=') {
                        Token::new(self,TokenType::LessEqual)
                    }else{
                        Token::new(self,TokenType::Less)
                    }
                }
            }

            // String
            '"' => {return self.string();}

            // Error            
            _ => Token::new(self,TokenType::Error)
        }        
    }


    
}

/***********/
/* TESTING */
/***********/

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_advance(){
        let source = vec!['h' as u8, 'e' as u8, 'l' as u8,'l' as u8,'o' as u8];
        
        let mut scan = Scanner::new(&source);

        assert_eq!(scan.line,1);        
        unsafe{
            assert_eq!(*scan.start, source[scan.start_index]);
            assert_eq!(*scan.current, source[scan.current_index]);

            for i in 0..source.len() {
                
                assert_eq!(scan.current_index,i);
                assert_eq!(scan.current_index(),i);

                let c = match scan.advance(){
                    Some(v)=>v,
                    None => panic!("PANIC!!")
                };                

                assert_eq!(c, source[i] as char);



            }

            assert_eq!(scan.current_index,source.len());
            assert_eq!(scan.current_index(),source.len());
            match scan.advance(){
                Some(v)=>panic!("Retrieved {}... should not have", v),
                None => assert!(true)
            };
        }
    }

    #[test]
    fn test_scan_single_character(){
        let raw_source = "(){},.-+;*/".to_string();
        let source : Vec<u8> = raw_source.into_bytes();
        let mut scanner = Scanner::new(&source);


        let token = scanner.scan_token();
        match token.token_type() {
            TokenType::LeftParen => {
                assert_eq!(scanner.current_index,1);
                assert_eq!(scanner.start_index,0);
                debug::debug_token(token, &source);
                let txt = token.source_text(&source);                
                assert_eq!("(",txt);         
            },
            _ =>{panic!("Incorrect token ==> {}",debug::debug_token(token, &source))},
        };

        let token = scanner.scan_token();
        match token.token_type() {
            TokenType::RightParen => {
                assert_eq!(scanner.current_index,2);
                assert_eq!(scanner.start_index,1);
                debug::debug_token(token, &source);
                let txt = token.source_text(&source);                
                assert_eq!(")",txt);         
            },
            _ =>{panic!("Incorrect token ==> {}",debug::debug_token(token, &source))},
        };

        let token = scanner.scan_token();
        match token.token_type() {
            TokenType::LeftBrace => {
                assert_eq!(scanner.current_index,3);
                assert_eq!(scanner.start_index,2);
                let txt = token.source_text(&source);                
                assert_eq!("{",txt);         
            },
            _ =>{panic!("Incorrect token ==> {}",debug::debug_token(token, &source))},
        };

        let token = scanner.scan_token();
        match token.token_type() {
            TokenType::RightBrace => {
                assert_eq!(scanner.current_index,4);
                assert_eq!(scanner.start_index,3);
                let txt = token.source_text(&source);                
                assert_eq!("}",txt);         
            },
            _ =>{panic!("Incorrect token ==> {}",debug::debug_token(token, &source))},
        };

        let token = scanner.scan_token();
        match token.token_type() {
            TokenType::Comma => {
                assert_eq!(scanner.current_index,5);
                assert_eq!(scanner.start_index,4);
                let txt = token.source_text(&source);                
                assert_eq!(",",txt);         
            },
            _ =>{panic!("Incorrect token ==> {}",debug::debug_token(token, &source))},
        };

        let token = scanner.scan_token();
        match token.token_type() {
            TokenType::Dot => {
                assert_eq!(scanner.current_index,6);
                assert_eq!(scanner.start_index,5);
                let txt = token.source_text(&source);                
                assert_eq!(".",txt);         
            },
            _ =>{panic!("Incorrect token ==> {}",debug::debug_token(token, &source))},
        };
        
        let token = scanner.scan_token();
        match token.token_type() {
            TokenType::Minus => {
                assert_eq!(scanner.current_index,7);
                assert_eq!(scanner.start_index,6);
                let txt = token.source_text(&source);                
                assert_eq!("-",txt);         
            },
            _ =>{panic!("Incorrect token ==> {}",debug::debug_token(token, &source))},
        };

        let token = scanner.scan_token();
        match token.token_type() {
            TokenType::Plus => {
                assert_eq!(scanner.current_index,8);
                assert_eq!(scanner.start_index,7);
                let txt = token.source_text(&source);                
                assert_eq!("+",txt);         
            },
            _ =>{panic!("Incorrect token ==> {}",debug::debug_token(token, &source))},
        };

        let token = scanner.scan_token();
        match token.token_type() {
            TokenType::Semicolon => {
                assert_eq!(scanner.current_index,9);
                assert_eq!(scanner.start_index,8);
                let txt = token.source_text(&source);                
                assert_eq!(";",txt);         
            },
            _ =>{panic!("Incorrect token ==> {}",debug::debug_token(token, &source))},
        };

        

        let token = scanner.scan_token();
        match token.token_type() {
            TokenType::Star => {
                assert_eq!(scanner.current_index,10);
                assert_eq!(scanner.start_index,9);
                let txt = token.source_text(&source);                
                assert_eq!("*",txt);         
            },
            _ =>{panic!("Incorrect token ==> {}",debug::debug_token(token, &source))},
        };    
        
        let token = scanner.scan_token();        
        match token.token_type() {
            TokenType::Slash => {
                assert_eq!(scanner.current_index,11);
                assert_eq!(scanner.start_index,10);
                let txt = token.source_text(&source);                
                assert_eq!("/",txt);                                            
            },
            _ =>{panic!("Incorrect token ==> {}",debug::debug_token(token, &source))},
        };

        let token = scanner.scan_token();
        match token.token_type() {
            TokenType::EOF => {
                assert_eq!(scanner.current_index,11);
                assert_eq!(scanner.start_index,11);
                let txt = token.source_text(&source);                
                assert_eq!("",txt);                                            
            },
            _ =>{panic!("Incorrect token ==> {}",debug::debug_token(token, &source))},
        };
    }// End of test_single_char

    #[test]
    fn test_scan_one_or_two_chars(){
        let raw_source = "! != = == > >= < <=".to_string();
        let source : Vec<u8> = raw_source.into_bytes();
        let mut scanner = Scanner::new(&source);


        let token = scanner.scan_token();
        match token.token_type() {
            TokenType::Bang => {                
                debug::debug_token(token, &source);
            },
            _ =>{panic!("Incorrect token ==> {}",debug::debug_token(token, &source))},
        };
        let token = scanner.scan_token();
        match token.token_type() {
            TokenType::BangEqual => {                
                debug::debug_token(token, &source);
            },
            _ =>{panic!("Incorrect token ==> {}",debug::debug_token(token, &source))},
        };

        let token = scanner.scan_token();
        match token.token_type() {
            TokenType::Equal => {                
                debug::debug_token(token, &source);
            },
            _ =>{panic!("Incorrect token ==> {}",debug::debug_token(token, &source))},
        };
        let token = scanner.scan_token();
        match token.token_type() {
            TokenType::EqualEqual => {
                debug::debug_token(token, &source);
            },
            _ =>{panic!("Incorrect token ==> {}",debug::debug_token(token, &source))},
        };

        let token = scanner.scan_token();
        match token.token_type() {
            TokenType::Greater => {
                debug::debug_token(token, &source);
            },
            _ =>{panic!("Incorrect token ==> {}",debug::debug_token(token, &source))},
        };
        let token = scanner.scan_token();
        match token.token_type() {
            TokenType::GreaterEqual => {
                debug::debug_token(token, &source);
            },
            _ =>{panic!("Incorrect token ==> {}",debug::debug_token(token, &source))},
        };

        let token = scanner.scan_token();
        match token.token_type() {
            TokenType::Less => {
                debug::debug_token(token, &source);
            },
            _ =>{panic!("Incorrect token ==> {}",debug::debug_token(token, &source))},
        };
        let token = scanner.scan_token();
        match token.token_type() {
            TokenType::LessEqual => {
                debug::debug_token(token, &source);
            },
            _ =>{panic!("Incorrect token ==> {}",debug::debug_token(token, &source))},
        };
        
    }// end of test_scan_one_or_two_chars()

    #[test]
    fn test_scan_string(){
        /* SIMPLE CASE */
        let s = "automovil de banana".to_string();
        let raw_source = format!("(\"{}\"",s);
        let source : Vec<u8> = raw_source.into_bytes();
        let mut scanner = Scanner::new(&source);


        let token = scanner.scan_token();
        match token.token_type() {
            TokenType::LeftParen => {
                assert_eq!(scanner.current_index,1);
                assert_eq!(scanner.start_index,0);
                debug::debug_token(token, &source);
            },
            _ =>{panic!("Incorrect token ==> {}",debug::debug_token(token, &source))},
        };

        let token = scanner.scan_token();
        match token.token_type() {
            TokenType::TokenString => {                
                let mut txt = token.source_text(&source);
                txt.retain(|x| x != '"');
                assert_eq!(s,txt);                            
            },
            _ =>{panic!("Incorrect token ==> {}",debug::debug_token(token, &source))},
        };

        /* WITH NEWLINE INSIDE */

        let s = "automovil de\nbanana".to_string();
        let raw_source = format!("(\"{}\"",s);
        let source : Vec<u8> = raw_source.into_bytes();
        let mut scanner = Scanner::new(&source);

        let token = scanner.scan_token();
        match token.token_type() {
            TokenType::LeftParen => {
                assert_eq!(scanner.current_index,1);
                assert_eq!(scanner.start_index,0);
                debug::debug_token(token, &source);                
            },
            _ =>{panic!("Incorrect token ==> {}",debug::debug_token(token, &source))},
        };

        let token = scanner.scan_token();
        match token.token_type() {
            TokenType::TokenString => {                
                let mut txt = token.source_text(&source);
                txt.retain(|x| x != '"');
                assert_eq!(s,txt);  
                assert_eq!(token.line(),1);
                assert_eq!(scanner.line(),2);                          
            },
            _ =>{panic!("Incorrect token ==> {}",debug::debug_token(token, &source))},
        };



        /* WITH  WHITESPACE AT THE END */
        let s = "automovil de banana".to_string();
        let raw_source = format!("(\"{}\"    ",s);
        let source : Vec<u8> = raw_source.into_bytes();
        let mut scanner = Scanner::new(&source);


        let token = scanner.scan_token();
        match token.token_type() {
            TokenType::LeftParen => {
                assert_eq!(scanner.current_index,1);
                assert_eq!(scanner.start_index,0);
                debug::debug_token(token, &source);
            },
            _ =>{panic!("Incorrect token ==> {}",debug::debug_token(token, &source))},
        };

        let token = scanner.scan_token();
        match token.token_type() {
            TokenType::TokenString => {                
                let mut txt = token.source_text(&source);
                txt.retain(|x| x != '"');
                assert_eq!(s,txt);                            
            },
            _ =>{panic!("Incorrect token ==> {}",debug::debug_token(token, &source))},
        };
    }// end of test_scan_string()


    #[test]
    fn test_scan_comments(){
        

        // Left Paren, Right Paren, EOF
        let s = "automovil de murcielago".to_string();
        let raw_source = format!("(/*{}\n // Bat car*/)",s);
        let source : Vec<u8> = raw_source.into_bytes();
        let mut scanner = Scanner::new(&source);

        let token = scanner.scan_token();
        match token.token_type() {
            TokenType::LeftParen => {                                
                assert_eq!(token.line(),1);
                assert_eq!(scanner.line(),1);
            },
            _ =>{panic!("Incorrect token ==> {}",debug::debug_token(token, &source))},
        };

        let token = scanner.scan_token();
        match token.token_type() {
            TokenType::RightParen => { 
                assert_eq!(token.line(),2);
                assert_eq!(scanner.line(),2);                               
            },
            _ =>{panic!("Incorrect token ==> {}",debug::debug_token(token, &source))},
        };


        let token = scanner.scan_token();
        match token.token_type() {
            TokenType::EOF => {                                
            },
            _ =>{panic!("Incorrect token ==> {}",debug::debug_token(token, &source))},
        };


        
        // This should generate no tokens
        let s = "//automovil de carrera".to_string();
        let raw_source = format!("{}",s);
        let source : Vec<u8> = raw_source.into_bytes();
        let mut scanner = Scanner::new(&source);


        let token = scanner.scan_token();
        match token.token_type() {
            TokenType::EOF => {                
            },
            _ =>{panic!("Incorrect token ==> {}",debug::debug_token(token, &source))},
        };


        // LeftParen, String,Nothing else
        let s = "automovil de murcielago".to_string();
        let raw_source = format!("(\"{}\" // Bat car",s);
        let source : Vec<u8> = raw_source.into_bytes();
        let mut scanner = Scanner::new(&source);

        let token = scanner.scan_token();
        match token.token_type() {
            TokenType::LeftParen => {                                
            },
            _ =>{panic!("Incorrect token ==> {}",debug::debug_token(token, &source))},
        };

        let token = scanner.scan_token();
        match token.token_type() {
            TokenType::TokenString => {                
                let mut txt = token.source_text(&source);
                txt.retain(|x| x != '"');
                assert_eq!(s,txt);                            
            },
            _ =>{panic!("Incorrect token ==> {}",debug::debug_token(token, &source))},
        };

        let token = scanner.scan_token();
        match token.token_type() {
            TokenType::EOF => {                                
            },
            _ =>{panic!("Incorrect token ==> {}",debug::debug_token(token, &source))},
        };
        

    }// end of test_scan_string()

    #[test]
    fn test_scan_int(){
        let s = 123;
        let raw_source = format!("+{}/ //",s);
        let source : Vec<u8> = raw_source.into_bytes();
        let mut scanner = Scanner::new(&source);


        let token = scanner.scan_token();
        match token.token_type() {
            TokenType::Plus => {
                assert_eq!(scanner.current_index,1);
                assert_eq!(scanner.start_index,0);
                debug::debug_token(token, &source);
            },
            _ =>{panic!("Incorrect token ==> {}",debug::debug_token(token, &source))},
        };

        let token = scanner.scan_token();
        match token.token_type() {
            TokenType::Number => {                
                let txt = token.source_text(&source);                                
                assert_eq!(s,txt.parse::<i32>().unwrap());                            
            },
            _ =>{panic!("Incorrect token ==> {}",debug::debug_token(token, &source))},
        };

        let token = scanner.scan_token();
        match token.token_type() {
            TokenType::Slash => {                                
            },
            _ =>{panic!("Incorrect token ==> {}",debug::debug_token(token, &source))},
        };
    }// end of test_scan_int()

    #[test]
    fn test_scan_float(){
        let s = 123.321;
        let raw_source = format!("+{}/ //",s);
        let source : Vec<u8> = raw_source.into_bytes();
        let mut scanner = Scanner::new(&source);


        let token = scanner.scan_token();
        match token.token_type() {
            TokenType::Plus => {
                assert_eq!(scanner.current_index,1);
                assert_eq!(scanner.start_index,0);
                debug::debug_token(token, &source);
            },
            _ =>{panic!("Incorrect token ==> {}",debug::debug_token(token, &source))},
        };

        let token = scanner.scan_token();
        match token.token_type() {
            TokenType::Number => {                
                let txt = token.source_text(&source);                                
                assert_eq!(s,txt.parse::<f64>().unwrap());                            
            },
            _ =>{panic!("Incorrect token ==> {}",debug::debug_token(token, &source))},
        };

        let token = scanner.scan_token();
        match token.token_type() {
            TokenType::Slash => {                                
            },
            _ =>{panic!("Incorrect token ==> {}",debug::debug_token(token, &source))},
        };
    }// end of test_scan_float()


    #[test]
    fn test_check_keyword(){                
        let raw_source = format!("break and more elements");
        let source : Vec<u8> = raw_source.into_bytes();
        let mut scanner = Scanner::new(&source);

        scanner.scan_token();
        assert!(scanner.check_keyword("break"));                
        assert!(!scanner.check_keyword("brea"));
        assert!(!scanner.check_keyword("breaker"));
    }

    #[test]
    fn test_scan_identifier(){

        // and
        let raw_source = format!(" and more elements");
        let source : Vec<u8> = raw_source.into_bytes();
        let mut scanner = Scanner::new(&source);

        let token = scanner.scan_token();
        match token.token_type() {
            TokenType::And => {                                

            },
            _ =>{panic!("Incorrect token ==> {}",debug::debug_token(token, &source))},
        };

        // break
        let raw_source = format!("break and more elements");
        let source : Vec<u8> = raw_source.into_bytes();
        let mut scanner = Scanner::new(&source);

        let token = scanner.scan_token();
        match token.token_type() {
            TokenType::Break => {                                

            },
            _ =>{panic!("Incorrect token ==> {}",debug::debug_token(token, &source))},
        };

        // else
        let raw_source = format!(" else and more elements");
        let source : Vec<u8> = raw_source.into_bytes();
        let mut scanner = Scanner::new(&source);

        let token = scanner.scan_token();
        match token.token_type() {
            TokenType::Else => {                                

            },
            _ =>{panic!("Incorrect token ==> {}",debug::debug_token(token, &source))},
        };

        // false        
        let raw_source = format!(" false and more elements");
        let source : Vec<u8> = raw_source.into_bytes();
        let mut scanner = Scanner::new(&source);

        let token = scanner.scan_token();
        match token.token_type() {
            TokenType::False => {                                

            },
            _ =>{panic!("Incorrect token ==> {}",debug::debug_token(token, &source))},
        };

        // fn
        let raw_source = format!(" fn and more elements");
        let source : Vec<u8> = raw_source.into_bytes();
        let mut scanner = Scanner::new(&source);

        let token = scanner.scan_token();
        match token.token_type() {
            TokenType::Function => {                                

            },
            _ =>{panic!("Incorrect token ==> {}",debug::debug_token(token, &source))},
        };

        // for
        let raw_source = format!(" for and more elements");
        let source : Vec<u8> = raw_source.into_bytes();
        let mut scanner = Scanner::new(&source);

        let token = scanner.scan_token();
        match token.token_type() {
            TokenType::For => {                                

            },
            _ =>{panic!("Incorrect token ==> {}",debug::debug_token(token, &source))},
        };
        
        // if
        let raw_source = format!(" false and more elements");
        let source : Vec<u8> = raw_source.into_bytes();
        let mut scanner = Scanner::new(&source);

        let token = scanner.scan_token();
        match token.token_type() {
            TokenType::False => {                                

            },
            _ =>{panic!("Incorrect token ==> {}",debug::debug_token(token, &source))},
        };

        // in 

        let raw_source = format!(" in false and more elements");
        let source : Vec<u8> = raw_source.into_bytes();
        let mut scanner = Scanner::new(&source);

        let token = scanner.scan_token();
        match token.token_type() {
            TokenType::In => {                                

            },
            _ =>{panic!("Incorrect token ==> {}",debug::debug_token(token, &source))},
        };

        // let

        let raw_source = format!("let false and more elements");
        let source : Vec<u8> = raw_source.into_bytes();
        let mut scanner = Scanner::new(&source);

        let token = scanner.scan_token();
        match token.token_type() {
            TokenType::Let => {                                

            },
            _ =>{panic!("Incorrect token ==> {}",debug::debug_token(token, &source))},
        };

        // nil 
        let raw_source = format!("nil false and more elements");
        let source : Vec<u8> = raw_source.into_bytes();
        let mut scanner = Scanner::new(&source);

        let token = scanner.scan_token();
        match token.token_type() {
            TokenType::Nil => {                                

            },
            _ =>{panic!("Incorrect token ==> {}",debug::debug_token(token, &source))},
        };

        // or

        let raw_source = format!(" or false and more elements");
        let source : Vec<u8> = raw_source.into_bytes();
        let mut scanner = Scanner::new(&source);

        let token = scanner.scan_token();
        match token.token_type() {
            TokenType::Or => {                                

            },
            _ =>{panic!("Incorrect token ==> {}",debug::debug_token(token, &source))},
        };

        // return

        let raw_source = format!("return() false and more elements");
        let source : Vec<u8> = raw_source.into_bytes();
        let mut scanner = Scanner::new(&source);

        let token = scanner.scan_token();
        match token.token_type() {
            TokenType::Return => {                                

            },
            _ =>{panic!("Incorrect token ==> {}",debug::debug_token(token, &source))},
        };

        // self
        let raw_source = format!("self.this false and more elements");
        let source : Vec<u8> = raw_source.into_bytes();
        let mut scanner = Scanner::new(&source);

        let token = scanner.scan_token();
        match token.token_type() {
            TokenType::TokenSelf => {                                

            },
            _ =>{panic!("Incorrect token ==> {}",debug::debug_token(token, &source))},
        };

        // true
        let raw_source = format!("true not false and more elements");
        let source : Vec<u8> = raw_source.into_bytes();
        let mut scanner = Scanner::new(&source);

        let token = scanner.scan_token();
        match token.token_type() {
            TokenType::True => {                                

            },
            _ =>{panic!("Incorrect token ==> {}",debug::debug_token(token, &source))},
        };
        
        // while
        let raw_source = format!(" while and more elements");
        let source : Vec<u8> = raw_source.into_bytes();
        let mut scanner = Scanner::new(&source);

        let token = scanner.scan_token();
        match token.token_type() {
            TokenType::While => {                                

            },
            _ =>{panic!("Incorrect token ==> {}",debug::debug_token(token, &source))},
        };


        // identifiers
        let raw_source = format!(" w2hile aand mfore e3lements");
        let source : Vec<u8> = raw_source.into_bytes();
        let mut scanner = Scanner::new(&source);

        for _ in 0..4{
            let token = scanner.scan_token();
            match token.token_type() {
                TokenType::Identifier => {                                
    
                },
                _ =>{panic!("Incorrect token ==> {}",debug::debug_token(token, &source))},
            };
            
        }
        let token = scanner.scan_token();
        match token.token_type() {
            TokenType::EOF => {                                

            },
            _ =>{panic!("Incorrect token ==> {}",debug::debug_token(token, &source))},
        };

        // various
        let raw_source = format!(" while 2the_identifier.more e3lements");
        let source : Vec<u8> = raw_source.into_bytes();
        let mut scanner = Scanner::new(&source);

    
        // while
        let token = scanner.scan_token();
        match token.token_type() {
            TokenType::While => {                                
                
            },
            _ =>{panic!("Incorrect token ==> {}",debug::debug_token(token, &source))},
        };

        // 2
        let token = scanner.scan_token();
        match token.token_type() {
            TokenType::Number => {                                
                
            },
            _ =>{panic!("Incorrect token ==> {}",debug::debug_token(token, &source))},
        };

        //the_identifier
        let token = scanner.scan_token();
        match token.token_type() {
            TokenType::Identifier => {                                
                
            },
            _ =>{panic!("Incorrect token ==> {}",debug::debug_token(token, &source))},
        };
            
        // .
        let token = scanner.scan_token();
        match token.token_type() {
            TokenType::Dot => {                                
                
            },
            _ =>{panic!("Incorrect token ==> {}",debug::debug_token(token, &source))},
        };
            
        //more
        let token = scanner.scan_token();
        match token.token_type() {
            TokenType::Identifier => {                                
                
            },
            _ =>{panic!("Incorrect token ==> {}",debug::debug_token(token, &source))},
        };
        //e3lements
        let token = scanner.scan_token();
        match token.token_type() {
            TokenType::Identifier => {                                
                
            },
            _ =>{panic!("Incorrect token ==> {}",debug::debug_token(token, &source))},
        };
        let token = scanner.scan_token();
        match token.token_type() {
            TokenType::EOF => {                                
                
            },
            _ =>{panic!("Incorrect token ==> {}",debug::debug_token(token, &source))},
        };




    }


}// End test module
