
use std::rc::Rc;

use crate::value_trait::ValueTrait;

use crate::nil::Nil;
use crate::number::Number;
use crate::boolean::Boolean;
use crate::function::Function;
use crate::array::Array;
use crate::object::Object;
use crate::string::StringV;


pub enum Value {
    /* Empty variable */

    Nil,

    /* Stack allocated variables */

    Number(Number),
    Bool(Boolean),
    Function(Function),
    
    /* Heap allocated variables */
    StringV(Rc<StringV>),
    Array(Rc<Array>),
    Object(Rc<Object>),

    
    
    /// Generic object, for extending
    /// this language
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

    pub fn get_array(&self)->Option<&Array>{
        match self{
            Value::Array(v)=>Some(v),
            _ => None
        }
    }

    pub fn get_object(&self)->Option<&Object>{
        match self{
            Value::Object(v)=>Some(v),
            _ => None
        }
    }

    pub fn get_string(&self)->Option<&StringV>{
        match self{
            Value::StringV(v)=>Some(v),
            _ => None
        }
    }

    pub fn get_function(&self)->Option<Function>{
        match self{
            Value::Function(v)=>Some(v.clone()),
            _ => None
        }
    }

    
    pub fn get_generic(&self)->Option<&Rc<dyn ValueTrait>>{
        match self {
            Value::Generic(v)=>Some(v),                            
            _ => None
        }
    }
    

    pub fn is_nil(&self)-> bool {
        match self {
            Value::Nil => true,
            _ => false
        }
    }

    
    
    pub fn copy_to_value(&self)-> Value {
        match self{
            Value::Nil=>ValueTrait::clone_to_value(&Nil),
            Value::Number(v)=>ValueTrait::clone_to_value(v),
            Value::Bool(v)=>ValueTrait::clone_to_value(v),
            Value::Function(v)=>Value::Function(v.clone()),
            Value::StringV(v)=>{
                // We clone the String.                
                v.clone_to_value()
            },
            Value::Array(v)=>{
                // We copy the reference to the Array
                Value::Array(Rc::clone(v))
            },
            Value::Object(v)=>{
                // We copy the reference to the Object
                Value::Object(Rc::clone(v))
            },
            Value::Generic(v)=>Value::Generic(Rc::clone(&v))
        }

    }
        
}



impl ValueTrait for Value  {

    fn type_name(&self)->String{        
        format!("Value({})",match self {
            Value::Nil=>Nil.type_name(),
            Value::Number(v)=>v.type_name(),
            Value::Bool(v)=>v.type_name(),
            Value::Function(v)=>v.type_name(),
            Value::Array(v)=>v.type_name(),
            Value::StringV(v)=>v.type_name(),
            Value::Object(v)=>v.type_name(),
            Value::Generic(v)=>v.type_name()
        })
    }

    fn to_string(&self)->String{        
        match self {
            Value::Nil => ValueTrait::to_string(&Nil::new()),
            Value::Number(v) => ValueTrait::to_string(v),
            Value::Bool(v) => ValueTrait::to_string(v),
            Value::Function(v)=>v.to_string(),
            Value::Array(v)=> v.to_string(),
            Value::StringV(v)=> v.to_string(),
            Value::Object(v)=> v.to_string(),
            Value::Generic(v) => v.to_string(),            
        }
    }

    
    fn clone_to_value(&self)-> Value {
        match self{
            Value::Nil=>ValueTrait::clone_to_value(&Nil),
            Value::Number(v)=>ValueTrait::clone_to_value(v),
            Value::Bool(v)=>ValueTrait::clone_to_value(v),
            // functions are cloned by reference
            Value::Function(v)=>Value::Function(v.clone()),
            Value::Array(v)=> {                
                v.clone_to_value()
            },
            Value::StringV(v)=>{                
                v.clone_to_value()
            },
            Value::Object(v)=>{                
                v.clone_to_value()
            },
            Value::Generic(v)=>{                
                v.clone_to_value()
            },
        }
    }
    


    fn not(&self)->Result<Value,String>{
        match self{
            Value::Nil=>Nil.not(),
            Value::Number(v)=>v.not(),
            Value::Bool(v)=>v.not(),
            Value::Function(v)=>v.not(),
            Value::Array(v)=>v.not(),
            Value::StringV(v)=>v.not(),
            Value::Object(v)=>v.not(),
            Value::Generic(v)=>v.not(),
        }
    }

    fn negate(&self)->Result<Value,String>{
        match self{
            Value::Nil=>Nil.negate(),
            Value::Number(v)=>v.negate(),
            Value::Bool(v)=>v.negate(),
            Value::Function(v)=>v.negate(),
            Value::Array(v)=>v.negate(),
            Value::StringV(v)=>v.negate(),
            Value::Object(v)=>v.negate(),
            Value::Generic(v)=>v.negate(),
        }
    }

