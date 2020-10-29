impl PPLValue{
    
    /// Transforms a Value into a Float. Meant to be used
    /// externally through the API
    pub fn to_pplfloat(&self) -> Result<PPLValue,String> {
        match self {
            // Numbers are easy
            PPLValue::PPLFloat(v)=>Ok(PPLValue::PPLFloat(*v)),
            PPLValue::PPLInt(v)=>Ok(PPLValue::PPLFloat(*v as f64)),
            // Let's try with string
            PPLValue::PPLString(v)=>{
                let s = v.parse::<f64>();
                match s {
                    Ok(v) => Ok(PPLValue::PPLFloat(v)),
                    Err(_) => Err(format!("Cannot transform string {} into '{}'", v,PPLValue::PPLFloat(1.2).ppl_type()))
                }                
            }
            _ => Err(format!("Cannot transform type '{}' into '{}'", self.ppl_type(), PPLValue::PPLFloat(1.2).ppl_type()))
        }
    }

    

    
}