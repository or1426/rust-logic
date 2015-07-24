use std::string;
use std::boxed;
use std::vec;

pub enum LType {
    Bool,
    List,
    Array(boxed::Box<LType>),
    Error,
    Type,
    Func((vec::Vec<LType>, boxed::Box<LType>)),
    SpecialForm,
    PlaceHolder,
    Unknown,
}

impl Clone for LType{
    fn clone(&self) -> LType{
        match self {
            &LType::Bool => LType::Bool,
            &LType::List => LType::List,
            &LType::Array(ref t) => LType::Array(t.clone()),
            &LType::Error => LType::Error,            
            &LType::Type => LType::Type,
            &LType::Func((ref sig, ref ret)) => LType::Func((sig.clone(), ret.clone())),
            &LType::SpecialForm => LType::SpecialForm,
            &LType::PlaceHolder => LType::PlaceHolder,
            &LType::Unknown => LType::Unknown,
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
            (&LType::Func(_), &LType::Func(_)) => true,
            (&LType::SpecialForm, &LType::SpecialForm) => true,
            (&LType::PlaceHolder, &LType::PlaceHolder) => true,
            (_, &LType::Unknown) => true,
            (&LType::Unknown, _) => true,
            _  => false,
        }
    }

    fn ne(&self, other:&LType)->bool{
        !self.eq(other)
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
            &LType::Func((ref sig, ref ret)) => {
                let string_vec: vec::Vec<string::String> = sig.iter().map(|t| t.to_string()).collect();
                format!("func({})->{}", string_vec.connect(", "), ret.to_string())
                },
            &LType::SpecialForm => "specialForm".to_string(),
            &LType::PlaceHolder => "placeHolder".to_string(),
            &LType::Unknown => "unknown".to_string(),
        }
    }
}
