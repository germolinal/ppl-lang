use crate::parser::*;
use crate::token::*;
use crate::operations::*;



/* PARSING FUNCTIONS */

pub fn unary(parser: &mut Parser){
        
    // Get the unary Token
    let token_type = parser.previous().token_type();

    // Consume the expresion after
    parser.parse_precedence(Precedence::Unary);

    // Emit the operation
    match token_type{
        TokenType::Minus => {
            parser.emit_byte(Operation::Negate);
        },
        TokenType::Bang => {
            parser.emit_byte(Operation::Not);
        },
        _ => {
            parser.internal_error_at_current(format!("Unknown Token in unary()")) 
        }
    };
}


pub fn number(parser: &mut Parser){
    let v = parser.previous().source_text(parser.source());            
    let the_v = match v.parse::<f64>(){
        Ok(v)=>v,
        Err(msg)=>{
            return parser.error_at_current(msg.to_string());
        }
    };    
    parser.emit_byte(Operation::PushNumber(the_v));
    
}

pub fn grouping(parser: &mut Parser){
    // left paren has been consumed
    parser.expression();
    if !parser.consume(TokenType::RightParen) {
        parser.error_at_current(format!("Expected ')' after expression"));
    }
}


pub fn binary(parser: &mut Parser){
    // Get the Binary
    let operator_type = parser.previous().token_type();

    // Compile what is after
    let rule = parser.get_rule(operator_type);
    match rule.next_precedence{
        Some(p)=>parser.parse_precedence(p),
        None => parser.internal_error_at_current(format!("No next precedence found for binary operation"))
    }

    // emit operation
    match operator_type {
        TokenType::Plus => {
            parser.emit_byte(Operation::Add)
        },
        TokenType::Minus => {
            parser.emit_byte(Operation::Subtract)
        },
        TokenType::Star => {
            parser.emit_byte(Operation::Multiply)
        },
        TokenType::Slash => {
            parser.emit_byte(Operation::Divide)
        },

        TokenType::EqualEqual => {
            parser.emit_byte(Operation::Equal)
        },
        TokenType::BangEqual => {
            parser.emit_byte(Operation::NotEqual);            
        },
        TokenType::Greater =>{
            parser.emit_byte(Operation::Greater)
        },
        TokenType::GreaterEqual=>{
            parser.emit_byte(Operation::GreaterEqual);            
        },
        TokenType::Less =>{
            parser.emit_byte(Operation::Less);
        },
        TokenType::LessEqual => {
            parser.emit_byte(Operation::LessEqual);
        },
        _ => parser.internal_error_at_current(format!("Unknown Token for Binary operation"))
    }
    
}

pub fn literal(parser: &mut Parser){
    match parser.previous().token_type(){
        TokenType::False => parser.emit_byte(Operation::PushBool(false)),
        TokenType::True => parser.emit_byte(Operation::PushBool(true)),        
        _ => parser.internal_error_at_current(format!("Unknown Token in literal()")) 
    }
}