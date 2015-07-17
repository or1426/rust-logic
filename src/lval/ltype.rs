use std::string;

pub enum LType{
    Bool,
    List,
    Error,
    Type,
}

impl Clone for LType{
    fn clone(&self) -> LType{
        match self {
            &LType::Bool => LType::Bool,
            &LType::List => LType::List,
            &LType::Error => LType::Error,            
            &LType::Type => LType::Type,            
        }
    }
}

impl LType{
    pub fn to_string(&self) -> string::String{
        match self {
            &LType::Bool => "bool".to_string(),
            &LType::List => "list".to_string(),
            &LType::Error => "error".to_string(),            
            &LType::Type => "type".to_string(),            
        }
    }
}
