use lval;
use std::vec;
use lval::ltype;
use env;

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

    pub fn eval(&self, args: &lval::LVal, environment: &mut env::Env) -> lval::LVal {
        match args.clone() {
            lval::LVal::List(v) => {
                for (sig_element_type, arg) in self.get_sig().iter().zip(v.iter()){
                    if sig_element_type.clone() != arg.get_type(){
                        return lval::LVal::Error(format!("Argument {} is not of type {}!",arg.to_string(), sig_element_type.to_string()));
                    }
                }
                (self.f)(args, environment)
            },
            _ => lval::LVal::Error("eval arg must be list".to_string()),
        }
    }

    fn get_top_arg_type(&self) -> option::Option<ltype::LType> {
        match self.sig.last() {
            None => None,
            Some(t) => Some((*t).clone()),
        }
    }

    pub fn apply_arg(&self, arg: lval::LVal) -> lval::LVal {
        match self.get_top_arg_type(){
            Some(required_type) =>
                if required_type == arg.get_type(){
                    let mut new_func = self.clone();
                    new_func.sig.pop();
                    new_func.applied_args.push(arg);
                    lval::LVal::Func(LFunc::Builtin(new_func))
                }else{
                    lval::LVal::Error(format!("TypeError! Expected {} found {} in applied arg", required_type.to_string(), arg.get_type().to_string()))
                },
            None => lval::LVal::Error("Tried to apply arg to fuction with no required args".to_string()),
        }
    }
}

pub enum LFunc{
    Builtin(BuiltinFn),
}

impl Clone for LFunc{
    fn clone(&self) -> LFunc {
        match self{
            &LFunc::Builtin(ref f) => LFunc::Builtin(f.clone()),
        }
    }
}

impl LFunc{
    pub fn eval(&self, args: &lval::LVal, environment: &mut env::Env) -> lval::LVal{
        match self {
            &LFunc::Builtin(ref b) => b.eval(args, environment),
        }
    }

    pub fn get_sig(&self)->vec::Vec<ltype::LType>{
        match self {
            &LFunc::Builtin(ref b) => b.get_sig(),
        }
    }
    pub fn get_ret_type(&self)->ltype::LType{
        match self {
            &LFunc::Builtin(ref b) => b.get_ret_type(),
        }
    }
    pub fn apply_arg(&self, arg: lval::LVal) -> lval::LVal{
        match self {
            &LFunc::Builtin(ref b) => b.apply_arg(arg),
        }
    }

    
}

pub fn def_fn(value: &lval::LVal, environment: &mut env::Env) -> lval::LVal {

    match environment.pop_frame(){
        None => lval::LVal::Error("Environment should contain some stack frames when using def".to_string()),
        Some(frame) =>
        {
            let return_value = match value {
                &lval::LVal::List(ref v) => if v.len() == 2 {
                    match (v[0].clone(), v[1].clone()) {
                        (lval::LVal::PlaceHolder(s), value) => {
                            environment.add_val(s.clone(), value.clone());
                            value
                        },
                        _ =>lval::LVal::Error("def must be called on a placeholder and another thing".to_string())
                    }
                }else{
                    lval::LVal::Error("def must be called with two arguments".to_string())
                },
                _ => lval::LVal::Error("def must be called on a list".to_string()),
            };

            environment.push_frame(frame);
            return_value
        },
    }
}
