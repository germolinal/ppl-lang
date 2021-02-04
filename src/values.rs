use std::any::Any;

use crate::value_trait::ValueTrait;

use crate::nil::Nil;
use crate::number::Number;
use crate::boolean::Boolean;


#[derive(Copy,Clone)]
pub enum Value {
    
    /// This represents an empty variable
    /// and should throw an error when it is 
    /// used (e.g., 3 * nil ==> Error).
    Nil,

    /// A number, fully allocated in the stack
    Number(Number),

    /// A Boolean, fully allocated in the stack
    Bool(Boolean),
        
    /// A reference to an object allocated in the heap
    HeapRef(usize),    

    /// A reference to an object allocated in the 
    /// package elements vector
    PackageRef(usize), 

    
        
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

    /*
    pub fn get_array(&self)->Option<&Array>{
        match self{
            //Value::Array(v)=>Some(v),
            _ => None
        }
    }

    pub fn get_object(&self)->Option<&Object>{
        match self{
            //Value::Object(v)=>Some(v),
            _ => None
        }
    }

    pub fn get_string(&self)->Option<&StringV>{
        match self{
            //Value::StringV(v)=>Some(v),
            _ => None
        }
    }
    
    pub fn get_function(&self)->Option<Function>{
        match self{
            Value::Function(v)=>Some(v.clone()),
            _ => None
        }
    }
    */

    
    
    

    pub fn is_nil(&self)-> bool {
        match self {
            Value::Nil => true,
            _ => false
        }
    }

    
    
    /*
    pub fn copy_to_value(&self) -> Value {
        match self{
            Value::Nil=>ValueTrait::clone_to_value(&Nil),
            Value::Number(v)=>ValueTrait::clone_to_value(v),
            Value::Bool(v)=>ValueTrait::clone_to_value(v),
            Value::Function(v)=>Value::Function(v.clone()),
            //Value::StringV(v)=>{
                // We clone the String.                
                v.clone_to_value()
            },
            //Value::Array(v)=>{
                // We copy the reference to the Array
                //Value::Array(Rc::clone(v))
            },
            //Value::Object(v)=>{
                // We copy the reference to the Object
                //Value::Object(Rc::clone(v))
            },
            //Value::Generic(v)=>//Value::Generic(Rc::clone(&v))
        }

    }
    */
        
}



impl ValueTrait for Value  {

    fn type_name(&self)->String{        
        format!("Value({})",match self {
            Value::Nil=>Nil.type_name(),
            Value::Number(v)=>v.type_name(),
            Value::Bool(v)=>v.type_name(),
            Value::HeapRef(_)=>format!("HeapReference"),
            Value::PackageRef(_)=>format!("PackageReference"),
            
            
        })
    }

    fn to_string(&self)->String{        
        match self {
            Value::Nil => ValueTrait::to_string(&Nil::new()),
            Value::Number(v) => ValueTrait::to_string(v),
            Value::Bool(v) => ValueTrait::to_string(v),
            Value::HeapRef(i)=>format!("HeapRef<{}>", i),                                  
            Value::PackageRef(i)=>format!("PackageRef<{}>", i),                                  
        }
    }

    
    fn clone_to_value(&self)-> Value {
        match self{
            Value::Nil=>ValueTrait::clone_to_value(&Nil),
            Value::Number(v)=>ValueTrait::clone_to_value(v),
            Value::Bool(v)=>ValueTrait::clone_to_value(v),            
            Value::HeapRef(i)=>{
                Value::HeapRef(*i)
            },       
            Value::PackageRef(i)=>{
                Value::PackageRef(*i)
            },                                  
        }
    }
    
    
    fn as_any(&self) -> &dyn Any{
        self
    }
    


    fn not(&self)->Result<Value,String>{
        match self{
            Value::Nil=>Nil.not(),
            Value::Number(v)=>v.not(),
            Value::Bool(v)=>v.not(),            
            Value::HeapRef(_)=>panic!("Trying to opeate over a Heap reference"),            
            Value::PackageRef(_)=>panic!("Trying to opeate over a Package reference"),            
        }
    }

    fn negate(&self)->Result<Value,String>{
        match self{
            Value::Nil=>Nil.negate(),
            Value::Number(v)=>v.negate(),
            Value::Bool(v)=>v.negate(),            
            Value::HeapRef(_)=>panic!("Trying to opeate over a Heap reference"),            
            Value::PackageRef(_)=>panic!("Trying to opeate over a Package reference"),            
        }
    }

