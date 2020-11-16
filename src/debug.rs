#[cfg(debug_assertions)]
pub mod debug {
    use crate::chunk::Chunk;
    use crate::operations::*;
    use crate::token::*;
    
    fn simple_instruction(name: &str, _offset: usize) {
        println!("{}",name);        
    }
    
    /*
    fn constant_instruction(name: &str, chunk: &Chunk, c_index: usize) {    
        let cons = chunk.constants();
        let c = &cons[c_index];
    
        println!("{}\tconst {} | '{}'", name,c_index, c.to_string());              
    }
    */

    

    

    
    /// Disassembles an Operation
    /// # Arguments:
    /// * op: The operation to be unassembled
    pub fn operation(ops: &[Operation], lines: &[usize],offset: usize) {        
        //let ops = chunk.code();
        let op = &ops[offset];
        let ln = lines[offset];
        //let ln = chunk.lines()[offset];
        print!("{:04} ", offset);    
        
        if ln > 0 {
            print!("(ln {})\t", ln)
        }else {
            print!("\t/\t")
        }
        
    
        match op {
            
            Operation::Return => {
                return simple_instruction(&"OP_RETURN", offset)
            },
    
            /*
            Operation::Constant(c_index) => {
                return constant_instruction(&"OP_CONSTANT", chunk, *c_index);
            },
            */
    
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
            /*,
            Operation::PushNil => {
                return simple_instruction("OP_PUSH_NIL", offset );
            },
            */
            Operation::PushBool(b) => {
                if *b {
                    return simple_instruction("OP_PUSH_TRUE", offset );
                }else{
                    return simple_instruction("OP_PUSH_FALSE", offset );
                }
            },

            Operation::PushNumber(v)=>{
                return println!("OP_PUSH_NUMBER | '{}'", v);         
            },
            /*
            Operation::PushString(v)=>{
                return println!("OP_PUSH_STRING | '\"{}'\"", v);         
            },
            Operation::PushArray(v)=>{
                return println!("OP_PUSH_ARRAY | '\"{} elements'\"", v);         
            },
            Operation::PushObject(_)=>{
                return println!("OP_PUSH_OBJECT");         
            },
            Operation::PushGeneric(v)=>{
                return println!("OP_PUSH_GENERIC | '{}'", v.type_name());         
            },
            */
            Operation::PushFunction(v)=>{
                return println!("OP_PUSH_FUNCTION | '{}()'", v.get_name());
            }


            Operation::PushVar(_)=>{
                return println!("OP_PUSH_VAR");
            },
            Operation::PushVarRef(i)=>{
                return println!("OP_PUSH_VAR_REF | {}", i);
            },
            Operation::PopVars(n)=>{
                return println!("OP_POP_VARS | {}",n);
            },
            Operation::DefineVar(n)=>{
                return println!("OP_DEFINE_VAR | {}",n);
            },

            Operation::Equal => {
                return simple_instruction("OP_EQUAL", offset );
            },
            Operation::NotEqual => {
                return simple_instruction("OP_NOT_EQUAL", offset );
            },

            Operation::Greater => {
                return simple_instruction("OP_GREATER", offset );
            },
            Operation::GreaterEqual => {
                return simple_instruction("OP_GREATER_EQUAL", offset );
            },

            Operation::Less => {
                return simple_instruction("OP_LESS", offset );
            },
            Operation::LessEqual => {
                return simple_instruction("OP_LESS_EQUAL", offset );
            },

            Operation::ForLoop(n_vars,body_length)=>{
                return println!("OP_FOR_LOOP | {} vars, length: {}",n_vars, body_length); 
            },

            Operation::JumpIfFalse(n)=>{
                return println!("OP_JUMP_IF_FALSE | {} ops",n); 
            },

            Operation::JumpIfTrue(n)=>{
                return println!("OP_JUMP_IF_TRUE | {} ops",n); 
            },

            Operation::JumpBack(n)=>{
                return println!("OP_JUMP_BACK | {} ops",n); 
            }

        }
        
    }
    
    /// Prints the instruction set into the 
    /// terminal... not really used outside of
    /// development environments    
    pub fn chunk(chunk : &Chunk, name: String){
        
        println!("== {} ==\n", name);
    
        let lines : &[usize]= &chunk.lines();
        let ops : &[Operation]= &chunk.code();

        for i in 0..ops.len(){
            operation(ops, lines, i);
        }
    }

    /// Retreives the token type name
    pub fn token_type(t : TokenType)->&'static str{
        match t {
            TokenType::EOF => "TOKEN_EOF",
            TokenType::Error => "TOKEN_ERROR",
            
            TokenType::LeftParen => "LEFT_PAREN", TokenType::RightParen => "RIGHT_PAREN",
            TokenType::LeftBrace => "LEFT_BRACE", TokenType::RightBrace => "RIGHT_BRACE",
            TokenType::LeftBracket => "LEFT_BRACKET", TokenType::RightBracket => "RIGHT_BRACKET",
    
            TokenType::Comma => "COMMA", TokenType::Dot => "DOT",
            TokenType::Minus => "MINUS", TokenType::Plus => "PLUS",  
            TokenType::Colon=>"COLON", TokenType::Slash => "SLASH", TokenType::Star => "STAR",     
            /*TokenType::Semicolon => "SEMICOLON",*/
            TokenType::Question => "QUESTION",

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
            //TokenType::Nil => "NIL",
            TokenType::Or => "OR",
            TokenType::Return => "RETURN",
            TokenType::TokenSelf => "SELF",
            TokenType::True => "TRUE",
            TokenType::While => "WHILE",
            TokenType::Class => "CLASS",

        }
    }
    
    /// Retrieves a token in debug mode
    pub fn token(token: Token, source:&Vec<u8>) -> String{
    
        let token_name = token_type(token.token_type());
                
        format!("{} '{}'", token_name,token.source_text(source) )
    }
       
}