    fn add(&self, other: &Value)->Result<Value,String>{        
        match self {
            Value::Nil=>Nil.add(other),
            Value::Number(v)=>v.add(other),
            Value::Bool(v)=>v.add(other),
            Value::Function(v)=>v.add(other),
            Value::Array(v)=>v.add(other),
            Value::StringV(v)=>v.add(other),
            Value::Object(v)=>v.add(other),
            Value::Generic(v)=>v.add(other),
        }
    }

    fn subtract(&self, other: &Value)->Result<Value,String>{        
        match self {
            Value::Nil => Nil::new().subtract(other),
            Value::Number(v) => v.subtract(other),
            Value::Bool(v) => v.subtract(other),
            Value::Function(v)=>v.subtract(other),
            Value::Array(v)=>v.subtract(other),
            Value::StringV(v)=>v.subtract(other),
            Value::Object(v)=>v.subtract(other),
            Value::Generic(v) => v.subtract(other),            
        }
    }

    fn multiply(&self, other: &Value)->Result<Value,String>{        
        match self {
            Value::Nil => Nil::new().multiply(other),
            Value::Number(v) => v.multiply(other),
            Value::Bool(v) => v.multiply(other),
            Value::Function(v)=>v.multiply(other),
            Value::Array(v)=>v.multiply(other),
            Value::StringV(v)=>v.multiply(other),
            Value::Object(v)=>v.multiply(other),
            Value::Generic(v) => v.multiply(other),            
        }
    }

    fn divide(&self, other: &Value)->Result<Value,String>{        
        match self {
            Value::Nil => Nil::new().divide(other),
            Value::Number(v) => v.divide(other),
            Value::Bool(v) => v.divide(other),
            Value::Function(v)=>v.divide(other),
            Value::Array(v)=>v.divide(other),
            Value::StringV(v)=>v.divide(other),
            Value::Object(v)=>v.divide(other),
            Value::Generic(v) => v.divide(other),            
        }
    }

    fn compare_equal(&self, other: &Value)->Result<Value,String>{
        match self {
            Value::Nil => Nil::new().compare_equal(other),
            Value::Number(v) => v.compare_equal(other),
            Value::Bool(v) => v.compare_equal(other),
            Value::Function(v)=>v.compare_equal(other),
            Value::Array(v)=>v.compare_equal(other),
            Value::StringV(v)=>v.compare_equal(other),
            Value::Object(v)=>v.compare_equal(other),
            Value::Generic(v) => v.compare_equal(other),            
        }
    }

    fn compare_not_equal(&self, other: &Value)->Result<Value,String>{
        match self {
            Value::Nil => Nil::new().compare_not_equal(other),
            Value::Number(v) => v.compare_not_equal(other),
            Value::Bool(v) => v.compare_not_equal(other),
            Value::Function(v)=>v.compare_not_equal(other),
            Value::Array(v)=>v.compare_not_equal(other),
            Value::StringV(v)=>v.compare_not_equal(other),
            Value::Object(v)=>v.compare_not_equal(other),
            Value::Generic(v) => v.compare_not_equal(other),            
        }
    }

    fn greater(&self, other: &Value)->Result<Value,String>{
        match self {
            Value::Nil => Nil::new().greater(other),
            Value::Number(v) => v.greater(other),
            Value::Bool(v) => v.greater(other),
            Value::Function(v)=>v.greater(other),
            Value::Array(v)=>v.greater(other),
            Value::StringV(v)=>v.greater(other),
            Value::Object(v)=>v.greater(other),
            Value::Generic(v) => v.greater(other),            
        }
    }

    fn less(&self, other: &Value)->Result<Value,String>{
        match self {
            Value::Nil => Nil::new().less(other),
            Value::Number(v) => v.less(other),
            Value::Bool(v) => v.less(other),
            Value::Function(v)=>v.less(other),
            Value::Array(v)=>v.less(other),
            Value::StringV(v)=>v.less(other),
            Value::Object(v)=>v.less(other),
            Value::Generic(v) => v.less(other),            
        }
    }

    fn greater_equal(&self, other: &Value)->Result<Value,String>{
        match self {
            Value::Nil => Nil::new().greater_equal(other),
            Value::Number(v) => v.greater_equal(other),
            Value::Bool(v) => v.greater_equal(other),
            Value::Function(v)=>v.greater_equal(other),
            Value::Array(v)=>v.greater_equal(other),
            Value::StringV(v)=>v.greater_equal(other),
            Value::Object(v)=>v.greater_equal(other),
            Value::Generic(v) => v.greater_equal(other),            
        }        
    }

    fn less_equal(&self, other: &Value)->Result<Value,String>{
        match self {
            Value::Nil => Nil::new().less_equal(other),
            Value::Number(v) => v.less_equal(other),
            Value::Bool(v) => v.less_equal(other),
            Value::Function(v)=>v.less_equal(other),
            Value::Array(v)=>v.less_equal(other),
            Value::StringV(v)=>v.less_equal(other),
            Value::Object(v)=>v.less_equal(other),
            Value::Generic(v) => v.less_equal(other),            
        }              
    }
}


