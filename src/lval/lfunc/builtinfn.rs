use lval;
use lval::lfunc;
use lval::ltype;
use env;

use std::vec;
use std::option;

pub struct BuiltinFn{
    f: fn(&lval::LVal, &mut env::Env) -> lval::LVal,
    sig: vec::Vec<ltype::LType>,
    ret_type: ltype::LType,
    environment: env::Env,
    applied_args: vec::Vec<lval::LVal>,
}

impl Clone for BuiltinFn{
    fn clone(&self) -> BuiltinFn{
        BuiltinFn {
            f: self.f,
            sig: self.sig.clone(),
            ret_type: self.ret_type.clone(),
            environment:self.environment.clone(),
            applied_args:self.applied_args.clone(),
        }
    }
}

impl BuiltinFn{
    pub fn new(f:fn(&lval::LVal, &mut env::Env) -> lval::LVal, sig:vec::Vec<ltype::LType>, ret_type: ltype::LType, environment:&env::Env) -> BuiltinFn{
        BuiltinFn{
            f:f,
            sig:sig,
            ret_type:ret_type,
            environment:environment.clone(),
            applied_args:vec::Vec::new(),
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
            (self.f)(&lval::LVal::List(self.applied_args.clone()), environment)
                
        }else{
            lval::LVal::Func(lfunc::LFunc::Builtin(self.clone()))
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
                    lval::LVal::Func(lfunc::LFunc::Builtin(new_func))
                }else{
                    lval::LVal::Error(format!("TypeError: Expected {} found {} in applied arg", required_type.to_string(), arg.get_type().to_string()))
                },
            None => lval::LVal::Error("Tried to apply arg to fuction with no required args".to_string()),
        }
    }
}
