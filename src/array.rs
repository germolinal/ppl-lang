use std::rc::Rc;

use crate::value_trait::ValueTrait;
use crate::values::Value;


pub type Array = Vec<Value>;


impl ValueTrait for Array {

    fn to_string(&self)->String{        
        format!("Array (length {})", self.len())
    }

    fn type_name(&self)->String{
        format!("Array")
    }

    fn clone_to_value(&self)->Value {
        let mut ret : Array = Vec::with_capacity(self.len());

        
        for v in self.iter(){
            ret.push(v.clone_to_value());
        }

        Value::Array(Rc::new(ret))
    }
}
