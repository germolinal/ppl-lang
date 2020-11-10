use std::rc::Rc;

use crate::number::Number;
use crate::boolean::Boolean;
use crate::nil::Nil;
use crate::value_trait::ValueTrait;


pub enum Value {
    Nil,
    Number(Number),
    Bool(Boolean),
    Generic(Rc<dyn ValueTrait>),
}


impl Value {

    pub fn get_number(&self)->Option<Number>{
        match self {
            Value::Number(v)=>Some(*v),
            _ => None
        }
    }

    pub fn get_bool(&self)->Option<Boolean>{
        match self {
            Value::Bool(v)=>Some(*v),
            _ => None
        }
    }

    
    pub fn get_generic(&self)->Option<&Rc<dyn ValueTrait>>{
        match self {
            Value::Generic(v)=>Some(v),                            
            _ => None
        }
    }
    

    pub fn is_nil(&self)->bool{
        match self {
            Value::Nil => true,
            _ => false
        }
    }

    
    pub fn copy(&self)-> Value {
        match self {
            Value::Nil => Value::Nil,
            Value::Number(v) => ValueTrait::clone(v),
            Value::Bool(v) => ValueTrait::clone(v),
            Value::Generic(v) => Value::Generic(Rc::clone(v))  
        }
    }
    
    
}



impl ValueTrait for Value  {

    fn type_name(&self)->String{
        format!("Value")
    }

    fn to_string(&self)->String{
        match self {
            Value::Nil => ValueTrait::to_string(&Nil::new()),
            Value::Number(v) => ValueTrait::to_string(v),
            Value::Bool(v) => ValueTrait::to_string(v),
            Value::Generic(v) => v.to_string(),            
        }
    }

    
    fn clone(&self)-> Value {
        match self {
            Value::Nil => Value::Nil,
            Value::Number(v) => ValueTrait::clone(v),
            Value::Bool(v) => ValueTrait::clone(v),
            Value::Generic(v) => unimplemented!()  
        }
    }
    


    fn not(&self)->Result<Value,String>{
        match self {
            Value::Nil => Nil::new().not(),
            Value::Number(v) => v.not(),
            Value::Bool(v) => v.not(),
            Value::Generic(v) => v.not(),            
        }
    }

    fn negate(&self)->Result<Value,String>{
        match self {
            Value::Nil => Nil::new().negate(),
            Value::Number(v) => v.negate(),
            Value::Bool(v) => v.negate(),
            Value::Generic(v) => v.negate(),            
        }
    }

    fn add(&self, other: Value)->Result<Value,String>{        
        match self {
            Value::Nil => Nil::new().add(other),
            Value::Number(v) => v.add(other),
            Value::Bool(v) => v.add(other),
            Value::Generic(v) => v.add(other),            
        }
    }

    fn subtract(&self, other: Value)->Result<Value,String>{        
        match self {
            Value::Nil => Nil::new().subtract(other),
            Value::Number(v) => v.subtract(other),
            Value::Bool(v) => v.subtract(other),
            Value::Generic(v) => v.subtract(other),            
        }
    }

    fn multiply(&self, other: Value)->Result<Value,String>{        
        match self {
            Value::Nil => Nil::new().multiply(other),
            Value::Number(v) => v.multiply(other),
            Value::Bool(v) => v.multiply(other),
            Value::Generic(v) => v.multiply(other),            
        }
    }

    fn divide(&self, other: Value)->Result<Value,String>{        
        match self {
            Value::Nil => Nil::new().divide(other),
            Value::Number(v) => v.divide(other),
            Value::Bool(v) => v.divide(other),
            Value::Generic(v) => v.divide(other),            
        }
    }

    fn compare_equal(&self, other: Value)->Result<Value,String>{
        match self {
            Value::Nil => Nil::new().compare_equal(other),
            Value::Number(v) => v.compare_equal(other),
            Value::Bool(v) => v.compare_equal(other),
            Value::Generic(v) => v.compare_equal(other),            
        }
    }

    fn compare_not_equal(&self, other: Value)->Result<Value,String>{
        match self {
            Value::Nil => Nil::new().compare_not_equal(other),
            Value::Number(v) => v.compare_not_equal(other),
            Value::Bool(v) => v.compare_not_equal(other),
            Value::Generic(v) => v.compare_not_equal(other),            
        }
    }

    fn greater(&self, other: Value)->Result<Value,String>{
        match self {
            Value::Nil => Nil::new().greater(other),
            Value::Number(v) => v.greater(other),
            Value::Bool(v) => v.greater(other),
            Value::Generic(v) => v.greater(other),            
        }
    }

    fn less(&self, other: Value)->Result<Value,String>{
        match self {
            Value::Nil => Nil::new().less(other),
            Value::Number(v) => v.less(other),
            Value::Bool(v) => v.less(other),
            Value::Generic(v) => v.less(other),            
        }
    }

    fn greater_equal(&self, other: Value)->Result<Value,String>{
        match self {
            Value::Nil => Nil::new().greater_equal(other),
            Value::Number(v) => v.greater_equal(other),
            Value::Bool(v) => v.greater_equal(other),
            Value::Generic(v) => v.greater_equal(other),            
        }        
    }

    fn less_equal(&self, other: Value)->Result<Value,String>{
        match self {
            Value::Nil => Nil::new().less_equal(other),
            Value::Number(v) => v.less_equal(other),
            Value::Bool(v) => v.less_equal(other),
            Value::Generic(v) => v.less_equal(other),            
        }              
    }
}


