pub enum LType{
    Bool,
    List,
    Type,
}

impl Clone for LType{
    fn clone(&self) -> LType{
        match self {
            &LType::Bool => LType::Bool,
            &LType::List => LType::List,
            &LType::Type => LType::Type,            
        }
    }
}
