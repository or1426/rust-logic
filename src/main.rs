extern crate linenoise;

mod lval;
mod ast;
mod env;

use lval::specialform;
use lval::lfunc;
use lval::ltype;

use std::boxed;

fn main() {
    let mut environment = env::Env::new();

    environment.add_val("1".to_string(), lval::LVal::Bool(true));
    environment.add_val("0".to_string(), lval::LVal::Bool(false));
    environment.add_val("bool_t".to_string(), lval::LVal::Type(ltype::LType::Bool));
    //environment.add_val("t_bool".to_string(), lval::LVal::Type(ltype::LType::Bool));

    environment.add_val("p".to_string(), lval::LVal::SpecialForm(specialform::SpecialForm::new(specialform::placeholder_fn, "p".to_string())));
    let e_cpy = environment.clone();
    environment.add_val("def".to_string(), lval::LVal::Func(lfunc::LFunc::Builtin(lfunc::builtinfn::BuiltinFn::new(lfunc::def_fn, vec![ltype::LType::PlaceHolder, ltype::LType::Unknown], ltype::LType::Unknown, &e_cpy))));

    environment.add_val("not".to_string(), lval::LVal::Func(lfunc::LFunc::Builtin(lfunc::builtinfn::BuiltinFn::new(lfunc::not_fn, vec![ltype::LType::Bool], ltype::LType::Bool, &e_cpy))));

    
    environment.add_val("::".to_string(), lval::LVal::Func(
        lfunc::LFunc::Builtin(
            lfunc::builtinfn::BuiltinFn::new(
                lfunc::func_type_fn, vec![ltype::LType::Array(boxed::Box::new(ltype::LType::Type)), ltype::LType::Type], ltype::LType::Type, &e_cpy))));
    environment.add_val("\\".to_string(), lval::LVal::SpecialForm(specialform::SpecialForm::new(specialform::lambda_fn, "\\".to_string())));
    loop {
        let val = linenoise::input(">>> ");
        match val {
            None => { break }
            Some(input) => {
                linenoise::history_add(&input[..]);
                match ast::Ast::new(input) {
                    Ok(a) => {
                        println!("<<< {} ", lval::LVal::new(a, &environment).eval(&mut environment).to_string());
                    },
                    Err(e) => println!("!!! {}", e),
                }                
            },
        }
    }
}
