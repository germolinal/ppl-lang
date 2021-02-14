use crate::value_trait::ValueTrait;
use std::any::Any;


pub struct Nil;

impl Nil {
    pub fn new()->Self{
        Nil{}
    }
}


impl ValueTrait for Nil {
    fn to_string(&self)->String{
        "nil".to_string()
    }

    fn type_name(&self)->String{
        "Nil".to_string()
    }        

    fn as_any(&self) -> &dyn Any{
        self
    } 
    
}


