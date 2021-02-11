use std::any::Any;

use crate::value_trait::ValueTrait;


pub type StringV = String;

impl ValueTrait for StringV {

    fn type_name(&self)->String{
        format!("String")
    }

    fn to_string(&self)->String{
        format!("{}",self)
    }

    fn as_any(&self) -> &dyn Any{
        self
    }
        
}