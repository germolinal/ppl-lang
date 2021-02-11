//use crate::token::Token;
use crate::number::Number;

/// Lists the operations available for the virtual machine
#[repr(u8)]
#[derive(Copy, Clone, PartialEq)]
pub enum Operation {
    Return,
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
    PushNumber(Number),  
    PushHeapRef(u8),
    
    GetLocal(u8),
    SetLocal(u8),
    GetGlobal(u8),
    GetFromPackage(usize),

    Pop(u8),
    DefineVars(u8),
    
    ForLoop(u8,u8),
    JumpIfFalse(u8),
    JumpIfTrue(u8),
    JumpBack(u8),

    Call(u8),
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