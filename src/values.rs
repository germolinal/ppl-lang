
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

#[derive(Copy,Clone)]
pub struct Value<'a>{
    value_type: ValueType,
    number: Option<f64>,
    boolean : Option<bool>,
    object: Option<&'a Object>
}

impl <'a>Value<'a> {

    /// Constructs a Nil 
    pub fn new_nil()->Self{
        Self{
            value_type: ValueType::Nil,
            number: None,
            boolean: None,
            object: None,
        }
    }

    /// Constructs a number
    pub fn new_number(v: f64)->Self{
        Self{
            value_type: ValueType::Number,
            number: Some(v),
            boolean: None,
            object: None,
        }
    }


    /// Constructs a boolean
    pub fn new_bool(v: bool)->Self{
        Self{
            value_type: ValueType::Bool,
            number: None,
            boolean: Some(v),
            object: None,
        }
    }

    /// Constructs an Object
    pub fn new_object(v: &'a Object)->Self{
        Self{
            value_type: ValueType::Object,
            number: None,
            boolean: None,
            object: Some(v),
        }
    }

    /// Gets the type of the value as a String 
    /// This is for giving feedback to the user... not 
    /// for internal use.
    pub fn typename(&self)-> &str {
        match self.value_type{            
            ValueType::Number => "Number",            
            ValueType::Bool => "Boolean",
            ValueType::Object => "Object",                
            ValueType::Nil => "Nil"
        }
    }

    /// Retrieves the ValueType
    pub fn value_type(&self)->ValueType{
        self.value_type
    }

    /// Retrieves the number contained within the 
    /// value, returns a Result.
    pub fn unrwap_number(&self)->Result<f64, String>{
        match self.value_type {
            ValueType::Number => {
                match self.number {
                    Some(v)=>Ok(v),
                    None => Err(format!("Trying to get a number out of an uninitialized 'Number' variable"))
                }
            },
            _ => Err(format!("Trying to get number out of '{}'",self.typename()))
        }
    }

    /// Retrieves the boolean contained within the 
    /// value, returns a Result.
    pub fn unrwap_boolean(&self)->Result<bool, String>{
        match self.value_type {
            ValueType::Bool => {
                match self.boolean {
                    Some(v)=>Ok(v),
                    None => Err(format!("Trying to get a boolean out of an uninitialized 'Boolean' variable"))
                }
            },
            _ => Err(format!("Trying to get a boolean out of '{}'",self.typename()))        
        }
    }

    /// Retrieves the boolean contained within the 
    /// value, returns a Result.
    pub fn unrwap_object(&self)->Result<&Object, String>{
        match self.value_type {
            ValueType::Object => {
                match self.object {
                    Some(v)=>Ok(v),
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
        match self.value_type { 
            // Numbers are easy
            ValueType::Number => self.unrwap_number(),                        
            _ => Err(format!("Cannot transform type '{}' into 'f64'", self.typename()))        
        }
    }
    

    pub fn to_bool(&self)->Result<bool,String>{
        match self.value_type {            
            ValueType::Bool =>self.unrwap_boolean(),
            _ => Err(format!("Cannot transform type '{}' into 'bool'", self.typename()))        
        }
    }

    pub fn to_string(&self)->String{
        match self.value_type {
            ValueType::Bool =>{
                match self.unrwap_boolean(){
                    Ok(v)=>format!("{}",v),
                    Err(_) => format!("'empty boolean'"),
                }
            },
            ValueType::Number =>{
                match self.unrwap_number(){
                    Ok(v)=>format!("{}",v),
                    Err(_) => format!("'empty number'"),
                }
            },
            ValueType::Object =>{
                match self.unrwap_object(){
                    Ok(v)=>format!("Object[{}]",v.class()),
                    Err(_) => format!("'empty number'"),
                }
            },
            ValueType::Nil => format!("Nil")                
            
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
