//use crate::token::Token;

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
    PushNumber(f64),  
    PushHeapRef(usize),
    
    GetLocal(usize),
    SetLocal(usize),
    GetGlobal(usize),
    GetFromPackage(usize),

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