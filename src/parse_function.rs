use crate::number::Number;
//use crate::heap_list::HeapList;
use crate::parser::*;
use crate::token::*;
use crate::operations::*;
use crate::function::Function;
use crate::compiler::Compiler;
use crate::string::StringV;
use crate::handler::PPLHandler;

/* PARSING FUNCTIONS */


/// Processes a unitary operation.
/// 
/// Does not use the 'can_assign'
pub fn unary<'a>(_can_assign: bool, parser: &mut Parser<'a>, handler: &mut PPLHandler, compiler: &mut Compiler<'a>){
        
    // Get the unary Token
    let token_type = parser.previous().token_type();

    // Consume the expresion after
    parser.parse_precedence(handler, compiler, Precedence::Unary);

    // Emit the operation
    match token_type{
        TokenType::Minus => {
            parser.emit_byte(Operation::Negate);
        },
        TokenType::Bang => {
            parser.emit_byte(Operation::Not);
        },
        _ => {
            parser.internal_error_at_current("Unknown Token in unary()".to_string()) 
        }
    };
}

pub fn string(_can_assign: bool, parser: &mut Parser, handler: &mut PPLHandler, _c: &mut Compiler){
    let s : StringV = parser.previous().source_text().to_string();
    let v = Box::new(s);
    let i = handler.heap.push(v);                
    
    parser.emit_byte(Operation::PushHeapRef(i));
    
}

pub fn array(_can_assign: bool, _parser: &mut Parser, _h: &mut PPLHandler, _c: &mut Compiler){
    unimplemented!();
    /*
    //parser.advance();
    
    let mut n : usize = 0;
    
    while !parser.consume(TokenType::RightBracket){        
        parser.expression();        
        n +=1;
        
        if !parser.consume(TokenType::Comma) && !parser.check(TokenType::RightBracket) {
            parser.error_at_current( "Expecting ',' between Array elements.".to_string() );
        }
        
        if parser.check(TokenType::EOF){
            parser.error_at_current( "Expecting ']' at the end of Array".to_string() );
        }
    }
    
    parser.emit_byte(Operation::PushArray(n));
    */
}

/// Parses a number... does not use the 'can_assign'
pub fn number(_can_assign: bool, parser: &mut Parser, _h: &mut PPLHandler, _c: &mut Compiler){
    let v = parser.previous().source_text();            
    let the_v = match v.parse::<Number>(){
        Ok(v)=>v,
        Err(msg)=>{
            return parser.error_at_current(msg.to_string());
        }
    };    
    parser.emit_byte(Operation::PushNumber(the_v));   
}


/// Parses an indexation (i.e. x[i]) operation
pub fn index(_can_assign: bool, _parser: &mut Parser, _h: &mut PPLHandler, _c: &mut Compiler){
    unimplemented!();
}


/// pushes arguments separated by commas
/// e.g. arg1, arg2, arg3,...
/// 
fn arg_list<'a>(parser: &mut Parser<'a>, handler: &mut PPLHandler, compiler: &mut Compiler<'a>, n: &mut u8){
    
   
    // Left Paren has been consumed
    loop {
                
        // Evaluate an expression
        parser.expression(handler, compiler);
        // Increase count
        *n+=1;

        
        // Consume the next comma, or return
        if !parser.consume(TokenType::Comma){
            return;
        }        

        // start over
    }
}

/// Parses a call...
///
/// Does not use the 'can_assign'
pub fn call<'a>(_can_assign: bool, parser:&mut Parser<'a>, handler: &mut PPLHandler, compiler: &mut Compiler<'a>){
    
    
    // Push arguments
    let mut n_args = 0;    

    // If not empty arglist
    if !parser.check(TokenType::RightParen){
        arg_list(parser, handler, compiler, &mut n_args);    
    }
    if !parser.consume(TokenType::RightParen){
        parser.error_at_current("Expected ')' after argument list in function call".to_string());
    }
    

    parser.emit_byte(Operation::Call(n_args));

}

/// Parses grouping (e.g., '(x*y/z)' )
/// 
/// Does not use the 'can_assign'
pub fn grouping<'a>(_can_assign: bool, parser: &mut Parser<'a>, handler: &mut PPLHandler, compiler: &mut Compiler<'a>){
    // left paren has been consumed
    parser.expression(handler, compiler);
    if !parser.consume(TokenType::RightParen) {
        parser.error_at_current( "Expected ')' after expression".to_string() );
    }
}

/// Parses binary operation.
/// 
/// Does not use the 'can_assign'
pub fn binary<'a>(_can_assign: bool, parser: &mut Parser<'a>, handler: &mut PPLHandler, compiler: &mut Compiler<'a>){
    // Get the Binary
    let operator_type = parser.previous().token_type();

    // Compile what is after
    let rule = parser.get_rule(operator_type);
    match rule.next_precedence{
        Some(precedence)=>parser.parse_precedence(handler, compiler, precedence),
        None => parser.internal_error_at_current("No next precedence found for binary operation".to_string())
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
        _ => parser.internal_error_at_current("Unknown Token for Binary operation".to_string())
    }
    
}

