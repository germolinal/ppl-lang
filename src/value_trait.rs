use crate::values::Value;
use crate::vm::VM;
use std::any::Any;

pub trait ValueTrait {
    // Basic i/o
    fn to_string(&self)->String;

    fn type_name(&self)->String;

    fn as_any(&self) -> &dyn Any;

    // Copy and clone
    fn clone_to_value(&self)->Value;

    fn call(&self, _h: &mut VM, _n: usize)->Result<usize,String>{
        Err(format!("Cannot Call type '{}'... it is not a Function", self.type_name()))
    }

    // Loops.
    fn get_next(&self)->Option<(Value,Value)>{
        None
    }
    

    // Operators
    fn not(&self)->Result<Value,String>{
        Err(format!("Operator '!' cannot be applied to type '{}'", self.type_name()))
    }
    
    fn negate(&self)->Result<Value,String>{
        Err(format!("Operator '-' cannot be applied to type '{}'", self.type_name()))
    }

    fn add(&self, _other: &Value)->Result<Value,String>{
        Err(format!("Operator '+' cannot be applied to type '{}'", self.type_name()))
    }

    fn subtract(&self, _other: &Value)->Result<Value,String>{
        Err(format!("Operator '-' cannot be applied to type '{}'", self.type_name()))
    }

    fn multiply(&self, _other: &Value)->Result<Value,String>{
        Err(format!("Operator '*' cannot be applied to type '{}'", self.type_name()))
    }

    fn divide(&self, _other: &Value)->Result<Value,String>{
        Err(format!("Operator '/' cannot be applied to type '{}'", self.type_name()))
    }

    fn compare_equal(&self, _other: &Value)->Result<Value,String>{
        Err(format!("Operator '==' cannot be applied to type '{}'", self.type_name()))
    }

    fn compare_not_equal(&self, other: &Value)->Result<Value,String>{
        match self.compare_equal(other){
            Ok(v)=>Ok(Value::Bool(!v.get_bool().unwrap())),
            Err(e)=>Err(e),
        }
    }

    fn greater(&self, _other: &Value)->Result<Value,String>{
        Err(format!("Operator '>' cannot be applied to type '{}'", self.type_name()))
    }

    fn less(&self, _other: &Value)->Result<Value,String>{
        Err(format!("Operator '<' cannot be applied to type '{}'", self.type_name()))
    }

    fn greater_equal(&self, other: &Value)->Result<Value,String>{
        match self.less(other){
            Ok(v)=>Ok(Value::Bool(!v.get_bool().unwrap())),
            Err(e)=>Err(e),
        }        
    }

    fn less_equal(&self, other: &Value)->Result<Value,String>{
        match self.greater(other){
            Ok(v)=>Ok(Value::Bool(!v.get_bool().unwrap())),
            Err(e)=>Err(e),
        }      
    }

    
    
}