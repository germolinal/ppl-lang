pub mod debug {
    use crate::chunk::*;
    use crate::operations::*;
    use crate::token::*;
    
    fn simple_instruction(name: &str, _offset: usize) {
        println!("{}",name);
        //return offset + 1;
    }
    
    fn constant_instruction(name: &str, chunk: &Chunk, _offset: usize, c_index: usize) {    
        let cons = chunk.constants();
        //let c_index = ops[offset+1].clone() as usize;
        let c = &cons[c_index];
    
        println!("{}\tconst {} | '{}'", name,c_index, c.to_string()); 
        //return offset+2;           
    }
    
    /// Disassembles an Operation
    /// # Arguments:
    /// * op: The operation to be unassembled
    pub fn disasemble_operation(chunk: &Chunk, offset: usize) {        
        let ops = &chunk.code();
        let op = &ops[offset];
        let ln = chunk.lines()[offset];
        print!("{:04} ", offset);    
        
        if ln > 0 {
            print!("(ln {})\t", ln)
        }else{
            print!("\t/\t")
        }
        
    
        match op {
            
            Operation::Return => {
                return simple_instruction(&"OP_RETURN", offset)
            },
    
            Operation::Constant(c_index) => {
                return constant_instruction(&"OP_CONSTANT", chunk, offset, *c_index);
            },
    
            Operation::Negate => {
                return simple_instruction("OP_NEGATE", offset);
            },
    
            Operation::Not=>{
                return simple_instruction("OP_NOT", offset);
            },
    
            Operation::Add => {
                return simple_instruction("OP_ADD", offset);
            },
    
            Operation::Subtract => {
                return simple_instruction("OP_SUBTRACT", offset);
            },
            
            Operation::Multiply => {
                return simple_instruction("OP_MULTIPLY", offset);
            },
            
            Operation::Divide => {
                return simple_instruction("OP_DIVIDE", offset);
            }
        }
        
    }
    
    /// Prints the instruction set into the 
    /// terminal... not really used outside of
    /// development environments
    fn disassemble(chunk : &Chunk, name: String){
        
        println!("== {} ==\n", name);
    
        for (i,_) in chunk.code().iter().enumerate(){
            disasemble_operation(chunk,i);
        }
    }
    
    /// Prints a token in debug mode
    pub fn debug_token(token: Token, source:&Vec<u8>) -> String{
    
        let token_name = match token.token_type() {
            TokenType::EOF => "TOKEN_EOF",
            TokenType::Error => "TOKEN_ERROR",
            
            TokenType::LeftParen => "LEFT_PAREN", TokenType::RightParen => "RIGHT_PAREN",
            TokenType::LeftBrace => "LEFT_BRACE", TokenType::RightBrace => "RIGHT_BRACE",
    
            TokenType::Comma => "COMMA", TokenType::Dot => "DOT",
            TokenType::Minus => "MINUS", TokenType::Plus => "PLUS",  
            TokenType::Semicolon => "SEMICOLON", TokenType::Slash => "SLASH", TokenType::Star => "STAR",     
            
            TokenType::Bang => "BANG!", TokenType::BangEqual => "BANG! EQUAL",
            TokenType::Equal => "EQUAL", TokenType::EqualEqual => "EQUAL EQUAL",
            TokenType::Greater => "GREATER", TokenType::GreaterEqual => "GREATER EQUAL",
            TokenType::Less => "LESS", TokenType::LessEqual => "LESS EQUAL",

            // Other literals
            TokenType::TokenString => "STRING",
            TokenType::Number => "NUMBER",
            TokenType::Identifier => "IDENTIFIER",

            // Keywords
            TokenType::And => "AND",
            TokenType::Break => "BREAK",
            TokenType::Else => "ELSE",
            TokenType::False => "FALSE",
            TokenType::Function => "FUNCTION",
            TokenType::For => "FOR",
            TokenType::If => "IF",
            TokenType::In => "IN",
            TokenType::Let => "LET",
            TokenType::Nil => "NIL",
            TokenType::Or => "OR",
            TokenType::Return => "RETURN",
            TokenType::TokenSelf => "SELF",
            TokenType::True => "TRUE",
            TokenType::While => "WHILE",

        };
                
        format!("{} '{}'", token_name,token.source_text(source) )
    }
    
    
    /***********/
    /* TESTING */
    /***********/
    
    #[cfg(test)]
    mod tests {
        use super::*;
        use crate::values::*;
    
        #[test]
        fn test_dissassemble(){
            let v = 1.2;
            let mut c = Chunk::new();
    
            let constant_i = c.add_constant(PPLValue::PPLFloat(v));                        
            c.write_operation(Operation::Constant(constant_i), 123);                
            c.write_operation(Operation::Return, 0);
            
            disassemble(&c, "The chunk".to_string());        
        }
    }
}
