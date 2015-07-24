use ast;
use lval;
use env;
use std::string;

pub struct SpecialForm{
    f: fn(ast::Ast, &env::Env)->lval::LVal,
    repr: string::String,
}


impl Clone for SpecialForm{
    fn clone(&self)->SpecialForm{
        SpecialForm{
            f:self.f,
            repr:self.repr.clone()
        }
    }
}

impl SpecialForm{
    pub fn new(f:fn(ast::Ast, &env::Env)->lval::LVal, repr:string::String)->SpecialForm{
        SpecialForm{
            f:f,
            repr:repr.clone(),
        }
    }
    pub fn apply(self, tree: ast::Ast, environment: &env::Env) -> lval::LVal {
        (self.f)(tree, environment)
    }
    pub fn to_string(&self)->string::String{
        self.repr.clone()
    }
}
