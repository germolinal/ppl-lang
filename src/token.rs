
#[derive(Clone, Copy, PartialEq)]
pub enum TokenType{
    // General
    EOF,
    Error,

    // Single character
    LeftParen, RightParen,
    LeftBrace, RightBrace,
    LeftBracket, RightBracket,
    Comma, Dot, Minus, Plus,
    /*Colon,*/ Slash, Star, //Semicolon,
    Question,

    // One or two characters
    Bang, BangEqual,
    Equal, EqualEqual,
    Greater, GreaterEqual,
    Less, LessEqual,

    // Other literals
    TokenString,
    Number,
    Identifier,
    Package,

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
pub struct Token<'a> {
    pub line: usize,    
    pub length: usize,
    pub start: usize,   
    pub txt: &'a [u8],    
    pub token_type: TokenType 
}


/// Compares by text inside the Token
impl <'a>PartialEq for Token<'a> {
    fn eq(&self, other: &Self) -> bool {
        if self.length == other.length && self.token_type == other.token_type {
            for i in 0..self.txt.len(){
                if self.txt[i] != other.txt[i]{
                    return false;
                }
            }
            true
        }else{
            false
        }
    }
}




impl <'a>Token<'a>{
    /*
    pub fn new(scanner : &'static Scanner, token_type: TokenType )-> Self{
        let src = scanner.source();
        let txt = &src[scanner.start_index()..scanner.current_index()];

        Self {
            line: scanner.line(),
            length: scanner.current_index() - scanner.start_index(),
            txt: txt,
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
    */

    pub fn line(&self)->usize{
        self.line
    }    

    pub fn token_type(&self)->TokenType{
        self.token_type
    }

    pub fn source_slice(&self)->&'a [u8]{
        self.txt        
    }

    pub fn source_text(&self)->&str{
        std::str::from_utf8(self.txt).unwrap()
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
            txt: &src[0..2],          
            token_type: TokenType::EOF,
        };
        
        assert_eq!("He".to_string(),token.source_text());
    }
}