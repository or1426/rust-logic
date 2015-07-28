pub mod builtinfn;
pub mod lfn;

use lval;
use std::vec;
use lval::ltype;
use env;

use std::boxed;
use std::option;

pub enum LFunc{
    Builtin(builtinfn::BuiltinFn),
    LFn(lfn::LFn),
}

impl Clone for LFunc{
    fn clone(&self) -> LFunc {
        match self{
            &LFunc::Builtin(ref f) => LFunc::Builtin(f.clone()),
            &LFunc::LFn(ref f) => LFunc::LFn(f.clone()),
        }
    }
}

impl LFunc{
    pub fn eval(&self, environment: &mut env::Env) -> lval::LVal{
        match self {
            &LFunc::Builtin(ref b) => b.eval(environment),
            &LFunc::LFn(ref f) => f.eval(environment),
        }
    }

    pub fn get_sig(&self)->vec::Vec<ltype::LType>{
        match self {
            &LFunc::Builtin(ref b) => b.get_sig(),
            &LFunc::LFn(ref f) => f.get_sig(),
        }
    }
    pub fn get_ret_type(&self)->ltype::LType{
        match self {
            &LFunc::Builtin(ref b) => b.get_ret_type(),
            &LFunc::LFn(ref f) => f.get_ret_type(),
        }
    }

    pub fn get_top_arg_type(&self) -> option::Option<ltype::LType> {
        match self {
            &LFunc::Builtin(ref b) => b.get_top_arg_type(),
            &LFunc::LFn(ref f) => f.get_top_arg_type(),
        }
    }
    
    pub fn apply_arg(&self, arg: lval::LVal) -> lval::LVal{
        match self {
            &LFunc::Builtin(ref b) => b.apply_arg(arg),
            &LFunc::LFn(ref f) => f.apply_arg(arg),
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


pub fn func_type_fn(value: &lval::LVal, environment: &mut env::Env) -> lval::LVal {
    match value {
        &lval::LVal::List(ref v) => match (v[0].clone(), v[1].clone()) {
            (lval::LVal::Array(a), lval::LVal::Type(t)) => if a.t == ltype::LType::Type {
                let mut type_vec : vec::Vec<ltype::LType> = vec::Vec::new();
                for element in a.v.iter(){
                    match element {
                        &lval::LVal::Type(ref t) => type_vec.push(t.clone()),
                        _ => (),
                    }
                }
                lval::LVal::Type(ltype::LType::Func((type_vec, boxed::Box::new(t.clone()))))
            } else {
                lval::LVal::Error("first parameter of func_def must be an array of types".to_string())
            },
            _ => lval::LVal::Error("func_def must be called on an array and a type".to_string()),
        },
        _ => lval::LVal::Error("func_def must be called on a list".to_string()),
    }
}

