use ast;
use lval;
use lval::lfunc;
use lval::ltype;
use env;

use std::vec;
use std::option;
use std::string;

pub struct LFn{
    tree: ast::Ast,
    environment: env::Env,
    sig: vec::Vec<ltype::LType>,
    ret_type: ltype::LType,
    applied_args: vec::Vec<lval::LVal>,
    formals:vec::Vec<string::String>,
}


impl Clone for LFn{
    fn clone(&self)->LFn{
        LFn{
            tree:self.tree.clone(),
            environment:self.environment.clone(),
            sig:self.sig.clone(),
            ret_type:self.ret_type.clone(),
            applied_args:self.applied_args.clone(),
            formals:self.formals.clone()
        }
    }
}

impl LFn{
    pub fn new(tree: ast::Ast, environment: env::Env, sig: vec::Vec<ltype::LType>, ret_type: ltype::LType, formals: vec::Vec<string::String>) -> LFn{
        LFn{
            tree: tree,
            environment: environment,
            sig:sig,
            ret_type:ret_type,
            applied_args:vec::Vec::new(),
            formals:formals,
        }
    }

    pub fn get_sig(&self)->vec::Vec<ltype::LType>{
        self.sig.clone()
    }
    pub fn get_ret_type(&self)->ltype::LType{
        self.ret_type.clone()
    }

    pub fn eval(&self, environment: &mut env::Env) -> lval::LVal {
        if self.sig.len() == 0 {
            environment.push_empty_frame();
            for (formal, applied_arg) in self.formals.iter().zip(self.applied_args.iter()){
                environment.add_val(formal.clone(), applied_arg.clone());
            }
            let ret = lval::LVal::new(self.tree.clone(), environment).eval(environment);
            environment.pop_frame();
            ret
        }else{            
            lval::LVal::Func(lfunc::LFunc::LFn(self.clone()))
        }
    }

    pub fn get_top_arg_type(&self) -> option::Option<ltype::LType> {
        match self.sig.first() {
            None => None,
            Some(t) => Some((*t).clone()),
        }
    }

    pub fn apply_arg(&self, arg: lval::LVal) -> lval::LVal {
        match self.get_top_arg_type(){
            Some(required_type) =>
                if required_type == arg.get_type(){
                    let mut new_func = self.clone();
                    new_func.sig.remove(0);
                    new_func.applied_args.push(arg);
                    lval::LVal::Func(lfunc::LFunc::LFn(new_func))
                }else{
                    lval::LVal::Error(format!("TypeError: Expected {} found {} in applied arg", required_type.to_string(), arg.get_type().to_string()))
                },
            None => lval::LVal::Error("Tried to apply arg to fuction with no required args".to_string()),
        }
    }
}
