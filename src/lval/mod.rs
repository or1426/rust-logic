mod ltype;

use std::vec;
use std::result;
use std::string;

use ast;
use env;

pub enum LVal{
    Bool(bool),
    List(vec::Vec<LVal>),
    Type(ltype::LType),
}

impl Clone for LVal{
    fn clone(&self) -> LVal{
        match self {
            &LVal::Bool(b) => LVal::Bool(b),
            &LVal::List(ref v) => LVal::List(v.clone()),
            &LVal::Type(ref t) => LVal::Type(t.clone()),
        }
    }
}

impl LVal{
    pub fn new(tree: ast::Ast, environment: env::Env) -> result::Result<LVal, string::String>{
        match tree {
            ast::Ast::Token(token) => match environment.lookup(token.clone()) {
                Some(value) => Ok(value),
                None => Err(format!("Error: Failed to find token \"{}\"in environment", token))
            },
            ast::Ast::SubList(v) => {
                let mut lval_vec : vec::Vec<LVal> = vec::Vec::new();
                for element in v.into_iter().map(|a| LVal::new(a, environment.clone())){
                    match element {
                        Ok(value) => lval_vec.push(value.clone()),
                        Err(s) => return Err(s),
                    };
                };
                Ok(LVal::List(lval_vec))
            },
        }
    }

    pub fn get_type(self) -> ltype::LType{
        match(self){
            LVal::Bool(_) => ltype::LType::Bool,
            LVal::List(_) => ltype::LType::List,
            LVal::Type(_) => ltype::LType::Type,
        }
    }
}
