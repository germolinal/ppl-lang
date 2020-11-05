
#[derive(Copy, Clone)]
#[repr(u8)] 
#[derive(PartialEq,PartialOrd)]
pub enum ValueType {      
    Number,          
    Bool,    
    Object,
    Nil,
}

pub struct Object{
    _content: bool
}

impl Object {
    pub fn class(&self)->String{
        "some class".to_string()
    }
}

/*
#[derive(Copy,Clone)]
pub struct Value<'a>{
    value_type: ValueType,
    number: Option<f64>,
    boolean : Option<bool>,
    object: Option<&'a Object>
}
*/
#[derive(Copy,Clone)]
pub enum Value<'a>{
    Number(f64),          
    Bool(bool),    
    Object(&'a Object),
    Nil,
}

impl <'a>Value<'a> {
    
    /// Constructs a number
    pub fn new_number(v: f64)->Self{
        Value::Number(v)
    }

    /// Constructs a boolean
    pub fn new_bool(v: bool)->Self{
        Value::Bool(v)
    }

    /// Constructs an Object
    pub fn new_object(v: &'a Object)->Self{
        Value::Object(v)
    }

    

    /// Gets the type of the value as a String 
    /// This is for giving feedback to the user... not 
    /// for internal use.
    pub fn typename(&self)-> &str {
        match self {            
            Value::Number(_) => "Number",            
            Value::Bool(_) => "Boolean",
            Value::Object(_) => "Object",                
            Value::Nil => "Nil"
        }
    }

    /// Retrieves the ValueType
    pub fn value_type(&self)->ValueType{
        match self {            
            Value::Number(_) => ValueType::Number,            
            Value::Bool(_) => ValueType::Bool,
            Value::Object(_) => ValueType::Object,                
            Value::Nil => ValueType::Nil
        }
    }

    /// Retrieves the number contained within the 
    /// value, returns a Result.
    pub fn unrwap_number(&self)->Result<f64, String>{
        match self {
            Value::Number(v) => {
                Ok(*v)                
            },
            _ => Err(format!("Trying to get number out of '{}'",self.typename()))
        }
    }

    /// Retrieves the boolean contained within the 
    /// value, returns a Result.
    pub fn unrwap_boolean(&self)->Result<bool, String>{
        match self {
            Value::Bool(v) => {
                Ok(*v)                
            },
            _ => Err(format!("Trying to get a boolean out of '{}'",self.typename()))        
        }
    }

    /// Retrieves the boolean contained within the 
    /// value, returns a Result.
    pub fn unrwap_object(&self)->Result<&Object, String>{
        match self {
            Value::Object(v) => {
                Ok(*v)                
            },
            _ => Err(format!("Trying to get an Object out of '{}'",self.typename()))
        }
    }

    /// Retrieves the Value as an f64. This does not 
    /// return an option just to make it quicker, and 
    /// it is thought to be used internally.
    pub fn to_f64(&self) -> Result<f64,String> {
        match self { 
            // Numbers are easy
            Value::Number(_) => self.unrwap_number(),                        
            _ => Err(format!("Cannot transform type '{}' into 'f64'", self.typename()))        
        }
    }
    

    pub fn to_bool(&self)->Result<bool,String>{
        match self {            
            Value::Bool(_) =>self.unrwap_boolean(),
            _ => Err(format!("Cannot transform type '{}' into 'bool'", self.typename()))        
        }
    }

    pub fn to_string(&self)->String{
        match self {
            Value::Bool(v) =>{
                format!("{}",v)
            },
            Value::Number(v) =>{
                format!("{}",v)
            },
            Value::Object(v) =>{                
                format!("Object[{}]",v.class())
            },
            Value::Nil => format!("Nil")                
            
        }
    }


    
}



/***********/
/* TESTING */
/***********/

#[cfg(test)]
mod tests {
    use super::*;
    
    /*********/
    // TO f64
    /*********/
    

    #[test]
    fn test_float_to_f64() {        
        let exp = 1.12312;
        let v = Value::new_number(exp);
        let found = v.to_f64().unwrap();
        assert_eq!(exp,found);
    }
    
    
    
}
