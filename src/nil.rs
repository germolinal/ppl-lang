use crate::value_trait::ValueTrait;
use crate::values::Value;

#[derive(Default)]
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

    

    fn clone(&self)->Value{
        Value::Nil
    }

}


