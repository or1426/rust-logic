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

/*
pub fn def_func(tree: ast::Ast, environment: &mut env::Env) -> lval::LVal {
    match tree {
        ast::Ast::SubList(v) => if v.len() == 3 {
            match v[1].clone() {
                ast::Ast::Token(token) => {
                    let value = lval::LVal::new(v[2].clone(), environment);
                    environment.add_val(token, value.clone());
                    return value;                                        
                },
                _ => lval::LVal::Error("defs first parameter must be a simple token".to_string()),
            }
        }else{
            lval::LVal::Error("def must be part of a list with three elements".to_string())
        },
        _ => lval::LVal::Error("def must be called in a list".to_string()),
    }
}
*/
