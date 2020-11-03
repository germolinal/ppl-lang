
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
    content: bool
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
    Number(Option<f64>),          
    Bool(Option<bool>),    
    Object(Option<&'a Object>),
    Nil,
}

impl <'a>Value<'a> {

    /// Constructs a Nil 
    pub fn new_nil()->Self{
        Value::Nil
    }

    /// Constructs a number
    pub fn new_number(v: f64)->Self{
        Value::Number(Some(v))
    }


    /// Constructs a boolean
    pub fn new_bool(v: bool)->Self{
        Value::Bool(Some(v))
    }

    /// Constructs an Object
    pub fn new_object(v: &'a Object)->Self{
        Value::Object(Some(v))
    }

    /// Constructs a number
    pub fn new_empty_number()->Self{
        Value::Number(None)
    }


    /// Constructs a boolean
    pub fn new_empty_bool()->Self{
        Value::Bool(None)
    }

    /// Constructs an Object
    pub fn new_empty_object()->Self{
        Value::Object(None)
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
                match v {
                    Some(v2)=>Ok(*v2),
                    None => Err(format!("Trying to get a number out of an uninitialized 'Number' variable"))
                }
            },
            _ => Err(format!("Trying to get number out of '{}'",self.typename()))
        }
    }

    /// Retrieves the boolean contained within the 
    /// value, returns a Result.
    pub fn unrwap_boolean(&self)->Result<bool, String>{
        match self {
            Value::Bool(v) => {
                match v {
                    Some(v2)=>Ok(*v2),
                    None => Err(format!("Trying to get a boolean out of an uninitialized 'Boolean' variable"))
                }
            },
            _ => Err(format!("Trying to get a boolean out of '{}'",self.typename()))        
        }
    }

    /// Retrieves the boolean contained within the 
    /// value, returns a Result.
    pub fn unrwap_object(&self)->Result<&Object, String>{
        match self {
            Value::Object(v) => {
                match v {
                    Some(v2)=>Ok(v2),
                    None => Err(format!("Trying to get an Object out of an uninitialized 'Object' variable"))
                }
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
                match v {
                    Some(v2)=>format!("{}",v2),
                    None => format!("'empty boolean'"),
                }
            },
            Value::Number(v) =>{
                match v {
                    Some(v2)=>format!("{}",v2),
                    None => format!("'empty number'"),
                }
            },
            Value::Object(v) =>{
                match v {
                    Some(v2)=>format!("Object[{}]",v2.class()),
                    None => format!("'empty number'"),
                }
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
    fn test_nil_to_f64() {        
        assert!(Value::new_nil().to_f64().is_err());
    }
    
    #[test]
    fn test_float_to_f64() {        
        let exp = 1.12312;
        let v = Value::new_number(exp);
        let found = v.to_f64().unwrap();
        assert_eq!(exp,found);
    }
    
    
    
}
