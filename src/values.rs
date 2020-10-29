
#[derive(Copy, Clone)]
pub enum PPLValue<'a> {
    PPLNil,
    PPLFloat(f64),
    PPLInt(i32),
    PPLString(&'a String),    
    PPLBool(bool),    
}


impl <'a>PPLValue<'a> {

    /// Gets the type of the value as a String 
    /// This is for giving feedback to the user... not 
    /// for internal use.
    pub fn ppl_type(&self)-> &str {
        match self{
            PPLValue::PPLNil => "Nil",
            PPLValue::PPLFloat(_) => "Float",
            PPLValue::PPLInt(_) => "Integer",
            PPLValue::PPLString(_) => "String",
            PPLValue::PPLBool(_) => "Boolean",
        }
    }

    /// Retrieves the Value as an f64. This does not 
    /// return an option just to make it quicker, and 
    /// it is thought to be used internally.
    pub fn to_f64(&self) -> f64 {
        match self {
            // Numbers are easy
            PPLValue::PPLFloat(v)=>*v,
            PPLValue::PPLInt(v)=>*v as f64,

            // We will try with string
            PPLValue::PPLString(v)=>{
                let s = v.parse::<f64>();
                match s {
                    Ok(v) => v,
                    Err(_) => panic!("Cannot transform string '{}' into 'f64'", v)
                }                
            }
            _ => panic!("Cannot transform type '{}' into 'f64'", self.ppl_type())
        }
    }

    /// Retrieves the Value as an i32. This does not 
    /// return an option just to make it quicker, and 
    /// it is thought to be used internally.
    pub fn to_i32(&self)->i32{
        match self {
            // Numbers are easy
            PPLValue::PPLFloat(v)=>*v as i32,
            PPLValue::PPLInt(v)=>*v as i32,
            // Let's try with string
            PPLValue::PPLString(v)=>{
                let s = v.parse::<i32>();
                match s {
                    Ok(v) => v,
                    Err(_) => panic!("Cannot transform string '{}' into 'i32'", v)
                }                
            }
            _ => panic!("Cannot transform type '{}' into 'i32'", self.ppl_type())
        }
    }

    /// Retrieves the Value as a String. This does not 
    /// return an option just to make it quicker, and 
    /// it is thought to be used internally.
    pub fn to_string(&self)->String{
        match self {            
            PPLValue::PPLFloat(v) => v.to_string(),
            PPLValue::PPLInt(v) => v.to_string(),            
            PPLValue::PPLString(v)=>(**v).clone(),
            PPLValue::PPLNil => "".to_string(),
            PPLValue::PPLBool(v)=>{ if *v { "true".to_string() }else{"false".to_string()} }
            
        }
    }

