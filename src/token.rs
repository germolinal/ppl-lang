use crate::scanner::*;

#[derive(Clone, Copy, PartialEq)]
pub enum TokenType{
    // General
    EOF,
    Error,

    // Single character
    LeftParen, RightParen,
    LeftBrace, RightBrace,
    Comma, Dot, Minus, Plus,
    Colon, Semicolon, Slash, Star,

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
    Class,
    Else,
    False, Function, For,
    If, In,
    Let,
    //Nil,
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
    start: usize,   
    token_type: TokenType 
}

impl Token{
    pub fn new(scanner : &Scanner, token_type: TokenType )-> Self{
        Self {
            line: scanner.line(),
            length: scanner.current_index() - scanner.start_index(),
            start: scanner.start_index(),
            token_type: token_type            
        }
    }
    pub fn new_with_line(scanner : &Scanner, token_type: TokenType, line: usize )-> Self{
        Self {
            line: line,
            length: scanner.current_index() - scanner.start_index(),
            start: scanner.start_index(),
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
        let ini = self.start;
        let fin = self.length + ini;
        match source.get(ini..fin){
            Some(v)=>{            
                let mut s : Vec<u8> = Vec::with_capacity(v.len());
                for b in v.iter(){
                    s.push(*b);
                }

                return String::from_utf8(s).unwrap();
            },
            None => panic!("Internal error... could not source text for token")
        }        

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
            start: 0,            
            token_type: TokenType::EOF,
        };

        assert_eq!("He".to_string(),token.source_text(&src));
    }
}