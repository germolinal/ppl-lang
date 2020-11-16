
use crate::parser::*;
use crate::token::*;
use crate::operations::*;
use crate::script_fn::ScriptFn;
use crate::function::Function;

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

pub fn string(parser: &mut Parser){
    /*
    let v = parser.previous().source_text(parser.source());                
    parser.emit_byte(Operation::PushString(Box::new(v)));
    */
    unimplemented!();
}

pub fn array(parser: &mut Parser){
    unimplemented!();
    /*
    //parser.advance();
    
    let mut n : usize = 0;
    
    while !parser.consume(TokenType::RightBracket){        
        parser.expression();        
        n +=1;
        
        if !parser.consume(TokenType::Comma) && !parser.check(TokenType::RightBracket) {
            parser.error_at_current(format!("Expecting ',' between Array elements."));
        }
        
        if parser.check(TokenType::EOF){
            parser.error_at_current(format!("Expecting ']' at the end of Array"));
        }
    }
    
    parser.emit_byte(Operation::PushArray(n));
    */
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



pub fn index(_parser: &mut Parser){
    unimplemented!();
}

pub fn call(_parser:&mut Parser){
    unimplemented!();
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

pub fn function(parser:&mut Parser, name: &String)->Option<Box<ScriptFn>>{
    // starts from the (), so it covers
    // both fn(){} and fn ID(){}
    // this becomes { let args[]; ...body...  }
    parser.show_tokens("function()");
    if !parser.consume(TokenType::LeftParen){
        parser.error_at_current(format!("Expecting '(' when defining function"));        
        return None;
    }

    // Get a copy of the current function
    let old_func = match parser.take_current_function(){
        Some(f)=>f,
        None => {
            parser.error_no_current_function();
            return None
        }
    };

    // Create a new function, and plug it to the 
    // parser
    let new_func = Box::new(ScriptFn::new(&name));
    parser.set_function(new_func); 

    // Open main scope
    parser.begin_scope();

    let mut n_vars : usize = 0;
    parser.var_declaration(&mut n_vars);
    if !parser.consume(TokenType::RightParen){
        parser.error_at_current(format!("Expecting ')' after variable list in function declaration"));        
        return None;
    }

    // Now the body of the function
    if !parser.consume(TokenType::LeftBrace){
        parser.error_at_current(format!("Expecting '{{' for opening body of function"));        
        return None;
    }

    // Open, process, and close body    
    parser.block();
    
    // Close main scope
    parser.end_scope();

    parser.show_tokens("function()  2");

    // Get the created function back.
    let mut new_func = match parser.take_current_function(){
        Some(f)=>f,
        None => {
            parser.error_no_current_function();
            return None
        }
    };
    
    // Restore the old one
    parser.set_function(old_func);
    new_func.set_n_args(n_vars);
    
    return Some(new_func);
}

/// Anonymous function parser
pub fn function_value(parser:&mut Parser){
    match function(parser,&format!("<Anonymous Function>")){
        Some(f)=>{
            // f is now the function.
            let v = Function::Script(f);
            parser.emit_byte(Operation::PushFunction(v))
        },
        None => {}
    }
}

pub fn variable( parser: &mut Parser){
    // search back for a variable with the same name
    let var_name = if parser.consume(TokenType::Identifier){
        parser.previous().source_text(parser.source())
    }else{
        return parser.error_at_current(format!("Expecting identifier after 'let'. Found '{}'",parser.previous().source_text(parser.source()) ))
    };

    if let Some(i) = parser.find_var(&var_name) {
        return parser.emit_byte(Operation::PushVarRef(i));
    }else{
        return parser.error_at_current(format!("Could not find Variable '{}'", var_name ))
    }
}

