mod ltype;
mod larray;
mod lfunc;
pub mod specialform;

use std::vec;
use std::result;
use std::string;
use std::boxed;

use ast;
use env;

pub enum LVal{
    Bool(bool),
    List(vec::Vec<LVal>),
    Array(larray::LArray),
    Error(string::String),
    Type(ltype::LType),
    SpecialForm(specialform::SpecialForm),
}

impl Clone for LVal{
    fn clone(&self) -> LVal{
        match self {
            &LVal::Bool(b) => LVal::Bool(b),
            &LVal::List(ref v) => LVal::List(v.clone()),
            &LVal::Array(ref a) => LVal::Array(a.clone()),
            &LVal::Error(ref s) => LVal::Error(s.clone()),
            &LVal::Type(ref t) => LVal::Type(t.clone()),
            &LVal::SpecialForm(ref f) => LVal::SpecialForm(f.clone()),
        }
    }
}

impl LVal{
    pub fn new(tree: ast::Ast, environment: &mut env::Env) -> LVal {
        match tree.clone() {
            ast::Ast::Token(token) => match environment.lookup(token.clone()) {
                LVal::Error(s) => return LVal::Error(format!("{}Environment lookup failed wnen attempting to pars\n", s)),
                x => return x,
            },
            ast::Ast::SubList(v) => {
                let lval_vec : vec::Vec<LVal> = v.into_iter().map(|a| LVal::new(a, environment)).collect();
                
                for element in lval_vec.iter(){
                    match element.clone() {
                        LVal::Error(s) => return LVal::Error(s),
                        LVal::SpecialForm(form) => return form.apply(tree, environment),
                        _ => (),
                    };
                };
                LVal::List(lval_vec)
            },
            ast::Ast::SubArray(v) => {
                let mut lval_vec : vec::Vec<LVal> = vec::Vec::new();
                                
                for element in v.into_iter().map(|a| LVal::new(a, environment)){
                    match element {
                        LVal::Error(s) => return LVal::Error(s),
                        value => lval_vec.push(value.clone()),
                    };
                };

                
                let t = if lval_vec.len() == 0 {
                    ltype::LType::Bool
                }else{
                    lval_vec[0].get_type()
                };

                for element in lval_vec.clone(){
                    if element.get_type() != t {
                        return LVal::Error("Array can only contain one type".to_string());
                    }
                }
                
                LVal::Array(larray::LArray::new(lval_vec, t))
            },

        }
    }

    pub fn eval(&self,environment: &mut env::Env) -> LVal{
        match self {
            &LVal::List(ref v) => if v.len() == 0 {
                self.clone()
            }else{
                LVal::List(v.iter().map(|value: &LVal| value.eval(environment)).collect())
            },
            _ => self.clone(),
        }
    }

    pub fn get_type(&self) -> ltype::LType{
        match(self){
            &LVal::Bool(_) => ltype::LType::Bool,
            &LVal::List(_) => ltype::LType::List,
            &LVal::Array(ref a) => ltype::LType::Array(boxed::Box::new(a.t.clone())),
            &LVal::Error(_) => ltype::LType::Error,
            &LVal::Type(_) => ltype::LType::Type,
            &LVal::SpecialForm(_) => ltype::LType::SpecialForm,
            
        }
    }

    pub fn to_string(&self) -> string::String {
        self.to_string_with_indent(0)
    }
    
    fn to_string_with_indent(&self, indent: i32) -> string::String {
        match self {
            &LVal::Bool(ref b) => match b {
                &true => "1".to_string(),
                &false => "0".to_string(),
            },
            &LVal::List(ref v) => {
                let mut s = "\n".to_string();
                for _ in 0..indent {
                    s.push_str(" ");
                }
                s.push_str("(");
                for element in v.iter() {
                    s.push_str(&element.to_string_with_indent(indent+1)[..]);
                    s.push_str(" ");
                }
                s.pop();
                s.push_str(")");
                s
            },

            &LVal::Array(ref a) => {
                let mut s = "\n".to_string();
                for _ in 0..indent {
                    s.push_str(" ");
                }
                s.push_str("[");
                for element in a.v.iter() {
                    s.push_str(&element.to_string_with_indent(indent+1)[..]);
                    s.push_str(" ");
                }
                s.pop();
                s.push_str("]");
                s
            },
            &LVal::Error(ref e) => format!("Error({})", e.to_string()),
            &LVal::Type(ref t) => format!("Type({})", t.to_string()),
            &LVal::SpecialForm(ref f) => f.to_string(),
        }
    }
}
