use ast;
use lval;
use env;
use lval::lfunc;
use lval::ltype;

use std::string;
use std::vec;
use std::boxed;

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

pub fn placeholder_fn(tree: ast::Ast, environment: &env::Env) -> lval::LVal {
    match tree {
        ast::Ast::SubList(v) => if v.len() == 2 {
            match v[1].clone() {
                ast::Ast::Token(token) => lval::LVal::PlaceHolder(token),
                _ => lval::LVal::Error("placeholders first parameter must be a simple token".to_string()),
            }
        }else{
            lval::LVal::Error("placeholder must be part of a list with two elements".to_string())
        },
        _ => lval::LVal::Error("placeholder must be called in a list".to_string()),
    }
}

pub fn lambda_fn(tree: ast::Ast, environment: &env::Env) -> lval::LVal {
    let mut tmp_env = environment.clone();
    match tree {
        ast::Ast::SubList(v) => if v.len() == 4 {
            match (v[1].clone(), v[2].clone(), v[3].clone()) {
                (type_parameter, ast::Ast::SubList(list_vec), code) => {
                    match lval::LVal::new(type_parameter, &tmp_env).eval(&mut tmp_env){
                        lval::LVal::Type(ltype::LType::Func((v,ret_type_box))) => {
                            let mut formals: vec::Vec<string::String> = vec::Vec::new();
                            for sub_ast in list_vec.iter() {
                                match sub_ast {
                                    &ast::Ast::Token(ref t) => formals.push(t.clone()),
                                    _ => return lval::LVal::Error("lambda parameter variables must be simple tokens".to_string()),
                                }
                            }
                            lval::LVal::Func(lfunc::LFunc::LFn(lfunc::lfn::LFn::new(code, environment.clone(), v.clone(), *ret_type_box, formals)))
                        },
                        _ => lval::LVal::Error("lambda parameter first parameter must be a function type".to_string()),
                    }                    
                },
                _ => lval::LVal::Error("lambda parameters must be a type and two lists".to_string()),
            }
        } else {
            lval::LVal::Error("lambda must be part of a list with four elements".to_string())
        },
        _ => lval::LVal::Error("lambda must be part of a list with four elements".to_string()),        
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
