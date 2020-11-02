use crate::chunk::*;


/// Lists the operations available for the virtual machine
#[repr(u8)]
pub enum Operation {
    Return,
    Constant(usize),
    Negate,
    Not,
    Add,
    Subtract,
    Multiply,
    Divide,
    PushNil,
    PushBool(bool),
    PushFloat(f64),
    PushInt(i32),
    Equal,
    Greater,
    Less,
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