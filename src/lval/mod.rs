use std::vec;

pub enum LVal{
    Bool(bool),
    SubList(vec::Vec<LVal>),
}

impl Clone for LVal{
    fn clone(&self) -> LVal{
        match self {
            &LVal::Bool(bool) => LVal::Bool(bool),
            &LVal::SubList(ref v) => LVal::SubList(v.clone()),
        }
    }
}