    pub fn to_bool(&self)->bool{
        match self {
            PPLValue::PPLBool(v)=>{ *v},
            _ => panic!("Cannot transform type '{}' into 'bool'", self.ppl_type())
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
    #[should_panic]
    fn test_nil_to_f64() {        
        PPLValue::PPLNil.to_f64();
    }
    
    #[test]
    fn test_float_to_f64() {        
        let exp = 1.12312;
        let v = PPLValue::PPLFloat(exp);
        let found = v.to_f64();
        assert_eq!(exp,found);
    }
    
    #[test]
    fn test_int_to_f64() {        
        let exp = 121;
        let v = PPLValue::PPLInt(exp);
        let found = v.to_f64();
        assert_eq!(exp as f64,found);
    }
    
    #[test]
    fn test_string_to_f64_ok() {        
        
        let exp = "1".to_string();
        let v = PPLValue::PPLString(&exp);
        let found = v.to_f64();
        assert_eq!(1.0,found);

        let exp = "1.12312".to_string();
        let v = PPLValue::PPLString(&exp);
        let found = v.to_f64();
        assert_eq!(1.12312,found);

        let exp = "-1.12312".to_string();
        let v = PPLValue::PPLString(&exp);
        let found = v.to_f64();
        assert_eq!(-1.12312,found);
    }

    #[test]
    #[should_panic]
    fn test_string_to_f64_not_ok() {        
        
        let exp = "A1".to_string();
        PPLValue::PPLString(&exp).to_f64();
        

        let exp = "V1.12312".to_string();
        PPLValue::PPLString(&exp).to_f64();
        
        let exp = "1-1.12312".to_string();
        PPLValue::PPLString(&exp).to_f64();        
    }
    

    /*********/
    // TO i32
    /*********/
    #[test]
    #[should_panic]
    fn test_nil_to_i32() {        
        PPLValue::PPLNil.to_i32();
    }
    
    #[test]
    fn test_float_to_i32() {        

        // This gets floored (i.e. it ends up being 1123)
        let exp = 1123.812312;
        let v = PPLValue::PPLFloat(exp);
        let found = v.to_i32();
        assert_eq!(exp as i32,found);        

        // This becomes -1123
        let exp = -1123.812312;
        let v = PPLValue::PPLFloat(exp);
        let found = v.to_i32();
        assert_eq!(exp as i32,found);        
    }
    
    #[test]
    fn test_int_to_i32() {        
        let exp = 121;
        let v = PPLValue::PPLInt(exp);
        let found = v.to_i32();
        assert_eq!(exp,found);


        let exp = -0621;
        let v = PPLValue::PPLInt(exp);
        let found = v.to_i32();
        assert_eq!(exp,found);
    }
    
    #[test]
    fn test_string_to_i32_ok() {        
        
        let exp = "1".to_string();
        let v = PPLValue::PPLString(&exp);
        let found = v.to_i32();
        assert_eq!(1,found);

        let exp = "11".to_string();
        let v = PPLValue::PPLString(&exp);
        let found = v.to_i32();
        assert_eq!(11,found);

        let exp = "-31".to_string();
        let v = PPLValue::PPLString(&exp);
        let found = v.to_i32();
        assert_eq!(-31,found);
    }

    #[test]
    #[should_panic]
    fn test_string_to_i32_not_ok() {        
        
        let exp = "A1".to_string();
        PPLValue::PPLString(&exp).to_i32();
        

        let exp = "V1.12312".to_string();
        PPLValue::PPLString(&exp).to_i32();
        
        let exp = "1-1.12312".to_string();
        PPLValue::PPLString(&exp).to_i32();        

        let exp = "1.12312".to_string();
        let v = PPLValue::PPLString(&exp);
        let found = v.to_i32();
        assert_eq!(1,found);

        let exp = "-1.12312".to_string();
        let v = PPLValue::PPLString(&exp);
        let found = v.to_i32();
        assert_eq!(-1,found);
    }

    /*********/
    // TO String
    /*********/
    #[test]    
    fn test_nil_to_string() {        
        let v = PPLValue::PPLNil.to_string();
        assert_eq!(v,"");
    }
    
    #[test]
    fn test_float_to_string() {        

        // This gets floored (i.e. it ends up being 1123)
        let exp = 1123.812312;
        let v = PPLValue::PPLFloat(exp);
        let found = v.to_string();
        assert_eq!(exp.to_string(),found);        

        // This becomes -1123
        let exp = -1123.812312;
        let v = PPLValue::PPLFloat(exp);
        let found = v.to_string();
        assert_eq!(exp.to_string(),found);        
    }
    
    #[test]
    fn test_int_to_string() {        
        let exp = 121;
        let v = PPLValue::PPLInt(exp);
        let found = v.to_string();
        assert_eq!(exp.to_string(),found);


        let exp = -0621;
        let v = PPLValue::PPLInt(exp);
        let found = v.to_string();
        assert_eq!(exp.to_string(),found);
    }
    
    #[test]
    fn test_string_to_string() {        
        
        let exp = "1".to_string();
        let v = PPLValue::PPLString(&exp);
        let found = v.to_string();
        assert_eq!("1".to_string(),found);

        let exp = "11".to_string();
        let v = PPLValue::PPLString(&exp);
        let found = v.to_string();
        assert_eq!("11".to_string(),found);

        let exp = "-31".to_string();
        let v = PPLValue::PPLString(&exp);
        let found = v.to_string();
        assert_eq!("-31".to_string(),found);
    }

    
}