    fn add(&self, other: &Value)->Result<Value,String>{        
        match self {
            Value::Nil=>Nil.add(other),
            Value::Number(v)=>v.add(other),
            Value::Bool(v)=>v.add(other),            
            Value::HeapRef(_)=>panic!("Trying to opeate over a Heap reference"),            
            Value::PackageRef(_)=>panic!("Trying to opeate over a Package reference"),            
        }
    }

    fn subtract(&self, other: &Value)->Result<Value,String>{        
        match self {
            Value::Nil => Nil::new().subtract(other),
            Value::Number(v) => v.subtract(other),
            Value::Bool(v) => v.subtract(other),            
            Value::HeapRef(_)=>panic!("Trying to opeate over a Heap reference"),            
            Value::PackageRef(_)=>panic!("Trying to opeate over a Package reference"),            
        }
    }

    fn multiply(&self, other: &Value)->Result<Value,String>{        
        match self {
            Value::Nil => Nil::new().multiply(other),
            Value::Number(v) => v.multiply(other),
            Value::Bool(v) => v.multiply(other),            
            Value::HeapRef(_)=>panic!("Trying to opeate over a Heap reference"),            
            Value::PackageRef(_)=>panic!("Trying to opeate over a Package reference"),            
            
        }
    }

    fn divide(&self, other: &Value)->Result<Value,String>{        
        match self {
            Value::Nil => Nil::new().divide(other),
            Value::Number(v) => v.divide(other),
            Value::Bool(v) => v.divide(other),            
            Value::HeapRef(_)=>panic!("Trying to opeate over a Heap reference"),            
            Value::PackageRef(_)=>panic!("Trying to opeate over a Package reference"),            
        }
    }

    fn compare_equal(&self, other: &Value)->Result<Value,String>{
        match self {
            Value::Nil => Nil::new().compare_equal(other),
            Value::Number(v) => v.compare_equal(other),
            Value::Bool(v) => v.compare_equal(other),
            Value::HeapRef(_)=>panic!("Trying to opeate over a Heap reference"),            
            Value::PackageRef(_)=>panic!("Trying to opeate over a Package reference"),            
        }
    }

    fn compare_not_equal(&self, other: &Value)->Result<Value,String>{
        match self {
            Value::Nil => Nil::new().compare_not_equal(other),
            Value::Number(v) => v.compare_not_equal(other),
            Value::Bool(v) => v.compare_not_equal(other),
            Value::HeapRef(_)=>panic!("Trying to opeate over a Heap reference"),            
            Value::PackageRef(_)=>panic!("Trying to opeate over a Package reference"),            
        }
    }

    fn greater(&self, other: &Value)->Result<Value,String>{
        match self {
            Value::Nil => Nil::new().greater(other),
            Value::Number(v) => v.greater(other),
            Value::Bool(v) => v.greater(other),
            Value::HeapRef(_)=>panic!("Trying to opeate over a Heap reference"),            
            Value::PackageRef(_)=>panic!("Trying to opeate over a Package reference"),            
        }
    }

    fn less(&self, other: &Value)->Result<Value,String>{
        match self {
            Value::Nil => Nil::new().less(other),
            Value::Number(v) => v.less(other),
            Value::Bool(v) => v.less(other),
            Value::HeapRef(_)=>panic!("Trying to opeate over a Heap reference"),            
            Value::PackageRef(_)=>panic!("Trying to opeate over a Package reference"),            
        }
    }

    fn greater_equal(&self, other: &Value)->Result<Value,String>{
        match self {
            Value::Nil => Nil::new().greater_equal(other),
            Value::Number(v) => v.greater_equal(other),
            Value::Bool(v) => v.greater_equal(other),
            Value::HeapRef(_)=>panic!("Trying to opeate over a Heap reference"),            
            Value::PackageRef(_)=>panic!("Trying to opeate over a Package reference"),            
        }        
    }

    fn less_equal(&self, other: &Value)->Result<Value,String>{
        match self {
            Value::Nil => Nil::new().less_equal(other),
            Value::Number(v) => v.less_equal(other),
            Value::Bool(v) => v.less_equal(other),
            Value::HeapRef(_)=>panic!("Trying to opeate over a Heap reference"),            
            Value::PackageRef(_)=>panic!("Trying to opeate over a Package reference"),            
        }              
    }
}


