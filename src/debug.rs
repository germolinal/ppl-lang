

use crate::operations::*;
use crate::token::*;

#[allow(dead_code)]
fn simple_instruction(name: &str, _offset: usize) {
    println!("{}",name);        
}








/// Disassembles an Operation
/// # Arguments:
/// * op: The operation to be unassembled
#[allow(dead_code)]
pub fn operation(ops: &[(Operation, usize)],offset: usize) {        
    //let ops = chunk.code();
    let (op, ln) = &ops[offset];                
    print!("{:04} ", offset);    
    
    if *ln > 0 {
        print!("(ln {})\t", ln)
    }else {
        print!("\t/\t")
    }
    

    match op {
        
        Operation::Return=> {
            println!("OP_RETURN");
        },            

        Operation::Negate => {
            simple_instruction("OP_NEGATE", offset);
        },

        Operation::Not=>{
            simple_instruction("OP_NOT", offset);
        },

        Operation::Add => {
            simple_instruction("OP_ADD", offset);
        },

        Operation::Subtract => {
            simple_instruction("OP_SUBTRACT", offset);
        },
        
        Operation::Multiply => {
            simple_instruction("OP_MULTIPLY", offset);
        },
        
        Operation::Divide => {
            simple_instruction("OP_DIVIDE", offset);
        }
        /*,
        Operation::PushNil => {
            simple_instruction("OP_PUSH_NIL", offset );
        },
        */
        Operation::PushBool(b) => {
            if *b {
                simple_instruction("OP_PUSH_TRUE", offset );
            }else{
                simple_instruction("OP_PUSH_FALSE", offset );
            }
        },

        Operation::PushNumber(v)=>{
            println!("OP_PUSH_NUMBER | '{}'", v);         
        },
        Operation::PushNil => {
            println!("OP_PUSH_NIL");         
        },
        /*
        Operation::PushString(v)=>{
            println!("OP_PUSH_STRING | '\"{}'\"", v);         
        },
        Operation::PushArray(v)=>{
            println!("OP_PUSH_ARRAY | '\"{} elements'\"", v);         
        },
        Operation::PushObject(_)=>{
            println!("OP_PUSH_OBJECT");         
        },
        Operation::PushGeneric(v)=>{
            println!("OP_PUSH_GENERIC | '{}'", v.type_name());         
        },
        */
        Operation::PushHeapRef(v)=>{
            println!("OP_PUSH_HEAP_REF | slot '{}'", v);
        }


        Operation::GetLocal(i)=>{
            println!("OP_GET_LOCAL | {}",i);
        },
        Operation::SetLocal(i)=>{
            println!("OP_SET_LOCAL | {}",i);
        },
        Operation::GetGlobal(i)=>{
            println!("OP_GET_LOCAL | {}",i);
        },
        Operation::GetFromPackage(i)=>{
            println!("OP_GET_FROM_PACKAGE | {}",i);
        },
        Operation::Pop(n)=>{
            println!("OP_POP | {}",n);
        },
        Operation::DefineVars(n)=>{
            println!("OP_DEFINE_VARS | {}",n);
        },

        Operation::Equal => {
            simple_instruction("OP_EQUAL", offset );
        },
        Operation::NotEqual => {
            simple_instruction("OP_NOT_EQUAL", offset );
        },

        Operation::Greater => {
            simple_instruction("OP_GREATER", offset );
        },
        Operation::GreaterEqual => {
            simple_instruction("OP_GREATER_EQUAL", offset );
        },

        Operation::Less => {
            simple_instruction("OP_LESS", offset );
        },
        Operation::LessEqual => {
            simple_instruction("OP_LESS_EQUAL", offset );
        },
        Operation::And => {
            simple_instruction("OP_AND", offset );
        },
        Operation::Or => {
            simple_instruction("OP_OR", offset );
        },

        Operation::ForLoop(n_vars,body_length)=>{
            println!("OP_FOR_LOOP | {} vars, length: {}",n_vars, body_length); 
        },

        Operation::JumpIfFalse(n)=>{
            println!("OP_JUMP_IF_FALSE | {} ops",n); 
        },

        Operation::JumpIfTrue(n)=>{
            println!("OP_JUMP_IF_TRUE | {} ops",n); 
        },

        Operation::JumpBack(n)=>{
            println!("OP_JUMP_BACK | {} ops",n); 
        },

        Operation::Call(n)=>{
            println!("OP_CALL | {} args",n); 
        }

    }
    
}

/// Prints the instruction set into the 
/// terminal... not really used outside of
/// development environments    
#[allow(dead_code)]
pub fn chunk(chunk : &[(Operation, usize)], name: String){
    
    println!("== {} ==\n", name);

    //let lines : &[usize]= &chunk.lines();
    //let ops : &[Operation]= &chunk.code();

    for i in 0..chunk.len(){
        operation(chunk, i);
    }
}

/// Retreives the token type name
#[allow(dead_code)]
pub fn token_type(t : TokenType)->&'static str{
    match t {
        TokenType::EOF => "TOKEN_EOF",
        TokenType::Error => "TOKEN_ERROR",
        
        TokenType::LeftParen => "LEFT_PAREN", TokenType::RightParen => "RIGHT_PAREN",
        TokenType::LeftBrace => "LEFT_BRACE", TokenType::RightBrace => "RIGHT_BRACE",
        TokenType::LeftBracket => "LEFT_BRACKET", TokenType::RightBracket => "RIGHT_BRACKET",

        TokenType::Comma => "COMMA", TokenType::Dot => "DOT",
        TokenType::Minus => "MINUS", TokenType::Plus => "PLUS",  
        //TokenType::Colon=>"COLON", 
        TokenType::Slash => "SLASH", TokenType::Star => "STAR",     
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
        TokenType::Package => "PACKAGE",

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
        TokenType::Return=> "RETURN",
        TokenType::TokenSelf => "SELF",
        TokenType::True => "TRUE",
        TokenType::While => "WHILE",
        TokenType::Class => "CLASS",

    }
}

/// Retrieves a token in debug mode
#[allow(dead_code)]
pub fn token(token: Token) -> String{

    let token_name = token_type(token.token_type());
            
    format!("{} '{}'", token_name,token.source_text() )
}
    
