use std::rc::Rc;
use std::collections::HashMap;

use crate::value_trait::ValueTrait;
use crate::values::Value;

pub type Object = HashMap<String,Value>;



impl ValueTrait for Object {
    
    fn to_string(&self)->String{
        format!("Object... to_string is unimplemented for now")
    }

    fn type_name(&self)->String{
        format!("Object")
    }

    fn clone_to_value(&self)->Value {
        let mut ret : Object = HashMap::new();
        
        for (k,v) in self.iter(){
            ret.insert(Clone::clone(k),ValueTrait::clone_to_value(v));
        }

        Value::Generic(Rc::new(ret))
    }
}

