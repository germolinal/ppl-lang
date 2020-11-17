//use crate::variable::Var;
//use std::rc::Rc;

//use crate::value_trait::ValueTrait;
//use crate::string::StringV;
//use crate::object::Object;
//use crate::function::Function;
use crate::values::Value;

/// Lists the operations available for the virtual machine
#[repr(u8)]
//#[derive(Clone)]
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
    //Constant(usize),    
    PushBool(bool),
    PushNumber(f64),  

    PushHeapRef(usize),
    PushVarRef(usize),
    EvalVar(usize),
    //PushFunction(Function),
    //PushString(Box<StringV>),
    //PushArray(usize), // the length of the array
    //PushObject(Box<Object>),
    //PushGeneric(Box<dyn ValueTrait>),

    PopVars(usize),
    DefineVars(usize),
    PushVar(Value),
    
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