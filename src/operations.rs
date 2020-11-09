//use crate::variable::*;
use crate::variable::Var;

/// Lists the operations available for the virtual machine
#[repr(u8)]
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
    //Constant(usize),
    PushBool(bool),
    PushNumber(f64),  
    PopVars(usize),
    DefineVar(usize),
    PushVar(Var),
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