/// Parses literals. 
/// 
/// Does not use the 'can_assign'
pub fn literal(_can_assign: bool, parser: &mut Parser, _h: &mut PPLHandler, _c: &mut Compiler){
    match parser.previous().token_type(){
        TokenType::False => parser.emit_byte(Operation::PushBool(false)),
        TokenType::True => parser.emit_byte(Operation::PushBool(true)),        
        _ => parser.internal_error_at_current("Unknown Token in literal()".to_string()) 
    }
}

/// Parses an anonymous function                    
pub fn function<'a>(parser : &mut Parser<'a>, handler: &mut PPLHandler, name: &'a [u8], _c: &mut Compiler<'a>)->Option<Function>{
        
    // starts from the (), so it covers
    // both 'let x = fn(){}' and 'fn ID(){}'
    // this becomes { let args[]; ...body...  }    
    if !parser.consume(TokenType::LeftParen){
        parser.error_at_current("Expecting '(' when defining function".to_string());        
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
    let new_func = Function::new_script(name);
    parser.set_function(new_func); 

    
    // Reset compiler (so it does not capture variables ot of its scope)
    let mut clean_compiler = Compiler::new(); 

    // Open main scope
    parser.begin_scope(&mut clean_compiler);

    let mut n_vars : u8 = 0;
    
    match parser.current().token_type(){
        // There are variables... declare them (but DO NOT define them)
        TokenType::Identifier => parser.var_declaration(handler, &mut clean_compiler, false, &mut n_vars),
        // Nothing to declare
        TokenType::RightParen => {},
        _ => {
            parser.error_at_current( "Expecting ')' or Variable Identifiers after '(' in function declaration.".to_string());        
            return None;
        }
    }
    
    
    if !parser.consume(TokenType::RightParen){
        parser.error_at_current( "Expecting ')' after variable list in function declaration".to_string() );        
        return None;
    }

    // Now the body of the function
    if !parser.consume(TokenType::LeftBrace){
        parser.error_at_current( "Expecting '{{' for opening body of function".to_string() );        
        return None;
    }

    // Open, process, and close body    
    parser.block(handler, &mut clean_compiler);
    
    // No end_scope()... this is done 
    // when processing the Return operation
    // parser.end_scope(&mut compiler);
    

    // Get the created function back.
    let mut new_func = match parser.take_current_function(){
        Some(f)=>f,
        None => {
            parser.error_no_current_function();
            return None
        }
    };
    
    // Check if the function returns anything. If not, 
    // Return Nil.
    let new_chunk = new_func.mut_chunk().unwrap();
    let c_len = new_chunk.len();        
    if c_len == 0 {
        // Nothing in the function... push return NIL
        new_chunk.push((Operation::PushNil, 0));
        new_chunk.push((Operation::Return, 0));
    }else {
        let (last_op,last_line) = new_chunk[c_len - 1];
        if Operation::Return != last_op {            
            new_chunk.push((Operation::PushNil, last_line));
            new_chunk.push((Operation::Return,  last_line));
        }
    }


    // Restore the old one
    parser.set_function(old_func);
    new_func.set_n_args(n_vars);
    
    Some(new_func)
}


/// Anonymous function parser
/// 
/// Does not use the 'can_assign'
pub fn function_value<'a>(_can_assign: bool, parser:&mut Parser<'a>, handler: &mut PPLHandler, compiler: &mut Compiler<'a>){
    
    if let Some(f) = function(parser, handler, "<Anonymous Function>".as_bytes(), compiler){        
        // f is now the function.
        let v = Box::new(f);
        let i = handler.heap.push(v);      

        parser.emit_byte(Operation::PushHeapRef(i));
    }
}

pub fn package_element<'a>(can_assign: bool, parser: &mut Parser<'a>, handler: &mut PPLHandler, _compiler: &mut Compiler<'a>){
    let pkg_name = parser.previous().source_text().to_string();
    
    let pkg = match handler.packages_dictionary.get(&pkg_name){
        Some(p)=>p,
        None => {panic!("Package '{}' not found", pkg_name)}
    };

    // consume package name
    parser.advance();

    let fn_name = parser.previous().source_text().to_string();

    let function_index = match pkg.get(&fn_name){
        Some(f)=>f,
        None => {panic!("Function '{}' not found in Package '{}", fn_name, pkg_name )}
    };

    if can_assign && parser.match_token(TokenType::Equal){
        panic!("Trying to reassign a Package function... Packages are inmutable");
    }else{
        parser.emit_byte(Operation::GetFromPackage(*function_index));
    }




}

pub fn variable<'a>(can_assign: bool, parser: &mut Parser<'a>, handler: &mut PPLHandler, compiler: &mut Compiler<'a>){
    // search back for a variable with the same name
    //let var_name = parser.previous();
        
    match compiler.get_local(parser.previous()){
        Some(i)=>{

            if can_assign && parser.match_token(TokenType::Equal){
                parser.expression(handler, compiler);
                parser.emit_byte(Operation::SetLocal(i))
            }else{
                parser.emit_byte(Operation::GetLocal(i));
            }
            
        },
        None => {
            // Global... needs to be a function.            
            if can_assign && parser.match_token(TokenType::Equal){
                panic!("Trying to reassign a global function");
            }else{
                match handler.heap.get_global_function(parser.previous()){
                    Some(i)=> parser.emit_byte(Operation::GetGlobal(i)),
                    None => {
                        panic!("Variable '{}' not found", parser.previous().source_text());
                    }
                }
                
            }
            
        }
    }
    
}


