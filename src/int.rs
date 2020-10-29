impl PPLValue{

    /// Transforms a Value into a Int. Meant to be used
    /// externally through the API
    pub fn to_pplint(&self)->Result<PPLValue,String>{
        match self {
            // Numbers are easy
            PPLValue::PPLFloat(v) => Ok(PPLValue::PPLInt(*v as i32)),
            PPLValue::PPLInt(v) => Ok(PPLValue::PPLInt(*v)),
            // Let's try with string
            PPLValue::PPLString(v)=>{
                let s = v.parse::<i32>();
                match s {
                    Ok(v) => Ok(PPLValue::PPLInt(v)),
                    Err(_) => Err(format!("Cannot transform string {} into '{}'", v,PPLValue::PPLInt(1).ppl_type()))
                }                
            }
            _ => panic!("Cannot transform type {} into '{}'", self.ppl_type(), PPLValue::PPLInt(1).ppl_type())
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