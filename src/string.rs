use std::rc::Rc;

use crate::value_trait::ValueTrait;
use crate::values::Value;

pub type StringV = String;

impl ValueTrait for StringV {

    fn type_name(&self)->String{
        format!("String")
    }

    fn to_string(&self)->String{
        format!("{}",self)
    }

    fn clone_to_value(&self)->Value{
        Value::StringV(Rc::new(
            ValueTrait::to_string(self)
        ))
    }
}