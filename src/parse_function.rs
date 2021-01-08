
use crate::parser::*;
use crate::token::*;
use crate::operations::*;
use crate::function::Function;
use crate::compiler::Compiler;

/* PARSING FUNCTIONS */
pub fn unary(parser: &mut Parser, compiler: &mut Compiler){
        
    // Get the unary Token
    let token_type = parser.previous().token_type();

    // Consume the expresion after
    parser.parse_precedence(compiler, Precedence::Unary);

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

pub fn string(_parser: &mut Parser, _c: &mut Compiler){
    /*
    let v = parser.previous().source_text(parser.source());                
    parser.emit_byte(Operation::PushString(Box::new(v)));
    */
    unimplemented!();
}

pub fn array(_parser: &mut Parser, _c: &mut Compiler){
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

pub fn number(parser: &mut Parser, _c: &mut Compiler){
    let v = parser.previous().source_text(parser.source());            
    let the_v = match v.parse::<f64>(){
        Ok(v)=>v,
        Err(msg)=>{
            return parser.error_at_current(msg.to_string());
        }
    };    
    parser.emit_byte(Operation::PushNumber(the_v));   
}



pub fn index(_parser: &mut Parser, _c: &mut Compiler){
    unimplemented!();
}


/// pushes arguments separated by commas
/// e.g. arg1, arg2, arg3,...
/// 
fn arg_list(parser: &mut Parser, compiler: &mut Compiler, n: &mut usize){
    
   
    // Left Paren has been consumed
    loop {
        
        // Evaluate an expression
        parser.expression(compiler);
        // Increase count
        *n+=1;

        
        // Consume the next comma, or return
        if !parser.consume(TokenType::Comma){
            return;
        }        

        // start over
    }
}

pub fn call(parser:&mut Parser, compiler: &mut Compiler){

    // Push arguments
    let mut n_args = 0;    

    // If not empty arglist
    if !parser.check(TokenType::RightParen){
        arg_list(parser, compiler, &mut n_args);    
    }
    if !parser.consume(TokenType::RightParen){
        parser.error_at_current(format!("Expected ')' after argument list in function call"));
    }
    
    parser.emit_byte(Operation::Call(n_args));

}

pub fn grouping(parser: &mut Parser, compiler: &mut Compiler){
    // left paren has been consumed
    parser.expression(compiler);
    if !parser.consume(TokenType::RightParen) {
        parser.error_at_current(format!("Expected ')' after expression"));
    }
}


pub fn binary(parser: &mut Parser, compiler: &mut Compiler){
    // Get the Binary
    let operator_type = parser.previous().token_type();

    // Compile what is after
    let rule = parser.get_rule(operator_type);
    match rule.next_precedence{
        Some(precedence)=>parser.parse_precedence(compiler, precedence),
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
        TokenType::And => {
            parser.emit_byte(Operation::And);
        },
        TokenType::Or => {
            parser.emit_byte(Operation::Or);
        },
        _ => parser.internal_error_at_current(format!("Unknown Token for Binary operation"))
    }
    
}

pub fn literal(parser: &mut Parser, _c: &mut Compiler){
    match parser.previous().token_type(){
        TokenType::False => parser.emit_byte(Operation::PushBool(false)),
        TokenType::True => parser.emit_byte(Operation::PushBool(true)),        
        _ => parser.internal_error_at_current(format!("Unknown Token in literal()")) 
    }
}

/// Parses an anonymous function
pub fn function(parser:&mut Parser, name: &String, _c: &mut Compiler)->Option<Function>{
    // starts from the (), so it covers
    // both 'let x = fn(){}' and 'fn ID(){}'
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
    let new_func = Function::new_script(&name);
    parser.set_function(new_func); 

    // Reset compiler
    let mut compiler = Compiler::new(vec![]);

    // Open main scope
    parser.begin_scope(&mut compiler);

    let mut n_vars : usize = 0;
    parser.show_tokens("before var_declaration()");
    match parser.current().token_type(){
        // There are variables... declare them
        TokenType::Identifier => parser.var_declaration(&mut compiler, &mut n_vars),
        // Nothing to declare
        TokenType::RightParen => {},
        _ => {
            parser.error_at_current(format!("Expecting ')' or Variable Identifiers after '(' in function declaration."));        
            return None;
        }

    }
    
    
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
    parser.block(&mut compiler);
    
    // Close main scope
    parser.end_scope(&mut compiler);

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
pub fn function_value(parser:&mut Parser, compiler: &mut Compiler){
    
    if let Some(f) = function(parser,&format!("<Anonymous Function>"), compiler){        
        // f is now the function.
        let v = Box::new(f);
        if let Some(i) = parser.push_constant(v){                
            parser.emit_byte(Operation::PushHeapRef(i));
        }        
    }
}


pub fn variable( parser: &mut Parser, compiler: &mut Compiler){
    // search back for a variable with the same name
    let var_name = parser.previous();
        
    match compiler.get_local(&var_name, parser.source()){
        Some(i)=>{
            parser.emit_byte(Operation::GetLocal(i))
        },
        None => panic!("Could not find Variable '{}'", var_name.source_text(parser.source() ))
    }
        
}


