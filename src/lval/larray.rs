use std::vec;
use lval::ltype;
use lval;

pub struct LArray{
    pub v: vec::Vec<lval::LVal>,
    pub t: ltype::LType,
}

impl LArray{
    pub fn new(v: vec::Vec<lval::LVal>, t: ltype::LType) -> LArray {
        LArray{
            v:v,
            t:t,
        }
    }

}
impl Clone for LArray{
    fn clone(&self) -> LArray {
        LArray::new(self.v.clone(), self.t.clone())
    }
}
