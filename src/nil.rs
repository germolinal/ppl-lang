use crate::value_trait::ValueTrait;
use crate::values::Value;
use std::any::Any;


pub struct Nil;

impl Nil {
    pub fn new()->Self{
        Nil{}
    }
}


impl ValueTrait for Nil {
    fn to_string(&self)->String{
        return format!("Nil");
    }

    fn type_name(&self)->String{
        return format!("Nil")
    }        

    fn as_any(&self) -> &dyn Any{
        self
    }    

    fn clone_to_value(&self)->Value{
        Value::Nil        
    }

}


