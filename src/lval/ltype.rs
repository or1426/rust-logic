use std::string;
use std::boxed;

pub enum LType {
    Bool,
    List,
    Array(boxed::Box<LType>),
    Error,
    Type,
    SpecialForm,
}

impl Clone for LType{
    fn clone(&self) -> LType{
        match self {
            &LType::Bool => LType::Bool,
            &LType::List => LType::List,
            &LType::Array(ref t) => LType::Array(t.clone()),
            &LType::Error => LType::Error,            
            &LType::Type => LType::Type,
            &LType::SpecialForm => LType::SpecialForm,            
        }
    }
}

impl PartialEq for LType{
    fn eq(&self, other: &LType) -> bool {
        match (self, other){
            (&LType::Bool, &LType::Bool) => true,
            (&LType::List, &LType::List) => true,
            (&LType::Array(ref t1), &LType::Array(ref t2)) => t1==t2,
            (&LType::Error, &LType::Error) => true,
            (&LType::Type, &LType::Type) => true,
            (&LType::SpecialForm, &LType::SpecialForm) => true,
            _  => false,
        }
    }
}

impl LType{
    pub fn to_string(&self) -> string::String{
        match self {
            &LType::Bool => "bool".to_string(),
            &LType::List => "list".to_string(),
            &LType::Array(ref t) => format!("array({})", t.to_string()),
            &LType::Error => "error".to_string(),            
            &LType::Type => "type".to_string(),
            &LType::SpecialForm => "specialForm".to_string(),
        }
    }
}
