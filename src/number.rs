use crate::value_trait::ValueTrait;
use crate::values::Value;


pub type Number = f64;



impl ValueTrait for Number {
    
    fn to_string(&self) -> String {
        return format!("{}",self);
    }

    fn type_name(&self)->String{
        return format!("Number")
    }

    fn clone_to_value(&self)->Value{
        Value::Number(*self)
    }


    fn negate(&self)->Result<Value,String>{        
        Ok(Value::Number(-self))
    }

    fn add(&self, other: &Value)->Result<Value,String>{
        match other {
            Value::Number(v) => {
                Ok(Value::Number(self + v))
            },
            _ => Err(format!("Cannot add '{}' and '{}'", self.type_name(), other.type_name()))
        }        
    }

    fn subtract(&self, other: &Value)->Result<Value,String>{
        match other {
            Value::Number(v) => {
                Ok(Value::Number(self - v))
            },
            _ => Err(format!("Cannot subtract '{}' and '{}'", self.type_name(), other.type_name()))
        }
    }

    fn multiply(&self, other: &Value)->Result<Value,String>{
        match other {
            Value::Number(v) => {
                Ok(Value::Number(self * v))
            },
            _ => Err(format!("Cannot multiply '{}' and '{}'", self.type_name(), other.type_name()))
        }
    }

    fn divide(&self, other: &Value)->Result<Value,String>{
        match other {
            Value::Number(v) => {
                Ok(Value::Number(self /v ))
            },
            _ => Err(format!("Cannot divide '{}' by '{}'", self.type_name(), other.type_name()))
        }
    }

    fn compare_equal(&self, other: &Value)->Result<Value,String>{
        match other {
            Value::Number(v) => {
                Ok(Value::Bool(self == v))                
            },
            _ => Err(format!("Comparing '{}' with '{}'", self.type_name(), other.type_name()))
        }
    }

    fn greater(&self, other: &Value)->Result<Value,String>{
        match other {
            Value::Number(v) => {
                Ok(Value::Bool(self > v))                
            },
            _ => Err(format!("Comparing '{}' with '{}'", self.type_name(), other.type_name()))
        }
    }

    fn less(&self, other: &Value)->Result<Value,String>{
        match other {
            Value::Number(v) => {
                Ok(Value::Bool(self < v))                
            },
            _ => Err(format!("Comparing '{}' with '{}'", self.type_name(), other.type_name()))
        }
    }

    fn greater_equal(&self, other: &Value)->Result<Value,String>{
        match other {
            Value::Number(v) => {
                Ok(Value::Bool(self >= v))                
            },
            _ => Err(format!("Comparing '{}' with '{}'", self.type_name(), other.type_name()))
        }
    }

    fn less_equal(&self, other: &Value)->Result<Value,String>{
        match other {
            Value::Number(v) => {
                Ok(Value::Bool(self <= v))                
            },
            _ => Err(format!("Comparing '{}' with '{}'", self.type_name(), other.type_name()))
        }
    }

}


