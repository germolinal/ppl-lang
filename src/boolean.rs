use crate::value_trait::ValueTrait;
use crate::values::Value;
use std::any::Any;

pub type Boolean = bool;

impl ValueTrait for Boolean {

    fn to_string(&self) -> String {
        format!("{}",self)
    }

    fn type_name(&self)->String{
        format!("Boolean")
    }

    fn as_any(&self) -> &dyn Any{
        self
    }

    

    fn clone_to_value(&self)->Value{
        Value::Bool(*self)
    }
    
    fn not(&self)->Result<Value,String>{
        Ok(Value::Bool(!self))
    }

    fn compare_equal(&self,other: &Value)->Result<Value,String>{
        if let Value::Bool(b) = other{
            Ok(Value::Bool(b == self))
        }else{
            Err(format!("Trying to compare Bool with {}",other.type_name()))
        }
    }

}