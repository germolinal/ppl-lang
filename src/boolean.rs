use crate::value_trait::ValueTrait;
use crate::values::Value;

pub type Boolean = bool;

impl ValueTrait for Boolean {

    fn to_string(&self) -> String {
        format!("{}",self)
    }

    fn type_name(&self)->String{
        format!("Boolean")
    }

    

    fn clone_to_value(&self)->Value{
        Value::Bool(*self)
    }
    
    fn not(&self)->Result<Value,String>{
        Ok(Value::Bool(!self))
    }

}