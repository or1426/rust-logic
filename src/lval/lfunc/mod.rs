pub mod builtinfn;

use lval;
use std::vec;
use lval::ltype;
use env;

use std::option;

pub enum LFunc{
    Builtin(builtinfn::BuiltinFn),
}

impl Clone for LFunc{
    fn clone(&self) -> LFunc {
        match self{
            &LFunc::Builtin(ref f) => LFunc::Builtin(f.clone()),
        }
    }
}

impl LFunc{
    pub fn eval(&self, environment: &mut env::Env) -> lval::LVal{
        match self {
            &LFunc::Builtin(ref b) => b.eval(environment),
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

    pub fn get_top_arg_type(&self) -> option::Option<ltype::LType> {
        match self {
            &LFunc::Builtin(ref b) => b.get_top_arg_type(),
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
