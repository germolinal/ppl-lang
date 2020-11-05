use crate::values::*;

impl <'a>Value<'a> {

    /// Transforms a Value into a Int. Meant to be used
    /// externally through the API
    pub fn to_ppl_number(&self)->Result<Value,String>{
        match self {
            // Numbers are easy
            Value::Number(v) => Ok(Value::Number(*v as f64)),
            
            /*
            // Let's try with string
            Value::(v)=>{
                let s = v.parse::<i32>();
                match s {
                    Ok(v) => Ok(PPLValue::PPLInt(v)),
                    Err(_) => Err(format!("Cannot transform string {} into '{}'", v,PPLValue::PPLInt(1).ppl_type()))
                }                
            }
            */
            // Everything else panics
            _ => panic!("Cannot transform type {} into 'Number'", self.typename())
        }
    }

}


/***********/
/* TESTING */
/***********/

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_pplint() {
        
    }
}