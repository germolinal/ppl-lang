use crate::value_trait::ValueTrait;


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
}


