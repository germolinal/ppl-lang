use crate::scanner::*;

#[derive(Clone, Copy)]
pub enum TokenType{
    // General
    EOF,
    Error,

    // Single character
    LeftParen, RightParen,
    LeftBrace, RightBrace,
    Comma, Dot, Minus, Plus,
    Semicolon, Slash, Star,

    // One or two characters
    Bang, BangEqual,
    Equal, EqualEqual,
    Greater, GreaterEqual,
    Less, LessEqual,

    // Other literals
    TokenString,
    Number,
    Identifier,

    // Keywords
    And,
    Break,
    Else,
    False, Function, For,
    If, In,
    Let,
    Nil,
    Or,
    Return,
    TokenSelf,
    True,
    While,
}



#[derive(Clone,Copy)]
pub struct Token {
    line: usize,    
    length: usize,
    start: *const u8,   
    token_type: TokenType 
}

impl Token{
    pub fn new(scanner : &Scanner, token_type: TokenType )-> Self{
        Self {
            line: scanner.line(),
            length: scanner.current_index() - scanner.start_index(),
            start: scanner.start().clone(),
            token_type: token_type            
        }
    }
    pub fn new_with_line(scanner : &Scanner, token_type: TokenType, line: usize )-> Self{
        Self {
            line: line,
            length: scanner.current_index() - scanner.start_index(),
            start: scanner.start().clone(),
            token_type: token_type            
        }
    }

    pub fn line(&self)->usize{
        self.line
    }    

    pub fn token_type(&self)->TokenType{
        self.token_type
    }

    pub fn source_text(&self, source: &Vec<u8>)->String{

        // Copy start.
        let mut start = self.start.clone();
        let mut v : Vec<char> = Vec::new();

        for _ in 0..self.length{
            unsafe{
                v.push(*start as char);
                start = start.add(1)
            }
        }
        
        let s : String = v.into_iter().collect();        
        return s;

    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_source_text(){
        let src = vec!['H' as u8,'e' as u8,'l' as u8, 'l' as u8, 'o' as u8];

        let token = Token{
            line: 1,
            length: 2,
            start: src.as_ptr(),            
            token_type: TokenType::EOF,
        };

        assert_eq!("He".to_string(),token.source_text(&src));
    }
}