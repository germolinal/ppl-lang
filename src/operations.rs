use crate::variable::*;

/// Lists the operations available for the virtual machine
#[repr(u8)]
pub enum Operation<'a> {
    Return,
    Negate,
    Not,
    Add,
    Subtract,
    Multiply,
    Divide,    
    Equal,
    Greater,
    Less,
    //Constant(usize),
    PushBool(bool),
    PushNumber(f64),  
    PushVar(Var<'a>),
    PopVars(usize),
    DefineVar(usize),
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