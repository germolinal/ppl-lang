//use crate::variable::Var;
//use std::rc::Rc;

//use crate::value_trait::ValueTrait;
//use crate::string::StringV;
//use crate::object::Object;
//use crate::function::Function;
use crate::values::Value;

/// Lists the operations available for the virtual machine
#[repr(u8)]
#[derive(Clone, Copy)]
pub enum Operation {
    Return(usize),
    Negate,
    Not,
    Add,
    Subtract,
    Multiply,
    Divide,    
    Equal,
    NotEqual,
    Greater,
    Less,
    GreaterEqual,
    LessEqual,
    And,
    Or,
        
    PushNil,
    PushBool(bool),
    PushNumber(f64),  
    PushHeapRef(usize),
    
    GetLocal(usize),
    SetLocal(usize),

    Pop(usize),
    DefineVars(usize),
    
    ForLoop(usize,usize),
    JumpIfFalse(usize),
    JumpIfTrue(usize),
    JumpBack(usize),

    Call(usize),
}





/***********/
/* TESTING */
/***********/

#[cfg(test)]
mod tests {    

    #[test]
    fn test_disassemble() {
        assert!(true);
    }
}