

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
    PushNil,
    Equal,
    Greater,
    Less,
    Constant(usize),
    PushBool(bool),
    PushNumber(f64),    
}
/*
impl Clone for Operation {
    fn clone(&self) -> Self {
        match self {
            Operation::Return => Operation::Return,
            Operation::Constant(e) => Operation::Constant(*e),
            Operation::Negate => Operation::Negate,
            Operation::Not => Operation::Not,
            
        }
    }
}
*/



/***********/
/* TESTING */
/***********/

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_disassemble() {
        assert!(true);
    }
}