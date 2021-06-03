//use crate::token::Token;
use crate::number::Number;

/// Lists the operations available for the virtual machine
#[repr(u8)]
#[derive(Copy, Clone, PartialEq)]
pub enum Operation {
    /// If within a function, then it calls return_op() function; otherwise, 
    /// exists the program
    Return,

    /// Pops one value from the stack and calls the negate() function on it.
    /// Pushes the result back to the stack.
    Negate,

    /// Pops one value from the stack and calls the not() function on it.
    /// Pushes the result back to the stack.
    Not,

    /// Pops two values from the stack and calls the add() function on the first one (the second one is an argument).
    /// Pushes the result back to the stack.
    Add,

    /// Pops two values from the stack and calls the subtract() function on the first one (the second one is an argument).
    /// Pushes the result back to the stack.
    Subtract,

    /// Pops two values from the stack and calls the multiply() function on the first one (the second one is an argument).
    /// Pushes the result back to the stack.
    Multiply,

    /// Pops two values from the stack and calls the divide() function on the first one (the second one is an argument).
    /// Pushes the result back to the stack.
    Divide,    

    /// Pops two values from the stack and calls the compare_equal() function on the first one (the second one is an argument).
    /// Pushes the result back to the stack.
    Equal,

    /// Pops two values from the stack and calls the compare_not_equal() function on the first one (the second one is an argument).
    /// Pushes the result back to the stack.
    NotEqual,

    /// Pops two values from the stack and calls the greater() function on the first one (the second one is an argument).
    /// Pushes the result back to the stack.
    Greater,

    /// Pops two values from the stack and calls the less() function on the first one (the second one is an argument).
    /// Pushes the result back to the stack.
    Less,

    /// Pops two values from the stack and calls the greater_equal() function on the first one (the second one is an argument).
    /// Pushes the result back to the stack.
    GreaterEqual,

    /// Pops two values from the stack and calls the less_equal() function on the first one (the second one is an argument).
    /// Pushes the result back to the stack.
    LessEqual,

    /// Pops two values from the stack and returns True if both are True; False otherwise.     
    /// If A or B are not booleans, it throws an error.
    And,

    /// Pops two values from the stack and returns True if either of them is True; False otherwise.     
    /// If A or B are not booleans, it throws an error.
    Or,
        
    /// Pushes a Nil value to the stack
    PushNil,

    /// Pushes a Boolean to the stack 
    PushBool(bool),

    /// Pushes a number ot the stack
    PushNumber(Number),  

    /// Pushes a reference (index) to the Heap into the Stack
    PushHeapRef(u8),
    
    /// Clones the an element on the stack and pushes it at the 
    /// end of the stack
    GetLocal(u8),

    /// Replaces a value in the stack by another value
    SetLocal(u8),

    /// Pushes a HeapRef into the stack. It must point to a function (or it fails)
    GetGlobal(u8),

    /// Pushes a PackageRef to the stack
    GetFromPackage(usize),

    /// Pops values from the stack
    Pop(u8),    
    
    /// Loops (unimplemented )
    ForLoop(u8,u8),

    /// Checks if the last value in the stack is False. If it 
    /// is, it jumps N operations
    JumpIfFalse(u8),

    /// Checks if the last value in the stack is True. If it 
    /// is, it jumps N operations
    JumpIfTrue(u8),

    /// Jumps back N operation
    JumpBack(u8),

    /// Calls a function
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