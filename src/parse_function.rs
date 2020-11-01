use crate::parser::*;
use crate::token::*;
use crate::operations::*;
use crate::values::*;

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
        _ => {
            unreachable!()
        }
    };
}


pub fn number(parser: &mut Parser){
    let v = parser.previous().source_text(parser.source());        
    if v.contains("."){
        let the_v = match v.parse::<f64>(){
            Ok(v)=>v,
            Err(msg)=>{
                return parser.error_at_current(msg.to_string());
            }
        };
        let index = parser.add_constant(PPLValue::PPLFloat(the_v));                        
        parser.emit_byte(Operation::Constant(index));
        
    }else{
        let the_v = match v.parse::<i32>(){
            Ok(v)=>v,
            Err(msg)=>{
                return parser.error_at_current(msg.to_string());
            }
        };
        let index = parser.add_constant(PPLValue::PPLInt(the_v));                        
        parser.emit_byte(Operation::Constant(index));

    }
}

pub fn grouping(parser: &mut Parser){
    // left paren has been consumed
    parser.expression();
    if parser.consume(TokenType::RightParen){
        parser.error_at_current(format!("Expected '(' after expression"));
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
        _ => parser.internal_error_at_current(format!("Unknown Token for Binary operation"))
    }
    
}
